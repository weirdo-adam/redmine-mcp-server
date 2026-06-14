use serde_json::{json, Map, Value};

use crate::config::Config;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ToolAccess {
    Read,
    Write,
    Delete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Feature {
    Attachments,
    Checklists,
    Relations,
    TimeEntries,
    Versions,
    Wiki,
    Watchers,
}

impl Feature {
    fn disabled(self, config: &Config) -> bool {
        match self {
            Self::Attachments => config.disabled_features.attachments,
            Self::Checklists => config.disabled_features.checklists,
            Self::Relations => config.disabled_features.relations,
            Self::TimeEntries => config.disabled_features.time_entries,
            Self::Versions => config.disabled_features.versions,
            Self::Wiki => config.disabled_features.wiki,
            Self::Watchers => config.disabled_features.watchers,
        }
    }

    fn env_name(self) -> &'static str {
        match self {
            Self::Attachments => "REDMINE_MCP_DISABLE_ATTACHMENTS",
            Self::Checklists => "REDMINE_MCP_DISABLE_CHECKLISTS",
            Self::Relations => "REDMINE_MCP_DISABLE_RELATIONS",
            Self::TimeEntries => "REDMINE_MCP_DISABLE_TIME_ENTRIES",
            Self::Versions => "REDMINE_MCP_DISABLE_VERSIONS",
            Self::Wiki => "REDMINE_MCP_DISABLE_WIKI",
            Self::Watchers => "REDMINE_MCP_DISABLE_WATCHERS",
        }
    }
}

pub(super) struct ToolDefinition {
    pub(super) name: &'static str,
    description: &'static str,
    access: ToolAccess,
    feature: Option<Feature>,
    schema: fn() -> Value,
}

impl ToolDefinition {
    pub(super) fn enabled(&self, config: &Config) -> bool {
        self.disabled_feature_env(config).is_none()
            && (!self.is_delete() || config.enable_deletes)
            && (!config.read_only || !self.is_write())
    }

    pub(super) fn disabled_feature_env(&self, config: &Config) -> Option<&'static str> {
        self.feature
            .filter(|feature| feature.disabled(config))
            .map(Feature::env_name)
    }

    pub(super) fn is_write(&self) -> bool {
        matches!(self.access, ToolAccess::Write | ToolAccess::Delete)
    }

    pub(super) fn is_delete(&self) -> bool {
        self.access == ToolAccess::Delete
    }

    pub(super) fn to_mcp_tool(&self) -> Value {
        json!({
            "name": self.name,
            "description": self.description,
            "inputSchema": (self.schema)()
        })
    }
}

pub(super) fn find(name: &str) -> Option<&'static ToolDefinition> {
    all().iter().find(|tool| tool.name == name)
}

pub(super) fn all() -> &'static [ToolDefinition] {
    TOOLS
}

