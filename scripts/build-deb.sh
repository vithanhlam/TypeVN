#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export PATH="${HOME}/.cargo/bin:${PATH}"
VER="${TYPEVN_VERSION:-0.1.11}"
ARCH="$(dpkg --print-architecture)"
PKG="ibus-typevn_${VER}_${ARCH}"
DIST="${ROOT}/dist"
STAGE="${DIST}/${PKG}"

echo "typevn: building release binaries..."
cargo build --release --manifest-path "${ROOT}/Cargo.toml" -p typevn-core -p ibus-typevn

rm -rf "${STAGE}"
mkdir -p \
  "${STAGE}/DEBIAN" \
  "${STAGE}/usr/libexec" \
  "${STAGE}/usr/share/ibus/component" \
  "${STAGE}/usr/bin" \
  "${STAGE}/usr/share/applications" \
  "${STAGE}/usr/share/doc/ibus-typevn"

install -m 755 "${ROOT}/target/release/ibus-typevn" "${STAGE}/usr/libexec/ibus-typevn"
install -m 755 "${ROOT}/scripts/typevn-settings" "${STAGE}/usr/bin/typevn-settings"
install -m 755 "${ROOT}/scripts/typevn-ctl" "${STAGE}/usr/bin/typevn-ctl"
sed "s|@EXEC@|/usr/libexec/ibus-typevn|g" \
  "${ROOT}/packaging/ibus/typevn.xml.in" \
  > "${STAGE}/usr/share/ibus/component/typevn.xml"
sed "s|@EXEC@|/usr/bin/typevn-settings|g" \
  "${ROOT}/packaging/typevn-settings.desktop.in" \
  > "${STAGE}/usr/share/applications/typevn.desktop"
gzip -9c "${ROOT}/CHANGELOG.MD" > "${STAGE}/usr/share/doc/ibus-typevn/changelog.gz"
cp "${ROOT}/README.MD" "${STAGE}/usr/share/doc/ibus-typevn/README"
if [[ -f "${ROOT}/TYPE.PNG" ]]; then
  mkdir -p "${STAGE}/usr/share/pixmaps"
  install -m 644 "${ROOT}/TYPE.PNG" "${STAGE}/usr/share/pixmaps/typevn.png"
fi

SIZE="$(du -sk "${STAGE}" | awk '{print $1}')"

cat > "${STAGE}/DEBIAN/control" <<EOF
Package: ibus-typevn
Version: ${VER}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: ibus, libibus-1.0-5, python3, python3-gi, gir1.2-gtk-4.0, gir1.2-adw-1
Maintainer: vithanhlam <vithanhlamseo@gmail.com>
Homepage: https://github.com/vithanhlam/TypeVN
Installed-Size: ${SIZE}
Description: TypeVN Vietnamese input method for IBus
 Fast Telex/VNI IME for Ubuntu GNOME (Wayland).
EOF

cat > "${STAGE}/DEBIAN/postinst" <<'EOF'
#!/bin/sh
set -e
if command -v ibus >/dev/null 2>&1; then
  ibus write-cache >/dev/null 2>&1 || true
fi
exit 0
EOF
chmod 755 "${STAGE}/DEBIAN/postinst"

cat > "${STAGE}/DEBIAN/prerm" <<'EOF'
#!/bin/sh
set -e
exit 0
EOF
chmod 755 "${STAGE}/DEBIAN/prerm"

cat > "${STAGE}/DEBIAN/postrm" <<'EOF'
#!/bin/sh
set -e
if command -v ibus >/dev/null 2>&1; then
  ibus write-cache >/dev/null 2>&1 || true
fi
exit 0
EOF
chmod 755 "${STAGE}/DEBIAN/postrm"

mkdir -p "${DIST}"
dpkg-deb --root-owner-group --build "${STAGE}" "${DIST}/${PKG}.deb"
echo "typevn: wrote ${DIST}/${PKG}.deb"
dpkg-deb --info "${DIST}/${PKG}.deb"
