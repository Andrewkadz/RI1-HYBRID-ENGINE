use ri1_core::constraints::{Consent, ConstraintResult, FieldContext, MetaEngine, OperatorClass, ResonanceEvent, ConstraintEngine, OperatorDef, ConditionalDef, OperatorGate, GateOutcome};
use ri1_symbolic::SymbolicEngine;

pub struct MetaEngineImpl {
    inner: SymbolicEngine,
    ops: Vec<OperatorDef>,
    conds: Vec<ConditionalDef>,
}

impl MetaEngineImpl {
    pub fn new_default() -> Self {
        // Φπε Symbols (20 Total) — exact list from PDFs
        let ops = vec![
            op("harmonic_equilibrium", "Φ"),
            op("transcendent_continuity", "Π"),
            op("ignition_initiation", "Γ"),
            op("micro_ignition", "ε"),
            op("fusion_transformation", "Δ"),
            op("micro_transformation", "δ"),
            op("oscillation", "Ψ"),
            op("structural_illumination", "Λ"),
            op("entanglement", "λ"),
            op("recursive_growth", "Γ̇"),
            op("closure_integration", "Ω"),
            op("will_force", "ω"),
            op("coexistence_plurality", "Σ"),
            op("emergent_system", "Ξ"),
            op("recurrence_pattern_echo", "ζ"),
            op("synchronicity_readiness", "τ"),
            op("perception_modulation", "ρ"),
            op("intention_vector", "Θ"),
            op("depth_index_modifier", "n"),
            op("measurement_perception_bridge", "X"),
        ];
        // Φπε Operators (7 Total) — exact set from PDFs
        let conds = vec![
            cond("flow_vector_directional_recursion", "→"),
            cond("simultaneity_coexistent_fields", "+"),
            cond("interaction_field_tension_interface", ":"),
            cond("disruption_recursive_instability", "/"),
            cond("orthogonality_non_interacting_fields", "|"),
            cond("loop_cycle_recursion_memory", "[]"),
            cond("stabilization_final_state_resolution", "="),
        ];
        Self { inner: SymbolicEngine::new_default(), ops, conds }
    }
}

impl MetaEngine for MetaEngineImpl {
    fn consent_check(&self, _ctx: &FieldContext) -> Consent {
        Consent { granted: true, subject: Some("default".into()), reason: None, section_ref: None }
    }

    fn evaluate_meta(
        &self,
        modality: &str,
        content: &str,
        _ctx: &FieldContext,
    ) -> (Vec<ConstraintResult>, Vec<ResonanceEvent>) {
        let mut events: Vec<ResonanceEvent> = Vec::new();
        // Gate pass: Φ (field-phase continuity) detection
        let phi = PhiGate;
        if content.contains(phi.symbol()) {
            let out = phi.apply(content);
            if out.stabilized {
                events.push(ResonanceEvent {
                    operator: OperatorClass::HarmonicStabilization,
                    message: out.note.unwrap_or_else(|| "Φ: field-phase continuity gate stabilized".into()),
                    section_ref: Some("001".into()),
                    symbol: Some("Φ".into()),
                });
            }
        }

        let results = self.inner.evaluate(modality, content);
        (results, events)
    }

    fn operators(&self) -> &[OperatorDef] { &self.ops }
    fn conditionals(&self) -> &[ConditionalDef] { &self.conds }
}

fn op(key: &str, symbol: &str) -> OperatorDef {
    OperatorDef { key: key.into(), symbol: symbol.into(), section_ref: None }
}

fn cond(key: &str, symbol: &str) -> ConditionalDef {
    ConditionalDef { key: key.into(), symbol: symbol.into(), section_ref: None }
}

// --- Gates ---
struct PhiGate;

impl OperatorGate for PhiGate {
    fn symbol(&self) -> &'static str { "Φ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,
            prevented_fusion: true,
            prevented_disruption: true,
            note: Some("Φ preserves tension: non-fusional, non-neutralizing, recursion-safe".into()),
        }
    }
}
