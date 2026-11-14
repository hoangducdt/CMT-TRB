// navigator.hid giả lập cho app desktop: WebView2 không có hộp chọn thiết bị WebHID, nên app tự mở
// PICkit Serial Analyzer bằng HID gốc của Windows và chuyển tiếp qua IPC. Chỉ cài đúng phần API mà
// tab TIU EEPROM dùng (requestDevice / getDevices / open / close / sendReport / inputreport / disconnect).
(() => {
  if (!window.ipc) return;
  const pending = new Map();
  let seq = 0;
  const call = (op, arg = '') => new Promise((resolve, reject) => {
    const id = ++seq;
    pending.set(id, [resolve, reject]);
    window.ipc.postMessage(id + '|' + op + '|' + arg);
  });
  const hex = (u8) => Array.from(u8, (b) => b.toString(16).padStart(2, '0')).join('');
  const unhex = (s) => {
    const out = new Uint8Array(s.length / 2);
    for (let i = 0; i < out.length; i++) out[i] = parseInt(s.substr(i * 2, 2), 16);
    return out;
  };

  class Device extends EventTarget {
    constructor(info) {
      super();
      this.vendorId = info.vendorId;
      this.productId = info.productId;
      this.productName = info.productName || '';
      this.collections = [];
      this.opened = false;
      this.oninputreport = null;
    }
    async open() { await call('open'); this.opened = true; }
    async close() { if (this.opened) await call('close'); this.opened = false; }
    async forget() { await this.close(); }
    async sendReport(reportId, data) {
      const u8 = data instanceof Uint8Array ? data
        : ArrayBuffer.isView(data) ? new Uint8Array(data.buffer, data.byteOffset, data.byteLength)
        : new Uint8Array(data);
      await call('send', reportId + ':' + hex(u8));
    }
  }

  const hid = new EventTarget();
  let device = null;
  hid.onconnect = null;
  hid.ondisconnect = null;
  hid.requestDevice = async () => {
    const info = await call('find');
    if (!info) return [];
    if (!device) device = new Device(info);
    return [device];
  };
  hid.getDevices = async () => (device ? [device] : []);

  window.__trbHid = {
    done(id, ok, value) {
      const p = pending.get(id);
      if (!p) return;
      pending.delete(id);
      if (ok) p[0](value);
      else p[1](new DOMException(String(value), 'NetworkError'));
    },
    input(h) {
      if (!device) return;
      const bytes = unhex(h);
      const e = new Event('inputreport');
      e.device = device;
      e.reportId = 0;
      e.data = new DataView(bytes.buffer);
      device.dispatchEvent(e);
      if (typeof device.oninputreport === 'function') device.oninputreport(e);
    },
    gone() {
      if (!device) return;
      const d = device;
      d.opened = false;
      device = null;
      const e = new Event('disconnect');
      e.device = d;
      hid.dispatchEvent(e);
      if (typeof hid.ondisconnect === 'function') hid.ondisconnect(e);
    },
  };
  Object.defineProperty(Navigator.prototype, 'hid', { get: () => hid, configurable: true });
})();
