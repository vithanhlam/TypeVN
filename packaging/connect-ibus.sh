#!/bin/sh
set -e
ROOT="${SNAP:-/snap/typevn/current}"
COMP="${HOME}/.local/share/ibus/component"
DROP="${HOME}/.config/systemd/user/org.freedesktop.IBus.session.GNOME.service.d"
mkdir -p "${COMP}" "${DROP}"
sed "s|@EXEC@|${ROOT}/libexec/ibus-typevn|g" \
  "${ROOT}/share/ibus/component/typevn.xml.in" \
  > "${COMP}/typevn.xml"
printf '%s\n' "[Service]" "Environment=IBUS_COMPONENT_PATH=${COMP}:/usr/share/ibus/component" \
  > "${DROP}/typevn.conf"
systemctl --user daemon-reload >/dev/null 2>&1 || true
systemctl --user restart org.freedesktop.IBus.session.GNOME.service >/dev/null 2>&1 || true
ibus write-cache >/dev/null 2>&1 || true
ibus engine typevn >/dev/null 2>&1 || true
echo "TypeVN connected to IBus. Add it in Settings → Keyboard if needed."
