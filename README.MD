<p align="center">
  <img src="TYPE.PNG" alt="TypeVN — cửa sổ cài đặt" width="720">
</p>

<h1 align="center">TypeVN</h1>

<p align="center">
  Bộ gõ tiếng Việt cho Ubuntu / GNOME (Wayland)<br>
  Telex · VNI · IBus · Rust<br>
  <strong>vithanhlam</strong>
</p>

<p align="center">
  <a href="https://github.com/vithanhlam/TypeVN/releases/tag/v0.1.6"><img src="https://img.shields.io/badge/release-v0.1.6-0ea5e9" alt="release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-22c55e" alt="license"></a>
  <a href="CHANGELOG.MD"><img src="https://img.shields.io/badge/changelog-0.1.6-6366f1" alt="changelog"></a>
</p>

Engine xử lý phím viết bằng **Rust** (đồng bộ, không queue, không mạng). Gắn vào Ubuntu qua **IBus**. Cửa sổ cài đặt GTK4/libadwaita. Không Electron, không Node.js, không Python trên đường gõ.

---

## Cài đặt

Chọn **một** cách. Sau khi cài, phải chọn nguồn nhập **TypeVN** (không phải layout XKB tên “Vietnamese”).

### Cách 1 — Gói `.deb` (nhanh)

1. Tải file: [ibus-typevn_0.1.6_amd64.deb](https://github.com/vithanhlam/TypeVN/releases/download/v0.1.6/ibus-typevn_0.1.6_amd64.deb)
2. Mở Terminal trong thư mục vừa tải, chạy:

```bash
sudo apt install ./ibus-typevn_0.1.6_amd64.deb
```

3. **Settings → Keyboard → Input Sources → Add → Vietnamese → TypeVN**  
   hoặc nhấn **Super+Space** đến khi thấy TypeVN / VN.
4. Mở app **TypeVN** (hoặc `typevn-settings`) để chọn Telex/VNI, tự sửa dấu, tự bật lúc đăng nhập.

### Cách 2 — Snap (Ubuntu Software / snapcraft)

```bash
sudo snap install typevn --classic
typevn.connect-ibus
```

Rồi thêm nguồn nhập **TypeVN** như trên. Gỡ: `sudo snap remove typevn`.

### Cách 3 — Từ mã nguồn (dành cho đang phát triển)

Cần `git`, `cargo` (Rust) và IBus (GNOME Ubuntu đã có sẵn).

```bash
git clone https://github.com/vithanhlam/TypeVN.git
cd TypeVN
./setup.sh
```

Script sẽ build, đăng ký IBus, bật TypeVN trên GNOME. Không cần sudo (cài vào `~/.local`).

### Gỡ cài đặt

- Bản `.deb`: `sudo apt remove ibus-typevn`
- Bản snap: `sudo snap remove typevn`
- Bản `./setup.sh`: `./scripts/uninstall-dev.sh`

---

## Dùng hàng ngày

1. Chọn nguồn nhập **TypeVN** (Super+Space).
2. Gõ Telex hoặc VNI như bảng dưới.
3. Tùy chỉnh trong **TypeVN** (ảnh phía trên): bảng gõ, bảng mã, tự sửa dấu, tự chạy khi đăng nhập.

### Telex (mặc định)

| Gõ | Ra |
|---|---|
| `aa` `aw` `ee` `oo` `ow` `uw` `dd` | â ă ê ô ơ ư đ |
| `s` `f` `r` `x` `j` | sắc huyền hỏi ngã nặng |
| `z` | bỏ dấu |

Ví dụ: `tieengs` → **tiếng** · `Vieetj` → **Việt** · `cuar` → **của** · `giups` → **giúp** · `dduwowcj` → **được**

### VNI

`1–5` dấu · `6` â/ê/ô · `7` ă/ơ · `8` ư · `9` đ · `0` bỏ dấu  

Ví dụ: `d9uo7c5` → **được** · `tie6ng1` → **tiếng**

### Phím tắt

Trên Ubuntu GNOME (Wayland), `Ctrl+Shift` và `Ctrl+Alt+T` thường **không tới bộ gõ** (GNOME/Terminal giữ trước). Dùng các phím dưới đây:

| Phím | Việc |
|---|---|
| **Shift+Space** | Việt / Anh |
| **Ctrl+Shift+1** | Telex |
| **Ctrl+Shift+2** | VNI |
| **Ctrl+Alt+M** | Đổi Telex ↔ VNI |
| **Ctrl+Alt+C** | Đổi bảng mã |
| **Ctrl+Alt+R** | Bật/tắt tự sửa dấu |

Khi bấm đúng, gõ hiện dòng **TypeVN · Telex** (hoặc VNI / Việt / Anh). Trong app **TypeVN** có công tắc **Bật phím tắt** (tắt hẳn nếu không cần), công tắc gõ tiếng Việt, đổi Telex/VNI, và nút **Đổi** để gán phím tắt (GNOME nhận phím dù IBus không thấy Shift+Space).

Cấu hình lưu tại `~/.config/typevn/config` (engine chỉ đọc lúc khởi động, không đọc file mỗi phím).

---

## Kiến trúc

```text
typevn-core       →  logic tiếng Việt (không IBus, không GUI)
ibus-typevn       →  adapter IBus mỏng
GNOME / Wayland
typevn-settings   →  GTK4, không nằm trên hot path gõ phím
```

Mỗi phím xử lý đồng bộ. Mục tiêu: **&lt; 1 ms** / keypress thông thường. Không telemetry.

## Phát triển

```bash
cargo test -p typevn-core
cargo bench -p typevn-core
./scripts/build-deb.sh
```

Nhật ký phiên bản: [CHANGELOG.MD](CHANGELOG.MD)

## Giấy phép & đóng góp

TypeVN thuộc bản quyền **vithanhlam**, phát hành dưới [MIT License](LICENSE). Mọi người có thể tham gia phát triển qua [CONTRIBUTING.md](CONTRIBUTING.md) (issue, pull request).
