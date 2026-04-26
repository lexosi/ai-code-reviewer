use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde::Serialize;

const MAX_MESSAGE_LEN: usize = 4096;
const TRUNCATION_SUFFIX: &str = "… (truncated)";

#[derive(Serialize)]
struct SendMessage<'a> {
    chat_id: &'a str,
    text: &'a str,
}

pub(crate) fn prepare_text(text: &str) -> String {
    if text.len() > MAX_MESSAGE_LEN {
        let cut = MAX_MESSAGE_LEN - TRUNCATION_SUFFIX.len();
        format!("{}{TRUNCATION_SUFFIX}", &text[..cut])
    } else {
        text.to_owned()
    }
}

pub fn send_message(bot_token: &str, chat_id: &str, text: &str) -> Result<()> {
    let body_text = prepare_text(text);
    let url = format!("https://api.telegram.org/bot{bot_token}/sendMessage");
    let body = SendMessage { chat_id, text: &body_text };

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&body)
        .send()
        .context("failed to send Telegram message")?;

    let status = response.status();
    if !status.is_success() {
        let resp_text = response.text().unwrap_or_default();
        bail!("Telegram API returned {status}: {resp_text}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_is_unchanged() {
        let text = "Hello, world!";
        assert_eq!(prepare_text(text), text);
    }

    #[test]
    fn text_at_exact_limit_is_unchanged() {
        let text = "a".repeat(MAX_MESSAGE_LEN);
        let result = prepare_text(&text);
        assert_eq!(result, text);
        assert_eq!(result.len(), MAX_MESSAGE_LEN);
    }

    #[test]
    fn text_over_limit_is_truncated() {
        let text = "b".repeat(MAX_MESSAGE_LEN + 100);
        let result = prepare_text(&text);
        assert!(result.len() <= MAX_MESSAGE_LEN + TRUNCATION_SUFFIX.len());
        assert!(result.ends_with(TRUNCATION_SUFFIX));
    }

    #[test]
    fn truncated_text_ends_with_suffix() {
        let text = "x".repeat(5000);
        let result = prepare_text(&text);
        assert!(result.ends_with(TRUNCATION_SUFFIX));
    }

    #[test]
    fn empty_text_is_unchanged() {
        assert_eq!(prepare_text(""), "");
    }
}
