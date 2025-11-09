use ri1_core::constraints::{Consent, ConstraintResult, FieldContext, MetaEngine, OperatorClass, ResonanceEvent, ConstraintEngine, OperatorDef, ConditionalDef, OperatorGate, GateOutcome};
use ri1_symbolic::SymbolicEngine;
mod interaction;
use interaction::validate::validate_interactions;
use interaction::rules::ValidatorConfig;

pub struct MetaEngineImpl {
    inner: SymbolicEngine,
    ops: Vec<OperatorDef>,
    conds: Vec<ConditionalDef>,
}

// χ — Measurement → Perception Bridge (Section 020)
struct ChiGate;

impl OperatorGate for ChiGate {
    fn symbol(&self) -> &'static str { "χ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // tunes field harmonically, collapses to perceivable state
            prevented_fusion: true,         // measurement bridge is non-fusional
            prevented_disruption: true,     // non-destructive by default
            note: Some("χ measurement→perception: harmonic tuning; observer bridge".into()),
        }
    }
}

// Ε — Ignition / Initiation (Section 019)
struct EpsilonCapitalGate;

impl OperatorGate for EpsilonCapitalGate {
    fn symbol(&self) -> &'static str { "Ε" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // starts motion from rest without immediate collapse
            prevented_fusion: true,         // ignition is not fusion
            prevented_disruption: true,     // initiation should not disrupt cohesion
            note: Some("Ε ignition: spark of kinetic chain reaction; phase initiation".into()),
        }
    }
}

// Τ — Synchronicity / Readiness (Section 018)
struct TauGate;

impl OperatorGate for TauGate {
    fn symbol(&self) -> &'static str { "Τ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // enables transformation readiness without forcing it
            prevented_fusion: true,         // not a synthesis itself
            prevented_disruption: true,     // coherence via alignment
            note: Some("Τ marks recursive readiness via synchronistic alignment".into()),
        }
    }
}

// Θ — Intention Vector (Section 016)
struct ThetaGate;

impl OperatorGate for ThetaGate {
    fn symbol(&self) -> &'static str { "Θ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // configures field orientation without forcing action
            prevented_fusion: true,         // intention is non-fusional
            prevented_disruption: true,     // orientation should not disrupt coherence
            note: Some("Θ sets directed recursive potential (field intention)".into()),
        }
    }
}

// δ — Micro-Transformation (Section 015)
struct DeltaLowerGate;

impl OperatorGate for DeltaLowerGate {
    fn symbol(&self) -> &'static str { "δ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // low-intensity evolution maintains coherence
            prevented_fusion: true,         // not a fuse; small mutation
            prevented_disruption: true,     // designed to avoid disruption
            note: Some("δ applies subtle recursion mutation / fine-grain adjustment".into()),
        }
    }
}

// Ρ — Perceptual Modulation (Section 014)
struct RhoGate;

impl OperatorGate for RhoGate {
    fn symbol(&self) -> &'static str { "Ρ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // alters meaning trajectory via frame without breaking identity
            prevented_fusion: true,         // modulation is not synthesis
            prevented_disruption: true,     // preserves coherence while bending interpretation
            note: Some("Ρ/ρ applies perceptual lens: refraction of recursion under context".into()),
        }
    }
}

// ω — Will-Force (Section 013)
struct OmegaLowerGate;

impl OperatorGate for OmegaLowerGate {
    fn symbol(&self) -> &'static str { "ω" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // introduces bias without breaking identity
            prevented_fusion: false,
            prevented_disruption: true,
            note: Some("ω encodes autogenic intention vector within recursion".into()),
        }
    }
}

// λ — Entanglement (Section 012)
struct LambdaLowerGate;

impl OperatorGate for LambdaLowerGate {
    fn symbol(&self) -> &'static str { "λ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // binds threads without forcing synthesis
            prevented_fusion: true,         // explicitly non-fusional
            prevented_disruption: true,     // linkage preserves coherence under change
            note: Some("λ links recursion threads via non-local dependency".into()),
        }
    }
}