static TOOLS: &[ToolDefinition] = &[
    ToolDefinition {
        name: "redmine_get_issue",
        description: "Get one Redmine issue by ID.",
        access: ToolAccess::Read,
        feature: None,
        schema: issue_get_schema,
    },
    ToolDefinition {
        name: "redmine_list_issues",
        description: "List Redmine issues using standard Redmine filters.",
        access: ToolAccess::Read,
        feature: None,
        schema: list_issues_schema,
    },
    ToolDefinition {
        name: "redmine_create_issue",
        description: "Create a Redmine issue.",
        access: ToolAccess::Write,
        feature: None,
        schema: create_issue_schema,
    },
    ToolDefinition {
        name: "redmine_update_issue",
        description: "Update a Redmine issue.",
        access: ToolAccess::Write,
        feature: None,
        schema: update_issue_schema,
    },
    ToolDefinition {
        name: "redmine_delete_issue",
        description: "Delete a Redmine issue.",
        access: ToolAccess::Delete,
        feature: None,
        schema: id_schema_issue,
    },
    ToolDefinition {
        name: "redmine_search",
        description: "Search Redmine records.",
        access: ToolAccess::Read,
        feature: None,
        schema: search_schema,
    },
    ToolDefinition {
        name: "redmine_get_attachment",
        description: "Get attachment metadata.",
        access: ToolAccess::Read,
        feature: Some(Feature::Attachments),
        schema: id_schema_attachment,
    },
    ToolDefinition {
        name: "redmine_download_attachment",
        description: "Download attachment content as base64.",
        access: ToolAccess::Read,
        feature: Some(Feature::Attachments),
        schema: id_schema_attachment,
    },
    ToolDefinition {
        name: "redmine_upload_attachment",
        description: "Upload an attachment and optionally attach it to an issue.",
        access: ToolAccess::Write,
        feature: Some(Feature::Attachments),
        schema: upload_attachment_schema,
    },
    ToolDefinition {
        name: "redmine_delete_attachment",
        description: "Delete an attachment.",
        access: ToolAccess::Delete,
        feature: Some(Feature::Attachments),
        schema: id_schema_attachment,
    },
    ToolDefinition {
        name: "redmine_list_issue_relations",
        description: "List relations for an issue.",
        access: ToolAccess::Read,
        feature: Some(Feature::Relations),
        schema: id_schema_issue,
    },
    ToolDefinition {
        name: "redmine_get_issue_relation",
        description: "Get one issue relation.",
        access: ToolAccess::Read,
        feature: Some(Feature::Relations),
        schema: id_schema_relation,
    },
    ToolDefinition {
        name: "redmine_add_issue_relation",
        description: "Add an issue relation.",
        access: ToolAccess::Write,
        feature: Some(Feature::Relations),
        schema: add_relation_schema,
    },
    ToolDefinition {
        name: "redmine_delete_issue_relation",
        description: "Delete an issue relation.",
        access: ToolAccess::Delete,
        feature: Some(Feature::Relations),
        schema: id_schema_relation,
    },
    ToolDefinition {
        name: "redmine_list_checklists",
        description: "List checklist items for an issue.",
        access: ToolAccess::Read,
        feature: Some(Feature::Checklists),
        schema: id_schema_issue,
    },
    ToolDefinition {
        name: "redmine_add_checklist_item",
        description: "Add a checklist item to an issue.",
        access: ToolAccess::Write,
        feature: Some(Feature::Checklists),
        schema: add_checklist_schema,
    },
    ToolDefinition {
        name: "redmine_update_checklist_item",
        description: "Update a checklist item.",
        access: ToolAccess::Write,
        feature: Some(Feature::Checklists),
        schema: update_checklist_schema,
    },
    ToolDefinition {
        name: "redmine_delete_checklist_item",
        description: "Delete a checklist item.",
        access: ToolAccess::Delete,
        feature: Some(Feature::Checklists),
        schema: checklist_id_schema,
    },
    ToolDefinition {
        name: "redmine_list_time_entries",
        description: "List time entries.",
        access: ToolAccess::Read,
        feature: Some(Feature::TimeEntries),
        schema: list_time_entries_schema,
    },
    ToolDefinition {
        name: "redmine_get_time_entry",
        description: "Get one time entry.",
        access: ToolAccess::Read,
        feature: Some(Feature::TimeEntries),
        schema: id_schema_time_entry,
    },
    ToolDefinition {
        name: "redmine_add_time_entry",
        description: "Add a time entry.",
        access: ToolAccess::Write,
        feature: Some(Feature::TimeEntries),
        schema: add_time_entry_schema,
    },
    ToolDefinition {
        name: "redmine_update_time_entry",
        description: "Update a time entry.",
        access: ToolAccess::Write,
        feature: Some(Feature::TimeEntries),
        schema: update_time_entry_schema,
    },
    ToolDefinition {
        name: "redmine_delete_time_entry",
        description: "Delete a time entry.",
        access: ToolAccess::Delete,
        feature: Some(Feature::TimeEntries),
        schema: id_schema_time_entry,
    },
    ToolDefinition {
        name: "redmine_list_versions",
        description: "List project versions.",
        access: ToolAccess::Read,
        feature: Some(Feature::Versions),
        schema: project_id_schema,
    },
    ToolDefinition {
        name: "redmine_get_version",
        description: "Get one version.",
        access: ToolAccess::Read,
        feature: Some(Feature::Versions),
        schema: id_schema_version,
    },
    ToolDefinition {
        name: "redmine_create_version",
        description: "Create a project version.",
        access: ToolAccess::Write,
        feature: Some(Feature::Versions),
        schema: create_version_schema,
    },
    ToolDefinition {
        name: "redmine_update_version",
        description: "Update a version.",
        access: ToolAccess::Write,
        feature: Some(Feature::Versions),
        schema: update_version_schema,
    },
    ToolDefinition {
        name: "redmine_delete_version",
        description: "Delete a version.",
        access: ToolAccess::Delete,
        feature: Some(Feature::Versions),
        schema: id_schema_version,
    },
    ToolDefinition {
        name: "redmine_list_watchers",
        description: "List watchers for an issue.",
        access: ToolAccess::Read,
        feature: Some(Feature::Watchers),
        schema: id_schema_issue,
    },
    ToolDefinition {
        name: "redmine_add_watcher",
        description: "Add an issue watcher.",
        access: ToolAccess::Write,
        feature: Some(Feature::Watchers),
        schema: watcher_schema,
    },
    ToolDefinition {
        name: "redmine_remove_watcher",
        description: "Remove an issue watcher.",
        access: ToolAccess::Delete,
        feature: Some(Feature::Watchers),
        schema: watcher_schema,
    },
    ToolDefinition {
        name: "redmine_list_time_entry_activities",
        description: "List time entry activities.",
        access: ToolAccess::Read,
        feature: Some(Feature::TimeEntries),
        schema: no_args_schema,
    },
    ToolDefinition {
        name: "redmine_list_projects",
        description: "List projects.",
        access: ToolAccess::Read,
        feature: None,
        schema: list_projects_schema,
    },
    ToolDefinition {
        name: "redmine_get_project",
        description: "Get one project.",
        access: ToolAccess::Read,
        feature: None,
        schema: project_id_schema,
    },
    ToolDefinition {
        name: "redmine_list_project_memberships",
        description: "List project memberships.",
        access: ToolAccess::Read,
        feature: None,
        schema: project_membership_list_schema,
    },
    ToolDefinition {
        name: "redmine_get_project_membership",
        description: "Get one project membership.",
        access: ToolAccess::Read,
        feature: None,
        schema: id_schema_membership,
    },
    ToolDefinition {
        name: "redmine_list_wiki_pages",
        description: "List project wiki pages.",
        access: ToolAccess::Read,
        feature: Some(Feature::Wiki),
        schema: project_id_schema,
    },
    ToolDefinition {
        name: "redmine_get_wiki_page",
        description: "Get one wiki page.",
        access: ToolAccess::Read,
        feature: Some(Feature::Wiki),
        schema: wiki_page_schema,
    },
    ToolDefinition {
        name: "redmine_list_issue_statuses",
        description: "List issue statuses.",
        access: ToolAccess::Read,
        feature: None,
        schema: no_args_schema,
    },
    ToolDefinition {
        name: "redmine_list_trackers",
        description: "List trackers.",
        access: ToolAccess::Read,
        feature: None,
        schema: no_args_schema,
    },
    ToolDefinition {
        name: "redmine_list_issue_priorities",
        description: "List issue priorities.",
        access: ToolAccess::Read,
        feature: None,
        schema: no_args_schema,
    },
    ToolDefinition {
        name: "redmine_list_issue_categories",
        description: "List project issue categories.",
        access: ToolAccess::Read,
        feature: None,
        schema: project_id_schema,
    },
    ToolDefinition {
        name: "redmine_list_custom_fields",
        description: "List custom fields.",
        access: ToolAccess::Read,
        feature: None,
        schema: no_args_schema,
    },
    ToolDefinition {
        name: "redmine_list_queries",
        description: "List saved queries.",
        access: ToolAccess::Read,
        feature: None,
        schema: list_queries_schema,
    },
    ToolDefinition {
        name: "redmine_list_users",
        description: "List users.",
        access: ToolAccess::Read,
        feature: None,
        schema: list_users_schema,
    },
    ToolDefinition {
        name: "redmine_get_current_user",
        description: "Get the authenticated Redmine user.",
        access: ToolAccess::Read,
        feature: None,
        schema: current_user_schema,
    },
];

