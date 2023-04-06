use serde::{Deserialize, Serialize};
use crate::response::*;

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIImageVariationRequest {
    pub image: Option<PathBuf>,
    pub n: u32,
    pub size: String,
    pub response_format: String,
    pub user: Option<String>,
}

impl OpenAIImageVariationRequest {
    pub fn new(image: Option<PathBuf>, n: u32, size: String, response_format: String, user: Option<String>) -> Self { Self { image, n, size, response_format, user } }

    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIImageVariationResponse: {:#?}", response_body);
        let response: OpenAIImageVariationResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIImageVariationResponse(response)
    }
}
