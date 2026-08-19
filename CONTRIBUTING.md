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

## Giấy phép đóng góp

Bằng việc gửi PR, bạn đồng ý cấp đóng góp dưới [MIT License](LICENSE).
Commit trên repo chính chỉ ghi author **vithanhlam**.

## Phạm vi phù hợp

- Sửa logic gõ Telex/VNI, IBus adapter, app cài đặt GTK.
- Test, benchmark, tài liệu, bản dịch.
- Không thêm telemetry, quảng cáo, hay phụ thuộc nặng không cần thiết.

## Liên hệ

vithanhlamseo@gmail.com
