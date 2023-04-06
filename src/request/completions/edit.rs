use serde::{Deserialize, Serialize};
use crate::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionEditRequest {
    pub model: String,
    pub input: String,
    pub instruction: String,
    pub temperature: f32,
    pub top_p: f32,
    pub n: u32,
}

impl OpenAICompletionEditRequest {
    pub fn new(model: String, input: String, instruction: String, temperature: f32, top_p: f32, n: u32) -> Self { Self { model, input, instruction, temperature, top_p, n } }

    pub fn process_response(self, response_body: String) -> OpenAIResponse {
            let chat_response: OpenAICompletionEditResponse = match serde_json::from_str(&response_body) {
                Ok(res) => {
                    res
                },
                Err(error) => {
                    error!("Error formatting response body: {:#?}", error);
                    std::process::exit(1)
                }
            };
            OpenAIResponse::OpenAICompletionEditResponse(chat_response)
    }
}
