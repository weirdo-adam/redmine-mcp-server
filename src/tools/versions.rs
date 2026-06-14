use super::*;

pub(super) fn list_versions(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let project_id = required(args, "project_id")?;
    get(
        client,
        &format!("/projects/{}/versions.json", path_segment(project_id)),
        Map::new(),
    )
}

pub(super) fn get_version(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
    let version_id = required(args, "version_id")?;
    get(
        client,
        &format!("/versions/{}.json", path_segment(version_id)),
        Map::new(),
    )
}

pub(super) fn create_version(
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

pub(super) fn update_version(
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

pub(super) fn delete_version(
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
