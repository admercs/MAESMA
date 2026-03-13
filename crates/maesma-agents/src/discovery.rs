//! Discovery agent — Phase 5.16 & Phase 11
//!
//! Structured residual analysis, hypothesis generation, ML learner proposals
//! (FNO/symbolic regression), validation gate, auto-manifest generation,
//! process lifecycle (candidate → provisional → validated → production),
//! and discovery provenance.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Residual analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualAnalysis {
    pub field: String,
    pub mean_residual: f64,
    pub variance: f64,
    pub max_abs: f64,
    pub spatial_autocorrelation: f64,
    pub severity: f64,
    pub attribution: String,
}

/// A hypothesis for a missing process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessHypothesis {
    pub id: String,
    pub residual_field: String,
    pub process_type: HypothesisType,
    pub description: String,
    pub confidence: f64,
    pub status: HypothesisStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HypothesisType {
    DiurnalEnergyBalance,
    LateralTransport,
    RecoveryCoupling,
    PhenologicalFeedback,
    MissingSourceSink,
    EventDriven,
    NonlinearResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HypothesisStatus {
    Proposed,
    UnderInvestigation,
    Confirmed,
    Rejected,
}

/// ML learner proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnerProposal {
    pub learner_type: LearnerType,
    pub target_field: String,
    pub input_variables: Vec<String>,
    pub estimated_complexity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearnerType {
    FourierNeuralOperator,
    DeepONet,
    SymbolicRegression,
    HybridPhysicsML,
}

/// Process lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessLifecycle {
    Candidate,
    Provisional,
    Validated,
    Production,
    Retired,
}

/// Validation gate result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationGate {
    pub skill_improvement: bool,
    pub conservation_100yr: bool,
    pub coupled_stability: bool,
    pub generalization: bool,
    pub sensitivity_ok: bool,
    pub passed: bool,
}

impl ValidationGate {
    pub fn evaluate(
        skill_delta: f64,
        conservation_err: f64,
        stable: bool,
        generalized: bool,
        sensitivity: f64,
    ) -> Self {
        let skill_improvement = skill_delta > 0.01;
        let conservation_100yr = conservation_err.abs() < 1e-6;
        let coupled_stability = stable;
        let generalization = generalized;
        let sensitivity_ok = sensitivity < 2.0;
        let passed = skill_improvement
            && conservation_100yr
            && coupled_stability
            && generalization
            && sensitivity_ok;
        Self {
            skill_improvement,
            conservation_100yr,
            coupled_stability,
            generalization,
            sensitivity_ok,
            passed,
        }
    }
}

/// Classify a residual pattern into a hypothesis type.
pub fn classify_residual(
    mean: f64,
    variance: f64,
    max_abs: f64,
    spatial_autocorr: f64,
) -> HypothesisType {
    if spatial_autocorr > 0.5 {
        HypothesisType::LateralTransport
    } else if mean.abs() > 0.5 && variance < 0.1 {
        HypothesisType::MissingSourceSink
    } else if max_abs > 3.0 * variance.sqrt().max(0.01) {
        HypothesisType::EventDriven
    } else if variance > 0.5 {
        HypothesisType::NonlinearResponse
    } else {
        HypothesisType::PhenologicalFeedback
    }
}

/// Score the severity of a residual.
pub fn severity_score(mean: f64, variance: f64, max_abs: f64, spatial_extent: f64) -> f64 {
    let magnitude = mean.abs() + variance.sqrt();
    let persistence = if variance > 0.1 { 1.0 } else { 0.5 };
    magnitude * spatial_extent * persistence * (1.0 + max_abs * 0.1)
}

pub struct DiscoveryAgent {
    id: AgentId,
}

impl Default for DiscoveryAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("discovery".into()),
        }
    }
}

