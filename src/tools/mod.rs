use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde_json::{json, Map, Value};

mod catalog;

use crate::config::{Config, DEFAULT_ATTACHMENT_MAX_BYTES};
use crate::redmine::{
    path_segment, value_to_string, RedmineClient, RedmineError, RequestOptions, ResponseBody,
    ResponseType,
};

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
        "redmine_get_issue" => get_issue(client, &args),
        "redmine_list_issues" => list_issues(client, &args),
        "redmine_create_issue" => create_issue(client, &args),
        "redmine_update_issue" => update_issue(client, &args),
        "redmine_delete_issue" => delete_issue(client, &args),
        "redmine_search" => get(client, "/search.json", filter_args(&args)),
        "redmine_get_attachment" => get_attachment(client, &args),
        "redmine_download_attachment" => download_attachment(client, &args),
        "redmine_upload_attachment" => upload_attachment(client, &args),
        "redmine_delete_attachment" => delete_attachment(client, &args),
        "redmine_list_issue_relations" => list_issue_relations(client, &args),
        "redmine_get_issue_relation" => get_issue_relation(client, &args),
        "redmine_add_issue_relation" => add_issue_relation(client, &args),
        "redmine_delete_issue_relation" => delete_issue_relation(client, &args),
        "redmine_list_checklists" => list_checklists(client, &args),
        "redmine_add_checklist_item" => add_checklist_item(client, &args),
        "redmine_update_checklist_item" => update_checklist_item(client, &args),
        "redmine_delete_checklist_item" => delete_checklist_item(client, &args),
        "redmine_list_time_entries" => get(client, "/time_entries.json", filter_args(&args)),
        "redmine_get_time_entry" => get_time_entry(client, &args),
        "redmine_add_time_entry" => add_time_entry(client, &args),
        "redmine_update_time_entry" => update_time_entry(client, &args),
        "redmine_delete_time_entry" => delete_time_entry(client, &args),
        "redmine_list_versions" => list_versions(client, &args),
        "redmine_get_version" => get_version(client, &args),
        "redmine_create_version" => create_version(client, &args),
        "redmine_update_version" => update_version(client, &args),
        "redmine_delete_version" => delete_version(client, &args),
        "redmine_list_watchers" => list_watchers(client, &args),
        "redmine_add_watcher" => add_watcher(client, &args),
        "redmine_remove_watcher" => remove_watcher(client, &args),
        "redmine_list_time_entry_activities" => get(
            client,
            "/enumerations/time_entry_activities.json",
            Map::new(),
        ),
        "redmine_list_projects" => get(client, "/projects.json", filter_args(&args)),
        "redmine_get_project" => get_project(client, &args),
        "redmine_list_project_memberships" => list_project_memberships(client, &args),
        "redmine_get_project_membership" => get_project_membership(client, &args),
        "redmine_list_wiki_pages" => list_wiki_pages(client, &args),
        "redmine_get_wiki_page" => get_wiki_page(client, &args),
        "redmine_list_issue_statuses" => get(client, "/issue_statuses.json", Map::new()),
        "redmine_list_trackers" => get(client, "/trackers.json", Map::new()),
        "redmine_list_issue_priorities" => {
            get(client, "/enumerations/issue_priorities.json", Map::new())
        }
        "redmine_list_issue_categories" => list_issue_categories(client, &args),
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

fn get_issue(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let mut query = Map::new();
    if let Some(include) = issue_includes(client, args.get("include")) {
        query.insert("include".to_string(), include);
    }
    get(
        client,
        &format!("/issues/{}.json", path_segment(issue_id)),
        query,
    )
}

fn list_issues(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    get(client, "/issues.json", filter_args(args))
}

fn create_issue(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let mut issue = optional_object_arg(args, "fields")?;
    merge_pick(
        &mut issue,
        args,
        &[
            "project_id",
            "subject",
            "description",
            "tracker_id",
            "status_id",
            "priority_id",
            "assigned_to_id",
            "category_id",
            "fixed_version_id",
            "parent_issue_id",
            "start_date",
            "due_date",
            "done_ratio",
            "estimated_hours",
            "custom_fields",
            "watcher_user_ids",
        ],
    );
    let target = target_from(&Value::Object(issue.clone()));

    let response = request_json(
        client,
        "POST",
        "/issues.json",
        write_query(client, args),
        json!({ "issue": issue }),
    )?;
    write_result(client, args, "create_issue", target, response)
}

