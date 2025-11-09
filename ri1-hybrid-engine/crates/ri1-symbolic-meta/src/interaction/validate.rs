use ri1_core::constraints::{OperatorClass, ResonanceEvent};

use super::rules::{notice, violation, ValidatorConfig};
use super::tokenize::{tokenize, TokenKind};

pub fn validate_interactions(content: &str, _cfg: &ValidatorConfig) -> Vec<ResonanceEvent> {
    let mut events = Vec::new();
    let tokens = tokenize(content);

    // 1) Bracket balance
    let mut bal = 0i32;
    for t in &tokens {
        match t.kind {
            TokenKind::LBracket => bal += 1,
            TokenKind::RBracket => bal -= 1,
            _ => {}
        }
        if bal < 0 {
            events.push(violation("Unmatched ']' detected", Some("[]")));
            break;
        }
    }
    if bal != 0 {
        events.push(violation("Unbalanced '[]' loop container", Some("[]")));
    }

    // 2) '=' should be terminal (no Δ/Ξ/Π after first '=')
    if let Some(eq_idx) = content.find('=') {
        let tail = &content[eq_idx+1..];
        if tail.contains('Δ') || tail.contains('Ξ') || tail.contains('Π') {
            events.push(violation("Post '=' transform detected (Δ/Ξ/Π). Stabilization should be terminal.", Some("=")));
        }
    }

    // 3) ':' contradictions with '|'
    if content.contains('|') && content.contains(':') {
        events.push(notice("Orthogonality '|' with interaction ':' in same scope", Some("|")));
    }

    // 4) Attempt to detect naive cycles with '→' in both directions
    if content.contains('→') {
        // Very simple heuristic: A → B and B → A both present
        // We just look for the literal reversed arrow presence and log a notice.
        if content.matches('→').count() > 1 {
            events.push(notice("Multiple '→' arcs; ensure no causal cycles (use '[]' for loops)", Some("→")));
        }
    }

    // 5) ':' cannot be terminal
    if content.trim_end().ends_with(':') {
        events.push(notice(": at end of expression; interaction requires counterpart", Some(":")));
    }

    events
}