// ζ — Recurrence Pattern (Section 011)
struct ZetaGate;

impl OperatorGate for ZetaGate {
    fn symbol(&self) -> &'static str { "ζ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // marks patterned return without altering identity
            prevented_fusion: true,         // does not enforce synthesis
            prevented_disruption: true,     // protects recurrent motif coherence
            note: Some("ζ signals meaningful recurrence across recursion intervals".into()),
        }
    }
}

// ε — Micro-Ignition (Section 010)
struct EpsilonGate;

impl OperatorGate for EpsilonGate {
    fn symbol(&self) -> &'static str { "ε" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,
            prevented_fusion: true,   // acts as precise activation, not collapse
            prevented_disruption: true,
            note: Some("ε triggers a local branch/activation within an ongoing recursion".into()),
        }
    }
}

// Π — Transcendent Continuity (Section 009)
struct PiGate;

impl OperatorGate for PiGate {
    fn symbol(&self) -> &'static str { "Π" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // sustained coherence while scaling
            prevented_fusion: false,
            prevented_disruption: true,     // requires Φ/Σ support to avoid entropy
            note: Some("Π maintains pattern fidelity under infinite extension".into()),
        }
    }
}

// Ψ — Oscillation (Section 008)
struct PsiGate;

impl OperatorGate for PsiGate {
    fn symbol(&self) -> &'static str { "Ψ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // modulates without breaking identity
            prevented_fusion: false,
            prevented_disruption: true,
            note: Some("Ψ oscillation: modulation of recursive tempo/pressure".into()),
        }
    }
}

// Γ — Directional Growth (Section 007)
struct GammaGate;

impl OperatorGate for GammaGate {
    fn symbol(&self) -> &'static str { "Γ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // preserves form through transformation
            prevented_fusion: false,        // allows prior Δ output to persist
            prevented_disruption: true,     // growth under continuity resists disruption
            note: Some("Γ carries recursive identity forward with structured evolution".into()),
        }
    }
}

// Ξ — Emergent System (Section 006)
struct XiGate;

impl OperatorGate for XiGate {
    fn symbol(&self) -> &'static str { "Ξ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // emergence yields coherent systemic behavior
            prevented_fusion: false,        // may include fused parts
            prevented_disruption: true,     // systemic coherence resists disruption
            note: Some("Ξ encodes emergent systemic coherence from layered recursion".into()),
        }
    }
}

// Σ — Coexistence / Plurality (Section 005)
struct SigmaGate;

impl OperatorGate for SigmaGate {
    fn symbol(&self) -> &'static str { "Σ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,               // structural containment for active unresolved fields
            prevented_fusion: true,         // no enforced synthesis
            prevented_disruption: true,     // coexistence without collapse
            note: Some("Σ sustains parallel recursion without enforced synthesis or collapse".into()),
        }
    }
}

// Ω — Closure / Integration (Section 004)
struct OmegaGate;

