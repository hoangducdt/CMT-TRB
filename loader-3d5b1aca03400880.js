// Trunk initializer: hiển thị tiến độ tải file .wasm trên màn hình loader.
export default function initializer() {
  const setStatus = (text) => {
    const el = document.getElementById('loader-status');
    if (el) el.textContent = text;
  };
  const setProgress = (pct) => {
    const bar = document.getElementById('loader-bar-fill');
    if (bar) bar.style.width = Math.min(100, pct) + '%';
  };
  const mb = (bytes) => (bytes / 1048576).toFixed(1);

  return {
    onStart: () => setStatus('Loading application...'),
    onProgress: ({ current, total }) => {
      if (!total) {
        setStatus(`Loading application... ${mb(current)} MB`);
        return;
      }
      // 90% cho phần tải, 10% còn lại cho bước khởi tạo giao diện
      setProgress((current / total) * 90);
      setStatus(`Loading application... ${mb(current)} / ${mb(total)} MB`);
    },
    onComplete: () => setProgress(90),
    onSuccess: () => {},
    onFailure: (error) => setStatus(`Load failed: ${error}`),
  };
}
