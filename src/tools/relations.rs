use super::*;

pub(super) fn list_issue_relations(
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

pub(super) fn get_issue_relation(
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

pub(super) fn add_issue_relation(
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

pub(super) fn delete_issue_relation(
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
