#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export PATH="${HOME}/.cargo/bin:${PATH}"

BIN_SRC="${ROOT}/target/release/ibus-typevn"
BIN_DST="${HOME}/.local/libexec/ibus-typevn"
COMP_DIR="${HOME}/.local/share/ibus/component"
XML_DST="${COMP_DIR}/typevn.xml"
DROPIN_DIR="${HOME}/.config/systemd/user/org.freedesktop.IBus.session.GNOME.service.d"
SYS_COMP="/usr/share/ibus/component"
COMPONENT_PATH="${COMP_DIR}:${SYS_COMP}"

echo "typevn: building release..."
cargo build --release --manifest-path "${ROOT}/Cargo.toml" -p typevn-core -p ibus-typevn

if [[ ! -x "${BIN_SRC}" ]]; then
    echo "typevn: build failed, missing ${BIN_SRC}" >&2
    exit 1
fi

mkdir -p "$(dirname "${BIN_DST}")" "${COMP_DIR}" "${DROPIN_DIR}" \
    "${HOME}/.local/bin" "${HOME}/.local/share/applications"
install -m 755 "${BIN_SRC}" "${BIN_DST}"
install -m 755 "${ROOT}/scripts/typevn-settings" "${HOME}/.local/bin/typevn-settings"
install -m 755 "${ROOT}/scripts/typevn-ctl" "${HOME}/.local/bin/typevn-ctl"
sed "s|@EXEC@|${HOME}/.local/bin/typevn-settings|g" \
    "${ROOT}/packaging/typevn-settings.desktop.in" \
    > "${HOME}/.local/share/applications/typevn.desktop"
sed "s|@EXEC@|${BIN_DST}|g" "${ROOT}/packaging/ibus/typevn.xml.in" > "${XML_DST}"

cat > "${DROPIN_DIR}/typevn.conf" <<EOF
[Service]
Environment=IBUS_COMPONENT_PATH=${COMPONENT_PATH}
EOF

systemctl --user daemon-reload

export IBUS_COMPONENT_PATH="${COMPONENT_PATH}"
ibus write-cache >/dev/null 2>&1 || true

if systemctl --user list-unit-files org.freedesktop.IBus.session.GNOME.service >/dev/null 2>&1; then
    systemctl --user reset-failed org.freedesktop.IBus.session.GNOME.service >/dev/null 2>&1 || true
    systemctl --user restart org.freedesktop.IBus.session.GNOME.service >/dev/null 2>&1 || true
else
    IBUS_COMPONENT_PATH="${COMPONENT_PATH}" ibus-daemon --panel disable --replace --daemonize || true
fi

sleep 1

python3 - <<'PY'
import ast
import subprocess

def gs_get(key):
    return subprocess.check_output(
        ["gsettings", "get", "org.gnome.desktop.input-sources", key],
        text=True,
    ).strip()

def gs_set(key, value):
    subprocess.check_call(["gsettings", "set", "org.gnome.desktop.input-sources", key, value])

try:
    sources = ast.literal_eval(gs_get("sources"))
except Exception:
    sources = [("xkb", "us")]

wanted = ("ibus", "typevn")
if wanted not in sources:
    sources.append(wanted)
    gs_set("sources", repr(sources).replace('"', "'"))

idx = sources.index(wanted)
gs_set("current", str(idx))
try:
    gs_set("mru-sources", repr([wanted] + [s for s in sources if s != wanted]).replace('"', "'"))
except Exception:
    pass
print(f"typevn: GNOME input source enabled (index {idx})")
PY

ibus engine typevn >/dev/null 2>&1 || true

if gsettings list-keys org.freedesktop.ibus.general >/dev/null 2>&1; then
    gsettings set org.freedesktop.ibus.general preload-engines "['typevn']" >/dev/null 2>&1 || true
fi

mkdir -p "${HOME}/.config/autostart"
install -m 644 "${ROOT}/packaging/typevn-autostart.desktop" \
    "${HOME}/.config/autostart/typevn.desktop"

if command -v im-config >/dev/null 2>&1; then
    im-config -n ibus >/dev/null 2>&1 || true
fi

echo "typevn: installed. Mở cài đặt: typevn-settings  (hoặc tìm 'TypeVN' trong app grid)"
echo "TypeVN sẽ tự chọn lại sau khi đăng nhập."
echo "Done."
