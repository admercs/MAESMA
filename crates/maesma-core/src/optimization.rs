//! Optimization loop types — Phase 7
//!
//! Combinatorial hypothesis engine, emulator screening, continuous calibration,
//! knowledgebase feedback loop, experiment orchestrator, and autonomous data
//! discovery loop.

use serde::{Deserialize, Serialize};

// ── 7.1 Combinatorial Hypothesis Engine ──

/// A configuration hypothesis to evaluate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHypothesis {
    /// Family-rung assignments (family name → rung label).
    pub rung_assignments: Vec<(String, String)>,
    /// Whether this is a single-family perturbation or factorial combination.
    pub kind: HypothesisKind,
    /// Expected computational cost (normalized).
    pub estimated_cost: f64,
    /// Priority score from hypothesis engine.
    pub priority: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HypothesisKind {
    SingleFamily,
    Factorial,
    Interaction,
    Pruned,
}

/// Enumerate single-family perturbations: for each family, try each rung.
pub fn enumerate_single_family(
    families: &[&str],
    available_rungs: &[&str],
) -> Vec<ConfigHypothesis> {
    let mut out = Vec::new();
    for &fam in families {
        for &rung in available_rungs {
            out.push(ConfigHypothesis {
                rung_assignments: vec![(fam.to_string(), rung.to_string())],
                kind: HypothesisKind::SingleFamily,
                estimated_cost: match rung {
                    "R0" => 1.0,
                    "R1" => 5.0,
                    "R2" => 25.0,
                    "R3" => 100.0,
                    _ => 10.0,
                },
                priority: 0.5,
            });
        }
    }
    out
}

/// Prune configurations that exceed budget.
pub fn prune_by_budget(configs: &mut Vec<ConfigHypothesis>, max_cost: f64) {
    configs.retain(|c| c.estimated_cost <= max_cost);
}

// ── 7.2 Emulator-Accelerated Screening ──

/// A surrogate model for pre-screening.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorRecord {
    pub emulator_id: String,
    pub family: String,
    pub fidelity: f64,
    pub training_runs: u32,
    pub stale: bool,
}

/// Screening outcome for a configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningOutcome {
    pub config_id: String,
    pub predicted_skill: f64,
    pub uncertainty: f64,
    pub passed: bool,
}

/// Screen configs using an emulator. Those with predicted skill below
/// the threshold (accounting for uncertainty) are eliminated.
pub fn screen_configs(
    configs: &[ConfigHypothesis],
    emulator_fidelity: f64,
    skill_threshold: f64,
) -> Vec<ScreeningOutcome> {
    configs
        .iter()
        .enumerate()
        .map(|(i, c)| {
            // Simplified screening: cost inversely correlated with predicted skill
            let predicted = emulator_fidelity * (1.0 - 0.1 / c.estimated_cost.max(0.1));
            let uncertainty = 1.0 - emulator_fidelity;
            ScreeningOutcome {
                config_id: format!("cfg_{i}"),
                predicted_skill: predicted,
                uncertainty,
                passed: predicted + uncertainty >= skill_threshold,
            }
        })
        .collect()
}

// ── 7.3 Continuous Calibration ──

/// Hierarchical calibration level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CalibrationLevel {
    Global,
    Regional,
    Regime,
}

/// A calibration task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationTask {
    pub parameter: String,
    pub level: CalibrationLevel,
    pub prior_mean: f64,
    pub prior_std: f64,
    pub posterior_mean: Option<f64>,
    pub posterior_std: Option<f64>,
    pub identifiable: bool,
}

/// Simple online Bayesian update for a parameter.
pub fn bayesian_parameter_update(
    prior_mean: f64,
    prior_var: f64,
    observation: f64,
    obs_var: f64,
) -> (f64, f64) {
    let k = prior_var / (prior_var + obs_var);
    let post_mean = prior_mean + k * (observation - prior_mean);
    let post_var = (1.0 - k) * prior_var;
    (post_mean, post_var)
}

// ── 7.4 Knowledgebase Feedback Loop ──

/// Feedback action to update the knowledgebase.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KbFeedbackAction {
    ReplaceExpertPrior,
    UpdateCostModel,
    TightenConstraint,
    RelaxConstraint,
    ShiftRungPreference,
    AddRegimeTag,
    SharpenParameterPrior,
    UpdateManifest,
}

/// A KB feedback entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KbFeedback {
    pub action: KbFeedbackAction,
    pub target: String,
    pub reason: String,
    pub evidence_runs: u32,
}

// ── 7.5 Experiment Orchestrator ──

/// Experiment lifecycle stage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExperimentStage {
    Queued,
    Compiling,
    Running,
    Scoring,
    Storing,
    Complete,
    Failed,
}

/// An experiment in the orchestrator queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub hypothesis: ConfigHypothesis,
    pub stage: ExperimentStage,
    pub budget_used: f64,
    pub result_skill: Option<f64>,
}

/// Check whether diminishing returns stop criterion is met.
pub fn diminishing_returns(recent_improvements: &[f64], threshold: f64) -> bool {
    if recent_improvements.len() < 3 {
        return false;
    }
    let mean_improvement: f64 =
        recent_improvements.iter().sum::<f64>() / recent_improvements.len() as f64;
    mean_improvement < threshold
}

// ── 7.6 Autonomous Data Discovery Loop ──

/// A data discovery trigger.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryTrigger {
    PeriodicGapAnalysis,
    NewProductAvailable,
    SkillPlateau,
    RegimeDetected,
}

/// A re-score request from the data discovery loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RescoreRequest {
    pub trigger: DiscoveryTrigger,
    pub affected_families: Vec<String>,
    pub new_product: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_single_family_count() {
        let configs = enumerate_single_family(&["fire", "hydrology"], &["R0", "R1"]);
        assert_eq!(configs.len(), 4);
        assert!(
            configs
                .iter()
                .all(|c| c.kind == HypothesisKind::SingleFamily)
        );
    }

    #[test]
    fn prune_by_budget_removes_expensive() {
        let mut configs = enumerate_single_family(&["fire"], &["R0", "R1", "R2", "R3"]);
        assert_eq!(configs.len(), 4);
        prune_by_budget(&mut configs, 10.0);
        assert!(configs.iter().all(|c| c.estimated_cost <= 10.0));
    }

    #[test]
    fn screen_configs_filters() {
        let configs = enumerate_single_family(&["fire"], &["R0", "R1"]);
        let outcomes = screen_configs(&configs, 0.9, 0.5);
        assert_eq!(outcomes.len(), 2);
    }

    #[test]
    fn bayesian_update_shifts_toward_obs() {
        let (post_mean, post_var) = bayesian_parameter_update(0.0, 1.0, 1.0, 1.0);
        assert!((post_mean - 0.5).abs() < 1e-10);
        assert!((post_var - 0.5).abs() < 1e-10);
    }

    #[test]
    fn diminishing_returns_detection() {
        assert!(diminishing_returns(&[0.001, 0.0005, 0.0002], 0.01));
        assert!(!diminishing_returns(&[0.1, 0.08, 0.06], 0.01));
    }

    #[test]
    fn experiment_lifecycle() {
        let hyp = ConfigHypothesis {
            rung_assignments: vec![("fire".into(), "R1".into())],
            kind: HypothesisKind::SingleFamily,
            estimated_cost: 5.0,
            priority: 0.8,
        };
        let exp = Experiment {
            id: "exp_1".into(),
            hypothesis: hyp,
            stage: ExperimentStage::Queued,
            budget_used: 0.0,
            result_skill: None,
        };
        assert_eq!(exp.stage, ExperimentStage::Queued);
    }
}
