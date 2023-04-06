
use serde::{Deserialize, Serialize};
use crate::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIImagesRequest {
    pub prompt: Option<String>,
    pub n: u32,
    pub size: String,
    pub response_format: String,
    pub user: Option<String>,
}

impl OpenAIImagesRequest {
    pub fn new(prompt: Option<String>, n: u32, size: String, response_format: String, user: Option<String>) -> Self { Self { prompt, n, size, response_format, user } }

    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIImagesResponse: {:#?}", response_body);
        let response: OpenAIImagesResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIImagesResponse(response)
    }
}
