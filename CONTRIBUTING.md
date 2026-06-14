# Contributing

Keep changes focused and include tests for behavior changes.

## Development Setup

Install Rust 1.75 or newer.

Clone the repository and run checks from the repository root.

## Checks

Run the local check script before opening a pull request:

```sh
scripts/check.sh
```

Individual commands:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The MCP server speaks newline-delimited JSON-RPC over stdio. Keep stdout reserved
for protocol messages; write diagnostics to stderr.

## Release

Update `Cargo.toml` and merge to `main`. The release workflow creates the
version tag, publishes the source release archive, and dispatches the Homebrew tap
`Bottle` workflow.

Configure the `HOMEBREW_TAP_TOKEN` repository secret before publishing a new
version. The token must be able to dispatch workflows in
`weirdo-adam/homebrew-tap`. Fine-grained tokens need repository access to that
tap and Actions read/write permission.

## Pull Requests

- Keep pull requests scoped to one behavior, tool group, or documentation change.
- Update `README.md`, `README.zh-CN.md`, or `docs/api-coverage.md` when changing
  user-facing behavior or API coverage.
- Include tests for new tools and read-only mode behavior for new write tools.
- Do not include Redmine API keys, private issue data, internal Redmine URLs, or
  generated local artifacts.

## Adding Redmine Tools

Before adding new tools, check `docs/api-coverage.md`.

Prefer read-only metadata tools when broadening coverage. Write tools must use
`ToolAccess::Write` or `ToolAccess::Delete` in `src/tools/catalog/mod.rs` and
must be rejected when `REDMINE_MCP_READ_ONLY=true`. Optional tool groups should
have `REDMINE_MCP_DISABLE_*` flags when the group is plugin-dependent,
high-volume, or not required for every installation.

High-risk administrative APIs should start with a design discussion before
implementation.
