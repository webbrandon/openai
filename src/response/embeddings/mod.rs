use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIEmbeddingResponse {
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: Usage,
}

impl OpenAIEmbeddingResponse {
    pub fn new(data: Vec<Embedding>, model: String, usage: Usage) -> Self { Self { data, model, usage } }

	pub fn print_response(self) {
		trace!("print embedding");
        println!("Tokens Given: {}", self.usage.prompt_tokens);
        println!("Tokens Returned: {}", self.usage.total_tokens);
		for embedding in self.data {
            let title = format!("Index {}({})", embedding.object, embedding.index);
            println!("{}", title);
            println!("{:?}", embedding.embedding);
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Embedding {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: u32
}

impl Embedding {
    pub fn new(object: String, embedding: Vec<f32>, index: u32) -> Self { Self { object, embedding, index } }

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

impl Usage {
    pub fn new(prompt_tokens: u32, total_tokens: u32) -> Self { Self { prompt_tokens, total_tokens } }

}
