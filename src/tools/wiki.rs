use super::*;

pub(super) fn list_wiki_pages(
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

pub(super) fn get_wiki_page(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Value, RedmineError> {
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

pub(super) fn wiki_includes(client: &RedmineClient, requested: Option<&Value>) -> Option<Value> {
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
