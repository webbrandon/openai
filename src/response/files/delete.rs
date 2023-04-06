use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFileDeleteResponse {
  id: String,
  object: String,
  deleted: bool,
}

impl OpenAIFileDeleteResponse {
    pub fn new(id: String, object: String, deleted: bool) -> Self { Self { id, object, deleted } }

	pub fn print_response(self) {
		trace!("print response");
		println!("{}({}) - Removed ( {} )",self.id, self.object, self.deleted);
	}
}
