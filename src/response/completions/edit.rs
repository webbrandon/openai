use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionEditResponse {
    pub choices: Vec<Choice>,
}

impl OpenAICompletionEditResponse {
    pub fn new(choices: Vec<Choice>) -> Self { Self { choices } }

	pub fn print_choices(self) {
		trace!("print choices");
		let choices_count = self.choices.len();
		for choice in self.choices {
			if choices_count == 1 {
				println!("{}",choice.remove_newline_prepend());
			} else {
                println!("OpenAI Response: {}",choice.remove_newline_prepend());
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub text: String,
}

impl Choice {
    pub fn new(text: String) -> Self { Self { text } }

	fn remove_newline_prepend(self) -> String {
		trace!("remove newline prepend");
		self.text.replacen("\n\n", "", 1)
	}
}
