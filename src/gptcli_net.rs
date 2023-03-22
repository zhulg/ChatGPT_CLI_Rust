use reqwest::{self};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct GptRequestParams<'a> {
    pub url: &'a str,
    pub api_key: &'a str,
    pub line: &'a str,
    pub max_tokens: u32,
    pub model: &'a str,
    pub temperature: f32,
}

#[derive(Debug)]
struct GptError {
    message: String,
}

impl Display for GptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GptError: {}", self.message)
    }
}

impl Error for GptError {}

#[derive(Debug, Serialize, Deserialize)]
struct GptRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    messages: Vec<GptMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GptMessage {
    role: String,
    content: String,
}

pub async fn send_gpt_request(params: GptRequestParams<'_>) -> Result<String, Box<dyn Error>> {
    let request = GptRequest {
        model: params.model.to_string(),
        max_tokens: params.max_tokens,
        temperature: params.temperature,
        messages: vec![GptMessage {
            role: "user".to_string(),
            content: params.line.to_string(),
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(params.url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", params.api_key))
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;

    // dbg!(response);
    // Ok("".to_string())

    let choices = response
        .get("choices")
        .and_then(|v| v.as_array())
        .ok_or_else(|| GptError {
            message: "Response doesn't contain 'choices' field".to_string(),
        })?;

    let message = choices
        .get(0)
        .and_then(|v| v.get("message"))
        .ok_or_else(|| GptError {
            message: "Response doesn't contain 'message' field".to_string(),
        })?;

    let response_content = message
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned();

    Ok(response_content)
}
