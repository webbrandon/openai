use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTunesResponse {
    data: Vec<OpenAIFineTune>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTune {
    id: String,
    object: String,
    updated_at: u32,
    created_at: u32,
    model: String,
    organization_id: String,
    status: String,
    // "fine_tuned_model": null,
    // "hyperparams": { ... },
    // "result_files": [],
    // "validation_files": [],
    // "training_files": [ { ... } ],
}

impl OpenAIFineTune {
    pub fn new(id: String, object: String, updated_at: u32, created_at: u32, model: String, organization_id: String, status: String) -> Self { Self { id, object, updated_at, created_at, model, organization_id, status } }
}

impl OpenAIFineTunesResponse {
    pub fn new(data: Vec<OpenAIFineTune>) -> Self { Self { data } }

	pub fn print_tunes(self) {
		trace!("print fine-tunes");
		for file in &self.data {
			println!("{}({}) - Model({}) - {}" ,file.id, file.object, file.model, file.status);
		}
        if self.data.is_empty() {
            debug!("No files belong to owner");
        }
	}
}
