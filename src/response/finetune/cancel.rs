use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneCancelResponse {
    id: String,
    object: String,
    updated_at: u32,
    created_at: u32,
    model: String,
    organization_id: String,
    status: String,
}

impl OpenAIFineTuneCancelResponse {
    pub fn new(id: String, object: String, updated_at: u32, created_at: u32, model: String, organization_id: String, status: String) -> Self { Self { id, object, updated_at, created_at, model, organization_id, status } }

	pub fn print_response(self) {
		trace!("print response");
		println!("{}({}) - Status ( {} )",self.id, self.model, self.status);
	}
}
