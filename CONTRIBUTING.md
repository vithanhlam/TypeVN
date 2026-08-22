# Đóng góp TypeVN

Cảm ơn bạn quan tâm TypeVN. Mọi người đều có thể tham gia phát triển.

## Cách tham gia

Sau khi clone, bật git hook (giữ author commit là người dùng git local):

```bash
./scripts/setup-git-hooks.sh
```

1. Mở **issue** mô tả lỗi hoặc đề xuất (tiếng Việt hoặc tiếng Anh đều được).
2. Fork repo, tạo branch, sửa code, chạy test:

   ```bash
   cargo test -p typevn-core
   ```

3. Gửi **pull request** ngắn gọn, giải thích *vì sao* cần thay đổi.

## Đóng góp macro sửa lỗi

Nếu chỉ cần thêm một cặp thay thế chính xác, không cần sửa Rust. Thêm vào
[`data/macros.json`](data/macros.json):

```json
{
  "Loix": "Lỗi",
  "loix": "lỗi"
}
```

Khóa là chuỗi phím người dùng gõ và giá trị là chuỗi TypeVN sẽ commit khi
người dùng kết thúc từ bằng Space, Enter hoặc dấu câu. Giữ nguyên chữ hoa/
chữ thường bằng cách thêm từng biến thể cần thiết. Không thêm các thay thế
mơ hồ như `toi` → `tôi`, vì chúng có thể làm hỏng tên riêng, tiếng Anh hoặc
username. Các sửa liên quan đến luật Telex/VNI nên đi vào `src/repair.rs` và
phải có test hồi quy.

Người dùng cũng có thể thêm macro cá nhân tại:

```text
~/.config/typevn/macros.json
```

Macro cá nhân có cùng định dạng và được ưu tiên hơn danh sách mặc định.

## Giấy phép đóng góp

Bằng việc gửi PR, bạn đồng ý cấp đóng góp dưới [MIT License](LICENSE).
Commit trên repo chính chỉ ghi author **vithanhlam**.

## Phạm vi phù hợp

- Sửa logic gõ Telex/VNI, IBus adapter, app cài đặt GTK.
- Test, benchmark, tài liệu, bản dịch.
- Không thêm telemetry, quảng cáo, hay phụ thuộc nặng không cần thiết.

## Liên hệ

vithanhlamseo@gmail.com
