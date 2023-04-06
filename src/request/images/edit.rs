use serde::{Deserialize, Serialize};
use crate::response::*;

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIImageEditRequest {
    pub prompt: Option<String>,
    pub image: Option<PathBuf>,
    pub mask: Option<PathBuf>,
    pub n: u32,
    pub size: String,
    pub response_format: String,
    pub user: Option<String>,
}

impl OpenAIImageEditRequest {
    pub fn new(prompt: Option<String>, image: Option<PathBuf>, mask: Option<PathBuf>, n: u32, size: String, response_format: String, user: Option<String>) -> Self { Self { prompt, image, mask, n, size, response_format, user } }

    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIImageEditResponse: {:#?}", response_body);
        let response: OpenAIImageEditResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIImageEditResponse(response)
    }
}
