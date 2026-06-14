# Redmine MCP Server

面向 Redmine 的独立 stdio Model Context Protocol 服务器。

提供 Redmine 问题、项目、元数据、Wiki、工时、附件、版本、关联、关注者和
Redmine Checklists 工具。

## 运行要求

- Rust 1.75 或更高版本，仅源码构建需要
- Redmine 已开启 REST API
- Redmine API key 具备目标项目所需权限
- 仅在使用检查清单工具时需要 Redmine Checklists 插件

## 安装

Homebrew：

```sh
brew install weirdo-adam/tap/redmine-mcp-server
```

本地仓库：

```sh
scripts/install-local.sh
```

## 使用

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
export REDMINE_MCP_READ_ONLY=true
redmine-mcp-server
```

本地开发：

```sh
export REDMINE_BASE_URL="https://redmine.example.com"
export REDMINE_API_KEY="your-api-key"
cargo run
```

服务通过 stdin/stdout 使用按行分隔的 JSON-RPC。日志和诊断信息写入 stderr。

## MCP 客户端示例

Claude Code：

```sh
claude mcp add redmine \
  --transport stdio \
  --env REDMINE_BASE_URL=https://redmine.example.com \
  --env REDMINE_API_KEY=your-api-key \
  --env REDMINE_MCP_READ_ONLY=true \
  -- redmine-mcp-server
```

Codex CLI：

```sh
codex mcp add \
  --env REDMINE_BASE_URL=https://redmine.example.com \
  --env REDMINE_API_KEY=your-api-key \
  --env REDMINE_MCP_READ_ONLY=true \
  redmine -- redmine-mcp-server
```

Codex `config.toml`：

```toml
[mcp_servers.redmine]
command = "redmine-mcp-server"
args = []

[mcp_servers.redmine.env]
REDMINE_BASE_URL = "https://redmine.example.com"
REDMINE_API_KEY = "your-api-key"
REDMINE_MCP_READ_ONLY = "true"
```

## 配置

| 变量 | 必填 | 默认值 | 说明 |
| --- | --- | --- | --- |
| `REDMINE_BASE_URL` | 是 | 无 | Redmine 实例地址。 |
| `REDMINE_API_KEY` | 是 | 无 | Redmine REST API key。 |
| `REDMINE_MCP_READ_ONLY` | 否 | `false` | 隐藏并拒绝写工具。 |
| `REDMINE_MCP_ENABLE_DELETES` | 否 | `false` | 暴露破坏性删除/移除工具。 |
| `REDMINE_TIMEOUT_MS` | 否 | `30000` | HTTP 请求超时时间，单位毫秒。 |

完整环境变量和客户端示例见
[docs/client-configuration.md](docs/client-configuration.md)。

## 开发

```sh
scripts/check.sh
scripts/package-release.sh
```

发布包会写入 `dist/`：

```text
redmine-mcp-server-<version>.tar.gz
redmine-mcp-server-<version>.tar.gz.sha256
```

## 安全

实际权限由配置的 Redmine API key 决定。建议为目标项目使用最小必要权限；不需要
写操作时，启用 `REDMINE_MCP_READ_ONLY=true`。

漏洞报告请参考 [SECURITY.md](SECURITY.md)。
