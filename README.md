# Redmine MCP Server

Standalone stdio Model Context Protocol server for Redmine.

It provides MCP tools for Redmine issues, projects, metadata, wiki pages, time
entries, attachments, versions, relations, watchers, and Redmine Checklists.

## Requirements

- Node.js 18.17 or newer
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
npm start
```

The server communicates over newline-delimited JSON-RPC on stdin/stdout. Logs
and diagnostics are written to stderr.

## Configuration

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `REDMINE_BASE_URL` | Yes | none | Redmine base URL. |
| `REDMINE_API_KEY` | Yes | none | Redmine REST API key. |
| `REDMINE_MCP_READ_ONLY` | No | `false` | Hide and reject write tools. |
| `REDMINE_MCP_ENABLE_DELETES` | No | `false` | Expose destructive delete/remove tools. |
| `REDMINE_TIMEOUT_MS` | No | `30000` | HTTP request timeout in milliseconds. |

Additional feature flags and client examples are documented in
[docs/client-configuration.md](docs/client-configuration.md).

## Development

```sh
scripts/check.sh
scripts/package-release.sh
```

Release archives are written to `dist/`:

```text
redmine-mcp-server-<version>.tar.gz
redmine-mcp-server-<version>.tar.gz.sha256
```

## Security

Permissions are determined by the configured Redmine API key. Use the least
privileged key practical for the target project, and enable
`REDMINE_MCP_READ_ONLY=true` when write operations are not required.

See [SECURITY.md](SECURITY.md) for vulnerability reporting.