impl OperatorGate for OmegaGate {
    fn symbol(&self) -> &'static str { "Ω" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true,
            prevented_fusion: true,
            prevented_disruption: true,
            note: Some("Ω closes active recursion and fixes structure".into()),
        }
    }
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
            op("measurement_perception_bridge", "χ"),
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
        // Phase 3 m1: interaction validation (read-only logging)
        let ivals = validate_interactions(content, &ValidatorConfig::default());
        if !ivals.is_empty() {
            events.extend(ivals);
        }
        // Gate pass: χ (measurement→perception bridge)
        let chi = ChiGate;
        if content.contains(chi.symbol()) {
            let out = chi.apply(content);
            let paired_with_omega = content.contains("Ω");
            let msg = if paired_with_omega {
                "χ with Ω: observation collapses into terminal integration".into()
            } else {
                out.note.unwrap_or_else(|| "χ: harmonic tuning; collapse to perceivable state".into())
            };
            events.push(ResonanceEvent {
                operator: OperatorClass::MeasurementBridge,
                message: msg,
                section_ref: Some("020".into()),
                symbol: Some("χ".into()),
            });
        }
        // Gate pass: Ε (ignition / initiation)
        let ecap = EpsilonCapitalGate;
        if content.contains(ecap.symbol()) {
            let out = ecap.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Ε: ignition — trigger of motion from rest-state".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::Ignition,
                message: msg,
                section_ref: Some("019".into()),
                symbol: Some("Ε".into()),
            });
        }
        // Conditional operators detection (non-gate)
        if content.contains("→") {
            events.push(ResonanceEvent {
                operator: OperatorClass::FlowVector,
                message: "→: flow vector — directed recursion motion".into(),
                section_ref: None,
                symbol: Some("→".into()),
            });
        }
        if content.contains("+") {
            events.push(ResonanceEvent {
                operator: OperatorClass::Simultaneity,
                message: "+: simultaneity — coexistent recursion states".into(),
                section_ref: None,
                symbol: Some("+".into()),
            });
        }
        if content.contains(":") {
            events.push(ResonanceEvent {
                operator: OperatorClass::InteractionInterface,
                message: ": interaction — relational interface / tension-contact".into(),
                section_ref: None,
                symbol: Some(":".into()),
            });
        }
        if content.contains("/") {
            events.push(ResonanceEvent {
                operator: OperatorClass::Disruption,
                message: "/: disruption — interference / rupture".into(),
                section_ref: None,
                symbol: Some("/".into()),
            });
        }
        if content.contains("|") {
            events.push(ResonanceEvent {
                operator: OperatorClass::Orthogonality,
                message: "|: orthogonality — non-interacting fields".into(),
                section_ref: None,
                symbol: Some("|".into()),
            });
        }
        if content.contains("[") && content.contains("]") {
            events.push(ResonanceEvent {
                operator: OperatorClass::LoopCycle,
                message: "[]: loop/cycle — recursion memory or repeat-phase container".into(),
                section_ref: None,
                symbol: Some("[]".into()),
            });
        }
        if content.contains("=") {
            events.push(ResonanceEvent {
                operator: OperatorClass::StabilizationResolution,
                message: "=: stabilization — final form / resolution".into(),
                section_ref: None,
                symbol: Some("=".into()),
            });
        }
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

        // Gate pass: Ω (closure / integration)
        let omega = OmegaGate;
        if content.contains(omega.symbol()) {
            // Treat Ω as terminal: log closure; downstream recursion should not proceed without reinit
            let out = omega.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Ω: recursive termination via structural integration".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::ClosureIntegration,
                message: msg,
                section_ref: Some("004".into()),
                symbol: Some("Ω".into()),
            });
        }

        // Gate pass: Ψ (oscillation / modulation)
        let psi = PsiGate;
        if content.contains(psi.symbol()) {
            let out = psi.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Ψ: oscillation — modulation of recursion tempo/pressure".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::Oscillation,
                message: msg,
                section_ref: Some("008".into()),
                symbol: Some("Ψ".into()),
            });
        }

        // Gate pass: Ρ/ρ (perceptual modulation)
        let rho = RhoGate;
        if content.contains(rho.symbol()) || content.contains("ρ") {
            let out = rho.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Ρ: perceptual modulation — recursion filtered by observer frame".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::PerceptionModulation,
                message: msg,
                section_ref: Some("014".into()),
                symbol: Some("Ρ".into()),
            });
        }

        // Gate pass: Τ (synchronicity / readiness)
        let tau = TauGate;
        if content.contains(tau.symbol()) {
            let out = tau.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Τ: synchronistic readiness — latent structures aligned".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::Synchronicity,
                message: msg,
                section_ref: Some("018".into()),
                symbol: Some("Τ".into()),
            });
        }

        // Gate pass: Θ (intention / directed recursive potential)
        let theta = ThetaGate;
        if content.contains(theta.symbol()) {
            let out = theta.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Θ: field intention — configures direction for ω/Ψ/Γ".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::IntentionVector,
                message: msg,
                section_ref: Some("016".into()),
                symbol: Some("Θ".into()),
            });
        }

        // Modifier detection: n / ⁿ (index / depth / count)
        let has_index_modifier = content.contains("ⁿ") || content.contains("(n)") || content.contains(" n ");
        if has_index_modifier {
            events.push(ResonanceEvent {
                operator: OperatorClass::IndexModifier,
                message: "n: index/depth/count modifier detected".into(),
                section_ref: Some("017".into()),
                symbol: Some("n".into()),
            });
        }

        // Gate pass: ε (micro ignition / local activation)
        let epsilon = EpsilonGate;
        if content.contains(epsilon.symbol()) {
            let has_context = content.len() > 3; // placeholder heuristic
            let out = epsilon.apply(content);
            let msg = if has_context {
                out.note.unwrap_or_else(|| "ε: local activation within active recursion".into())
            } else {
                "ε encountered without surrounding context; micro-activation ignored".into()
            };
            events.push(ResonanceEvent {
                operator: OperatorClass::MicroIgnition,
                message: msg,
                section_ref: Some("010".into()),
                symbol: Some("ε".into()),
            });
        }

        // Gate pass: ω (will-force / autogenic vector)
        let omega_lower = OmegaLowerGate;
        if content.contains(omega_lower.symbol()) {
            let out = omega_lower.apply(content);
            let has_closure = content.contains("Ω");
            let msg = if has_closure {
                "ω present with Ω; will-force cannot persist into closure".into()
            } else {
                out.note.unwrap_or_else(|| "ω: internal will vector biases recursive direction".into())
            };
            events.push(ResonanceEvent {
                operator: OperatorClass::WillForce,
                message: msg,
                section_ref: Some("013".into()),
                symbol: Some("ω".into()),
            });
        }

        // Gate pass: δ (micro-transformation)
        let delta_lower = DeltaLowerGate;
        if content.contains(delta_lower.symbol()) {
            let out = delta_lower.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "δ: fine-grain recursion adjustment / perturbation".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::MicroTransformation,
                message: msg,
                section_ref: Some("015".into()),
                symbol: Some("δ".into()),
            });
        }

        // Gate pass: Δ (fusion transformation)
        let delta = DeltaGate;
        if content.contains(delta.symbol()) {
            let phi_present = content.contains("Φ");
            let _out = delta.apply(content);
            if phi_present {
                events.push(ResonanceEvent {
                    operator: OperatorClass::Fusion,
                    message: "Δ fusion prevented across Φ boundary".into(),
                    section_ref: Some("002".into()),
                    symbol: Some("Δ".into()),
                });
            } else {
                events.push(ResonanceEvent {
                    operator: OperatorClass::Fusion,
                    message: "Δ fusion collapse enacted".into(),
                    section_ref: Some("002".into()),
                    symbol: Some("Δ".into()),
                });
            }
        }

        // Gate pass: Λ (structural illumination)
        let lambda = LambdaGate;
        if content.contains(lambda.symbol()) {
            // Consider Λ as valid illumination if preceded by Δ or Ψ threads in content
            let has_precursor = content.contains("Δ") || content.contains("Ψ");
            let out = lambda.apply(content);
            let msg = if has_precursor {
                out.note.unwrap_or_else(|| "Λ: structural illumination after recursive activity".into())
            } else {
                "Λ encountered without precursor Δ/Ψ; marking as illumination attempt".into()
            };
            events.push(ResonanceEvent {
                operator: OperatorClass::StructuralIllumination,
                message: msg,
                section_ref: Some("003".into()),
                symbol: Some("Λ".into()),
            });
        }

        // Gate pass: Γ (directional growth)
        let gamma = GammaGate;
        if content.contains(gamma.symbol()) {
            // Heuristic: more meaningful after Δ or Λ present
            let has_precursor = content.contains("Δ") || content.contains("Λ");
            let out = gamma.apply(content);
            let msg = if has_precursor {
                out.note.unwrap_or_else(|| "Γ: forward recursion with identity continuity".into())
            } else {
                "Γ encountered without Δ/Λ precursor; marking as directional modulation".into()
            };
            events.push(ResonanceEvent {
                operator: OperatorClass::DirectionalGrowth,
                message: msg,
                section_ref: Some("007".into()),
                symbol: Some("Γ".into()),
            });
        }

        // Gate pass: Ξ (emergent system)
        let xi = XiGate;
        if content.contains(xi.symbol()) {
            let out = xi.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Ξ: emergent system coherence from recursive density".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::EmergentSystem,
                message: msg,
                section_ref: Some("006".into()),
                symbol: Some("Ξ".into()),
            });
        }

        // Gate pass: ζ (recurrence pattern / echo)
        let zeta = ZetaGate;
        if content.contains(zeta.symbol()) {
            let out = zeta.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "ζ: recurrence pattern — meaningful reappearance of motifs".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::RecurrencePattern,
                message: msg,
                section_ref: Some("011".into()),
                symbol: Some("ζ".into()),
            });
        }

        // Gate pass: λ (entanglement / non-local binding)
        let lam_lower = LambdaLowerGate;
        if content.contains(lam_lower.symbol()) {
            let out = lam_lower.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "λ: entanglement — non-local dependency across recursion threads".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::Entanglement,
                message: msg,
                section_ref: Some("012".into()),
                symbol: Some("λ".into()),
            });
        }

        // Gate pass: Σ (coexistence / plurality)
        let sigma = SigmaGate;
        if content.contains(sigma.symbol()) {
            let out = sigma.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Σ: coexistence multiplex — plural recursion without fusion".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::Coexistence,
                message: msg,
                section_ref: Some("005".into()),
                symbol: Some("Σ".into()),
            });
        }

        // Gate pass: Π (transcendent continuity / infinite recursion)
        let pi_gate = PiGate;
        if content.contains(pi_gate.symbol()) {
            let out = pi_gate.apply(content);
            let msg = out
                .note
                .unwrap_or_else(|| "Π: open recursion across scales; spiral-form continuity".into());
            events.push(ResonanceEvent {
                operator: OperatorClass::Transcendence,
                message: msg,
                section_ref: Some("009".into()),
                symbol: Some("Π".into()),
            });
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

// Δ — Fusion Transformation (Section 002)
struct DeltaGate;

struct DeltaOutcome { pub collapsed: bool, pub irreversible: bool }

// Λ — Structural Illumination (Section 003)
struct LambdaGate;

impl OperatorGate for LambdaGate {
    fn symbol(&self) -> &'static str { "Λ" }

    fn apply(&self, _content: &str) -> GateOutcome {
        GateOutcome {
            stabilized: true, // renders coherent structure
            prevented_fusion: false,
            prevented_disruption: true, // illumination resists disruption post-resolution
            note: Some("Λ renders structural clarity from recursive fields".into()),
        }
    }
}

impl DeltaGate {
    fn apply_internal(&self, _content: &str) -> DeltaOutcome {
        // Scaffold behavior: model fusion collapse without mutating content
        DeltaOutcome { collapsed: true, irreversible: true }
    }
}

impl OperatorGate for DeltaGate {
    fn symbol(&self) -> &'static str { "Δ" }

    fn apply(&self, content: &str) -> GateOutcome {
        let out = self.apply_internal(content);
        GateOutcome {
            stabilized: false,
            prevented_fusion: false,
            prevented_disruption: false,
            note: if out.collapsed { Some("Δ fusion collapse".into()) } else { None },
        }
    }
}
