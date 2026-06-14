use std::env;

pub const DEFAULT_ATTACHMENT_MAX_BYTES: u64 = 10_485_760;
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;

#[derive(Clone, Debug, Default)]
pub struct DisabledFeatures {
    pub attachments: bool,
    pub checklists: bool,
    pub relations: bool,
    pub time_entries: bool,
    pub versions: bool,
    pub wiki: bool,
    pub watchers: bool,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub read_only: bool,
    pub enable_deletes: bool,
    pub silent_writes: bool,
    pub disabled_features: DisabledFeatures,
    pub attachment_max_bytes: u64,
    pub timeout_ms: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            base_url: trim_trailing_slash(&env::var("REDMINE_BASE_URL").unwrap_or_default()),
            api_key: env::var("REDMINE_API_KEY").unwrap_or_default(),
            read_only: parse_bool(env::var("REDMINE_MCP_READ_ONLY").ok().as_deref(), false),
            enable_deletes: parse_bool(
                env::var("REDMINE_MCP_ENABLE_DELETES").ok().as_deref(),
                false,
            ),
            silent_writes: parse_bool(env::var("REDMINE_SILENT_WRITES").ok().as_deref(), false),
            disabled_features: DisabledFeatures {
                attachments: parse_bool(
                    env::var("REDMINE_MCP_DISABLE_ATTACHMENTS").ok().as_deref(),
                    false,
                ),
                checklists: parse_bool(
                    env::var("REDMINE_MCP_DISABLE_CHECKLISTS").ok().as_deref(),
                    false,
                ),
                relations: parse_bool(
                    env::var("REDMINE_MCP_DISABLE_RELATIONS").ok().as_deref(),
                    false,
                ),
                time_entries: parse_bool(
                    env::var("REDMINE_MCP_DISABLE_TIME_ENTRIES").ok().as_deref(),
                    false,
                ),
                versions: parse_bool(
                    env::var("REDMINE_MCP_DISABLE_VERSIONS").ok().as_deref(),
                    false,
                ),
                wiki: parse_bool(env::var("REDMINE_MCP_DISABLE_WIKI").ok().as_deref(), false),
                watchers: parse_bool(
                    env::var("REDMINE_MCP_DISABLE_WATCHERS").ok().as_deref(),
                    false,
                ),
            },
            attachment_max_bytes: positive_number(
                env::var("REDMINE_MCP_ATTACHMENT_MAX_BYTES").ok().as_deref(),
                DEFAULT_ATTACHMENT_MAX_BYTES,
            ),
            timeout_ms: positive_number(
                env::var("REDMINE_TIMEOUT_MS").ok().as_deref(),
                DEFAULT_TIMEOUT_MS,
            ),
        }
    }
}

pub fn parse_bool(value: Option<&str>, fallback: bool) -> bool {
    let Some(value) = value else {
        return fallback;
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return fallback;
    }
    matches!(
        trimmed.to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

pub fn positive_number(value: Option<&str>, fallback: u64) -> u64 {
    value
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(fallback)
}

fn trim_trailing_slash(value: &str) -> String {
    value.trim_end_matches('/').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_boolean_values() {
        assert!(parse_bool(Some("true"), false));
        assert!(parse_bool(Some("1"), false));
        assert!(parse_bool(Some("yes"), false));
        assert!(parse_bool(Some("on"), false));
        assert!(!parse_bool(Some("false"), false));
        assert!(parse_bool(Some(""), true));
    }

    #[test]
    fn parses_positive_numbers() {
        assert_eq!(positive_number(Some("2048"), 10), 2048);
        assert_eq!(positive_number(Some("0"), 10), 10);
        assert_eq!(positive_number(Some("invalid"), 10), 10);
    }
}
