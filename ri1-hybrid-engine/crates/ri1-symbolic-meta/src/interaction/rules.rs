use ri1_core::constraints::{OperatorClass, ResonanceEvent};

#[derive(Debug, Clone)]
pub struct RuleViolation {
    pub message: String,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ValidatorConfig {}

pub fn violation(message: impl Into<String>, symbol: Option<&str>) -> ResonanceEvent {
    ResonanceEvent {
        operator: OperatorClass::InteractionViolation,
        message: message.into(),
        section_ref: None,
        symbol: symbol.map(|s| s.to_string()),
    }
}

pub fn notice(message: impl Into<String>, symbol: Option<&str>) -> ResonanceEvent {
    ResonanceEvent {
        operator: OperatorClass::InteractionNotice,
        message: message.into(),
        section_ref: None,
        symbol: symbol.map(|s| s.to_string()),
    }
}
