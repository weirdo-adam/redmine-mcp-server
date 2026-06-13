# Client Configuration

Use absolute commands when configuring GUI clients. Keep real API keys in
user-scoped client settings or environment variables, not in committed project
files.

## Homebrew

After installing with Homebrew:

```sh
brew install weirdo-adam/tap/redmine-mcp-server
```

Most clients can use:

```text
redmine-mcp-server
```

Apple Silicon Homebrew usually installs the executable at:

```text
/opt/homebrew/bin/redmine-mcp-server
```

Intel macOS Homebrew usually installs it at:

```text
/usr/local/bin/redmine-mcp-server
```

## Local Checkout

From the repository root:

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
export REDMINE_MCP_READ_ONLY=true
npm start
```

Smoke test:

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize"}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
  | npm start
```

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

The Zed extension can use its bundled server by default. To force Zed to use a
Homebrew-installed standalone server, override the context server command:

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
- Optional feature flag example: `REDMINE_MCP_DISABLE_WIKI=true`
- Optional attachment limit: `REDMINE_MCP_ATTACHMENT_MAX_BYTES=10485760`

See [api-coverage.md](api-coverage.md) for the exposed Redmine API scope and
feature flags.
