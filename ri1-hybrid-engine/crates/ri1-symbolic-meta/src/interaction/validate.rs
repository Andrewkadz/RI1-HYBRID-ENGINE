use ri1_core::constraints::ResonanceEvent;

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
    // 2) '=' should be terminal (no Δ/Ξ/Π after first '=') — char-aware scan
    let mut seen_eq = false;
    for (_bi, ch) in content.char_indices() {
        if !seen_eq {
            if ch == '=' { seen_eq = true; }
        } else {
            if ch == 'Δ' || ch == 'Ξ' || ch == 'Π' {
                events.push(violation("Post '=' transform detected (Δ/Ξ/Π). Stabilization should be terminal.", Some("=")));
                break;
            }
        }
    }

    // 3) ':' contradictions with '|'
    if content.contains('|') && content.contains(':') {
        // scope-aware: detect at same bracket nesting level
        let mut bal = 0i32;
        let mut seen_colon_levels = Vec::new();
        for t in &tokens {
            match t.kind {
                TokenKind::LBracket => bal += 1,
                TokenKind::RBracket => bal -= 1,
                TokenKind::Colon => seen_colon_levels.push(bal),
                TokenKind::Pipe => {
                    if seen_colon_levels.iter().any(|lvl| *lvl == bal) {
                        events.push(notice("'|' orthogonality with ':' interaction at same scope", Some("|")));
                        break;
                    }
                }
                _ => {}
            }
        }
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

    // 5b) '=' cannot be terminal (requires finalized right-hand form)
    if content.trim_end().ends_with('=') {
        events.push(violation("=: at end of expression; stabilization requires finalized form", Some("=")));
    }

    // 6) ':' operand adjacency (must have operands on both sides that are not operators)
    for (i, t) in tokens.iter().enumerate() {
        if let TokenKind::Colon = t.kind {
            // find prev non-operator token
            let prev_ok = (0..i).rev().find_map(|j| match tokens[j].kind {
                TokenKind::Symbol(_) | TokenKind::RBracket => Some(true),
                TokenKind::LBracket | TokenKind::Arrow | TokenKind::Plus | TokenKind::Colon | TokenKind::Slash | TokenKind::Pipe | TokenKind::Equals => Some(false),
            }).unwrap_or(false);
            // find next non-operator token
            let next_ok = ((i+1)..tokens.len()).find_map(|j| match tokens[j].kind {
                TokenKind::Symbol(_) | TokenKind::LBracket => Some(true),
                TokenKind::RBracket | TokenKind::Arrow | TokenKind::Plus | TokenKind::Colon | TokenKind::Slash | TokenKind::Pipe | TokenKind::Equals => Some(false),
            }).unwrap_or(false);
            if !(prev_ok && next_ok) {
                events.push(violation(": requires operands on both sides", Some(":")));
            }
        }
    }

    // 7) 'Ω' should be terminal (no Δ/Ξ/Π after first Ω) — char-aware scan
    let mut seen_omega = false;
    for (_bi, ch) in content.char_indices() {
        if !seen_omega {
            if ch == 'Ω' { seen_omega = true; }
        } else {
            if ch == 'Δ' || ch == 'Ξ' || ch == 'Π' {
                events.push(violation("Post 'Ω' transform detected (Δ/Ξ/Π). Closure should be terminal.", Some("Ω")));
                break;
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use ri1_core::constraints::OperatorClass;

    #[test]
    fn colon_terminal_and_adjacency() {
        let ev = validate_interactions("Ψ :", &ValidatorConfig::default());
        assert!(ev.iter().any(|e| matches!(e.operator, OperatorClass::InteractionNotice)));
        assert!(ev.iter().any(|e| matches!(e.operator, OperatorClass::InteractionViolation)));
    }

    #[test]
    fn omega_term_violation() {
        let ev = validate_interactions("ΔΩ → Π", &ValidatorConfig::default());
        assert!(ev.iter().any(|e| matches!(e.operator, OperatorClass::InteractionViolation) && e.symbol.as_deref()==Some("Ω")));
    }

    #[test]
    fn equals_term_violation() {
        let ev = validate_interactions("[ΔΨ] → Ξ =", &ValidatorConfig::default());
        assert!(ev.iter().any(|e| matches!(e.operator, OperatorClass::InteractionViolation) && e.symbol.as_deref()==Some("=") ));
    }

    #[test]
    fn pipe_colon_same_scope_notice() {
        let ev = validate_interactions("Ψ : Φ | Γ", &ValidatorConfig::default());
        assert!(ev.iter().any(|e| matches!(e.operator, OperatorClass::InteractionNotice)));
    }
}
