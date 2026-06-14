#!/usr/bin/env sh
set -eu

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd "$SCRIPT_DIR/.." && pwd)
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' "$REPO_ROOT/Cargo.toml" | head -n 1)
PACKAGE_NAME="redmine-mcp-server-${VERSION}"
DIST_DIR="$REPO_ROOT/dist"
STAGE_DIR="$DIST_DIR/$PACKAGE_NAME"
ARCHIVE="$DIST_DIR/$PACKAGE_NAME.tar.gz"

if [ -z "$VERSION" ]; then
  echo "Unable to read version from Cargo.toml" >&2
  exit 1
fi

cd "$REPO_ROOT"

scripts/check.sh

mkdir -p "$DIST_DIR"
rm -rf "$STAGE_DIR" "$ARCHIVE" "$ARCHIVE.sha256"
mkdir -p "$STAGE_DIR"

cp "$REPO_ROOT/Cargo.toml" "$STAGE_DIR/Cargo.toml"
cp "$REPO_ROOT/Cargo.lock" "$STAGE_DIR/Cargo.lock"
cp "$REPO_ROOT/README.md" "$STAGE_DIR/README.md"
cp "$REPO_ROOT/README.zh-CN.md" "$STAGE_DIR/README.zh-CN.md"
cp "$REPO_ROOT/LICENSE" "$STAGE_DIR/LICENSE"
cp "$REPO_ROOT/SECURITY.md" "$STAGE_DIR/SECURITY.md"
cp "$REPO_ROOT/CONTRIBUTING.md" "$STAGE_DIR/CONTRIBUTING.md"
cp -R "$REPO_ROOT/docs" "$STAGE_DIR/docs"
cp -R "$REPO_ROOT/scripts" "$STAGE_DIR/scripts"
cp -R "$REPO_ROOT/src" "$STAGE_DIR/src"
cp -R "$REPO_ROOT/tests" "$STAGE_DIR/tests"

tar -C "$DIST_DIR" -czf "$ARCHIVE" "$PACKAGE_NAME"
shasum -a 256 "$ARCHIVE" > "$ARCHIVE.sha256"

echo "Created:"
echo "  $ARCHIVE"
echo "  $ARCHIVE.sha256"
