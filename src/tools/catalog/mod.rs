use serde_json::{json, Value};

use crate::config::Config;

mod schema;

use schema::*;

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
