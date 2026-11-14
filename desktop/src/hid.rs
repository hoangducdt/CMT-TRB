//! Cầu nối HID cho tab TIU EEPROM: mở PICkit Serial Analyzer (04D8:0036) bằng HID gốc (`hidapi`),
//! nhận lệnh từ `hid_shim.js` qua IPC và đẩy input report ngược lại trang bằng `evaluate_script`.

use hidapi::HidApi;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::thread;
use tao::event_loop::EventLoopProxy;

/// PICkit Serial Analyzer (khớp `tiueeprom/pksa.rs`).
const VENDOR_ID: u16 = 0x04D8;
const PRODUCT_ID: u16 = 0x0036;

/// Sự kiện gửi về vòng lặp chính (nơi giữ webview).
pub enum UserEvent {
    /// Tin nhắn IPC từ trang: `id|op|arg`.
    Ipc(String),
    /// Đoạn script cần chạy trong trang.
    Script(String),
}

struct Session {
    tx: Sender<Vec<u8>>,
    stop: Arc<AtomicBool>,
}

#[derive(Default)]
pub struct Bridge {
    api: Option<HidApi>,
    session: Option<Session>,
}

impl Bridge {
    /// Xử lý một lệnh từ trang; trả script phản hồi.
    pub fn handle(&mut self, msg: &str, proxy: &EventLoopProxy<UserEvent>) -> String {
        let mut parts = msg.splitn(3, '|');
        let id = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);
        let op = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("");
        let result = match op {
            "find" => self.find(),
            "open" => self.open(proxy).map(|()| "true".to_string()),
            "close" => {
                self.close();
                Ok("true".into())
            }
            "send" => self.send(arg).map(|()| "true".to_string()),
            _ => Err(format!("Lệnh không hỗ trợ: {op}")),
        };
        match result {
            Ok(v) => format!("window.__trbHid&&window.__trbHid.done({id},true,{v})"),
            Err(e) => format!("window.__trbHid&&window.__trbHid.done({id},false,{})", js_str(&e)),
        }
    }

    fn api(&mut self) -> Result<&mut HidApi, String> {
        if self.api.is_none() {
            self.api = Some(HidApi::new().map_err(|e| e.to_string())?);
        }
        self.api.as_mut().ok_or_else(|| "HID không khả dụng".to_string())
    }

    /// Thông tin PICkit đang cắm (JSON) hoặc `null`.
    fn find(&mut self) -> Result<String, String> {
        let api = self.api()?;
        api.refresh_devices().map_err(|e| e.to_string())?;
        let found = api.device_list().find(|d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID);
        Ok(match found {
            Some(d) => format!(
                "{{\"vendorId\":{VENDOR_ID},\"productId\":{PRODUCT_ID},\"productName\":{}}}",
                js_str(d.product_string().unwrap_or("PICkit Serial Analyzer"))
            ),
            None => "null".into(),
        })
    }

    fn open(&mut self, proxy: &EventLoopProxy<UserEvent>) -> Result<(), String> {
        if self.session.is_some() {
            return Ok(());
        }
        let dev = self.api()?.open(VENDOR_ID, PRODUCT_ID).map_err(|e| e.to_string())?;
        let (tx, rx) = channel::<Vec<u8>>();
        let stop = Arc::new(AtomicBool::new(false));
        let stop2 = stop.clone();
        let proxy = proxy.clone();
        // Luồng I/O riêng: ghi các report chờ gửi, đọc input report (chờ tối đa 5 ms mỗi vòng)
        thread::spawn(move || {
            let mut buf = [0u8; 65];
            while !stop2.load(Ordering::Relaxed) {
                while let Ok(out) = rx.try_recv() {
                    if dev.write(&out).is_err() {
                        let _ = proxy.send_event(UserEvent::Script("window.__trbHid&&window.__trbHid.gone()".into()));
                        return;
                    }
                }
                match dev.read_timeout(&mut buf, 5) {
                    Ok(0) => {}
                    Ok(n) => {
                        let js = format!("window.__trbHid&&window.__trbHid.input(\"{}\")", hex(&buf[..n]));
                        let _ = proxy.send_event(UserEvent::Script(js));
                    }
                    Err(_) => {
                        // Rút cáp hoặc thiết bị lỗi
                        let _ = proxy.send_event(UserEvent::Script("window.__trbHid&&window.__trbHid.gone()".into()));
                        return;
                    }
                }
            }
        });
        self.session = Some(Session { tx, stop });
        Ok(())
    }

    fn close(&mut self) {
        if let Some(s) = self.session.take() {
            s.stop.store(true, Ordering::Relaxed);
        }
    }

    /// `arg` = `reportId:hex`; hidapi cần byte đầu là report ID.
    fn send(&mut self, arg: &str) -> Result<(), String> {
        let (rid, data) = arg.split_once(':').ok_or("Report không hợp lệ")?;
        let mut out = vec![rid.parse::<u8>().map_err(|e| e.to_string())?];
        out.extend(unhex(data).ok_or("Report không hợp lệ")?);
        match &self.session {
            Some(s) => s.tx.send(out).map_err(|_| {
                self.session = None;
                "Thiết bị đã ngắt kết nối".to_string()
            }),
            None => Err("Thiết bị chưa được mở".into()),
        }
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    s.len().is_multiple_of(2).then_some(())?;
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok()).collect()
}

/// Chuỗi JS an toàn (chỉ dùng cho thông báo lỗi / tên thiết bị).
fn js_str(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 || c == '<' || c == '>' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
