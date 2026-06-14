use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::Value;

#[test]
fn stdio_server_answers_initialize_and_tools_list() {
    let responses = run_server(&[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    ]);

    assert_eq!(responses[0]["result"]["serverInfo"]["name"], "redmine");
    assert_eq!(
        responses[0]["result"]["serverInfo"]["version"],
        env!("CARGO_PKG_VERSION")
    );

    let tools = responses[1]["result"]["tools"].as_array().unwrap();
    assert!(tools.iter().any(|tool| tool["name"] == "redmine_get_issue"));
    assert!(tools.iter().any(|tool| tool["name"] == "redmine_search"));
    assert!(!tools
        .iter()
        .any(|tool| tool["name"] == "redmine_delete_issue"));
}

#[test]
fn read_only_mode_rejects_write_tool_without_redmine_request() {
    let responses = run_server(&[
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"redmine_create_issue","arguments":{"project_id":"demo","subject":"blocked"}}}"#,
    ]);

    assert_eq!(responses[0]["result"]["isError"], true);
    assert!(responses[0]["result"]["content"][0]["text"]
        .as_str()
        .unwrap()
        .contains("REDMINE_MCP_READ_ONLY is enabled"));
}

fn run_server(messages: &[&str]) -> Vec<Value> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_redmine-mcp-server"))
        .env("REDMINE_BASE_URL", "https://redmine.example.com")
        .env("REDMINE_API_KEY", "secret")
        .env("REDMINE_MCP_READ_ONLY", "true")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = child.stdin.take().unwrap();
        for message in messages {
            writeln!(&mut stdin, "{message}").unwrap();
        }
    }

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
