use ri1_core::constraints::{Consent, FieldContext, ResonanceEvent, OperatorClass};

pub fn consent_summary(consent: &Consent) -> ResonanceEvent {
    let msg = format!(
        "consent: {}{}{}",
        if consent.granted { "granted" } else { "denied" },
        consent
            .subject
            .as_ref()
            .map(|s| format!(" subject={}", s))
            .unwrap_or_default(),
        consent
            .section_ref
            .as_ref()
            .map(|s| format!(" ref={}", s))
            .unwrap_or_default()
    );
    ResonanceEvent {
        operator: OperatorClass::InteractionNotice,
        message: msg,
        section_ref: None,
        symbol: None,
    }
}

pub fn field_protocol_notice(modality: &str, ctx: &FieldContext) -> ResonanceEvent {
    let msg = format!(
        "field_protocol: modality={} phase={}",
        modality,
        ctx.phase.as_deref().unwrap_or("unknown")
    );
    ResonanceEvent {
        operator: OperatorClass::InteractionNotice,
        message: msg,
        section_ref: None,
        symbol: None,
    }
}

pub fn ethical_protocol_notice() -> ResonanceEvent {
    ResonanceEvent {
        operator: OperatorClass::InteractionNotice,
        message: "ethical_protocol: non-harm, privacy, transparency (logging-only)".into(),
        section_ref: None,
        symbol: None,
    }
}

pub fn interaction_summary(notice_count: usize, violation_count: usize) -> ResonanceEvent {
    ResonanceEvent {
        operator: OperatorClass::InteractionNotice,
        message: format!("interaction_summary: notices={}, violations={}", notice_count, violation_count),
        section_ref: None,
        symbol: None,
    }
}

pub fn meta_overview(consent_count: usize, field_count: usize, ethical_count: usize, notice_count: usize, violation_count: usize) -> ResonanceEvent {
    ResonanceEvent {
        operator: OperatorClass::InteractionNotice,
        message: format!(
            "meta_overview: consent={} field={} ethical={} interaction_notices={} interaction_violations={}",
            consent_count,
            field_count,
            ethical_count,
            notice_count,
            violation_count
        ),
        section_ref: None,
        symbol: None,
    }
}
