use ri1_symbolic_meta::MetaEngineImpl;
use ri1_core::constraints::FieldContext;

fn json_norm(v: &serde_json::Value) -> serde_json::Value {
    // Round resonance_index to 2 decimals and operator weights to 2 decimals to avoid float noise
    let mut v = v.clone();
    if let Some(r) = v.get_mut("resonance_index") {
        if let Some(f) = r.as_f64() { *r = serde_json::Value::from((f * 100.0).round() / 100.0); }
    }
    if let Some(arr) = v.get_mut("operator_influence").and_then(|x| x.as_array_mut()) {
        for it in arr.iter_mut() {
            if let Some(w) = it.get_mut("weight") {
                if let Some(f) = w.as_f64() { *w = serde_json::Value::from((f * 100.0).round() / 100.0); }
            }
        }
    }
    v
}

#[test]
fn golden_cooperative_minimal() {
    let eng = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let (_c, _e, snap) = eng.evaluate_meta_with_snapshot("text", "Φ + Σ", &ctx);
    let got = json_norm(&serde_json::to_value(&snap).unwrap());
    // shape checks
    assert!(got.get("operator_influence").is_some());
    assert!(got.get("cooperation_count").unwrap().as_u64().unwrap() >= 1);
    assert_eq!(got.get("conflict_count").unwrap().as_u64().unwrap(), 0);
    // resonance bound tight for cooperative case
    let r = got.get("resonance_index").unwrap().as_f64().unwrap();
    assert!(r >= 0.95 && r <= 1.0, "expected high resonance_index, got {}", r);
}

#[test]
fn golden_conflicted() {
    let eng = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let (_c, _e, snap) = eng.evaluate_meta_with_snapshot("text", ": | /", &ctx);
    let got = json_norm(&serde_json::to_value(&snap).unwrap());
    let r = got.get("resonance_index").unwrap().as_f64().unwrap();
    // conflicted should be notably below 1
    assert!(r <= 0.85, "expected lower resonance for conflict, got {}", r);
    // conflicts counted
    let conflicts = got.get("conflict_count").unwrap().as_u64().unwrap();
    assert!(conflicts >= 1);
}

#[test]
fn golden_closure_heavy() {
    let eng = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let (_c, _e, snap) = eng.evaluate_meta_with_snapshot("text", "Δ Λ Ψ Ω =", &ctx);
    let got = json_norm(&serde_json::to_value(&snap).unwrap());
    // presence of negotiation edges for Δ+Λ or Ψ+Λ reinforce
    let edges = got.get("negotiation").unwrap().as_array().unwrap();
    assert!(edges.iter().any(|e| e.get("relation").and_then(|s| s.as_str())==Some("reinforce")));
}
