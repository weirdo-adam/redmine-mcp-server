# Client Configuration

Use absolute commands when configuring GUI clients. Keep real API keys in
user-scoped client settings or environment variables, not in committed project
files.

## Native Install (Recommended)

Install the latest native binary release:

macOS/Linux:

```sh
curl -fsSL https://github.com/weirdo-adam/redmine-mcp-server/releases/latest/download/install.sh | bash
```

Windows PowerShell:

```powershell
irm https://github.com/weirdo-adam/redmine-mcp-server/releases/latest/download/install.ps1 | iex
```

Windows Command Prompt:

```cmd
curl -fsSL https://github.com/weirdo-adam/redmine-mcp-server/releases/latest/download/install.cmd -o install.cmd && install.cmd && del install.cmd
```

The installer downloads the matching GitHub Release archive, verifies the
`.sha256` checksum, and installs `redmine-mcp-server`.

Default install locations:

```text
macOS/Linux: ~/.local/bin/redmine-mcp-server
Windows:     %LOCALAPPDATA%\redmine-mcp-server\bin\redmine-mcp-server.exe
```

Override the install directory with `REDMINE_MCP_INSTALL_DIR`. Install a
specific release with `REDMINE_MCP_VERSION`, for example:

```sh
curl -fsSL https://github.com/weirdo-adam/redmine-mcp-server/releases/latest/download/install.sh | REDMINE_MCP_VERSION=0.1.1 bash
```

## Homebrew

After installing with Homebrew:

```sh
brew install weirdo-adam/tap/redmine-mcp-server
```

Most clients can use:

```text
redmine-mcp-server
```

Apple Silicon Homebrew installs the executable at:

```text
/opt/homebrew/bin/redmine-mcp-server
```

Intel macOS Homebrew installs the executable at:

```text
/usr/local/bin/redmine-mcp-server
```

## Local Checkout

From the repository root:

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
export REDMINE_MCP_READ_ONLY=true
cargo run
```

Smoke test:

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize"}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
  | cargo run --quiet
```

## Environment Variables

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `REDMINE_BASE_URL` | Yes | none | Redmine base URL. Trailing slashes are ignored. |
| `REDMINE_API_KEY` | Yes | none | Redmine REST API key. |
| `REDMINE_MCP_READ_ONLY` | No | `false` | Hide and reject write tools. |
| `REDMINE_MCP_ENABLE_DELETES` | No | `false` | Expose delete and remove tools. |
| `REDMINE_SILENT_WRITES` | No | `false` | Suppress write response bodies by default and request `notify=false`. |
| `REDMINE_TIMEOUT_MS` | No | `30000` | HTTP request timeout in milliseconds. |
| `REDMINE_MCP_ATTACHMENT_MAX_BYTES` | No | `10485760` | Maximum attachment download size in bytes. |
| `REDMINE_MCP_DISABLE_ATTACHMENTS` | No | `false` | Disable attachment tools. |
| `REDMINE_MCP_DISABLE_CHECKLISTS` | No | `false` | Disable Redmine Checklists tools. |
| `REDMINE_MCP_DISABLE_RELATIONS` | No | `false` | Disable issue relation tools. |
| `REDMINE_MCP_DISABLE_TIME_ENTRIES` | No | `false` | Disable time entry tools. |
| `REDMINE_MCP_DISABLE_VERSIONS` | No | `false` | Disable version tools. |
| `REDMINE_MCP_DISABLE_WATCHERS` | No | `false` | Disable watcher tools. |
| `REDMINE_MCP_DISABLE_WIKI` | No | `false` | Disable wiki tools. |

Boolean variables accept `1`, `true`, `yes`, or `on` as true values.

## One-Command Local Install

Install into a user directory and create a launcher:

```sh
scripts/install-local.sh
```

Default install directory:

```text
~/.local/share/redmine-mcp-server
```

Override the install directory with `REDMINE_MCP_INSTALL_DIR`:

```sh
REDMINE_MCP_INSTALL_DIR="$HOME/.local/share/redmine-mcp-server" scripts/install-local.sh
```

After installation, external clients can use:

```text
~/.local/share/redmine-mcp-server/bin/redmine-mcp-server
```

## Zed Custom Command

Zed extensions that support custom context server commands can point to this
standalone server:

```json
{
  "context_servers": {
    "redmine": {
      "command": {
        "path": "/opt/homebrew/bin/redmine-mcp-server",
        "arguments": []
      },
      "settings": {
        "REDMINE_BASE_URL": "https://redmine.example.com",
        "REDMINE_API_KEY": "your-api-key",
        "REDMINE_MCP_READ_ONLY": true
      }
    }
  }
}
```

## Claude Code

Claude Code can register a local stdio MCP server with `claude mcp add`:

```sh
claude mcp add redmine \
  --transport stdio \
  --env REDMINE_BASE_URL=https://redmine.example.com \
  --env REDMINE_API_KEY=your-api-key \
  --env REDMINE_MCP_READ_ONLY=true \
  -- redmine-mcp-server
```

For a project-scoped configuration, use `.mcp.json` and keep secrets in
environment variables:

```json
{
  "mcpServers": {
    "redmine": {
      "type": "stdio",
      "command": "redmine-mcp-server",
      "args": [],
      "env": {
        "REDMINE_BASE_URL": "https://redmine.example.com",
        "REDMINE_API_KEY": "${REDMINE_API_KEY}",
        "REDMINE_MCP_READ_ONLY": "true"
      }
    }
  }
}
```

## Claude Desktop

Add the server to the user-level Claude Desktop MCP configuration:

```json
{
  "mcpServers": {
    "redmine": {
      "command": "/opt/homebrew/bin/redmine-mcp-server",
      "args": [],
      "env": {
        "REDMINE_BASE_URL": "https://redmine.example.com",
        "REDMINE_API_KEY": "your-api-key",
        "REDMINE_MCP_READ_ONLY": "true"
      }
    }
  }
}
```

Restart Claude Desktop after changing the configuration.

## Codex

Codex CLI can register the server with `codex mcp add`:

```sh
codex mcp add \
  --env REDMINE_BASE_URL=https://redmine.example.com \
  --env REDMINE_API_KEY=your-api-key \
  --env REDMINE_MCP_READ_ONLY=true \
  redmine -- redmine-mcp-server
```

Codex clients that support local stdio MCP servers can also use a TOML server
entry in the user or project Codex configuration:

```toml
[mcp_servers.redmine]
command = "redmine-mcp-server"
args = []

[mcp_servers.redmine.env]
REDMINE_BASE_URL = "https://redmine.example.com"
REDMINE_API_KEY = "your-api-key"
REDMINE_MCP_READ_ONLY = "true"
```

Prefer user-scoped configuration for real API keys. Use project-scoped
configuration only with environment-variable indirection or read-only test
credentials.

## Other MCP Clients

Use the same stdio contract:

- Command: `redmine-mcp-server`
- Arguments: `[]`
- Required environment: `REDMINE_BASE_URL`, `REDMINE_API_KEY`
- Optional safety setting: `REDMINE_MCP_READ_ONLY=true`
- Optional destructive delete/remove opt-in: `REDMINE_MCP_ENABLE_DELETES=true`
- Optional feature flags: `REDMINE_MCP_DISABLE_*`

See [api-coverage.md](api-coverage.md) for the exposed Redmine API scope and
feature flags.
