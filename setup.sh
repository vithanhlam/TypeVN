#!/usr/bin/env bash
# One-shot: build TypeVN and enable it as the current GNOME input method.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
"${ROOT}/scripts/setup-git-hooks.sh"
exec "${ROOT}/scripts/install-dev.sh"
