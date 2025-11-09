use tracing::info;

use crate::modality::Modality;

pub struct Orchestrator {
    // TODO: add registry, config, and constraint hooks
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn register_modality<M: Modality + 'static>(&mut self, _m: M) {
        // TODO: register modality plugin
        info!("Registered modality");
    }
}
