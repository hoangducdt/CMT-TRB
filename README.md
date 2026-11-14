# TRB Helper (CMT-TRB)

Công cụ web tra cứu và hỗ trợ sửa chữa TIU dành cho nhóm **TRB-CMT – Intel Products Vietnam**: thông tin TIU / IPN, tìm channel fail từ test data, tra pin / channel và xem trực quan trên pinmap.

**Phiên bản:** 2.1.0 · **Build:** 2026-11-14

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
| **Thông Tin** | Thông tin tool owner, tiến độ công cụ và lịch sử phiên bản. |

### Bố cục chung

- Tiêu đề tab luôn ghim ở trên; nút **ⓘ** bên phải mở / đóng panel thông tin của tab với các mục **Thông tin**, **Hướng dẫn** (từng bước, có ảnh, bấm để phóng to) và **Nhật ký** (TIU EEPROM).
- Ở các tab dạng thẻ (TRB-CMT, Thông Tin TIU, Thông Tin IPN, Web Thường Dùng), bấm ảnh hoặc nút ⓘ của thẻ để xem chi tiết ở panel bên phải, bấm lại để ẩn; các nút icon trên thẻ mở nhanh mail, Teams, pinmap, trang web…
- Tiến trình đang chạy hiện thành thanh tiến trình dưới tiêu đề; trạng thái của tab (kết nối PICkit, số pin, chú giải lịch ca…) nằm giữa thanh chân trang.

### Bản đồ pin trực quan

Dùng chung cho mọi công cụ, hiển thị như một tab (thanh công cụ trên tiêu đề, số pin ở chân trang):

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

## Bản desktop (dự phòng offline)

Khi trang web không truy cập được, dùng bản desktop cho Windows 10 / 11: bấm nút tải xuống (icon) ở góc phải tiêu đề Trang Chủ (hoặc tải `TRB-Helper-Desktop.zip` trong mục Releases của repo này), giải nén rồi mở `TRB Helper.exe`.

- Ứng dụng có cửa sổ riêng, chứa sẵn toàn bộ trang, không cần mạng và không cần cài đặt (dùng Microsoft Edge WebView2 có sẵn trong Windows).
- Đủ các chức năng như bản web, kể cả **TIU EEPROM** (kết nối PICkit Serial Analyzer trực tiếp qua USB).
- Cài đặt và map EEPROM đã lưu nằm ở `%LOCALAPPDATA%\TRB Helper`.

## Tác giả

Hoàng Anh Đức – [@hoangducdt](https://github.com/hoangducdt) · MIT License
