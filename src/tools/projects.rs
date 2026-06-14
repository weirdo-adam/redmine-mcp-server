use super::*;

pub(super) fn get_project(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
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

pub(super) fn list_project_memberships(
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

pub(super) fn get_project_membership(
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

pub(super) fn list_issue_categories(
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
