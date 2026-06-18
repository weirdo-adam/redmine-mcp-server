# Promotion Drafts

Use these drafts after publishing a release. Replace the repository URL if the
project moves.

Repository:

```text
https://github.com/weirdo-adam/redmine-mcp-server
```

## English Launch Post

Title:

```text
Redmine MCP Server: connect Redmine issues, wiki, time entries, and attachments
to Claude Code, Codex, and other MCP clients
```

Body:

```text
I built Redmine MCP Server, a standalone stdio MCP server for teams that still
run Redmine and want AI coding agents to work with project context safely.

It exposes Redmine issues, projects, metadata, wiki pages, time entries,
attachments, versions, relations, watchers, and Redmine Checklists as MCP tools.
You can use it from Claude Code, Claude Desktop, Codex, Zed, or any stdio MCP
client.

Example prompts:

- Summarize issue #1234, including the latest comments and current blockers.
- List my open high-priority issues in project `mobile-app`.
- Find stale critical issues that have not been updated in the last 7 days.
- Draft a Redmine comment from this investigation result before updating the issue.
- Show unreleased versions and the open issues under each.

The safety model is intentionally simple: permissions come from the Redmine API
key, read-only mode is supported, and destructive delete/remove tools are hidden
unless explicitly enabled.

Install with Homebrew:

brew install weirdo-adam/tap/redmine-mcp-server

Repository:
https://github.com/weirdo-adam/redmine-mcp-server
```

## Chinese Launch Post

标题：

```text
Redmine MCP Server：让 Claude Code / Codex 直接读取和更新 Redmine
```

正文：

```text
我做了一个 Redmine MCP Server，适合还在使用 Redmine 的团队把项目管理数据接入
Claude Code、Claude Desktop、Codex、Zed 等 MCP 客户端。

它通过 stdio 暴露 Redmine 的 issue、项目、元数据、Wiki、工时、附件、版本、问题
关联、关注者和 Redmine Checklists。典型用法不是替代 Redmine，而是让 AI 助手在
排查问题、总结历史评论、整理版本范围、登记工时、补充 issue 评论时能直接使用
Redmine 上下文。

可以这样问：

- 总结 issue #1234，包括最新评论和当前阻塞点。
- 列出某项目里分配给我的高优先级未关闭问题。
- 找出最近 7 天没有更新的严重问题。
- 先根据排查结论草拟评论，确认后再写入 Redmine。
- 列出未发布版本以及每个版本下仍未关闭的问题。

安全模型比较直接：权限由 Redmine API key 决定；支持只读模式；删除/移除类工具默认
不暴露，必须显式开启。

Homebrew 安装：

brew install weirdo-adam/tap/redmine-mcp-server

仓库：
https://github.com/weirdo-adam/redmine-mcp-server
```

## Short Posts

English:

```text
Released Redmine MCP Server: a standalone stdio MCP server that connects Redmine
issues, wiki pages, time entries, attachments, versions, watchers, and
checklists to Claude Code, Codex, Zed, and other MCP clients.

Read-only mode is supported, and destructive tools are opt-in.

https://github.com/weirdo-adam/redmine-mcp-server
```

Chinese:

```text
发布了 Redmine MCP Server：把 Redmine 的 issue、Wiki、工时、附件、版本、
关注者和 checklist 接入 Claude Code、Codex、Zed 等 MCP 客户端。

支持只读模式，删除/移除类工具默认关闭。

https://github.com/weirdo-adam/redmine-mcp-server
```

## Suggested GitHub Topics

```text
redmine
mcp
model-context-protocol
claude-code
codex
rust
issue-tracker
project-management
```

## Release Checklist

- Confirm `scripts/check.sh` passes.
- Update `CHANGELOG.md`.
- Tag or publish the intended version.
- Verify the GitHub release has source and binary assets.
- Verify the Homebrew install command works.
- Add or update the suggested GitHub topics.
- Post the launch copy to the selected communities.
