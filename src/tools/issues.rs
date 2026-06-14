use super::*;

pub(super) fn get_issue(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
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

pub(super) fn list_issues(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    get(client, "/issues.json", filter_args(args))
}

pub(super) fn create_issue(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
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

pub(super) fn update_issue(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
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

pub(super) fn delete_issue(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    delete_with_target(
        client,
        args,
        "delete_issue",
        "issue_id",
        "/issues/{id}.json",
    )
}
