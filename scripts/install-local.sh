#!/usr/bin/env sh
set -eu

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd "$SCRIPT_DIR/.." && pwd)
INSTALL_DIR=${REDMINE_MCP_INSTALL_DIR:-"$HOME/.local/share/redmine-mcp-server"}

if ! command -v node >/dev/null 2>&1; then
  echo "node is required. Install Node.js 18.17 or newer." >&2
  exit 1
fi

NODE_MAJOR=$(node -e 'process.stdout.write(process.versions.node.split(".")[0])')
if [ "$NODE_MAJOR" -lt 18 ]; then
  echo "Node.js 18.17 or newer is required." >&2
  exit 1
fi

mkdir -p "$INSTALL_DIR/server/src" "$INSTALL_DIR/docs" "$INSTALL_DIR/scripts" "$INSTALL_DIR/bin"

cp "$REPO_ROOT/server/index.js" "$INSTALL_DIR/server/index.js"
cp "$REPO_ROOT/server/src/"*.js "$INSTALL_DIR/server/src/"
cp "$REPO_ROOT/package.json" "$INSTALL_DIR/package.json"
cp "$REPO_ROOT/README.md" "$INSTALL_DIR/README.md"
cp "$REPO_ROOT/README.zh-CN.md" "$INSTALL_DIR/README.zh-CN.md"
cp "$REPO_ROOT/LICENSE" "$INSTALL_DIR/LICENSE"
cp "$REPO_ROOT/docs/"*.md "$INSTALL_DIR/docs/"
cp "$REPO_ROOT/scripts/install-local.sh" "$INSTALL_DIR/scripts/install-local.sh"

{
  printf '%s\n' '#!/usr/bin/env sh'
  printf 'exec node "%s/server/index.js" "$@"\n' "$INSTALL_DIR"
} > "$INSTALL_DIR/bin/redmine-mcp-server"
chmod +x "$INSTALL_DIR/bin/redmine-mcp-server" "$INSTALL_DIR/scripts/install-local.sh"

node --check "$INSTALL_DIR/server/index.js" >/dev/null
node --check "$INSTALL_DIR/server/src/redmine.js" >/dev/null
node --check "$INSTALL_DIR/server/src/tools.js" >/dev/null

cat <<EOF
Installed Redmine MCP server to:
  $INSTALL_DIR

Standalone command:
  $INSTALL_DIR/bin/redmine-mcp-server

Required environment:
  REDMINE_BASE_URL=https://redmine.example.com
  REDMINE_API_KEY=your-api-key

Claude/Codex/Zed custom-command entrypoint:
  command: $INSTALL_DIR/bin/redmine-mcp-server

For complete client examples, see:
  $INSTALL_DIR/docs/client-configuration.md
EOF