fn update_issue(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let fields = Value::Object(object_arg(args, "fields")?);
    let response = request_json(
        client,
        "PUT",
        &format!("/issues/{}.json", path_segment(issue_id)),
        write_query(client, args),
        json!({ "issue": fields }),
    )?;
    write_result(
        client,
        args,
        "update_issue",
        json!({ "issue_id": issue_id }),
        response,
    )
}

fn delete_issue(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    delete_with_target(
        client,
        args,
        "delete_issue",
        "issue_id",
        "/issues/{id}.json",
    )
}

fn get_attachment(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let attachment_id = required(args, "attachment_id")?;
    get(
        client,
        &format!("/attachments/{}.json", path_segment(attachment_id)),
        Map::new(),
    )
}

fn download_attachment(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let attachment_id = required(args, "attachment_id")?;
    let metadata = get_attachment(client, args)?;
    let attachment = metadata
        .get("attachment")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    let filename = args
        .get("filename")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            attachment
                .get("filename")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .ok_or_else(|| {
            RedmineError::input(
                "filename is required when attachment metadata does not include one",
            )
        })?;

    let max_bytes = attachment_max_bytes(client, args);
    if let Some(size) = attachment.get("filesize").and_then(Value::as_u64) {
        if size > max_bytes {
            return Err(RedmineError::input(format!(
                "Attachment {} is {} bytes, exceeding max_bytes {}",
                value_to_string(attachment_id),
                size,
                max_bytes
            )));
        }
    }

    let mut options = RequestOptions {
        accept: Some("*/*".to_string()),
        response_type: ResponseType::Buffer,
        ..Default::default()
    };
    let response = client.request(
        "GET",
        &format!(
            "/attachments/download/{}/{}",
            path_segment(attachment_id),
            percent_encode(&filename)
        ),
        std::mem::take(&mut options),
    )?;
    let ResponseBody::Buffer(bytes) = response.body else {
        return Err(RedmineError::runtime("Attachment response was not binary"));
    };
    if bytes.len() as u64 > max_bytes {
        return Err(RedmineError::input(format!(
            "Downloaded attachment {} is {} bytes, exceeding max_bytes {}",
            value_to_string(attachment_id),
            bytes.len(),
            max_bytes
        )));
    }

    let encoding = args
        .get("encoding")
        .and_then(Value::as_str)
        .unwrap_or("base64");
    let mut result = Map::new();
    result.insert("attachment_id".to_string(), attachment_id.clone());
    result.insert("filename".to_string(), json!(filename));
    result.insert(
        "content_type".to_string(),
        json!(response
            .headers
            .get("content-type")
            .cloned()
            .or_else(|| attachment
                .get("content_type")
                .and_then(Value::as_str)
                .map(str::to_string))
            .unwrap_or_else(|| "application/octet-stream".to_string())),
    );
    result.insert("size".to_string(), json!(bytes.len()));
    result.insert("encoding".to_string(), json!(encoding));
    if encoding == "utf8" {
        result.insert(
            "content".to_string(),
            json!(String::from_utf8_lossy(&bytes).to_string()),
        );
    } else {
        result.insert("content_base64".to_string(), json!(BASE64.encode(bytes)));
    }
    Ok(Value::Object(result))
}

