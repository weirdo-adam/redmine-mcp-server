use super::*;
use crate::config::DEFAULT_ATTACHMENT_MAX_BYTES;
use crate::redmine::{ResponseBody, ResponseType};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

pub(super) fn get_attachment(
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

pub(super) fn download_attachment(
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

pub(super) fn upload_attachment(
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

pub(super) fn delete_attachment(
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

pub(super) fn attachment_write_result(
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

pub(super) fn attachment_max_bytes(client: &RedmineClient, args: &Map<String, Value>) -> u64 {
    let configured_limit = if client.config.attachment_max_bytes > 0 {
        client.config.attachment_max_bytes
    } else {
        DEFAULT_ATTACHMENT_MAX_BYTES
    };

    args.get("max_bytes")
        .and_then(Value::as_u64)
        .filter(|value| *value > 0)
        .unwrap_or(configured_limit)
}

pub(super) fn attachment_filename(args: &Map<String, Value>) -> Result<String, RedmineError> {
    let filename = value_to_string(required(args, "filename")?);
    if filename.contains('/') || filename.contains('\\') || filename == "." || filename == ".." {
        return Err(RedmineError::input(
            "filename must be a file name, not a path",
        ));
    }
    Ok(filename)
}

pub(super) fn attachment_upload_bytes(args: &Map<String, Value>) -> Result<Vec<u8>, RedmineError> {
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
