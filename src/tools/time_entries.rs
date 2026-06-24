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
    let time_entry = build_time_entry(client, args)?;
    let target = target_from(&Value::Object(time_entry.clone()));

    let response = request_json(
        client,
        "POST",
        "/time_entries.json",
        write_query(client, args),
        json!({ "time_entry": time_entry }),
    )?;
    write_result(client, args, "add_time_entry", target, response)
}

fn build_time_entry(
    client: &RedmineClient,
    args: &Map<String, Value>,
) -> Result<Map<String, Value>, RedmineError> {
    if optional(args, "issue_id").is_none() && optional(args, "project_id").is_none() {
        return Err(RedmineError::input(
            "Either issue_id or project_id is required",
        ));
    }
    let mut time_entry = optional_object_arg(args, "fields")?;
    merge_pick(
        &mut time_entry,
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

    if !time_entry.contains_key("activity_id") {
        if let Some(activity_name) = optional(args, "activity_name") {
            time_entry.insert(
                "activity_id".to_string(),
                resolve_time_entry_activity_id(client, activity_name)?,
            );
        }
    }

    if !time_entry.contains_key("activity_id") {
        return Err(RedmineError::input(
            "Either activity_id or activity_name is required",
        ));
    }

    Ok(time_entry)
}

fn resolve_time_entry_activity_id(
    client: &RedmineClient,
    activity_name: &Value,
) -> Result<Value, RedmineError> {
    let activity_name = value_to_string(activity_name);
    let normalized = normalize_activity_name(&activity_name);
    if normalized.is_empty() {
        return Err(RedmineError::input("activity_name is required"));
    }

    let response = get(
        client,
        "/enumerations/time_entry_activities.json",
        Map::new(),
    )?;
    let activities = response
        .get("time_entry_activities")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            RedmineError::runtime(
                "Redmine time entry activities response did not include time_entry_activities",
            )
        })?;

    let matches = activities
        .iter()
        .filter(|activity| {
            activity
                .get("name")
                .map(value_to_string)
                .map(|name| normalize_activity_name(&name) == normalized)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    match matches.as_slice() {
        [activity] => activity
            .get("id")
            .cloned()
            .ok_or_else(|| RedmineError::runtime("Matched time entry activity has no id")),
        [] => Err(RedmineError::input(format!(
            "Unknown time entry activity: {activity_name}"
        ))),
        _ => Err(RedmineError::input(format!(
            "Multiple time entry activities matched: {activity_name}"
        ))),
    }
}

fn normalize_activity_name(value: &str) -> String {
    value.trim().to_ascii_lowercase()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DisabledFeatures, DEFAULT_ATTACHMENT_MAX_BYTES};
    use serde_json::json;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;
    use std::thread;

    fn client() -> RedmineClient {
        RedmineClient::new(Config {
            base_url: "https://redmine.example.com".to_string(),
            api_key: "secret".to_string(),
            read_only: false,
            enable_deletes: false,
            silent_writes: false,
            disabled_features: DisabledFeatures::default(),
            attachment_max_bytes: DEFAULT_ATTACHMENT_MAX_BYTES,
            timeout_ms: 30_000,
        })
    }

    #[test]
    fn build_time_entry_merges_fields_and_explicit_values() {
        let args = Map::from_iter([
            ("issue_id".to_string(), json!(42)),
            ("hours".to_string(), json!(2)),
            ("activity_id".to_string(), json!(7)),
            ("comments".to_string(), json!("Investigated timeout")),
            (
                "fields".to_string(),
                json!({
                    "custom_fields": [
                        {
                            "id": 3,
                            "value": "backend"
                        }
                    ],
                    "comments": "from fields"
                }),
            ),
        ]);

        let time_entry = build_time_entry(&client(), &args).unwrap();

        assert_eq!(time_entry["issue_id"], json!(42));
        assert_eq!(time_entry["hours"], json!(2));
        assert_eq!(time_entry["activity_id"], json!(7));
        assert_eq!(time_entry["comments"], json!("Investigated timeout"));
        assert_eq!(time_entry["custom_fields"][0]["value"], json!("backend"));
    }

    #[test]
    fn build_time_entry_requires_activity_identifier_or_name() {
        let args = Map::from_iter([
            ("issue_id".to_string(), json!(42)),
            ("hours".to_string(), json!(2)),
        ]);

        let error = build_time_entry(&client(), &args).unwrap_err();

        assert_eq!(
            error.message,
            "Either activity_id or activity_name is required"
        );
        assert!(error.input_error);
    }

    #[test]
    fn add_time_entry_resolves_activity_name_before_posting() {
        let server = MockRedmine::start();
        let mut client = client();
        client.config.base_url = server.base_url();
        let args = json!({
            "issue_id": 42,
            "hours": 2,
            "activity_name": "Development",
            "comments": "Investigated timeout",
            "fields": {
                "custom_fields": [
                    {
                        "id": 3,
                        "value": "backend"
                    }
                ]
            },
            "silent": true
        });

        let result = crate::tools::call_tool(&client, "redmine_add_time_entry", args).unwrap();

        assert_eq!(result["ok"], json!(true));
        let observed = server.join();
        assert_eq!(observed[0], "GET /enumerations/time_entry_activities.json");
        assert_eq!(observed[1], "POST /time_entries.json?notify=false");
        assert_eq!(
            observed[2],
            json!({
                "time_entry": {
                    "activity_id": 7,
                    "comments": "Investigated timeout",
                    "custom_fields": [
                        {
                            "id": 3,
                            "value": "backend"
                        }
                    ],
                    "hours": 2,
                    "issue_id": 42
                }
            })
            .to_string()
        );
    }

    struct MockRedmine {
        address: std::net::SocketAddr,
        handle: thread::JoinHandle<Vec<String>>,
    }

    impl MockRedmine {
        fn start() -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let handle = thread::spawn(move || {
                let mut observed = Vec::new();

                let (stream, _) = listener.accept().unwrap();
                let mut stream = BufReader::new(stream);
                let mut request_line = String::new();
                stream.read_line(&mut request_line).unwrap();
                observed.push(request_line.trim_end().replace(" HTTP/1.1", ""));
                read_headers_and_body(&mut stream);
                write_json_response(
                    stream.get_mut(),
                    200,
                    r#"{"time_entry_activities":[{"id":7,"name":"Development"},{"id":8,"name":"Testing"}]}"#,
                );
                drop(stream);

                let (stream, _) = listener.accept().unwrap();
                let mut stream = BufReader::new(stream);
                let mut request_line = String::new();
                stream.read_line(&mut request_line).unwrap();
                observed.push(request_line.trim_end().replace(" HTTP/1.1", ""));
                observed.push(read_headers_and_body(&mut stream));
                write_json_response(stream.get_mut(), 201, "{}");
                drop(stream);

                observed
            });

            Self { address, handle }
        }

        fn base_url(&self) -> String {
            format!("http://{}", self.address)
        }

        fn join(self) -> Vec<String> {
            self.handle.join().unwrap()
        }
    }

    fn read_headers_and_body(reader: &mut BufReader<std::net::TcpStream>) -> String {
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                break;
            }
            if let Some((name, value)) = trimmed.split_once(':') {
                if name.eq_ignore_ascii_case("content-length") {
                    content_length = value.trim().parse().unwrap();
                }
            }
        }

        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        String::from_utf8(body).unwrap()
    }

    fn write_json_response(stream: &mut std::net::TcpStream, status: u16, body: &str) {
        let reason = match status {
            200 => "OK",
            201 => "Created",
            _ => "OK",
        };
        write!(
            stream,
            "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
        .unwrap();
    }
}
