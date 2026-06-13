# Contributing

Thanks for contributing. Keep changes focused and include tests for behavior
changes.

## Development Setup

Install Node.js 18.17 or newer.

Clone the repository and run checks from the repository root.

## Checks

Run the local check script before opening a pull request:

```sh
scripts/check.sh
```

Individual commands:

```sh
npm run lint:js
npm test
```

The MCP server speaks newline-delimited JSON-RPC over stdio. Keep stdout reserved
for protocol messages; write diagnostics to stderr.

## Pull Requests

- Keep pull requests scoped to one behavior, tool group, or documentation change.
- Update `README.md`, `README.zh-CN.md`, or `docs/api-coverage.md` when changing
  user-facing behavior or API coverage.
- Include tests for new tools and read-only mode behavior for new write tools.
- Do not include Redmine API keys, private issue data, internal Redmine URLs, or
  generated local artifacts.

## Adding Redmine Tools

Before adding new tools, check `docs/api-coverage.md`.

Read-only metadata tools are preferred for broadening coverage. Write tools must
be listed in the `WRITE_TOOLS` set and must be rejected when
`REDMINE_MCP_READ_ONLY=true`. Optional groups should have corresponding
`REDMINE_MCP_DISABLE_*` flags when the group is plugin-dependent, high-volume, or
not useful for every installation.

High-risk administrative APIs should start with a design discussion before
implementation.
