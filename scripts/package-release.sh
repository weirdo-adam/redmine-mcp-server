#!/usr/bin/env sh
set -eu

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd "$SCRIPT_DIR/.." && pwd)
VERSION=$(node -p "JSON.parse(require('fs').readFileSync('$REPO_ROOT/package.json', 'utf8')).version")
PACKAGE_NAME="redmine-mcp-server-${VERSION}"
DIST_DIR="$REPO_ROOT/dist"
STAGE_DIR="$DIST_DIR/$PACKAGE_NAME"
ARCHIVE="$DIST_DIR/$PACKAGE_NAME.tar.gz"

if [ -z "$VERSION" ]; then
  echo "Unable to read version from package.json" >&2
  exit 1
fi

cd "$REPO_ROOT"

scripts/check.sh

mkdir -p "$DIST_DIR"
rm -rf "$STAGE_DIR" "$ARCHIVE" "$ARCHIVE.sha256"
mkdir -p "$STAGE_DIR/server/src" "$STAGE_DIR/docs" "$STAGE_DIR/scripts" "$STAGE_DIR/bin"

cp "$REPO_ROOT/package.json" "$STAGE_DIR/package.json"
cp "$REPO_ROOT/README.md" "$STAGE_DIR/README.md"
cp "$REPO_ROOT/README.zh-CN.md" "$STAGE_DIR/README.zh-CN.md"
cp "$REPO_ROOT/LICENSE" "$STAGE_DIR/LICENSE"
cp "$REPO_ROOT/SECURITY.md" "$STAGE_DIR/SECURITY.md"
cp "$REPO_ROOT/CONTRIBUTING.md" "$STAGE_DIR/CONTRIBUTING.md"
cp "$REPO_ROOT/server/index.js" "$STAGE_DIR/server/index.js"
cp "$REPO_ROOT/server/src/"*.js "$STAGE_DIR/server/src/"
cp "$REPO_ROOT/docs/"*.md "$STAGE_DIR/docs/"
cp "$REPO_ROOT/scripts/install-local.sh" "$STAGE_DIR/scripts/install-local.sh"

{
  printf '%s\n' '#!/usr/bin/env sh'
  printf '%s\n' 'SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)'
  printf '%s\n' 'REPO_ROOT=$(CDPATH= cd "$SCRIPT_DIR/.." && pwd)'
  printf '%s\n' 'exec node "$REPO_ROOT/server/index.js" "$@"'
} > "$STAGE_DIR/bin/redmine-mcp-server"

chmod +x "$STAGE_DIR/server/index.js" \
  "$STAGE_DIR/bin/redmine-mcp-server" \
  "$STAGE_DIR/scripts/install-local.sh"

tar -C "$DIST_DIR" -czf "$ARCHIVE" "$PACKAGE_NAME"
shasum -a 256 "$ARCHIVE" > "$ARCHIVE.sha256"

echo "Created:"
echo "  $ARCHIVE"
echo "  $ARCHIVE.sha256"