fn upload_attachment(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let filename = attachment_filename(args)?;
    let bytes = attachment_upload_bytes(args)?;
    let max_bytes = attachment_max_bytes(client, args);
    if bytes.len() as u64 > max_bytes {
        return Err(RedmineError::input(format!(
            "Attachment upload is {} bytes, exceeding max_bytes {}",
            bytes.len(),
            max_bytes
        )));
    }

    let mut query = Map::new();
    query.insert("filename".to_string(), json!(filename));
    let upload_response = client.request(
        "POST",
        "/uploads.json",
        RequestOptions {
            query,
            raw_body: Some(bytes),
            content_type: Some("application/octet-stream".to_string()),
            ..Default::default()
        },
    )?;
    let token = upload_response
        .body
        .clone()
        .as_value()
        .pointer("/upload/token")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RedmineError::runtime("Redmine upload response did not include upload.token")
        })?
        .to_string();

    let mut upload = Map::new();
    upload.insert("token".to_string(), json!(token));
    upload.insert("filename".to_string(), json!(filename));
    merge_pick(&mut upload, args, &["description", "content_type"]);

    if let Some(issue_id) = optional(args, "issue_id") {
        let response = request_json(
            client,
            "PUT",
            &format!("/issues/{}.json", path_segment(issue_id)),
            write_query(client, args),
            json!({ "issue": { "uploads": [Value::Object(upload.clone())] } }),
        )?;
        return attachment_write_result(
            client,
            args,
            "upload_attachment",
            json!({ "issue_id": issue_id, "filename": filename, "token": token }),
            response,
            Value::Object(upload),
        );
    }

    attachment_write_result(
        client,
        args,
        "upload_attachment",
        json!({ "filename": filename, "token": token }),
        upload_response,
        Value::Object(upload),
    )
}

fn delete_attachment(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    delete_with_target(
        client,
        args,
        "delete_attachment",
        "attachment_id",
        "/attachments/{id}.json",
    )
}

fn list_issue_relations(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    get(
        client,
        &format!("/issues/{}/relations.json", path_segment(issue_id)),
        Map::new(),
    )
}

fn get_issue_relation(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let relation_id = required(args, "relation_id")?;
    get(
        client,
        &format!("/relations/{}.json", path_segment(relation_id)),
        Map::new(),
    )
}

fn add_issue_relation(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let issue_to_id = required(args, "issue_to_id")?;
    let relation_type = required(args, "relation_type")?;
    let relation = pick_defined(args, &["issue_to_id", "relation_type", "delay"]);
    let response = request_json(
        client,
        "POST",
        &format!("/issues/{}/relations.json", path_segment(issue_id)),
        write_query(client, args),
        json!({ "relation": relation }),
    )?;
    write_result(
        client,
        args,
        "add_issue_relation",
        json!({ "issue_id": issue_id, "issue_to_id": issue_to_id, "relation_type": relation_type }),
        response,
    )
}

fn delete_issue_relation(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    delete_with_target(
        client,
        args,
        "delete_issue_relation",
        "relation_id",
        "/relations/{id}.json",
    )
}

fn list_checklists(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let mut query = Map::new();
    query.insert("include".to_string(), json!("checklists"));
    let response = get(
        client,
        &format!("/issues/{}.json", path_segment(issue_id)),
        query,
    )?;
    Ok(json!({
        "issue_id": issue_id,
        "checklists": response.pointer("/issue/checklists").cloned().unwrap_or_else(|| json!([]))
    }))
}

fn add_checklist_item(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let checklist = pick_defined(args, &["subject", "is_done", "position"]);
    let response = update_checklist(client, args, issue_id, checklist)?;
    write_result(
        client,
        args,
        "add_checklist_item",
        json!({ "issue_id": issue_id }),
        response,
    )
}

fn update_checklist_item(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let checklist_id = required(args, "checklist_id")?;
    let mut checklist = pick_defined(args, &["subject", "is_done", "position"]);
    checklist.insert("id".to_string(), checklist_id.clone());
    let response = update_checklist(client, args, issue_id, checklist)?;
    write_result(
        client,
        args,
        "update_checklist_item",
        json!({ "issue_id": issue_id, "checklist_id": checklist_id }),
        response,
    )
}

fn delete_checklist_item(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let checklist_id = required(args, "checklist_id")?;
    let checklist = Map::from_iter([
        ("id".to_string(), checklist_id.clone()),
        ("_destroy".to_string(), Value::Bool(true)),
    ]);
    let response = update_checklist(client, args, issue_id, checklist)?;
    write_result(
        client,
        args,
        "delete_checklist_item",
        json!({ "issue_id": issue_id, "checklist_id": checklist_id }),
        response,
    )
}

fn get_time_entry(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let time_entry_id = required(args, "time_entry_id")?;
    get(
        client,
        &format!("/time_entries/{}.json", path_segment(time_entry_id)),
        Map::new(),
    )
}

