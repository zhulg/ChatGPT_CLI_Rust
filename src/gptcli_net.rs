use reqwest::{self};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct GptRequestParams<'a> {
    pub url: &'a str,
    pub api_key: &'a str,
    pub max_tokens: u32,
    pub model: &'a str,
    pub temperature: f32,
    pub messages: &'a Vec<GptMessage>,
    pub cancel_request: &'a Arc<AtomicBool>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GptRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    messages: Vec<GptMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GptMessage {
    pub role: String,
    pub content: String,
}

pub async fn send_gpt_request(params: &GptRequestParams<'_>) -> Result<String, Box<dyn Error>> {
    let request = GptRequest {
        model: params.model.to_string(),
        max_tokens: params.max_tokens,
        temperature: params.temperature,
        messages: params.messages.clone(),
    };

    // dbg!(request.clone());
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

    if params.cancel_request.load(Ordering::SeqCst) {
        println!("Request cancelled by user 2 ...");
        return Err(Box::new(GptError {
            message: "Request cancelled by user...".to_string(),
        }));
    }

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
