use serde::{Deserialize, Serialize};
use crate::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFileDeleteRequest {
    pub filename: String,
}

impl OpenAIFileDeleteRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIFileDeleteResponse: {:#?}", response_body);
        let response: OpenAIFileDeleteResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIFileDeleteResponse(response)
    }
}
