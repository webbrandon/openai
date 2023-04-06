use serde::{Deserialize, Serialize};
use crate::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionsRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub user: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    pub top_p: f32,
    pub n: u32,
    pub stream: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    pub echo: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub best_of: u32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<String>,
}

impl OpenAICompletionsRequest {
    pub fn new(model: String, prompt: String, max_tokens: usize, temperature: f32, user: String, suffix: Option<String>, top_p: f32, n: u32, stream: bool, logprobs: Option<u32>, echo: bool, stop: Option<Vec<String>>, presence_penalty: f32, frequency_penalty: f32, best_of: u32, logit_bias: Option<String>) -> Self { Self { model, prompt, max_tokens, temperature, user, suffix, top_p, n, stream, logprobs, echo, stop, presence_penalty, frequency_penalty, best_of, logit_bias } }

    pub fn process_response(self, response_body: String) -> OpenAIResponse {
            let chat_response: OpenAICompletionsResponse = match serde_json::from_str(&response_body) {
                Ok(res) => {
                    res
                },
                Err(error) => {
                    error!("Error formatting response body: {:#?}", error);
                    std::process::exit(1)
                }
            };
            OpenAIResponse::OpenAICompletionsResponse(chat_response)
    }
}
