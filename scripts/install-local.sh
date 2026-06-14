#!/usr/bin/env sh
set -eu

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd "$SCRIPT_DIR/.." && pwd)
INSTALL_DIR=${REDMINE_MCP_INSTALL_DIR:-"$HOME/.local/share/redmine-mcp-server"}

cd "$REPO_ROOT"
cargo build --release

mkdir -p "$INSTALL_DIR/bin" "$INSTALL_DIR/docs"

cp "$REPO_ROOT/target/release/redmine-mcp-server" "$INSTALL_DIR/bin/redmine-mcp-server"
cp "$REPO_ROOT/README.md" "$INSTALL_DIR/README.md"
cp "$REPO_ROOT/README.zh-CN.md" "$INSTALL_DIR/README.zh-CN.md"
cp "$REPO_ROOT/LICENSE" "$INSTALL_DIR/LICENSE"
cp "$REPO_ROOT/docs/"*.md "$INSTALL_DIR/docs/"
chmod +x "$INSTALL_DIR/bin/redmine-mcp-server"

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
