use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

use crate::modality::{GenerationRequest, GenerationResponse, Modality};
use crate::constraints::{ConstraintEngine, MetaEngine, Severity, ConstraintResult, ResonanceEvent, FieldContext};

pub struct Orchestrator {
    registry: HashMap<&'static str, Arc<dyn Modality>>, // name -> modality
    constraint_engine: Option<Arc<dyn ConstraintEngine>>, // legacy engine
    meta_engine: Option<Arc<dyn MetaEngine>>,             // preferred meta engine
    last_meta_log: Vec<ResonanceEvent>,
    last_results: Vec<ConstraintResult>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self { registry: HashMap::new(), constraint_engine: None, meta_engine: None, last_meta_log: Vec::new(), last_results: Vec::new() }
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

    pub fn set_constraint_engine<E: ConstraintEngine + 'static>(&mut self, engine: E) {
        self.constraint_engine = Some(Arc::new(engine));
        info!("constraint_engine = set");
    }

    pub fn set_meta_engine<E: MetaEngine + 'static>(&mut self, engine: E) {
        self.meta_engine = Some(Arc::new(engine));
        info!("meta_engine = set");
    }

    pub fn evaluate(&self, modality: &str, content: &str) -> Vec<ConstraintResult> {
        if let Some(engine) = &self.meta_engine {
            let ctx = FieldContext::default();
            let (_results, _res_log) = engine.evaluate_meta(modality, content, &ctx);
            return _results;
        }
        match &self.constraint_engine {
            Some(engine) => engine.evaluate(modality, content),
            None => Vec::new(),
        }
    }

    pub fn evaluate_with_events(&self, modality: &str, content: &str) -> (Vec<ConstraintResult>, Vec<ResonanceEvent>) {
        if let Some(engine) = &self.meta_engine {
            let ctx = FieldContext::default();
            return engine.evaluate_meta(modality, content, &ctx);
        }
        match &self.constraint_engine {
            Some(engine) => (engine.evaluate(modality, content), Vec::new()),
            None => (Vec::new(), Vec::new()),
        }
    }
    pub fn last_meta_events(&self) -> &Vec<ResonanceEvent> { &self.last_meta_log }

    pub fn generate(&self, modality: &str, req: GenerationRequest) -> Option<GenerationResponse> {
        match self.registry.get(modality) {
            Some(m) => {
                let out = m.generate(req);
                // Prefer meta engine: consent check + evaluate + block on hard failures
                if let Some(engine) = &self.meta_engine {
                    let ctx = FieldContext::default();
                    let consent = engine.consent_check(&ctx);
                    if !consent.granted {
                        warn!("generation_blocked_by_consent");
                        return None;
                    }
                    let (results, events) = engine.evaluate_meta(modality, &out.content, &ctx);
                    let hard_fail = results.iter().any(|r| !r.passed && r.severity == Severity::Hard);
                    if hard_fail {
                        warn!("generation_blocked_by_hard_constraint");
                        return None;
                    }
                    // store logs
                    // NOTE: using interior mutability would avoid &self, but keep simple for now by shadowing
                    // (we won't expose mutation of logs here to keep API simple in Phase 2 scaffold)
                } else if let Some(engine) = &self.constraint_engine {
                    let results = engine.evaluate(modality, &out.content);
                    let hard_fail = results.iter().any(|r| !r.passed && r.severity == Severity::Hard);
                    if hard_fail {
                        warn!("generation_blocked_by_hard_constraint");
                        return None;
                    }
                }
                Some(out)
            }
            None => {
                warn!("unknown_modality = {}", modality);
                None
            }
        }
    }
}

