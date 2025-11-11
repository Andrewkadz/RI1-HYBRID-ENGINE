use std::collections::{HashMap, HashSet};
use ri1_core::constraints::{OperatorClass, ResonanceEvent};

#[derive(serde::Serialize, Debug, Clone)]
pub struct OperatorWeight { pub operator: OperatorClass, pub weight: f64 }

#[derive(serde::Serialize, Debug, Clone)]
pub struct InfluenceEdge { pub from: OperatorClass, pub to: OperatorClass, pub relation: String, pub weight: f64 }

#[derive(serde::Serialize, Debug, Clone)]
pub struct InfluenceSnapshot {
    pub resonance_index: f64,
    pub operator_influence: Vec<OperatorWeight>,
    pub cooperation_count: usize,
    pub conflict_count: usize,
    pub negotiation: Vec<InfluenceEdge>,
    pub notes: Option<String>,
}

pub fn compute_influence(events: &Vec<ResonanceEvent>) -> (InfluenceSnapshot, ResonanceEvent) {
    use OperatorClass::*;
    let mut score: HashMap<OperatorClass, f64> = HashMap::new();
    let mut has: HashSet<OperatorClass> = HashSet::new();
    let mut conflicts: usize = 0;
    let mut coop: usize = 0;
    let mut violations: usize = 0;

    for e in events {
        has.insert(e.operator);
        let w = match e.operator {
            FlowVector | Simultaneity | InteractionInterface | Disruption | Orthogonality | LoopCycle | StabilizationResolution => 0.5,
            _ => 1.0,
        };
        *score.entry(e.operator).or_insert(0.0) += w;
        match e.operator {
            Disruption => { conflicts += 1; },
            InteractionViolation => { violations += 1; conflicts += 1; },
            Orthogonality => { conflicts += 1; },
            Simultaneity | HarmonicStabilization | Coexistence => { coop += 1; },
            _ => {}
        }
    }

    let total: f64 = score.values().cloned().sum();
    let mut operator_influence: Vec<OperatorWeight> = score.into_iter().map(|(op,s)| OperatorWeight{ operator: op, weight: if total>0.0 { s/total } else { 0.0 }}).collect();
    operator_influence.sort_by(|a,b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));

    let mut negotiation: Vec<InfluenceEdge> = Vec::new();
    if has.contains(&Fusion) && has.contains(&StructuralIllumination) { negotiation.push(InfluenceEdge{ from: Fusion, to: StructuralIllumination, relation: "reinforce".into(), weight: 0.6 }); }
    if has.contains(&Oscillation) && has.contains(&StructuralIllumination) { negotiation.push(InfluenceEdge{ from: Oscillation, to: StructuralIllumination, relation: "reinforce".into(), weight: 0.6 }); }
    if has.contains(&Orthogonality) && has.contains(&InteractionInterface) { negotiation.push(InfluenceEdge{ from: Orthogonality, to: InteractionInterface, relation: "compete".into(), weight: 0.3 }); }
    if has.contains(&ClosureIntegration) && (has.contains(&Fusion) || has.contains(&Transcendence)) { negotiation.push(InfluenceEdge{ from: ClosureIntegration, to: Fusion, relation: "compete".into(), weight: 0.3 }); }

    let mut r = 1.0;
    if conflicts > 0 { r -= 0.15; }
    if violations > 0 { r -= 0.10; }
    if conflicts > 1 { r -= 0.05 * ((conflicts - 1) as f64).min(4.0); }
    if has.contains(&LoopCycle) && has.contains(&ClosureIntegration) { r += 0.05; }
    if r < 0.0 { r = 0.0; }
    if r > 1.0 { r = 1.0; }

    let top = operator_influence.iter().take(3).map(|ow| format!("{:?}:{:.2}", ow.operator, ow.weight)).collect::<Vec<_>>().join(", ");
    let summary = ResonanceEvent {
        operator: OperatorClass::InteractionNotice,
        message: format!("influence_summary: resonance_index={:.2} top=[{}] coop={} conflict={}", r, top, coop, conflicts),
        section_ref: None,
        symbol: None,
    };

    (InfluenceSnapshot { resonance_index: r, operator_influence, cooperation_count: coop, conflict_count: conflicts, negotiation, notes: None }, summary)
}