fn add_time_entry(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    if optional(args, "issue_id").is_none() && optional(args, "project_id").is_none() {
        return Err(RedmineError::input(
            "Either issue_id or project_id is required",
        ));
    }
    let time_entry = pick_defined(
        args,
        &[
            "issue_id",
            "project_id",
            "spent_on",
            "hours",
            "activity_id",
            "comments",
            "user_id",
        ],
    );
    let response = request_json(
        client,
        "POST",
        "/time_entries.json",
        write_query(client, args),
        json!({ "time_entry": time_entry }),
    )?;
    write_result(
        client,
        args,
        "add_time_entry",
        target_from(&Value::Object(time_entry)),
        response,
    )
}

fn update_time_entry(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let time_entry_id = required(args, "time_entry_id")?;
    let fields = object_arg(args, "fields")?;
    let response = request_json(
        client,
        "PUT",
        &format!("/time_entries/{}.json", path_segment(time_entry_id)),
        write_query(client, args),
        json!({ "time_entry": fields }),
    )?;
    write_result(
        client,
        args,
        "update_time_entry",
        json!({ "time_entry_id": time_entry_id }),
        response,
    )
}

fn delete_time_entry(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    delete_with_target(
        client,
        args,
        "delete_time_entry",
        "time_entry_id",
        "/time_entries/{id}.json",
    )
}

fn list_versions(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    get(
        client,
        &format!("/projects/{}/versions.json", path_segment(project_id)),
        Map::new(),
    )
}

fn get_version(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let version_id = required(args, "version_id")?;
    get(
        client,
        &format!("/versions/{}.json", path_segment(version_id)),
        Map::new(),
    )
}

fn create_version(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    let version = pick_defined(
        args,
        &[
            "name",
            "description",
            "effective_date",
            "status",
            "sharing",
            "wiki_page_title",
        ],
    );
    let response = request_json(
        client,
        "POST",
        &format!("/projects/{}/versions.json", path_segment(project_id)),
        write_query(client, args),
        json!({ "version": version }),
    )?;
    write_result(
        client,
        args,
        "create_version",
        json!({ "project_id": project_id }),
        response,
    )
}

fn update_version(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let version_id = required(args, "version_id")?;
    let fields = object_arg(args, "fields")?;
    let response = request_json(
        client,
        "PUT",
        &format!("/versions/{}.json", path_segment(version_id)),
        write_query(client, args),
        json!({ "version": fields }),
    )?;
    write_result(
        client,
        args,
        "update_version",
        json!({ "version_id": version_id }),
        response,
    )
}

fn delete_version(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    delete_with_target(
        client,
        args,
        "delete_version",
        "version_id",
        "/versions/{id}.json",
    )
}

fn list_watchers(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let mut query = Map::new();
    query.insert("include".to_string(), json!("watchers"));
    let response = get(
        client,
        &format!("/issues/{}.json", path_segment(issue_id)),
        query,
    )?;
    Ok(json!({
        "issue_id": issue_id,
        "watchers": response.pointer("/issue/watchers").cloned().unwrap_or_else(|| json!([]))
    }))
}

fn add_watcher(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let user_id = required(args, "user_id")?;
    let response = request_json(
        client,
        "POST",
        &format!("/issues/{}/watchers.json", path_segment(issue_id)),
        write_query(client, args),
        json!({ "user_id": user_id }),
    )?;
    write_result(
        client,
        args,
        "add_watcher",
        json!({ "issue_id": issue_id, "user_id": user_id }),
        response,
    )
}

fn remove_watcher(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let issue_id = required(args, "issue_id")?;
    let user_id = required(args, "user_id")?;
    let response = client.request(
        "DELETE",
        &format!(
            "/issues/{}/watchers/{}.json",
            path_segment(issue_id),
            path_segment(user_id)
        ),
        RequestOptions {
            query: write_query(client, args),
            ..Default::default()
        },
    )?;
    write_result(
        client,
        args,
        "remove_watcher",
        json!({ "issue_id": issue_id, "user_id": user_id }),
        response,
    )
}

fn get_project(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    let mut query = Map::new();
    if let Some(include) = optional(args, "include") {
        query.insert("include".to_string(), include.clone());
    }
    get(
        client,
        &format!("/projects/{}.json", path_segment(project_id)),
        query,
    )
}

