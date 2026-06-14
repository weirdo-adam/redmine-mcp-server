use std::io::{self, BufRead, Write};

use redmine_mcp_server::config::Config;
use redmine_mcp_server::redmine::{RedmineClient, RedmineError};
use redmine_mcp_server::tools::{call_tool, error_payload, list_tools};
use serde_json::{json, Value};

const PROTOCOL_VERSION: &str = "2024-11-05";

fn main() {
    let config = Config::from_env();
    let client = RedmineClient::new(config);
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            continue;
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let message = match serde_json::from_str::<Value>(trimmed) {
            Ok(message) => message,
            Err(error) => {
                write_error(
                    &mut stdout,
                    Value::Null,
                    -32700,
                    &format!("Parse error: {error}"),
                );
                continue;
            }
        };

        match handle_message(&client, &message) {
            Ok(Some(result)) => {
                if let Some(id) = message.get("id") {
                    write_result(&mut stdout, id.clone(), result);
                }
            }
            Ok(None) => {}
            Err(error) => {
                if let Some(id) = message.get("id") {
                    let code = if error.input_error { -32602 } else { -32603 };
                    write_error(&mut stdout, id.clone(), code, &error.message);
                } else {
                    eprintln!("{}", error.message);
                }
            }
        }
    }
}

fn handle_message(client: &RedmineClient, message: &Value) -> Result<Option<Value>, RedmineError> {
    let method = message
        .get("method")
        .and_then(Value::as_str)
        .ok_or_else(|| RedmineError::input("method is required"))?;

    match method {
        "initialize" => Ok(Some(json!({
            "protocolVersion": message
                .pointer("/params/protocolVersion")
                .and_then(Value::as_str)
                .unwrap_or(PROTOCOL_VERSION),
            "capabilities": {
                "tools": {
                    "listChanged": false
                }
            },
            "serverInfo": {
                "name": "redmine",
                "version": env!("CARGO_PKG_VERSION")
            }
        }))),
        "tools/list" => Ok(Some(json!({ "tools": list_tools(&client.config) }))),
        "tools/call" => handle_tool_call(
            client,
            message.get("params").cloned().unwrap_or_else(|| json!({})),
        )
        .map(Some),
        "resources/list" => Ok(Some(json!({ "resources": [] }))),
        "prompts/list" => Ok(Some(json!({ "prompts": [] }))),
        "ping" => Ok(Some(json!({}))),
        "notifications/initialized" => Ok(None),
        _ => Err(RedmineError::input(format!("Unsupported method: {method}"))),
    }
}

fn handle_tool_call(client: &RedmineClient, params: Value) -> Result<Value, RedmineError> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| RedmineError::input("tools/call requires params.name"))?;
    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));

    match call_tool(client, name, args) {
        Ok(payload) => Ok(content_result(payload, false)),
        Err(error) if error.input_error => Err(error),
        Err(error) => Ok(content_result(error_payload(&error), true)),
    }
}

fn content_result(payload: Value, is_error: bool) -> Value {
    json!({
        "content": [
            {
                "type": "text",
                "text": serde_json::to_string_pretty(&payload).unwrap_or_else(|_| payload.to_string())
            }
        ],
        "isError": is_error
    })
}

fn write_result(stdout: &mut io::Stdout, id: Value, result: Value) {
    let _ = writeln!(
        stdout,
        "{}",
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        })
    );
    let _ = stdout.flush();
}

fn write_error(stdout: &mut io::Stdout, id: Value, code: i64, message: &str) {
    let _ = writeln!(
        stdout,
        "{}",
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": code,
                "message": message
            }
        })
    );
    let _ = stdout.flush();
}
