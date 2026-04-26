#!/usr/bin/env bash
# =============================================================
# Sakaloka-Universe — Version Bump Script
#
# Cascading environment-based versioning: {prod}.{demo}.{sit}.{dev}
#
# Usage: ./scripts/bump-version.sh <env>
#   env: dev | sit | demo | prod
#
# Rules:
#   dev  → increment 4th digit, keep others
#   sit  → increment 3rd digit, reset 4th to 0
#   demo → increment 2nd digit, reset 3rd+4th to 0
#   prod → increment 1st digit, reset 2nd+3rd+4th to 0
# =============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION_FILE="$REPO_ROOT/VERSION"

if [ $# -lt 1 ]; then
    echo "Usage: $0 <dev|sit|demo|prod>"
    exit 1
fi

ENV="$1"

if [ ! -f "$VERSION_FILE" ]; then
    echo "0.0.0.0" > "$VERSION_FILE"
fi

CURRENT=$(cat "$VERSION_FILE" | tr -d '[:space:]')

IFS='.' read -r PROD DEMO SIT DEV <<< "$CURRENT"
PROD=${PROD:-0}
DEMO=${DEMO:-0}
SIT=${SIT:-0}
DEV=${DEV:-0}

case "$ENV" in
    dev)
        DEV=$((DEV + 1))
        ;;
    sit)
        SIT=$((SIT + 1))
        DEV=0
        ;;
    demo)
        DEMO=$((DEMO + 1))
        SIT=0
        DEV=0
        ;;
    prod)
        PROD=$((PROD + 1))
        DEMO=0
        SIT=0
        DEV=0
        ;;
    *)
        echo "Unknown environment: $ENV"
        echo "Usage: $0 <dev|sit|demo|prod>"
        exit 1
        ;;
esac

NEW_VERSION="$PROD.$DEMO.$SIT.$DEV"
CARGO_VERSION="$PROD.$DEMO.$SIT"
echo "$NEW_VERSION" > "$VERSION_FILE"

for toml in "$REPO_ROOT/apps/api/Cargo.toml" \
            "$REPO_ROOT/apps/venus/src-tauri/Cargo.toml" \
            "$REPO_ROOT/libs/core/Cargo.toml" \
            "$REPO_ROOT/libs/secure/Cargo.toml" \
            "$REPO_ROOT/libs/data/Cargo.toml"; do
    if [ -f "$toml" ]; then
        awk -v ver="$CARGO_VERSION" '!done && /^version = "/ { print "version = \"" ver "\""; done=1; next } 1' "$toml" > "$toml.tmp"
        mv "$toml.tmp" "$toml"
    fi
done

VENUS_PKG="$REPO_ROOT/apps/venus/package.json"
if [ -f "$VENUS_PKG" ]; then
    sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" "$VENUS_PKG"
    rm -f "$VENUS_PKG.bak"
fi

echo "$NEW_VERSION"
