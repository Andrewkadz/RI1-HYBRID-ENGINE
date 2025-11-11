use ri1_symbolic_meta::{MetaEngineImpl, InfluenceSnapshot};
use ri1_core::constraints::FieldContext;

fn sum_weights(s: &InfluenceSnapshot) -> f64 {
    s.operator_influence.iter().map(|w| w.weight).sum::<f64>()
}

#[test]
fn weight_normalization_nonempty() {
    let engine = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let content = "Φ Δ Λ Ψ Ω : / | ="; // rich mixture to trigger many events
    let (_constraints, _events, snap) = engine.evaluate_meta_with_snapshot("text", content, &ctx);
    let total = sum_weights(&snap);
    assert!(total > 0.0, "weights should be positive when events exist");
    assert!((total - 1.0).abs() < 1e-6, "weights should normalize to 1.0; got {}", total);
}

#[test]
fn resonance_index_bounds() {
    let engine = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    for content in [
        "Φ",                // simple, cooperative
        "| : /",            // more conflict
        "Δ Λ Ψ Ω = | : /",  // mixture
        "",                 // empty content -> expect 0 weight sum; bounds still apply
    ] {
        let (_c, _e, snap) = engine.evaluate_meta_with_snapshot("text", content, &ctx);
        assert!(snap.resonance_index >= 0.0 && snap.resonance_index <= 1.0,
            "resonance index out of bounds: {} for content '{}'", snap.resonance_index, content);
    }
}

#[test]
fn negotiation_edges_presence_when_applicable() {
    let engine = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    // Fusion + StructuralIllumination → expect reinforce edge
    let (_c1, _e1, s1) = engine.evaluate_meta_with_snapshot("text", "Δ Λ", &ctx);
    assert!(s1.negotiation.iter().any(|e| e.relation == "reinforce"), "expected reinforce edge for Δ + Λ");

    // Oscillation + StructuralIllumination → expect reinforce edge
    let (_c2, _e2, s2) = engine.evaluate_meta_with_snapshot("text", "Ψ Λ", &ctx);
    assert!(s2.negotiation.iter().any(|e| e.relation == "reinforce"), "expected reinforce edge for Ψ + Λ");

    // Orthogonality + InteractionInterface → expect compete edge
    let (_c3, _e3, s3) = engine.evaluate_meta_with_snapshot("text", "| :", &ctx);
    assert!(s3.negotiation.iter().any(|e| e.relation == "compete"), "expected compete edge for | + :");
}

#[test]
fn snapshot_json_shape() {
    let engine = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let (_c, _e, snap) = engine.evaluate_meta_with_snapshot("text", "Φ : | =", &ctx);
    let j = serde_json::to_value(&snap).expect("json");
    assert!(j.get("resonance_index").is_some());
    assert!(j.get("operator_influence").is_some());
    assert!(j.get("cooperation_count").is_some());
    assert!(j.get("conflict_count").is_some());
    assert!(j.get("negotiation").is_some());
    // spot-check inner types
    let oi = j.get("operator_influence").unwrap().as_array().unwrap();
    if let Some(first) = oi.first() {
        assert!(first.get("operator").is_some());
        assert!(first.get("weight").is_some());
    }
}
