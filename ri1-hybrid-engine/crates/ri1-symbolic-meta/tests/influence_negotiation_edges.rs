use ri1_symbolic_meta::MetaEngineImpl;
use ri1_core::constraints::FieldContext;

#[test]
fn reinforce_edges_cover_delta_lambda_and_psi_lambda() {
    let eng = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let (_c1, _e1, s1) = eng.evaluate_meta_with_snapshot("text", "Δ Λ", &ctx);
    assert!(s1.negotiation.iter().any(|e| e.relation == "reinforce"), "Δ+Λ should show reinforce edge");

    let (_c2, _e2, s2) = eng.evaluate_meta_with_snapshot("text", "Ψ Λ", &ctx);
    assert!(s2.negotiation.iter().any(|e| e.relation == "reinforce"), "Ψ+Λ should show reinforce edge");
}

#[test]
fn compete_edges_cover_omega_with_delta_or_pi() {
    let eng = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    // Ω with Δ
    let (_c1, _e1, s1) = eng.evaluate_meta_with_snapshot("text", "Ω Δ", &ctx);
    assert!(s1.negotiation.iter().any(|e| e.relation == "compete"), "Ω+Δ should show compete edge");
    // Ω with Π
    let (_c2, _e2, s2) = eng.evaluate_meta_with_snapshot("text", "Ω Π", &ctx);
    assert!(s2.negotiation.iter().any(|e| e.relation == "compete"), "Ω+Π should show compete edge");
}

#[test]
fn orthogonality_and_colon_compete_edge() {
    let eng = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let (_c, _e, s) = eng.evaluate_meta_with_snapshot("text", "| :", &ctx);
    assert!(s.negotiation.iter().any(|e| e.relation == "compete"), "|+: should show compete edge");
}

#[test]
fn no_edge_when_only_one_side_present() {
    let eng = MetaEngineImpl::new_default();
    let ctx = FieldContext::default();
    let (_c, _e, s) = eng.evaluate_meta_with_snapshot("text", "Δ", &ctx);
    assert!(s.negotiation.is_empty(), "single operator should not produce negotiation edges");
}
