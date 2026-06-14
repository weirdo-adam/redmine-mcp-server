use serde_json::{json, Map, Value};

mod attachments;
mod catalog;
mod checklists;
mod issues;
mod projects;
mod relations;
mod time_entries;
mod versions;
mod watchers;
mod wiki;
use crate::config::Config;
use crate::redmine::{path_segment, value_to_string, RedmineClient, RedmineError, RequestOptions};

const DEFAULT_ISSUE_INCLUDES: &[&str] = &["journals", "watchers", "checklists", "relations"];

pub fn list_tools(config: &Config) -> Vec<Value> {
    catalog::all()
        .iter()
        .filter(|tool| tool.enabled(config))
        .map(catalog::ToolDefinition::to_mcp_tool)
        .collect()
}

pub fn call_tool(client: &RedmineClient, name: &str, args: Value) -> Result<Value, RedmineError> {
    let Some(tool) = catalog::find(name) else {
        return Err(RedmineError::input(format!("Unknown tool: {name}")));
    };
    if let Some(env_name) = tool.disabled_feature_env(&client.config) {
        return Err(RedmineError::runtime(format!(
            "{env_name} is enabled; tool {name} is disabled"
        )));
    }
    if tool.is_delete() && !client.config.enable_deletes {
        return Err(RedmineError::runtime(format!(
            "REDMINE_MCP_ENABLE_DELETES is not enabled; delete/remove tool {name} is disabled"
        )));
    }
    if client.config.read_only && tool.is_write() {
        return Err(RedmineError::runtime(format!(
            "REDMINE_MCP_READ_ONLY is enabled; write tool {name} is disabled"
        )));
    }

    let args = args.as_object().cloned().unwrap_or_default();
    match name {
        "redmine_get_issue" => issues::get_issue(client, &args),
        "redmine_list_issues" => issues::list_issues(client, &args),
        "redmine_create_issue" => issues::create_issue(client, &args),
        "redmine_update_issue" => issues::update_issue(client, &args),
        "redmine_delete_issue" => issues::delete_issue(client, &args),
        "redmine_search" => get(client, "/search.json", filter_args(&args)),
        "redmine_get_attachment" => attachments::get_attachment(client, &args),
        "redmine_download_attachment" => attachments::download_attachment(client, &args),
        "redmine_upload_attachment" => attachments::upload_attachment(client, &args),
        "redmine_delete_attachment" => attachments::delete_attachment(client, &args),
        "redmine_list_issue_relations" => relations::list_issue_relations(client, &args),
        "redmine_get_issue_relation" => relations::get_issue_relation(client, &args),
        "redmine_add_issue_relation" => relations::add_issue_relation(client, &args),
        "redmine_delete_issue_relation" => relations::delete_issue_relation(client, &args),
        "redmine_list_checklists" => checklists::list_checklists(client, &args),
        "redmine_add_checklist_item" => checklists::add_checklist_item(client, &args),
        "redmine_update_checklist_item" => checklists::update_checklist_item(client, &args),
        "redmine_delete_checklist_item" => checklists::delete_checklist_item(client, &args),
        "redmine_list_time_entries" => get(client, "/time_entries.json", filter_args(&args)),
        "redmine_get_time_entry" => time_entries::get_time_entry(client, &args),
        "redmine_add_time_entry" => time_entries::add_time_entry(client, &args),
        "redmine_update_time_entry" => time_entries::update_time_entry(client, &args),
        "redmine_delete_time_entry" => time_entries::delete_time_entry(client, &args),
        "redmine_list_versions" => versions::list_versions(client, &args),
        "redmine_get_version" => versions::get_version(client, &args),
        "redmine_create_version" => versions::create_version(client, &args),
        "redmine_update_version" => versions::update_version(client, &args),
        "redmine_delete_version" => versions::delete_version(client, &args),
        "redmine_list_watchers" => watchers::list_watchers(client, &args),
        "redmine_add_watcher" => watchers::add_watcher(client, &args),
        "redmine_remove_watcher" => watchers::remove_watcher(client, &args),
        "redmine_list_time_entry_activities" => get(
            client,
            "/enumerations/time_entry_activities.json",
            Map::new(),
        ),
        "redmine_list_projects" => get(client, "/projects.json", filter_args(&args)),
        "redmine_get_project" => projects::get_project(client, &args),
        "redmine_list_project_memberships" => projects::list_project_memberships(client, &args),
        "redmine_get_project_membership" => projects::get_project_membership(client, &args),
        "redmine_list_wiki_pages" => wiki::list_wiki_pages(client, &args),
        "redmine_get_wiki_page" => wiki::get_wiki_page(client, &args),
        "redmine_list_issue_statuses" => get(client, "/issue_statuses.json", Map::new()),
        "redmine_list_trackers" => get(client, "/trackers.json", Map::new()),
        "redmine_list_issue_priorities" => {
            get(client, "/enumerations/issue_priorities.json", Map::new())
        }
        "redmine_list_issue_categories" => projects::list_issue_categories(client, &args),
        "redmine_list_custom_fields" => get(client, "/custom_fields.json", Map::new()),
        "redmine_list_queries" => get(client, "/queries.json", filter_args(&args)),
        "redmine_list_users" => get(client, "/users.json", filter_args(&args)),
        "redmine_get_current_user" => get(client, "/users/current.json", filter_args(&args)),
        _ => Err(RedmineError::input(format!("Unknown tool: {name}"))),
    }
}

