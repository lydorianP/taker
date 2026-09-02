pub mod local;
pub mod cloud;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub text: String,
    pub tokens_used: Option<u32>,
}

pub trait ModelBackend: Send + Sync {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, String>;
    fn name(&self) -> &str;
}
