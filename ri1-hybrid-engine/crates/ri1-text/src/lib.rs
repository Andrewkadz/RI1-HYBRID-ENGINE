use ri1_core::modality::{GenerationRequest, GenerationResponse, Modality};

pub struct BasicText;

impl Modality for BasicText {
    fn name(&self) -> &'static str { "text" }

    fn generate(&self, req: GenerationRequest) -> GenerationResponse {
        GenerationResponse { content: format!("TEXT: {}", req.prompt) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_text_generates() {
        let m = BasicText;
        let out = m.generate(GenerationRequest { prompt: "hello".into() });
        assert!(out.content.contains("hello"));
    }
}