pub fn error_payload(error: &RedmineError) -> Value {
    let mut payload = Map::new();
    payload.insert("ok".to_string(), Value::Bool(false));
    payload.insert("error".to_string(), Value::String(error.message.clone()));
    if let Some(status) = error.status {
        payload.insert("status".to_string(), json!(status));
    }
    if let Some(method) = &error.method {
        payload.insert("method".to_string(), json!(method));
    }
    if let Some(path) = &error.path {
        payload.insert("path".to_string(), json!(path));
    }
    if let Some(details) = &error.details {
        payload.insert("details".to_string(), details.clone());
    }
    Value::Object(payload)
}

fn get(
    client: &RedmineClient,
    path: &str,
    query: Map<String, Value>,
) -> Result<Value, RedmineError> {
    let response = client.request(
        "GET",
        path,
        RequestOptions {
            query,
            ..Default::default()
        },
    )?;
    Ok(response.body.as_value())
}

fn request_json(
    client: &RedmineClient,
    method: &str,
    path: &str,
    query: Map<String, Value>,
    body: Value,
) -> Result<crate::redmine::RedmineResponse, RedmineError> {
    client.request(
        method,
        path,
        RequestOptions {
            query,
            body: Some(body),
            ..Default::default()
        },
    )
}

fn delete_with_target(
    client: &RedmineClient,
    args: &Map<String, Value>,
    operation: &str,
    key: &str,
    path_template: &str,
) -> Result<Value, RedmineError> {
    let id = required(args, key)?;
    let response = client.request(
        "DELETE",
        &path_template.replace("{id}", &path_segment(id)),
        RequestOptions {
            query: write_query(client, args),
            ..Default::default()
        },
    )?;
    write_result(client, args, operation, json!({ key: id }), response)
}

fn write_result(
    client: &RedmineClient,
    args: &Map<String, Value>,
    operation: &str,
    target: Value,
    response: crate::redmine::RedmineResponse,
) -> Result<Value, RedmineError> {
    let mut result = Map::new();
    result.insert("ok".to_string(), Value::Bool(true));
    result.insert("operation".to_string(), json!(operation));
    result.insert("target".to_string(), target);
    result.insert("status".to_string(), json!(response.status));
    if !is_silent_write(client, args) {
        result.insert("response".to_string(), response.body.as_value());
    }
    Ok(Value::Object(result))
}

fn write_query(client: &RedmineClient, args: &Map<String, Value>) -> Map<String, Value> {
    if let Some(notify) = args.get("notify") {
        return Map::from_iter([(
            "notify".to_string(),
            json!(parse_bool_value(notify, true).to_string()),
        )]);
    }
    if is_silent_write(client, args) {
        return Map::from_iter([("notify".to_string(), json!("false"))]);
    }
    Map::new()
}

fn is_silent_write(client: &RedmineClient, args: &Map<String, Value>) -> bool {
    args.get("silent")
        .map(|value| parse_bool_value(value, false))
        .unwrap_or(client.config.silent_writes)
}