fn list_project_memberships(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    get(
        client,
        &format!("/projects/{}/memberships.json", path_segment(project_id)),
        pick_defined(args, &["offset", "limit"]),
    )
}

fn get_project_membership(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let membership_id = required(args, "membership_id")?;
    get(
        client,
        &format!("/memberships/{}.json", path_segment(membership_id)),
        Map::new(),
    )
}

fn list_wiki_pages(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    get(
        client,
        &format!("/projects/{}/wiki/index.json", path_segment(project_id)),
        Map::new(),
    )
}

fn get_wiki_page(client: &RedmineClient, args: &Map<String, Value>) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    let title = required(args, "title")?;
    let version_path = optional(args, "version")
        .map(|version| format!("/{}", path_segment(version)))
        .unwrap_or_default();
    let mut query = Map::new();
    if let Some(include) = wiki_includes(client, args.get("include")) {
        query.insert("include".to_string(), include);
    }
    get(
        client,
        &format!(
            "/projects/{}/wiki/{}{}.json",
            path_segment(project_id),
            path_segment(title),
            version_path
        ),
        query,
    )
}

fn list_issue_categories(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    get(
        client,
        &format!(
            "/projects/{}/issue_categories.json",
            path_segment(project_id)
        ),
        Map::new(),
    )
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

fn update_checklist(
    client: &RedmineClient,
    args: &Map<String, Value>,
    issue_id: &Value,
    checklist: Map<String, Value>,
) -> Result<crate::redmine::RedmineResponse, RedmineError> {
    request_json(
        client,
        "PUT",
        &format!("/issues/{}.json", path_segment(issue_id)),
        write_query(client, args),
        json!({ "issue": { "checklists_attributes": [Value::Object(checklist)] } }),
    )
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

fn attachment_write_result(
    client: &RedmineClient,
    args: &Map<String, Value>,
    operation: &str,
    target: Value,
    response: crate::redmine::RedmineResponse,
    upload: Value,
) -> Result<Value, RedmineError> {
    let mut result = Map::new();
    result.insert("ok".to_string(), Value::Bool(true));
    result.insert("operation".to_string(), json!(operation));
    result.insert("target".to_string(), target);
    result.insert("status".to_string(), json!(response.status));
    result.insert("upload".to_string(), upload);
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

fn wiki_includes(client: &RedmineClient, requested: Option<&Value>) -> Option<Value> {
    let includes = match requested {
        Some(Value::Array(values)) => values
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    let filtered = includes
        .into_iter()
        .filter(|include| include != "attachments" || !client.config.disabled_features.attachments)
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

fn attachment_max_bytes(client: &RedmineClient, args: &Map<String, Value>) -> u64 {
    args.get("max_bytes")
        .and_then(Value::as_u64)
        .filter(|value| *value > 0)
        .unwrap_or_else(|| {
            if client.config.attachment_max_bytes > 0 {
                client.config.attachment_max_bytes
            } else {
                DEFAULT_ATTACHMENT_MAX_BYTES
            }
        })
}

fn attachment_filename(args: &Map<String, Value>) -> Result<String, RedmineError> {
    let filename = value_to_string(required(args, "filename")?);
    if filename.contains('/') || filename.contains('\\') || filename == "." || filename == ".." {
        return Err(RedmineError::input(
            "filename must be a file name, not a path",
        ));
    }
    Ok(filename)
}

fn attachment_upload_bytes(args: &Map<String, Value>) -> Result<Vec<u8>, RedmineError> {
    let has_base64 = optional(args, "content_base64").is_some();
    let has_text = optional(args, "content").is_some();
    if has_base64 == has_text {
        return Err(RedmineError::input(
            "Exactly one of content_base64 or content is required",
        ));
    }
    if let Some(content) = optional(args, "content") {
        return Ok(value_to_string(content).into_bytes());
    }
    let normalized = value_to_string(required(args, "content_base64")?)
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    BASE64
        .decode(normalized)
        .map_err(|_| RedmineError::input("content_base64 must be valid base64"))
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
    use crate::config::DisabledFeatures;

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
