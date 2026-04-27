use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const MODEL: &str = "claude-haiku-4-5-20251001";
const API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";

const SYSTEM_PROMPT: &str = "\
Eres un experto revisor de código. Analiza el git diff proporcionado y da una revisión concisa.

Enfócate en:
- Errores de lógica y bugs
- Vulnerabilidades de seguridad (inyección, problemas de autenticación, secretos expuestos, deserialización insegura)
- Problemas de rendimiento
- Mejoras de calidad y mantenibilidad del código

Formatea tu respuesta con secciones claras. Sé directo y concreto. \
Si el diff se ve limpio, dilo brevemente.";

#[derive(Serialize)]
struct Request<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct Response {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct ContentBlock {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
}

pub(crate) fn extract_text_from_response(json: &str) -> Result<String> {
    let parsed: Response = serde_json::from_str(json).context("failed to parse response JSON")?;
    parsed
        .content
        .into_iter()
        .find(|b| b.kind == "text")
        .and_then(|b| b.text)
        .context("response contained no text block")
}

pub fn review_diff(api_key: &str, diff: &str) -> Result<String> {
    let prompt = format!("Please review this git diff:\n\n```diff\n{diff}\n```");

    let body = Request {
        model: MODEL,
        max_tokens: 1024,
        system: SYSTEM_PROMPT,
        messages: vec![Message { role: "user", content: &prompt }],
    };

    let client = Client::new();
    let response = client
        .post(API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .json(&body)
        .send()
        .context("failed to send request to Claude API")?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        bail!("Claude API returned {status}: {text}");
    }

    let text = response.text().context("failed to read Claude API response body")?;
    extract_text_from_response(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_text_block_from_valid_response() {
        let json = r#"{
            "content": [
                {"type": "text", "text": "Looks good to me!"}
            ]
        }"#;
        let result = extract_text_from_response(json).unwrap();
        assert_eq!(result, "Looks good to me!");
    }

    #[test]
    fn skips_non_text_blocks_and_finds_text() {
        let json = r#"{
            "content": [
                {"type": "tool_use", "text": null},
                {"type": "text", "text": "Review complete."}
            ]
        }"#;
        let result = extract_text_from_response(json).unwrap();
        assert_eq!(result, "Review complete.");
    }

    #[test]
    fn empty_content_array_returns_error() {
        let json = r#"{"content": []}"#;
        assert!(extract_text_from_response(json).is_err());
    }

    #[test]
    fn no_text_block_returns_error() {
        let json = r#"{
            "content": [
                {"type": "tool_use", "text": null}
            ]
        }"#;
        assert!(extract_text_from_response(json).is_err());
    }

    #[test]
    fn invalid_json_returns_error() {
        assert!(extract_text_from_response("not json").is_err());
    }

    #[test]
    fn prompt_contains_diff() {
        let diff = "+fn foo() {}";
        let prompt = format!("Please review this git diff:\n\n```diff\n{diff}\n```");
        assert!(prompt.contains("+fn foo() {}"));
        assert!(prompt.contains("```diff"));
    }
}
