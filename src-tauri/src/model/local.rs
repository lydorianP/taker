use super::{CompletionRequest, CompletionResponse, ModelBackend};

pub struct LocalBackend {
    #[allow(dead_code)]
    model_path: String,
}

impl LocalBackend {
    pub fn new(model_path: String) -> Self {
        LocalBackend { model_path }
    }
}

impl ModelBackend for LocalBackend {
    fn complete(&self, _request: &CompletionRequest) -> Result<CompletionResponse, String> {
        Err("Local inference not implemented yet".into())
    }

    fn name(&self) -> &str {
        "local"
    }
}
