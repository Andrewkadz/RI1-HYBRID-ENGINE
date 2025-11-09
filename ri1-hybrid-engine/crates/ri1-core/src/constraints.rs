use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Soft,
    Hard,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Soft => write!(f, "soft"),
            Severity::Hard => write!(f, "hard"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintResult {
    pub passed: bool,
    pub severity: Severity,
    pub name: &'static str,
    pub message: Option<String>,
}

pub trait Constraint {
    fn name(&self) -> &'static str;
    fn check(&self, text: &str) -> ConstraintResult;
}

pub trait ConstraintEngine: Send + Sync {
    fn evaluate(&self, modality: &str, content: &str) -> Vec<ConstraintResult>;
}

// --- Phase 2: Meta Engine Interfaces ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorClass {
    Coexistence,
    Fusion,
    Transcendence,
    HarmonicStabilization,
    QualiaGateway,
    Entanglement,
    StructuralIllumination,
    ClosureIntegration,
    EmergentSystem,
    DirectionalGrowth,
    Oscillation,
    MicroIgnition,
    RecurrencePattern,
    WillForce,
}

#[derive(Debug, Clone)]
pub struct Consent {
    pub granted: bool,
    pub subject: Option<String>,
    pub reason: Option<String>,
    pub section_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FieldContext {
    pub phase: Option<String>,
    pub source: Option<String>,
    pub field_id: Option<String>,
}

impl Default for FieldContext {
    fn default() -> Self { Self { phase: Some("alpha".into()), source: None, field_id: None } }
}

#[derive(Debug, Clone)]
pub struct ResonanceEvent {
    pub operator: OperatorClass,
    pub message: String,
    pub section_ref: Option<String>,
    pub symbol: Option<String>,
}

pub trait MetaEngine: Send + Sync {
    fn consent_check(&self, ctx: &FieldContext) -> Consent;
    fn evaluate_meta(
        &self,
        modality: &str,
        content: &str,
        ctx: &FieldContext,
    ) -> (Vec<ConstraintResult>, Vec<ResonanceEvent>);
    fn operators(&self) -> &[OperatorDef];
    fn conditionals(&self) -> &[ConditionalDef];
}

#[derive(Debug, Clone)]
pub struct OperatorDef {
    pub key: String,          // canonical name
    pub symbol: String,       // e.g., Φ, Λ, etc.
    pub section_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConditionalDef {
    pub key: String,      // canonical name
    pub symbol: String,   // e.g., +, /, :
    pub section_ref: Option<String>,
}

// --- Field-Phase Operator Gates (e.g., Φ) ---
#[derive(Debug, Clone)]
pub struct GateOutcome {
    pub stabilized: bool,
    pub prevented_fusion: bool,
    pub prevented_disruption: bool,
    pub note: Option<String>,
}

pub trait OperatorGate: Send + Sync {
    fn symbol(&self) -> &'static str;
    fn apply(&self, content: &str) -> GateOutcome;
}
