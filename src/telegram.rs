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
        let head = crate::util::truncate_on_char_boundary(text, cut);
        format!("{head}{TRUNCATION_SUFFIX}")
    } else {
        text.to_owned()
    }
}

pub fn send_message(bot_token: &str, chat_id: &str, text: &str) -> Result<()> {
    let body_text = prepare_text(text);
    let url = format!("https://api.telegram.org/bot{bot_token}/sendMessage");
    let body = SendMessage {
        chat_id,
        text: &body_text,
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("failed to build HTTP client")?;
    let response = client
        .post(&url)
        .json(&body)
        .send()
        .map_err(|e| e.without_url())
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

    #[test]
    fn multibyte_over_limit_truncates_without_panic() {
        // 'á' is 2 bytes in UTF-8; repeating MAX_MESSAGE_LEN times yields
        // ~2 * 4096 bytes, well over MAX_MESSAGE_LEN, and the naive byte cut
        // at MAX_MESSAGE_LEN - suffix_len lands mid-char (odd/even boundary).
        let text = "á".repeat(MAX_MESSAGE_LEN);
        assert!(text.len() > MAX_MESSAGE_LEN);

        // Must not panic on the mid-char byte boundary.
        let result = prepare_text(&text);

        // `result` is a String, hence always valid UTF-8; assert the contract.
        assert!(result.ends_with(TRUNCATION_SUFFIX));
        assert!(result.len() <= MAX_MESSAGE_LEN + TRUNCATION_SUFFIX.len());
    }

    #[test]
    fn emoji_over_limit_truncates_without_panic() {
        // '😀' is 4 bytes in UTF-8; the byte cut can land on any of 3 mid-char
        // offsets, exercising the boundary walk-back.
        let text = "😀".repeat(MAX_MESSAGE_LEN);
        assert!(text.len() > MAX_MESSAGE_LEN);

        let result = prepare_text(&text);

        assert!(result.ends_with(TRUNCATION_SUFFIX));
        assert!(result.len() <= MAX_MESSAGE_LEN + TRUNCATION_SUFFIX.len());
    }
}
