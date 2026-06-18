# Redmine MCP Server

[![CI](https://github.com/weirdo-adam/redmine-mcp-server/actions/workflows/ci.yml/badge.svg)](https://github.com/weirdo-adam/redmine-mcp-server/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Homebrew](https://img.shields.io/badge/Homebrew-weirdo--adam%2Ftap-orange.svg)](https://github.com/weirdo-adam/homebrew-tap)

Connect Redmine to MCP clients such as Claude Code, Claude Desktop, Codex, and
Zed. The server lets AI agents search, summarize, triage, and update Redmine
work while keeping access scoped to a normal Redmine API key.

[简体中文](README.zh-CN.md)

## What You Can Ask

Once configured, an MCP client can answer Redmine-aware requests such as:

- "Summarize issue #1234, including the latest comments and current blockers."
- "List my open high-priority issues in project `mobile-app`."
- "Find stale critical issues that have not been updated in the last 7 days."
- "Add this investigation result as a comment on issue #1234."
- "Show unreleased versions for this project and the open issues under each."
- "Create a 2-hour time entry for the deployment work I just finished."

## Features

- Issue search, read, create, update, and optional delete tools.
- Project, membership, tracker, status, priority, custom field, query, and user
  lookup tools.
- Wiki pages, time entries, attachments, versions, issue relations, watchers,
  and Redmine Checklists support.
- Read-only mode for analysis-only agents.
- Destructive delete/remove tools disabled by default.
- Standalone stdio transport with no web service or database access layer.

## Requirements

- Rust 1.75 or newer, only for source builds
- Redmine REST API enabled
- Redmine API key with the required project permissions
- Redmine Checklists plugin, only when checklist tools are used

## Quick Start

Homebrew:

```sh
brew install weirdo-adam/tap/redmine-mcp-server
```

Local checkout:

```sh
scripts/install-local.sh
```

Run the server:

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

## Documentation

- [Client configuration](docs/client-configuration.md): Homebrew paths, local
  checkout setup, Claude Code, Claude Desktop, Codex, Zed, and generic stdio
  clients.
- [Prompt cookbook](docs/prompt-cookbook.md): copyable prompts for issue triage,
  release planning, time tracking, wiki lookup, and safe write workflows.
- [API coverage](docs/api-coverage.md): supported Redmine API areas, feature
  flags, and current scope rules.
- [Promotion drafts](docs/promotion-post.md): launch copy, short posts, suggested
  GitHub topics, and release checklist.
- [Changelog](CHANGELOG.md): release notes and notable project changes.

## Development

```sh
scripts/check.sh
scripts/package-release.sh
scripts/package-binary.sh aarch64-apple-darwin macos aarch64
```

Source release archives are written to `dist/`:

```text
redmine-mcp-server-<version>.tar.gz
redmine-mcp-server-<version>.tar.gz.sha256
```

Binary release archives use OS/CPU names:

```text
redmine-mcp-server-<version>-<os>-<arch>.tar.gz
redmine-mcp-server-<version>-<os>-<arch>.tar.gz.sha256
```

## Security

The server does not need direct database access and does not store your API key.
Permissions are determined by the configured Redmine API key.

Use the least privileged key practical for the target project, and enable
`REDMINE_MCP_READ_ONLY=true` when write operations are not required. Destructive
delete/remove tools are hidden unless `REDMINE_MCP_ENABLE_DELETES=true` is set,
and they are still blocked by read-only mode.

See [SECURITY.md](SECURITY.md) for vulnerability reporting.
