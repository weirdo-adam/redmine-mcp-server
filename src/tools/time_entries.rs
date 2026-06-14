use super::*;

pub(super) fn get_time_entry(
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

pub(super) fn add_time_entry(
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

pub(super) fn update_time_entry(
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

pub(super) fn delete_time_entry(
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
