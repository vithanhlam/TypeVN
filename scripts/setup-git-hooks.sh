#!/usr/bin/env bash
# Enable repo hooks that strip AI Co-authored-by lines from commits.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
git -C "${ROOT}" config core.hooksPath githooks
echo "typevn: git hooks enabled (githooks/) — extra Co-authored-by trailers will be removed on commit."
