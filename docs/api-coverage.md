# Redmine REST API Coverage

This extension covers common Redmine workflows for Zed agents. It is not a
complete Redmine REST API implementation.

Status definitions:

- Supported: core Redmine REST endpoints for this area are exposed.
- Partial: only selected endpoints or safe read-only operations are exposed.
- Planned: useful for this MCP server, but not implemented yet.
- Not supported: not currently targeted for this extension.

## Coverage Status

| Redmine API area | Status | MCP tools | Notes |
| --- | --- | --- | --- |
| Issues | Partial | `redmine_list_issues`, `redmine_get_issue`, `redmine_create_issue`, `redmine_update_issue`, `redmine_delete_issue` | Core CRUD is implemented. The list tool models common filters, not every Redmine filter/operator. Delete is destructive, disabled by default, requires `REDMINE_MCP_ENABLE_DELETES=true`, and is still guarded by `REDMINE_MCP_READ_ONLY`. |
| Search | Supported | `redmine_search` | Read-only. |
| Projects | Partial | `redmine_list_projects`, `redmine_get_project` | Read-only project lookup. Project create/update/delete is not implemented. |
| Project memberships | Partial | `redmine_list_project_memberships`, `redmine_get_project_membership` | Read-only. Membership create/update/delete is not implemented. |
| Issue statuses | Supported | `redmine_list_issue_statuses` | Read-only. |
| Trackers | Supported | `redmine_list_trackers` | Read-only. |
| Enumerations and priorities | Partial | `redmine_list_issue_priorities`, `redmine_list_time_entry_activities` | Document categories are not exposed. |
| Issue categories | Partial | `redmine_list_issue_categories` | Read-only list by project. Category create/update/delete is not implemented. |
| Custom fields | Supported | `redmine_list_custom_fields` | Redmine requires administrator privileges for this endpoint. |
| Saved queries | Supported | `redmine_list_queries` | Lists queries visible to the API user. |
| Users | Partial | `redmine_list_users`, `redmine_get_current_user` | User administration is not implemented. |
| Groups | Not supported | Not implemented | No current plan. |
| Roles | Not supported | Not implemented | No current plan. |
| Issue relations | Supported | `redmine_list_issue_relations`, `redmine_get_issue_relation`, `redmine_add_issue_relation`, `redmine_delete_issue_relation` | Write tools are disabled by `REDMINE_MCP_READ_ONLY=true`. Delete is disabled by default and requires `REDMINE_MCP_ENABLE_DELETES=true`. |
| Time entries | Supported | `redmine_list_time_entries`, `redmine_get_time_entry`, `redmine_add_time_entry`, `redmine_update_time_entry`, `redmine_delete_time_entry`, `redmine_list_time_entry_activities` | Can be disabled with `REDMINE_MCP_DISABLE_TIME_ENTRIES=true`. Delete is disabled by default and requires `REDMINE_MCP_ENABLE_DELETES=true`. |
| Versions | Supported | `redmine_list_versions`, `redmine_get_version`, `redmine_create_version`, `redmine_update_version`, `redmine_delete_version` | Can be disabled with `REDMINE_MCP_DISABLE_VERSIONS=true`. Delete is disabled by default and requires `REDMINE_MCP_ENABLE_DELETES=true`. |
| Watchers | Supported | `redmine_list_watchers`, `redmine_add_watcher`, `redmine_remove_watcher` | Can be disabled with `REDMINE_MCP_DISABLE_WATCHERS=true`. Remove is disabled by default and requires `REDMINE_MCP_ENABLE_DELETES=true`. |
| Attachments | Supported | `redmine_get_attachment`, `redmine_download_attachment`, `redmine_upload_attachment`, `redmine_delete_attachment` | Upload and delete are write tools guarded by `REDMINE_MCP_READ_ONLY`. Delete is destructive and also requires `REDMINE_MCP_ENABLE_DELETES=true`. Download returns base64 or UTF-8 content and is limited by `REDMINE_MCP_ATTACHMENT_MAX_BYTES`. |
| Wiki pages | Partial | `redmine_list_wiki_pages`, `redmine_get_wiki_page` | Read-only. Wiki page create/update/delete is not implemented. Can be disabled with `REDMINE_MCP_DISABLE_WIKI=true`. |
| News | Not supported | Not implemented | No current plan. |
| Files | Not supported | Not implemented | No current plan. Redmine project files are distinct from issue attachments. |
| Project administration | Not supported | Not implemented | High-impact administrative operations are out of scope. |
| User administration | Not supported | Not implemented | High-impact administrative operations are out of scope. |
| Checklists plugin | Supported | `redmine_list_checklists`, `redmine_add_checklist_item`, `redmine_update_checklist_item`, `redmine_delete_checklist_item` | Requires the Redmine Checklists plugin. Can be disabled with `REDMINE_MCP_DISABLE_CHECKLISTS=true`. Delete is disabled by default and requires `REDMINE_MCP_ENABLE_DELETES=true`. |

## Scope Rules

The default tool set prioritizes issue triage, metadata lookup, time tracking,
versions, relations, watchers, attachments, project membership lookup, wiki page
lookup, and checklists. Optional tool groups are enabled by default and can be
disabled with `REDMINE_MCP_DISABLE_*` environment variables.

Administrative APIs are intentionally not part of the first publish scope.
Destructive delete/remove tools are implemented but disabled by default as the
minimum safety mode. New write tools must be compatible with
`REDMINE_MCP_READ_ONLY` and must have tests proving that read-only mode blocks
the operation before any Redmine request is made.

## Recommended Next Additions

The first publish scope has no remaining planned Redmine REST API additions.
Future expansion should be evaluated against tool count, write risk, and common
agent workflows before adding new default tools.

## References

- [Redmine REST API](https://www.redmine.org/projects/redmine/wiki/Rest_api)
- [Redmine REST Issues](https://www.redmine.org/projects/redmine/wiki/Rest_Issues)
- [Redmine REST Attachments](https://www.redmine.org/projects/redmine/wiki/Rest_Attachments)
- [Redmine REST Time Entries](https://www.redmine.org/projects/redmine/wiki/Rest_TimeEntries)
- [Redmine REST Projects](https://www.redmine.org/projects/redmine/wiki/Rest_Projects)
- [Redmine REST Project Memberships](https://www.redmine.org/projects/redmine/wiki/Rest_Memberships)
- [Redmine REST Wiki Pages](https://www.redmine.org/projects/redmine/wiki/Rest_WikiPages)
- [Redmine REST Files](https://www.redmine.org/projects/redmine/wiki/Rest_Files)
