use ri1_core::constraints::{Consent, ConstraintResult, FieldContext, MetaEngine, OperatorClass, ResonanceEvent, ConstraintEngine};
use ri1_symbolic::SymbolicEngine;

pub struct MetaEngineImpl {
    inner: SymbolicEngine,
}

impl MetaEngineImpl {
    pub fn new_default() -> Self {
        Self { inner: SymbolicEngine::new_default() }
    }
}

impl MetaEngine for MetaEngineImpl {
    fn consent_check(&self, _ctx: &FieldContext) -> Consent {
        Consent { granted: true, subject: Some("default".into()), reason: None }
    }

    fn evaluate_meta(
        &self,
        modality: &str,
        content: &str,
        _ctx: &FieldContext,
    ) -> (Vec<ConstraintResult>, Vec<ResonanceEvent>) {
        let results = self.inner.evaluate(modality, content);
        let events = vec![ResonanceEvent { operator: OperatorClass::HarmonicStabilization, message: "stabilized".into() }];
        (results, events)
    }
}
