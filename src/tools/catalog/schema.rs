use serde_json::{json, Map, Value};

pub(super) fn no_args_schema() -> Value {
    schema([], [])
}

pub(super) fn id_schema_issue() -> Value {
    schema(["issue_id"], [integer("issue_id", "Redmine issue ID.")])
}

pub(super) fn id_schema_attachment() -> Value {
    schema(
        ["attachment_id"],
        [integer("attachment_id", "Redmine attachment ID.")],
    )
}

pub(super) fn id_schema_relation() -> Value {
    schema(
        ["relation_id"],
        [integer("relation_id", "Redmine issue relation ID.")],
    )
}

pub(super) fn id_schema_time_entry() -> Value {
    schema(
        ["time_entry_id"],
        [integer("time_entry_id", "Redmine time entry ID.")],
    )
}

pub(super) fn id_schema_version() -> Value {
    schema(
        ["version_id"],
        [integer("version_id", "Redmine version ID.")],
    )
}

pub(super) fn id_schema_membership() -> Value {
    schema(
        ["membership_id"],
        [integer("membership_id", "Redmine project membership ID.")],
    )
}

pub(super) fn project_id_schema() -> Value {
    schema(
        ["project_id"],
        [id_or_string(
            "project_id",
            "Redmine project ID or identifier.",
        )],
    )
}

pub(super) fn issue_get_schema() -> Value {
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

pub(super) fn list_issues_schema() -> Value {
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

pub(super) fn create_issue_schema() -> Value {
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

pub(super) fn update_issue_schema() -> Value {
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

pub(super) fn search_schema() -> Value {
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

pub(super) fn upload_attachment_schema() -> Value {
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

pub(super) fn add_relation_schema() -> Value {
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

pub(super) fn add_checklist_schema() -> Value {
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

pub(super) fn update_checklist_schema() -> Value {
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

pub(super) fn checklist_id_schema() -> Value {
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

pub(super) fn list_time_entries_schema() -> Value {
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

pub(super) fn add_time_entry_schema() -> Value {
    schema(
        ["hours"],
        [
            integer("issue_id", "Issue ID."),
            id_or_string("project_id", "Project ID or identifier."),
            number("hours", "Spent hours."),
            integer("activity_id", "Activity ID."),
            string_prop(
                "activity_name",
                "Activity name to resolve when activity_id is not provided.",
            ),
            string_prop("spent_on", "Spent date in YYYY-MM-DD format."),
            string_prop("comments", "Time entry comments."),
            object_prop("fields", "Additional Redmine time entry fields."),
            write_notify_prop(),
            silent_prop(),
        ],
    )
}

pub(super) fn update_time_entry_schema() -> Value {
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

pub(super) fn create_version_schema() -> Value {
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

pub(super) fn update_version_schema() -> Value {
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

pub(super) fn watcher_schema() -> Value {
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

pub(super) fn list_projects_schema() -> Value {
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

pub(super) fn project_membership_list_schema() -> Value {
    schema(
        ["project_id"],
        [
            id_or_string("project_id", "Project ID or identifier."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

pub(super) fn wiki_page_schema() -> Value {
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

pub(super) fn list_queries_schema() -> Value {
    schema(
        [],
        [
            id_or_string("project_id", "Project filter."),
            integer("limit", "Maximum number of records."),
            integer("offset", "Pagination offset."),
        ],
    )
}

pub(super) fn list_users_schema() -> Value {
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

pub(super) fn current_user_schema() -> Value {
    schema(
        [],
        [bool_prop(
            "include_memberships",
            "Include user memberships.",
        )],
    )
}

pub(super) fn schema<const R: usize, const P: usize>(
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

pub(super) fn string_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "string", "description": description }),
    )
}

pub(super) fn integer(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "integer", "description": description }),
    )
}

pub(super) fn number(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "number", "description": description }),
    )
}

pub(super) fn bool_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "boolean", "description": description }),
    )
}

pub(super) fn object_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (
        name,
        json!({ "type": "object", "description": description }),
    )
}

pub(super) fn array_prop(name: &'static str, description: &'static str) -> (&'static str, Value) {
    (name, json!({ "type": "array", "description": description }))
}

pub(super) fn id_or_string(name: &'static str, description: &'static str) -> (&'static str, Value) {
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

pub(super) fn string_array_enum(
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

pub(super) fn write_notify_prop() -> (&'static str, Value) {
    bool_prop("notify", "Send Redmine email notifications.")
}

pub(super) fn silent_prop() -> (&'static str, Value) {
    bool_prop("silent", "Suppress write response payload when possible.")
}
