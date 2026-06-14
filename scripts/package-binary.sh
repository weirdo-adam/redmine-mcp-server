#!/usr/bin/env sh
set -eu

if [ "$#" -ne 3 ]; then
  echo "Usage: scripts/package-binary.sh <target> <os> <arch>" >&2
  exit 1
fi

TARGET="$1"
OS="$2"
ARCH="$3"

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd "$SCRIPT_DIR/.." && pwd)
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' "$REPO_ROOT/Cargo.toml" | head -n 1)
BINARY_NAME="redmine-mcp-server"
DIST_DIR="$REPO_ROOT/dist"
PACKAGE_NAME="$BINARY_NAME-$VERSION-$OS-$ARCH"
STAGE_DIR="$DIST_DIR/$PACKAGE_NAME"
ARCHIVE="$DIST_DIR/$PACKAGE_NAME.tar.gz"
EXE_SUFFIX=""

if [ -z "$VERSION" ]; then
  echo "Unable to read version from Cargo.toml" >&2
  exit 1
fi

if [ "$OS" = "windows" ]; then
  EXE_SUFFIX=".exe"
fi

cd "$REPO_ROOT"

cargo build --release --target "$TARGET"

mkdir -p "$DIST_DIR"
rm -rf "$STAGE_DIR" "$ARCHIVE" "$ARCHIVE.sha256"
mkdir -p "$STAGE_DIR"

cp "$REPO_ROOT/target/$TARGET/release/$BINARY_NAME$EXE_SUFFIX" "$STAGE_DIR/$BINARY_NAME$EXE_SUFFIX"
cp "$REPO_ROOT/README.md" "$STAGE_DIR/README.md"
cp "$REPO_ROOT/LICENSE" "$STAGE_DIR/LICENSE"

tar -C "$STAGE_DIR" -czf "$ARCHIVE" .

if command -v shasum >/dev/null 2>&1; then
  shasum -a 256 "$ARCHIVE" > "$ARCHIVE.sha256"
else
  sha256sum "$ARCHIVE" > "$ARCHIVE.sha256"
fi

echo "Created:"
echo "  $ARCHIVE"
echo "  $ARCHIVE.sha256"
