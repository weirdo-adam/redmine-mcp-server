# Changelog

All notable changes to this project are documented here.

## Unreleased

## 0.1.1

- Added time entry activity-name resolution so time entry creation can look up
  Redmine custom activities when `activity_id` is not provided.
- Fixed time entry creation to include additional `fields` in the Redmine
  request body.
- Improved README positioning, examples, documentation links, and security
  messaging for new users.
- Added a prompt cookbook with copyable Redmine MCP workflows.
- Added package metadata for repository discovery.

## 0.1.0

- Initial standalone stdio MCP server for Redmine.
- Added MCP tools for issues, projects, metadata, wiki pages, time entries,
  attachments, versions, relations, watchers, and Redmine Checklists.
- Added read-only mode and delete/remove opt-in safeguards.
- Added Homebrew, local install, packaging, and release workflow support.