fn no_args_schema() -> Value {
    schema([], [])
}

fn id_schema_issue() -> Value {
    schema(["issue_id"], [integer("issue_id", "Redmine issue ID.")])
}

fn id_schema_attachment() -> Value {
    schema(
        ["attachment_id"],
        [integer("attachment_id", "Redmine attachment ID.")],
    )
}

fn id_schema_relation() -> Value {
    schema(
        ["relation_id"],
        [integer("relation_id", "Redmine issue relation ID.")],
    )
}

fn id_schema_time_entry() -> Value {
    schema(
        ["time_entry_id"],
        [integer("time_entry_id", "Redmine time entry ID.")],
    )
}

fn id_schema_version() -> Value {
    schema(
        ["version_id"],
        [integer("version_id", "Redmine version ID.")],
    )
}

fn id_schema_membership() -> Value {
    schema(
        ["membership_id"],
        [integer("membership_id", "Redmine project membership ID.")],
    )
}

fn project_id_schema() -> Value {
    schema(
        ["project_id"],
        [id_or_string(
            "project_id",
            "Redmine project ID or identifier.",
        )],
    )
}

fn issue_get_schema() -> Value {
    schema(
        ["issue_id"],
        [
            integer("issue_id", "Redmine issue ID."),
            string_array_enum(
                "include",
                "Related issue sections to include.",
                &[
                    "journals",
                    "watchers",
                    "checklists",
                    "relations",
                    "attachments",
                    "children",
                    "changesets",
                    "allowed_statuses",
                ],
            ),
        ],
    )
}

