use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GenerationRequest {
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerationResponse {
    pub content: String,
}

pub trait Modality: Send + Sync {
    fn name(&self) -> &'static str;
    fn generate(&self, req: GenerationRequest) -> GenerationResponse;
}
