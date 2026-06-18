# Redmine MCP Server

[![CI](https://github.com/weirdo-adam/redmine-mcp-server/actions/workflows/ci.yml/badge.svg)](https://github.com/weirdo-adam/redmine-mcp-server/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Homebrew](https://img.shields.io/badge/Homebrew-weirdo--adam%2Ftap-orange.svg)](https://github.com/weirdo-adam/homebrew-tap)

把 Redmine 接入 Claude Code、Claude Desktop、Codex、Zed 等 MCP 客户端。AI
助手可以搜索、总结、分派、更新 Redmine 工作项，同时权限仍然受普通 Redmine API
key 约束。

[English](README.md)

## 可以这样问

配置完成后，可以在 MCP 客户端里提出类似请求：

- “总结 issue #1234，包括最新评论和当前阻塞点。”
- “列出 `mobile-app` 项目里分配给我的高优先级未关闭问题。”
- “找出最近 7 天没有更新的严重问题。”
- “把这段排查结论追加到 issue #1234 的评论里。”
- “列出这个项目还没发布的版本，以及每个版本下仍未关闭的问题。”
- “给刚完成的部署工作登记 2 小时工时。”

## 功能

- 问题搜索、读取、创建、更新，以及可选删除。
- 项目、成员、跟踪标签、状态、优先级、自定义字段、查询和用户查询。
- Wiki、工时、附件、版本、问题关联、关注者和 Redmine Checklists 支持。
- 适合只分析不写入场景的只读模式。
- 破坏性删除/移除工具默认关闭。
- 独立 stdio 传输，不需要额外 Web 服务或数据库访问层。

## 运行要求

- Rust 1.75 或更高版本，仅源码构建需要
- Redmine 已开启 REST API
- Redmine API key 具备目标项目所需权限
- 仅在使用 checklist 工具时需要 Redmine Checklists 插件

## 快速开始

Homebrew：

```sh
brew install weirdo-adam/tap/redmine-mcp-server
```

本地仓库：

```sh
scripts/install-local.sh
```

运行服务：

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

## 文档

- [客户端配置](docs/client-configuration.md)：Homebrew 路径、本地仓库、
  Claude Code、Claude Desktop、Codex、Zed 和通用 stdio 客户端配置。
- [Prompt 示例](docs/prompt-cookbook.md)：问题分拣、版本规划、工时登记、
  Wiki 查询和安全写入流程的可复制示例。
- [API 覆盖范围](docs/api-coverage.md)：已支持的 Redmine API、功能开关和当前范围。
- [推广草稿](docs/promotion-post.md)：发布帖、短文案、GitHub topics 和 release
  checklist。
- [更新日志](CHANGELOG.md)：版本说明和重要变更记录。

## 开发

```sh
scripts/check.sh
scripts/package-release.sh
scripts/package-binary.sh aarch64-apple-darwin macos aarch64
```

源码发布包会写入 `dist/`：

```text
redmine-mcp-server-<version>.tar.gz
redmine-mcp-server-<version>.tar.gz.sha256
```

二进制发布包使用 OS/CPU 命名：

```text
redmine-mcp-server-<version>-<os>-<arch>.tar.gz
redmine-mcp-server-<version>-<os>-<arch>.tar.gz.sha256
```

## 安全

服务不需要直接访问数据库，也不会存储 API key。实际权限由配置的 Redmine API key
决定。

建议为目标项目使用最小必要权限；不需要写操作时，启用
`REDMINE_MCP_READ_ONLY=true`。破坏性删除/移除工具只有在设置
`REDMINE_MCP_ENABLE_DELETES=true` 后才会暴露，并且仍会被只读模式拦截。

漏洞报告请参考 [SECURITY.md](SECURITY.md)。