fn list_issues_schema() -> Value {
    schema(
        [],
        [
            id_or_string("project_id", "Project filter."),
            string_prop("status_id", "Status filter such as open, closed, or an ID."),
            integer("tracker_id", "Tracker filter."),
            integer("assigned_to_id", "Assignee filter."),
            integer("fixed_version_id", "Fixed version filter."),
            integer("query_id", "Saved query ID."),
            string_prop("sort", "Redmine sort expression."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

fn create_issue_schema() -> Value {
    schema(
        ["project_id", "subject"],
        [
            id_or_string("project_id", "Project ID or identifier."),
            string_prop("subject", "Issue subject."),
            string_prop("description", "Issue description."),
            object_prop("fields", "Additional Redmine issue fields."),
            integer("tracker_id", "Tracker ID."),
            integer("status_id", "Status ID."),
            integer("priority_id", "Priority ID."),
            integer("assigned_to_id", "Assignee user ID."),
            integer("category_id", "Issue category ID."),
            integer("fixed_version_id", "Fixed version ID."),
            integer("parent_issue_id", "Parent issue ID."),
            string_prop("start_date", "Start date in YYYY-MM-DD format."),
            string_prop("due_date", "Due date in YYYY-MM-DD format."),
            integer("done_ratio", "Completion percentage."),
            number("estimated_hours", "Estimated hours."),
            array_prop("custom_fields", "Custom field values."),
            array_prop("watcher_user_ids", "Watcher user IDs."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn update_issue_schema() -> Value {
    schema(
        ["issue_id", "fields"],
        [
            integer("issue_id", "Redmine issue ID."),
            object_prop("fields", "Issue fields to update."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn search_schema() -> Value {
    schema(
        ["q"],
        [
            string_prop("q", "Search query."),
            id_or_string("scope", "Project scope."),
            bool_prop("all_words", "Require all words."),
            bool_prop("titles_only", "Search titles only."),
            bool_prop("issues", "Include issues."),
            bool_prop("open_issues", "Limit issue search to open issues."),
            bool_prop("wiki_pages", "Include wiki pages."),
            bool_prop("messages", "Include messages."),
            bool_prop("news", "Include news."),
            bool_prop("documents", "Include documents."),
            bool_prop("changesets", "Include changesets."),
            bool_prop("attachments", "Include attachments."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

fn upload_attachment_schema() -> Value {
    schema(
        ["filename", "content_base64"],
        [
            string_prop("filename", "Attachment filename."),
            string_prop("content_base64", "Attachment content encoded as base64."),
            string_prop("content_type", "Attachment content type."),
            integer("issue_id", "Optional issue ID to attach the upload to."),
            string_prop("description", "Attachment description."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn add_relation_schema() -> Value {
    schema(
        ["issue_id", "issue_to_id", "relation_type"],
        [
            integer("issue_id", "Source issue ID."),
            integer("issue_to_id", "Target issue ID."),
            string_prop("relation_type", "Redmine relation type."),
            integer("delay", "Relation delay in days."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn add_checklist_schema() -> Value {
    schema(
        ["issue_id", "subject"],
        [
            integer("issue_id", "Redmine issue ID."),
            string_prop("subject", "Checklist item subject."),
            bool_prop("is_done", "Completion state."),
            integer("position", "Checklist item position."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn update_checklist_schema() -> Value {
    schema(
        ["issue_id", "checklist_id"],
        [
            integer("issue_id", "Redmine issue ID."),
            integer("checklist_id", "Checklist item ID."),
            string_prop("subject", "Checklist item subject."),
            bool_prop("is_done", "Completion state."),
            integer("position", "Checklist item position."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn checklist_id_schema() -> Value {
    schema(
        ["issue_id", "checklist_id"],
        [
            integer("issue_id", "Redmine issue ID."),
            integer("checklist_id", "Checklist item ID."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn list_time_entries_schema() -> Value {
    schema(
        [],
        [
            integer("issue_id", "Issue filter."),
            id_or_string("project_id", "Project filter."),
            integer("user_id", "User filter."),
            integer("activity_id", "Activity filter."),
            string_prop("from", "Start date in YYYY-MM-DD format."),
            string_prop("to", "End date in YYYY-MM-DD format."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

fn add_time_entry_schema() -> Value {
    schema(
        ["hours", "activity_id"],
        [
            integer("issue_id", "Issue ID."),
            id_or_string("project_id", "Project ID or identifier."),
            number("hours", "Spent hours."),
            integer("activity_id", "Activity ID."),
            string_prop("spent_on", "Spent date in YYYY-MM-DD format."),
            string_prop("comments", "Time entry comments."),
            object_prop("fields", "Additional Redmine time entry fields."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn update_time_entry_schema() -> Value {
    schema(
        ["time_entry_id", "fields"],
        [
            integer("time_entry_id", "Time entry ID."),
            object_prop("fields", "Time entry fields to update."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn create_version_schema() -> Value {
    schema(
        ["project_id", "name"],
        [
            id_or_string("project_id", "Project ID or identifier."),
            string_prop("name", "Version name."),
            string_prop("description", "Version description."),
            string_prop("status", "Version status."),
            string_prop("sharing", "Version sharing mode."),
            string_prop("due_date", "Due date in YYYY-MM-DD format."),
            string_prop("wiki_page_title", "Linked wiki page title."),
            object_prop("fields", "Additional Redmine version fields."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn update_version_schema() -> Value {
    schema(
        ["version_id", "fields"],
        [
            integer("version_id", "Version ID."),
            object_prop("fields", "Version fields to update."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn watcher_schema() -> Value {
    schema(
        ["issue_id", "user_id"],
        [
            integer("issue_id", "Issue ID."),
            integer("user_id", "User ID."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

fn list_projects_schema() -> Value {
    schema(
        [],
        [
            string_array_enum(
                "include",
                "Related project sections to include.",
                &["trackers", "issue_categories", "enabled_modules"],
            ),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

fn project_membership_list_schema() -> Value {
    schema(
        ["project_id"],
        [
            id_or_string("project_id", "Project ID or identifier."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

fn wiki_page_schema() -> Value {
    schema(
        ["project_id", "title"],
        [
            id_or_string("project_id", "Project ID or identifier."),
            string_prop("title", "Wiki page title."),
            integer("version", "Wiki page version."),
            string_array_enum(
                "include",
                "Related wiki sections to include.",
                &["attachments"],
            ),
        ],
    )
}

fn list_queries_schema() -> Value {
    schema(
        [],
        [
            id_or_string("project_id", "Project filter."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

fn list_users_schema() -> Value {
    schema(
        [],
        [
            string_prop("name", "Name filter."),
            integer("group_id", "Group filter."),
            integer("status", "User status filter."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

fn current_user_schema() -> Value {
    schema(
        [],
        [bool_prop(
            "include_memberships",
            "Include user memberships.",
        )],
    )
}

fn schema<const R: usize, const P: usize>(
    required: [&'static str; R],
    properties: [(&'static str, Value); P],
) -> Value {
    let mut property_map = Map::new();
    for (name, value) in properties {
        property_map.insert(name.to_string(), value);
    }

    let required = required.iter().copied().collect::<Vec<_>>();

    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": property_map,
        "required": required
    })
}

fn string_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "string", "description": description }),
    )
}

fn integer(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "integer", "description": description }),
    )
}

fn number(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "number", "description": description }),
    )
}

fn bool_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "boolean", "description": description }),
    )
}

fn object_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "object", "description": description }),
    )
}

fn array_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (name, json!({ "type": "array", "description": description }))
}

fn id_or_string(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({
            "description": description,
            "oneOf": [
                { "type": "integer" },
                { "type": "string" }
            ]
        }),
    )
}

fn string_array_enum(
    name: &'static str,
    description: &'static str,
    values: &[&'static str],
) -> (&'static str, Value) {
    (
        name,
        json!({
            "type": "array",
            "description": description,
            "items": {
                "type": "string",
                "enum": values
            }
        }),
    )
}

fn write_notify_prop() -> (&'static str, Value) {
    bool_prop("notify", "Send Redmine email notifications.")
}

fn silent_prop() -> (&'static str, Value) {
    bool_prop("silent", "Suppress write response payload when possible.")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn tool_names_are_unique_and_have_schemas() {
        let mut names = HashSet::new();
        for tool in all() {
            assert!(
                names.insert(tool.name),
                "duplicate tool name: {}",
                tool.name
            );
            assert_eq!((tool.schema)()["type"], "object");
        }
    }
}
