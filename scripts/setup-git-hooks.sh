#!/usr/bin/env bash
# Enable the repository's optional commit hooks.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
git -C "${ROOT}" config core.hooksPath githooks
echo "typevn: git hooks enabled (githooks/). Contributor attribution is preserved."
