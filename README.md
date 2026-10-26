# TRB Helper (CMT-TRB)

Công cụ web tra cứu và hỗ trợ sửa chữa TIU dành cho nhóm **TRB-CMT – Intel Products Vietnam**: thông tin TIU / IPN, tìm channel fail từ test data, tra pin / channel và xem trực quan trên pinmap.

**Phiên bản:** 2.0.0 · **Build:** 2026-10-26

> Repo này chỉ chứa bản đã build để phát hành. Mã nguồn và dữ liệu gốc nằm ở repo phát triển riêng.

## Chức năng

| Mục | Mô tả |
| --- | --- |
| **Trang Chủ** | Thống kê số sản phẩm / thành viên / công cụ / IPN; lối tắt tới các công cụ và **Bản Đồ Module** (sơ đồ khu vực, tìm module theo tên, tự định vị khu TRB-CMT). |
| **TRB-CMT** | Danh sách thành viên: WWID, điện thoại, gửi mail, chat Teams, trang Workers. |
| **Thông Tin TIU** | Thông tin từng TIU: tên TIU trái/phải, sản phẩm chạy được, Part Number, Pogo Pin, Socket, linh kiện… Mở **pinmap đầy đủ** hoặc từng **group pin** đã tô sẵn. Linh kiện trùng IPN có thể bấm để xem chi tiết. |
| **Tìm Channel Fail** | Dán test data (nhiều Part) hoặc nạp file từ `\\ssfile1.ss.intel.com\cmtdata\prod\Lot_Operation`. Lấy FailData / Composite / StrgVal / FailPins / từ khoá tuỳ chọn, tự nhận diện sản phẩm từ TIU, tra ra Pin – Channel – Linh kiện – vị trí HiFix. Lọc theo sản phẩm → TIU → Site → Bin, loại trừ channel / data bằng mẫu `*`, sao chép danh sách pin theo site và xem pin fail trên pinmap. |
| **Công Cụ Pinmap** | Nạp file `.cfg`, `.xlsx` (CRES / MTP) hoặc `.pinmap`, tự xoay / chuẩn hoá toạ độ, xem bảng pin và vẽ pinmap. |
| **Trợ Giúp Pin Finder** | Dán danh sách từ web Pin Finder, lấy cột Position, sao chép danh sách và tô màu trên pinmap của sản phẩm đã chọn. |
| **Dữ Liệu Channels** | Chọn sản phẩm / site / channel → bảng Pin, Linh kiện, HiFix và tô màu trên pinmap. |
| **Dữ Liệu Pins** | Chọn sản phẩm / site / pin → bảng Channel, Linh kiện, HiFix và tô màu trên pinmap. |
| **Thông Tin IPN** | Danh sách IPN thường dùng (Pogo Pin, socket, vật tư…) và các TIU dùng chung. |
| **Web Thường Dùng** | Lối tắt tới các trang nội bộ hay dùng. |
| **Trợ Giúp & Hướng Dẫn** | Hướng dẫn từng bước (có ảnh, bấm để phóng to) cho từng công cụ. |
| **Thông Tin** | Thông tin tool owner, tiến độ công cụ và lịch sử phiên bản. |

### Bản đồ pin trực quan

Dùng chung cho mọi công cụ:

- Tô màu pin được chọn, thêm pin tuỳ chọn, xoá màu; sao chép danh sách pin đang tô.
- Lật ngang / dọc, xoay trái / phải, tăng / giảm khoảng cách pin, zoom bằng con lăn, kéo để di chuyển.
- **Chèn văn bản** (nhấp đúp): font (kể cả font hệ thống trên Chrome / Edge 103+), cỡ, màu, **B** / *I* / U, căn lề, khung nền / viền.
- **Vẽ hình**: đường thẳng nhiều đoạn (có mũi tên), đa giác đều, elip, đường tròn; chọn / di chuyển / chỉnh điểm.
- **Lưu ảnh** PNG và **lưu dữ liệu** `.pinmap` (gồm cả văn bản và hình đã vẽ; mở lại được bằng Công Cụ Pinmap).

### Khác

- 4 ngôn ngữ: Tiếng Việt, English, 中文, Bahasa Melayu.
- Giao diện tối / sáng; ngôn ngữ, theme và trạng thái thu gọn sidebar được ghi nhớ.
- Dùng được trên điện thoại (menu dạng ngăn kéo).

## Sử dụng

Mở trang GitHub Pages của repo này. Lần đầu trình duyệt tải khoảng 12 MB (phần lớn là ảnh TIU / IPN dạng WebP, đã nằm sẵn trong ứng dụng); các lần sau dùng bộ nhớ đệm.

Yêu cầu trình duyệt hỗ trợ WebAssembly: Chrome / Edge / Firefox / Safari bản hiện hành. Mọi xử lý (đọc test data, đọc Excel…) diễn ra ngay trên máy, không gửi dữ liệu đi đâu.

## Nội dung repo

| File | Vai trò |
| --- | --- |
| `index.html` | Trang chính và màn hình tải |
| `trb-helper-*_bg.wasm` | Toàn bộ ứng dụng (giao diện, dữ liệu, ảnh) dạng WebAssembly |
| `trb-helper-*.js` | Đoạn nạp WebAssembly (sinh tự động) |
| `loader-*.js` | Hiển thị tiến độ tải |
| `favicon-*.svg` | Biểu tượng trên tab trình duyệt |
| `.github/workflows/deploy.yml` | Tự động phát hành lên GitHub Pages mỗi khi push lên `main` |
| `.nojekyll` | Tắt xử lý Jekyll của GitHub Pages |

Phần `*` trong tên file là mã băm, đổi mỗi lần build để trình duyệt không dùng nhầm bản cũ.

## Phát hành

1. Lần đầu: **Settings → Pages → Build and deployment → Source: GitHub Actions**.
2. Mỗi lần push lên `main`, workflow **Deploy GitHub Pages** tự đưa bản mới lên; xem tiến trình ở tab **Actions**.

## Tác giả

Hoàng Anh Đức – [@hoangducdt](https://github.com/hoangducdt) · MIT License