fn filter_args(args: &Map<String, Value>) -> Map<String, Value> {
    args.iter()
        .filter(|(key, value)| {
            !matches!(key.as_str(), "silent" | "notify" | "fields")
                && !matches!(value, Value::Null)
                && !matches!(value, Value::String(text) if text.is_empty())
        })
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

fn issue_includes(client: &RedmineClient, requested: Option<&Value>) -> Option<Value> {
    let includes = match requested {
        Some(Value::Array(values)) if !values.is_empty() => values
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect::<Vec<_>>(),
        _ => DEFAULT_ISSUE_INCLUDES
            .iter()
            .map(|value| value.to_string())
            .collect(),
    };
    let filtered = includes
        .into_iter()
        .filter(|include| match include.as_str() {
            "checklists" => !client.config.disabled_features.checklists,
            "relations" => !client.config.disabled_features.relations,
            "watchers" => !client.config.disabled_features.watchers,
            "attachments" => !client.config.disabled_features.attachments,
            _ => true,
        })
        .collect::<Vec<_>>();
    (!filtered.is_empty()).then(|| json!(filtered))
}

fn required<'a>(args: &'a Map<String, Value>, key: &str) -> Result<&'a Value, RedmineError> {
    optional(args, key).ok_or_else(|| RedmineError::input(format!("{key} is required")))
}

fn optional<'a>(args: &'a Map<String, Value>, key: &str) -> Option<&'a Value> {
    match args.get(key) {
        Some(Value::Null) => None,
        Some(Value::String(value)) if value.is_empty() => None,
        Some(value) => Some(value),
        None => None,
    }
}

fn object_arg(args: &Map<String, Value>, key: &str) -> Result<Map<String, Value>, RedmineError> {
    match required(args, key)? {
        Value::Object(value) => Ok(value.clone()),
        _ => Err(RedmineError::input(format!("{key} must be an object"))),
    }
}

fn optional_object_arg(
    args: &Map<String, Value>,
    key: &str,
) -> Result<Map<String, Value>, RedmineError> {
    match optional(args, key) {
        Some(Value::Object(value)) => Ok(value.clone()),
        Some(_) => Err(RedmineError::input(format!("{key} must be an object"))),
        None => Ok(Map::new()),
    }
}

fn pick_defined(args: &Map<String, Value>, keys: &[&str]) -> Map<String, Value> {
    let mut result = Map::new();
    merge_pick(&mut result, args, keys);
    result
}

fn merge_pick(target: &mut Map<String, Value>, args: &Map<String, Value>, keys: &[&str]) {
    for key in keys {
        if let Some(value) = args.get(*key) {
            if !matches!(value, Value::Null) {
                target.insert((*key).to_string(), value.clone());
            }
        }
    }
}

fn target_from(value: &Value) -> Value {
    let Some(object) = value.as_object() else {
        return json!({});
    };
    Value::Object(pick_defined(object, &["issue_id", "project_id", "user_id"]))
}

fn parse_bool_value(value: &Value, fallback: bool) -> bool {
    match value {
        Value::Bool(value) => *value,
        Value::String(value) if value.trim().is_empty() => fallback,
        Value::String(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Value::Number(value) => value.as_i64() == Some(1),
        _ => fallback,
    }
}

fn percent_encode(value: &str) -> String {
    percent_encoding::utf8_percent_encode(value, percent_encoding::NON_ALPHANUMERIC).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DisabledFeatures, DEFAULT_ATTACHMENT_MAX_BYTES};

    fn config() -> Config {
        Config {
            base_url: "https://redmine.example.com".to_string(),
            api_key: "secret".to_string(),
            read_only: false,
            enable_deletes: false,
            silent_writes: false,
            disabled_features: DisabledFeatures::default(),
            attachment_max_bytes: DEFAULT_ATTACHMENT_MAX_BYTES,
            timeout_ms: 30_000,
        }
    }

    #[test]
    fn list_tools_hides_deletes_by_default() {
        let names = list_tools(&config())
            .into_iter()
            .map(|tool| tool["name"].as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        assert!(names.contains(&"redmine_get_issue".to_string()));
        assert!(!names.contains(&"redmine_delete_issue".to_string()));
    }

    #[test]
    fn list_tools_hides_read_only_write_tools() {
        let mut config = config();
        config.read_only = true;
        config.enable_deletes = true;
        let names = list_tools(&config)
            .into_iter()
            .map(|tool| tool["name"].as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        assert!(names.contains(&"redmine_get_issue".to_string()));
        assert!(!names.contains(&"redmine_create_issue".to_string()));
        assert!(!names.contains(&"redmine_delete_issue".to_string()));
    }

    #[test]
    fn feature_disable_flags_hide_grouped_tools() {
        let mut config = config();
        config.disabled_features.attachments = true;
        config.disabled_features.wiki = true;
        let names = list_tools(&config)
            .into_iter()
            .map(|tool| tool["name"].as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        assert!(!names.contains(&"redmine_get_attachment".to_string()));
        assert!(!names.contains(&"redmine_get_wiki_page".to_string()));
    }
}
