use super::*;

pub(super) fn list_checklists(
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

pub(super) fn add_checklist_item(
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

pub(super) fn update_checklist_item(
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

pub(super) fn delete_checklist_item(
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

pub(super) fn update_checklist(
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
