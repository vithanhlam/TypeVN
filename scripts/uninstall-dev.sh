#!/usr/bin/env bash
set -euo pipefail

rm -f "${HOME}/.local/share/ibus/component/typevn.xml"
rm -f "${HOME}/.local/libexec/ibus-typevn"
rm -f "${HOME}/.local/bin/typevn-settings"
rm -f "${HOME}/.local/share/applications/typevn.desktop"
rm -f "${HOME}/.config/autostart/typevn.desktop"
rm -f "${HOME}/.config/systemd/user/org.freedesktop.IBus.session.GNOME.service.d/typevn.conf"
rmdir "${HOME}/.config/systemd/user/org.freedesktop.IBus.session.GNOME.service.d" 2>/dev/null || true

sudo rm -f /usr/share/ibus/component/typevn.xml 2>/dev/null || true
sudo rm -f /usr/libexec/ibus-typevn 2>/dev/null || true

systemctl --user daemon-reload >/dev/null 2>&1 || true
if systemctl --user list-unit-files org.freedesktop.IBus.session.GNOME.service >/dev/null 2>&1; then
    systemctl --user reset-failed org.freedesktop.IBus.session.GNOME.service >/dev/null 2>&1 || true
    systemctl --user restart org.freedesktop.IBus.session.GNOME.service >/dev/null 2>&1 || true
fi

ibus write-cache >/dev/null 2>&1 || true

if command -v gsettings >/dev/null 2>&1; then
    python3 - <<'PY'
import ast, subprocess
try:
    sources = ast.literal_eval(subprocess.check_output(
        ["gsettings", "get", "org.gnome.desktop.input-sources", "sources"], text=True).strip())
except Exception:
    raise SystemExit(0)
sources = [s for s in sources if s != ("ibus", "typevn")]
if not sources:
    sources = [("xkb", "us")]
subprocess.check_call(["gsettings", "set", "org.gnome.desktop.input-sources", "sources",
                       repr(sources).replace('"', "'")])
subprocess.check_call(["gsettings", "set", "org.gnome.desktop.input-sources", "current", "0"])
PY
fi

echo "typevn: uninstalled."
