use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneEventsResponse {
    data: Vec<OpenAIFineTuneEvent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneEvent {
    created_at: u32,
    level: String,
    message: String,
}

impl OpenAIFineTuneEvent {
    pub fn new(created_at: u32, level: String, message: String) -> Self { Self { created_at, level, message } }
}

impl OpenAIFineTuneEventsResponse {
    pub fn new(data: Vec<OpenAIFineTuneEvent>) -> Self { Self { data } }

	pub fn print_events(self) {
		trace!("print events");
		for event in &self.data {
			println!("{}::{}: {}" , event.level, event.created_at, event.message);
		}
        if self.data.is_empty() {
            debug!("No events belong to fine-tune");
        }
	}
}
