use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIAudioTranslationResponse {
  text: String,
}

impl OpenAIAudioTranslationResponse {
    pub fn new(text: String) -> Self { Self { text } }

	pub fn print_response(self) {
		trace!("print response");
		println!("{}", self.text);
	}
}
