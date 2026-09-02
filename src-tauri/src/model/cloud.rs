use super::{CompletionRequest, CompletionResponse, ModelBackend};

pub struct CloudBackend {
    #[allow(dead_code)]
    endpoint: String,
    #[allow(dead_code)]
    api_key: String,
    #[allow(dead_code)]
    model_name: String,
}

impl CloudBackend {
    pub fn new(endpoint: String, api_key: String, model_name: String) -> Self {
        CloudBackend {
            endpoint,
            api_key,
            model_name,
        }
    }
}

impl ModelBackend for CloudBackend {
    fn complete(&self, _request: &CompletionRequest) -> Result<CompletionResponse, String> {
        Err("Cloud inference not implemented yet".into())
    }

    fn name(&self) -> &str {
        "cloud"
    }
}
