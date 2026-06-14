use super::*;

pub(super) fn list_watchers(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
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

pub(super) fn add_watcher(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
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

pub(super) fn remove_watcher(
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
