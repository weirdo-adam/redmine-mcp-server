# Prompt Cookbook

Use these prompts after adding `redmine-mcp-server` to an MCP client. Start with
`REDMINE_MCP_READ_ONLY=true` for exploration and switch it off only when the
agent should create or update Redmine data.

## Issue Triage

```text
Summarize issue #1234. Include the current status, assignee, priority, latest
comments, related issues, attachments, checklist state, and likely blockers.
```

```text
List my open high-priority issues in project `mobile-app`. Group them by status
and highlight anything that has not been updated in the last 7 days.
```

```text
Find recently updated issues in project `backend-api` that mention login,
timeout, or database errors. Show the issue id, subject, status, priority, and
the latest relevant note.
```

## Release Planning

```text
Show unreleased versions for project `mobile-app`. For each version, list open
issues grouped by priority and call out blockers or issues without assignees.
```

```text
Compare open issues assigned to version `2.4.0` with the current checklist
state. Identify work that is likely still incomplete.
```

## Safe Write Workflows

Ask the agent to draft changes before writing them:

```text
Draft a Redmine comment for issue #1234 based on this investigation result.
Do not update Redmine yet. Show me the exact comment first.
```

Then approve the update:

```text
Add the drafted comment to issue #1234.
```

For structured updates:

```text
Update issue #1234 to status `In Progress`, assign it to me, and add a short
comment explaining that I am investigating the regression.
```

## Time Tracking

```text
Show my time entries for this week and group them by project and issue.
```

```text
Add a 2-hour time entry to issue #1234 with activity `Development` and comment
`Investigated login timeout regression`.
```

## Wiki And Attachments

```text
Find wiki pages in project `backend-api` related to deployment or rollback.
Summarize the current rollback procedure.
```

```text
Fetch the attachments on issue #1234 and identify which ones look relevant to
the crash report. Do not download large files unless needed.
```

## Chinese Examples

```text
总结 issue #1234，包括当前状态、负责人、优先级、最新评论、关联问题、附件、
checklist 状态和可能的阻塞点。
```

```text
列出 `mobile-app` 项目中分配给我的高优先级未关闭问题，按状态分组，并标出最近
7 天没有更新的问题。
```

```text
先根据下面的排查结论草拟一条 Redmine 评论，不要直接更新。等我确认后再写入
issue #1234。
```
