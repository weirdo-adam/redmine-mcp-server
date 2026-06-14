use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json::{Map, Value};
use url::Url;

use crate::config::Config;

#[derive(Clone, Debug)]
pub struct RedmineError {
    pub message: String,
    pub status: Option<u16>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub details: Option<Value>,
    pub input_error: bool,
}

impl RedmineError {
    pub fn input(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: None,
            method: None,
            path: None,
            details: None,
            input_error: true,
        }
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: None,
            method: None,
            path: None,
            details: None,
            input_error: false,
        }
    }

    fn request(message: impl Into<String>, method: &str, path: &str) -> Self {
        Self {
            message: message.into(),
            status: None,
            method: Some(method.to_string()),
            path: Some(path.to_string()),
            details: None,
            input_error: false,
        }
    }

    fn response(
        message: impl Into<String>,
        status: u16,
        method: &str,
        path: &str,
        details: Value,
    ) -> Self {
        Self {
            message: message.into(),
            status: Some(status),
            method: Some(method.to_string()),
            path: Some(path.to_string()),
            details: Some(details),
            input_error: false,
        }
    }
}

impl std::fmt::Display for RedmineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for RedmineError {}

#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    pub query: Map<String, Value>,
    pub body: Option<Value>,
    pub raw_body: Option<Vec<u8>>,
    pub content_type: Option<String>,
    pub accept: Option<String>,
    pub response_type: ResponseType,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResponseType {
    #[default]
    Json,
    Buffer,
}

#[derive(Clone, Debug)]
pub enum ResponseBody {
    Json(Value),
    Buffer(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct RedmineResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: ResponseBody,
}

#[derive(Clone)]
pub struct RedmineClient {
    pub config: Config,
    agent: ureq::Agent,
}

impl RedmineClient {
    pub fn new(config: Config) -> Self {
        let agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_millis(config.timeout_ms)))
            .build()
            .into();
        Self { config, agent }
    }

    pub fn request(
        &self,
        method: &str,
        path: &str,
        options: RequestOptions,
    ) -> Result<RedmineResponse, RedmineError> {
        self.assert_configured()?;

        let url = build_url(&self.config.base_url, path, &options.query)?;
        let request = ureq::http::Request::builder()
            .method(method)
            .uri(&url)
            .header("X-Redmine-API-Key", self.config.api_key.as_str())
            .header(
                "Accept",
                options.accept.as_deref().unwrap_or("application/json"),
            );

        let result = if let Some(raw_body) = options.raw_body.as_ref() {
            let request = request
                .header(
                    "Content-Type",
                    options
                        .content_type
                        .as_deref()
                        .unwrap_or("application/octet-stream"),
                )
                .body(raw_body.as_slice())
                .map_err(|error| {
                    RedmineError::request(format!("Invalid Redmine request: {error}"), method, path)
                })?;
            self.agent.run(request)
        } else if let Some(body) = options.body.as_ref() {
            let request = request
                .header("Content-Type", "application/json")
                .body(body.to_string())
                .map_err(|error| {
                    RedmineError::request(format!("Invalid Redmine request: {error}"), method, path)
                })?;
            self.agent.run(request)
        } else {
            let request = request.body(()).map_err(|error| {
                RedmineError::request(format!("Invalid Redmine request: {error}"), method, path)
            })?;
            self.agent.run(request)
        };

        match result {
            Ok(response)
                if response.status().is_client_error() || response.status().is_server_error() =>
            {
                let status = response.status().as_u16();
                let parsed = parse_response(method, path, response, ResponseType::Json)
                    .map(|response| response.body.as_value())
                    .unwrap_or_else(|_| Value::Null);
                Err(RedmineError::response(
                    format!("Redmine {method} {path} returned HTTP {status}"),
                    status,
                    method,
                    path,
                    parsed,
                ))
            }
            Ok(response) => parse_response(method, path, response, options.response_type),
            Err(error) => Err(RedmineError::request(
                format!("Redmine request failed: {error}"),
                method,
                path,
            )),
        }
    }

    fn assert_configured(&self) -> Result<(), RedmineError> {
        if self.config.base_url.is_empty() {
            return Err(RedmineError::input("REDMINE_BASE_URL is not configured"));
        }
        if self.config.api_key.is_empty() {
            return Err(RedmineError::input("REDMINE_API_KEY is not configured"));
        }
        Ok(())
    }
}

impl ResponseBody {
    pub fn as_value(self) -> Value {
        match self {
            Self::Json(value) => value,
            Self::Buffer(bytes) => Value::String(String::from_utf8_lossy(&bytes).into_owned()),
        }
    }
}

pub fn build_url(
    base_url: &str,
    path: &str,
    query: &Map<String, Value>,
) -> Result<String, RedmineError> {
    let normalized_path = path.strip_prefix('/').unwrap_or(path);
    let mut url = Url::parse(&format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        normalized_path
    ))
    .map_err(|error| RedmineError::input(format!("Invalid Redmine URL: {error}")))?;

    for (key, value) in query {
        if should_skip_query_value(value) {
            continue;
        }
        url.query_pairs_mut().append_pair(key, &query_value(value));
    }

    Ok(url.to_string())
}

pub fn path_segment(value: &Value) -> String {
    utf8_percent_encode(&value_to_string(value), NON_ALPHANUMERIC).to_string()
}

pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        Value::Number(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn parse_response(
    method: &str,
    path: &str,
    response: ureq::http::Response<ureq::Body>,
    response_type: ResponseType,
) -> Result<RedmineResponse, RedmineError> {
    let status = response.status().as_u16();
    let mut headers = HashMap::new();
    if let Some(content_type) = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
    {
        headers.insert("content-type".to_string(), content_type.to_string());
    }

    if status == 204 {
        return Ok(RedmineResponse {
            status,
            headers,
            body: ResponseBody::Json(Value::Null),
        });
    }

    if response_type == ResponseType::Buffer {
        let mut bytes = Vec::new();
        let (_, body) = response.into_parts();
        body.into_reader()
            .read_to_end(&mut bytes)
            .map_err(|error| {
                RedmineError::request(
                    format!("Redmine response read failed: {error}"),
                    method,
                    path,
                )
            })?;
        return Ok(RedmineResponse {
            status,
            headers,
            body: ResponseBody::Buffer(bytes),
        });
    }

    let (_, body) = response.into_parts();
    let mut text = String::new();
    body.into_reader()
        .read_to_string(&mut text)
        .map_err(|error| {
            RedmineError::request(
                format!("Redmine response read failed: {error}"),
                method,
                path,
            )
        })?;
    let body = if text.is_empty() {
        Value::Null
    } else {
        serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text))
    };

    Ok(RedmineResponse {
        status,
        headers,
        body: ResponseBody::Json(body),
    })
}

fn should_skip_query_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(value) => value.is_empty(),
        _ => false,
    }
}

fn query_value(value: &Value) -> String {
    match value {
        Value::Array(values) => values
            .iter()
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(","),
        other => value_to_string(other),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn build_url_trims_base_url_and_serializes_arrays() {
        let query = Map::from_iter([("include".to_string(), json!(["watchers", "checklists"]))]);
        assert_eq!(
            build_url("https://redmine.example.com/", "/issues/42.json", &query).unwrap(),
            "https://redmine.example.com/issues/42.json?include=watchers%2Cchecklists"
        );
    }

    #[test]
    fn encodes_path_segments() {
        assert_eq!(path_segment(&json!("Wiki Page")), "Wiki%20Page");
    }
}
