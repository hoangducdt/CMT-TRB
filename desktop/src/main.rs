//! TRB Helper bản desktop — dự phòng dùng offline khi trang web không truy cập được.
//!
//! App có cửa sổ riêng (WebView2 có sẵn trên Windows 10/11, không mở trình duyệt). Toàn bộ bản web
//! (index.html, .js, .wasm) được nhúng vào exe lúc build và phục vụ qua giao thức nội bộ `trb`
//! (`http://trb.localhost/` trên Windows) → không cần mạng, không mở cổng mạng nào.
//! WebView2 không có hộp chọn thiết bị WebHID → tab TIU EEPROM dùng cầu nối HID gốc (`hid.rs`, `hid_shim.js`).
//! Dữ liệu trình duyệt (ngôn ngữ, theme, map EEPROM đã lưu…) nằm ở `%LOCALAPPDATA%\TRB Helper`.

#![cfg_attr(windows, windows_subsystem = "windows")]

use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Command;

use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::WindowBuilder;
use wry::http::{Request, Response, StatusCode, header};
use wry::{NewWindowResponse, WebContext, WebViewBuilder};

mod hid;
use hid::{Bridge, UserEvent};

include!(concat!(env!("OUT_DIR"), "/site.rs"));

/// `navigator.hid` giả lập (TIU EEPROM) — chạy trước mọi script của trang.
const HID_SHIM: &str = include_str!("hid_shim.js");

/// Trang đầu: giao thức tuỳ biến `trb` (Windows ánh xạ sang `http://trb.localhost/`).
#[cfg(windows)]
const START_URL: &str = "http://trb.localhost/";
#[cfg(not(windows))]
const START_URL: &str = "trb://localhost/";

fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let ipc_proxy = proxy.clone();
    let window = WindowBuilder::new()
        .with_title("TRB Helper")
        .with_inner_size(LogicalSize::new(1440.0, 900.0))
        .with_min_inner_size(LogicalSize::new(400.0, 500.0))
        .build(&event_loop)
        .expect("Không tạo được cửa sổ");

    let mut context = WebContext::new(Some(data_dir()));
    let builder = WebViewBuilder::new_with_web_context(&mut context)
        .with_custom_protocol("trb".into(), |_id, req| serve(&req))
        .with_url(START_URL)
        .with_devtools(false)
        .with_initialization_script(HID_SHIM)
        .with_ipc_handler(move |req| {
            let _ = ipc_proxy.send_event(UserEvent::Ipc(req.body().clone()));
        })
        // Link ra ngoài (mail, Teams, trang web…) mở bằng ứng dụng mặc định của hệ thống
        .with_navigation_handler(|url| {
            if is_internal(&url) {
                return true;
            }
            open_external(&url);
            false
        })
        .with_new_window_req_handler(|url, _| {
            if !is_internal(&url) {
                open_external(&url);
            }
            NewWindowResponse::Deny
        });

    #[cfg(not(target_os = "linux"))]
    let webview = builder.build(&window)?;
    #[cfg(target_os = "linux")]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        builder.build_gtk(window.default_vbox().expect("Không có vbox GTK"))?
    };

    let mut bridge = Bridge::default();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(UserEvent::Ipc(msg)) => {
                let js = bridge.handle(&msg, &proxy);
                let _ = webview.evaluate_script(&js);
            }
            Event::UserEvent(UserEvent::Script(js)) => {
                let _ = webview.evaluate_script(&js);
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    })
}

/// Trả file đã nhúng theo đường dẫn của yêu cầu.
fn serve(req: &Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let path = match req.uri().path() {
        "" | "/" => "/index.html",
        p => p,
    };
    match FILES.iter().find(|(p, _, _)| *p == path) {
        Some((_, ctype, body)) => Response::builder()
            .header(header::CONTENT_TYPE, *ctype)
            .header(header::CACHE_CONTROL, "no-cache")
            .body(Cow::Borrowed(*body))
            .unwrap_or_else(|_| not_found()),
        None => not_found(),
    }
}

fn not_found() -> Response<Cow<'static, [u8]>> {
    let mut r = Response::new(Cow::Borrowed(&b"Not found"[..]));
    *r.status_mut() = StatusCode::NOT_FOUND;
    r
}

/// URL thuộc chính app (trang nội bộ, blob / data của ảnh và tệp tải xuống).
fn is_internal(url: &str) -> bool {
    ["http://trb.localhost", "https://trb.localhost", "trb://", "blob:", "data:", "about:"].iter().any(|p| url.starts_with(p))
}

/// Mở URL bằng ứng dụng mặc định (trình duyệt, Outlook, Teams…).
fn open_external(url: &str) {
    // Chỉ chuyển các giao thức an toàn cho hệ điều hành
    if !["http://", "https://", "mailto:", "msteams:"].iter().any(|p| url.starts_with(p)) {
        return;
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let _ = Command::new("rundll32").args(["url.dll,FileProtocolHandler", url]).creation_flags(CREATE_NO_WINDOW).spawn();
    }
    #[cfg(not(windows))]
    {
        let _ = Command::new("xdg-open").arg(url).spawn();
    }
}

/// Thư mục dữ liệu của WebView2 (localStorage, cache…), giữ nguyên giữa các lần mở.
fn data_dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(std::env::temp_dir);
    let dir = base.join("TRB Helper");
    let _ = std::fs::create_dir_all(&dir);
    dir
}
