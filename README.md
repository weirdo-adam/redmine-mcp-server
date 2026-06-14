# Redmine MCP Server

Standalone stdio Model Context Protocol server for Redmine.

It provides MCP tools for Redmine issues, projects, metadata, wiki pages, time
entries, attachments, versions, relations, watchers, and Redmine Checklists.

## Requirements

- Rust 1.75 or newer, only for source builds
- Redmine REST API enabled
- Redmine API key with the required project permissions
- Redmine Checklists plugin, only when checklist tools are used

## Installation

Homebrew:

```sh
brew install weirdo-adam/tap/redmine-mcp-server
```

Local checkout:

```sh
scripts/install-local.sh
```

## Usage

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
export REDMINE_MCP_READ_ONLY=true
redmine-mcp-server
```

Development checkout:

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
cargo run
```

The server communicates over newline-delimited JSON-RPC on stdin/stdout. Logs
and diagnostics are written to stderr.

## MCP Client Examples

Claude Code:

```sh
claude mcp add redmine \
  --transport stdio \
  --env REDMINE_BASE_URL=https://redmine.example.com \
  --env REDMINE_API_KEY=your-api-key \
  --env REDMINE_MCP_READ_ONLY=true \
  -- redmine-mcp-server
```

Codex CLI:

```sh
codex mcp add \
  --env REDMINE_BASE_URL=https://redmine.example.com \
  --env REDMINE_API_KEY=your-api-key \
  --env REDMINE_MCP_READ_ONLY=true \
  redmine -- redmine-mcp-server
```

Codex `config.toml`:

```toml
[mcp_servers.redmine]
command = "redmine-mcp-server"
args = []

[mcp_servers.redmine.env]
REDMINE_BASE_URL = "https://redmine.example.com"
REDMINE_API_KEY = "your-api-key"
REDMINE_MCP_READ_ONLY = "true"
```

## Configuration

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `REDMINE_BASE_URL` | Yes | none | Redmine base URL. |
| `REDMINE_API_KEY` | Yes | none | Redmine REST API key. |
| `REDMINE_MCP_READ_ONLY` | No | `false` | Hide and reject write tools. |
| `REDMINE_MCP_ENABLE_DELETES` | No | `false` | Expose destructive delete/remove tools. |
| `REDMINE_TIMEOUT_MS` | No | `30000` | HTTP request timeout in milliseconds. |

Complete environment variables and client examples are documented in
[docs/client-configuration.md](docs/client-configuration.md).

## Development

```sh
scripts/check.sh
scripts/package-release.sh
```

Source release archives are written to `dist/`:

```text
redmine-mcp-server-<version>.tar.gz
redmine-mcp-server-<version>.tar.gz.sha256
```

## Security

Permissions are determined by the configured Redmine API key. Use the least
privileged key practical for the target project, and enable
`REDMINE_MCP_READ_ONLY=true` when write operations are not required.

See [SECURITY.md](SECURITY.md) for vulnerability reporting.
