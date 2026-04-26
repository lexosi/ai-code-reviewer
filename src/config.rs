use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub anthropic_api_key: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    #[serde(default = "default_max_diff_chars")]
    pub max_diff_chars: usize,
}

fn default_max_diff_chars() -> usize {
    8000
}

impl Config {
    pub fn load() -> Result<Self> {
        let exe = std::env::current_exe().context("failed to get binary path")?;
        let dir = exe.parent().context("binary has no parent directory")?;
        let path = dir.join("config.toml");
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        Self::from_toml_str(&contents)
    }

    pub(crate) fn from_toml_str(s: &str) -> Result<Self> {
        toml::from_str(s).context("failed to parse config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_CONFIG: &str = r#"
        anthropic_api_key  = "sk-ant-test"
        telegram_bot_token = "123:TOKEN"
        telegram_chat_id   = "-100999"
        max_diff_chars     = 4000
    "#;

    const MINIMAL_CONFIG: &str = r#"
        anthropic_api_key  = "sk-ant-min"
        telegram_bot_token = "456:MIN"
        telegram_chat_id   = "777"
    "#;

    #[test]
    fn parses_all_fields() {
        let cfg = Config::from_toml_str(FULL_CONFIG).unwrap();
        assert_eq!(cfg.anthropic_api_key, "sk-ant-test");
        assert_eq!(cfg.telegram_bot_token, "123:TOKEN");
        assert_eq!(cfg.telegram_chat_id, "-100999");
        assert_eq!(cfg.max_diff_chars, 4000);
    }

    #[test]
    fn max_diff_chars_defaults_to_8000() {
        let cfg = Config::from_toml_str(MINIMAL_CONFIG).unwrap();
        assert_eq!(cfg.max_diff_chars, 8000);
    }

    #[test]
    fn missing_required_field_returns_error() {
        let bad = r#"telegram_bot_token = "x" \n telegram_chat_id = "y""#;
        assert!(Config::from_toml_str(bad).is_err());
    }

    #[test]
    fn empty_string_returns_error() {
        assert!(Config::from_toml_str("").is_err());
    }
}
