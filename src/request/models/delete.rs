use serde::{Deserialize, Serialize};
use crate::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIModelDeleteRequest {
    pub model_name: String,
}

impl OpenAIModelDeleteRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIModelDeleteResponse: {:#?}", response_body);
        let response: OpenAIModelDeleteResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIModelDeleteResponse(response)
    }
}
