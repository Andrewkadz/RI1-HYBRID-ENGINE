use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

use crate::modality::{GenerationRequest, GenerationResponse, Modality};

pub struct Orchestrator {
    registry: HashMap<&'static str, Arc<dyn Modality>>, // name -> modality
}

impl Orchestrator {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    pub fn register_modality_arc<M: Modality + 'static>(&mut self, m: Arc<M>) {
        let name = m.name();
        self.registry.insert(name, m);
        info!("registered_modality = {}", name);
    }

    pub fn register_modality<M: Modality + 'static>(&mut self, m: M) {
        self.register_modality_arc(Arc::new(m));
    }

    pub fn modalities(&self) -> Vec<&'static str> {
        self.registry.keys().copied().collect()
    }

    pub fn generate(&self, modality: &str, req: GenerationRequest) -> Option<GenerationResponse> {
        match self.registry.get(modality) {
            Some(m) => Some(m.generate(req)),
            None => {
                warn!("unknown_modality = {}", modality);
                None
            }
        }
    }
}