#[async_trait]
impl Agent for DiscoveryAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Discovery
    }
    fn description(&self) -> &str {
        "Proposes novel process representations via structured residual analysis, \
         hypothesis generation, ML learners, and auto-manifest generation"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("analyze");

        match action {
            "analyze" => {
                let residuals = ctx
                    .params
                    .get("residual_stats")
                    .and_then(|v| v.as_object())
                    .cloned()
                    .unwrap_or_default();
                if residuals.is_empty() {
                    return Ok(AgentResult::fail("No residual statistics provided"));
                }
                let mut analyses = Vec::new();
                let mut hypotheses = Vec::new();
                for (field, stats) in &residuals {
                    let mean = stats.get("mean").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let var = stats
                        .get("variance")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let max_a = stats.get("max_abs").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let spatial = stats
                        .get("spatial_autocorrelation")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let sev = severity_score(mean, var, max_a, 1.0);
                    let hyp_type = classify_residual(mean, var, max_a, spatial);
                    analyses.push(ResidualAnalysis {
                        field: field.clone(),
                        mean_residual: mean,
                        variance: var,
                        max_abs: max_a,
                        spatial_autocorrelation: spatial,
                        severity: sev,
                        attribution: format!("{:?}", hyp_type),
                    });
                    if sev > 0.1 {
                        hypotheses.push(ProcessHypothesis {
                            id: format!("hyp_{}", field),
                            residual_field: field.clone(),
                            process_type: hyp_type,
                            description: format!("Missing process for {}", field),
                            confidence: (1.0 - 1.0 / (1.0 + sev)).min(0.95),
                            status: HypothesisStatus::Proposed,
                        });
                    }
                }
                let n = hypotheses.len();
                info!(
                    fields = residuals.len(),
                    hypotheses = n,
                    "Discovery analysis"
                );
                Ok(AgentResult::ok(format!(
                    "Analyzed {} fields, {} hypotheses",
                    residuals.len(),
                    n
                ))
                .with_data(serde_json::json!({ "analyses": analyses, "hypotheses": hypotheses }))
                .with_next("evaluate proposals via benchmarking"))
            }
            "propose_learners" => {
                let field = ctx
                    .params
                    .get("field")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let proposals = vec![
                    LearnerProposal {
                        learner_type: LearnerType::SymbolicRegression,
                        target_field: field.into(),
                        input_variables: vec!["temperature".into(), "moisture".into()],
                        estimated_complexity: 0.3,
                    },
                    LearnerProposal {
                        learner_type: LearnerType::FourierNeuralOperator,
                        target_field: field.into(),
                        input_variables: vec![
                            "temperature".into(),
                            "moisture".into(),
                            "wind".into(),
                        ],
                        estimated_complexity: 0.8,
                    },
                    LearnerProposal {
                        learner_type: LearnerType::HybridPhysicsML,
                        target_field: field.into(),
                        input_variables: vec!["temperature".into(), "moisture".into()],
                        estimated_complexity: 0.6,
                    },
                ];
                let data = serde_json::json!({ "proposals": proposals });
                Ok(AgentResult::ok(format!(
                    "{} learner proposals for {}",
                    proposals.len(),
                    field
                ))
                .with_data(data))
            }
            "validate" => {
                let skill_delta = ctx
                    .params
                    .get("skill_delta")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let cons_err = ctx
                    .params
                    .get("conservation_error")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let stable = ctx
                    .params
                    .get("stable")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                let generalized = ctx
                    .params
                    .get("generalized")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                let sensitivity = ctx
                    .params
                    .get("sensitivity")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                let gate = ValidationGate::evaluate(
                    skill_delta,
                    cons_err,
                    stable,
                    generalized,
                    sensitivity,
                );
                let data = serde_json::json!({ "gate": gate });
                let msg = if gate.passed {
                    "Validation PASSED"
                } else {
                    "Validation FAILED"
                };
                Ok(AgentResult::ok(msg).with_data(data))
            }
            _ => {
                let data = serde_json::json!({
                    "available_actions": ["analyze", "propose_learners", "validate"],
                    "lifecycle_states": ["candidate", "provisional", "validated", "production", "retired"],
                });
                Ok(AgentResult::ok("Discovery agent status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_lateral_transport() {
        let t = classify_residual(0.1, 0.1, 0.5, 0.8);
        assert_eq!(t, HypothesisType::LateralTransport);
    }

    #[test]
    fn classify_missing_source() {
        let t = classify_residual(1.0, 0.05, 1.1, 0.1);
        assert_eq!(t, HypothesisType::MissingSourceSink);
    }

    #[test]
    fn validation_gate_passes() {
        let g = ValidationGate::evaluate(0.05, 1e-8, true, true, 1.5);
        assert!(g.passed);
    }

    #[test]
    fn validation_gate_fails_conservation() {
        let g = ValidationGate::evaluate(0.05, 0.1, true, true, 1.5);
        assert!(!g.passed);
    }

    #[test]
    fn severity_increases_with_mean() {
        let s1 = severity_score(0.1, 0.1, 0.5, 1.0);
        let s2 = severity_score(1.0, 0.1, 0.5, 1.0);
        assert!(s2 > s1);
    }
}
