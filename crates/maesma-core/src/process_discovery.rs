//! Process discovery framework — Phase 11
//!
//! Residual analysis, hypothesis generation, ML learning pipeline,
//! validation gates, and knowledgebase deposit of discovered processes.

use serde::{Deserialize, Serialize};

// ── 11.1 Residual Analysis Framework ──

/// Structured bias detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasDetection {
    pub variable: String,
    pub region: String,
    pub season: String,
    pub magnitude: f64,
    pub spatial_autocorrelation: f64,
    pub persistent_across_calibrations: bool,
}

/// Attribution method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttributionMethod {
    PartialCorrelation,
    GrangerCausality,
    TransferEntropy,
}

/// Severity score components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityComponents {
    pub magnitude: f64,
    pub extent: f64,
    pub persistence: f64,
    pub relevance: f64,
}

impl SeverityComponents {
    pub fn composite(&self) -> f64 {
        self.magnitude * self.extent * self.persistence * self.relevance
    }
}

// ── 11.2 Hypothesis Generation ──

/// Residual classification → process type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResidualClass {
    DiurnalCycle,
    LateralTransport,
    PostEventRecovery,
    SeasonalPhase,
    SpatialGradient,
    Unclassified,
}

/// Classification rule: map residual pattern → process type.
pub fn classify_residual_pattern(
    diurnal_amplitude: f64,
    spatial_gradient: f64,
    post_event_lag: f64,
    seasonal_phase_error: f64,
) -> ResidualClass {
    if diurnal_amplitude > 0.5 {
        ResidualClass::DiurnalCycle
    } else if spatial_gradient > 0.3 {
        if post_event_lag > 0.0 {
            ResidualClass::PostEventRecovery
        } else {
            ResidualClass::SpatialGradient
        }
    } else if seasonal_phase_error > 0.2 {
        ResidualClass::SeasonalPhase
    } else if post_event_lag > 0.0 {
        ResidualClass::PostEventRecovery
    } else {
        ResidualClass::Unclassified
    }
}

/// Process hypothesis status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryHypothesisStatus {
    Proposed,
    TrainingData,
    Training,
    Validating,
    Validated,
    Rejected,
}

/// A registered hypothesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessHypothesisEntry {
    pub id: String,
    pub residual_class: ResidualClass,
    pub description: String,
    pub status: DiscoveryHypothesisStatus,
    pub literature_refs: Vec<String>,
}

// ── 11.3 ML Process Learning Pipeline ──

/// ML learner type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MlLearnerType {
    Fno,
    DeepONet,
    SymbolicRegression,
    HybridPhysicsMl,
}

/// Training configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub learner: MlLearnerType,
    pub physics_informed_loss: bool,
    pub region_cross_validation: bool,
    pub complexity_penalty: f64,
}

/// Training result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub learner: MlLearnerType,
    pub train_skill: f64,
    pub val_skill: f64,
    pub test_skill: f64,
    pub complexity: f64,
    pub conservation_error: f64,
}

// ── 11.4 Validation & Knowledgebase Deposit ──

/// Multi-criteria validation gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCriteria {
    pub skill_improvement_threshold: f64,
    pub conservation_horizon_years: f64,
    pub stability_check: bool,
    pub generalization_check: bool,
    pub sensitivity_check: bool,
}

impl Default for ValidationCriteria {
    fn default() -> Self {
        Self {
            skill_improvement_threshold: 0.05,
            conservation_horizon_years: 100.0,
            stability_check: true,
            generalization_check: true,
            sensitivity_check: true,
        }
    }
}

/// Evaluate training result against validation criteria.
pub fn evaluate_validation_gate(
    result: &TrainingResult,
    baseline_skill: f64,
    criteria: &ValidationCriteria,
) -> bool {
    let skill_ok = result.test_skill - baseline_skill >= criteria.skill_improvement_threshold;
    let conservation_ok = result.conservation_error < 0.01;
    let generalization_ok =
        !criteria.generalization_check || (result.test_skill / result.val_skill.max(1e-10)) > 0.8;
    skill_ok && conservation_ok && generalization_ok
}

/// Process lifecycle stage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveredProcessStage {
    Candidate,
    Provisional,
    Validated,
    Production,
    Retired,
}

/// Auto-manifest for a discovered process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredManifest {
    pub process_name: String,
    pub origin: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub scale_envelope: String,
    pub regime_tags: Vec<String>,
    pub cost_estimate: f64,
    pub skill_estimate: f64,
    pub stage: DiscoveredProcessStage,
    pub provenance_chain: Vec<String>,
}

// ── 11.5 Integration With Core Loop ──

/// Discovery budget for a cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryBudget {
    pub max_hypotheses: u32,
    pub max_training_hours: f64,
    pub max_validation_runs: u32,
}

impl Default for DiscoveryBudget {
    fn default() -> Self {
        Self {
            max_hypotheses: 10,
            max_training_hours: 24.0,
            max_validation_runs: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_diurnal() {
        assert_eq!(
            classify_residual_pattern(0.8, 0.1, 0.0, 0.0),
            ResidualClass::DiurnalCycle,
        );
    }

    #[test]
    fn classify_lateral() {
        assert_eq!(
            classify_residual_pattern(0.1, 0.5, 0.0, 0.0),
            ResidualClass::SpatialGradient,
        );
    }

    #[test]
    fn classify_post_event() {
        assert_eq!(
            classify_residual_pattern(0.1, 0.5, 1.0, 0.0),
            ResidualClass::PostEventRecovery,
        );
    }

    #[test]
    fn severity_composite() {
        let s = SeverityComponents {
            magnitude: 0.8,
            extent: 0.5,
            persistence: 0.9,
            relevance: 1.0,
        };
        assert!((s.composite() - 0.36).abs() < 1e-10);
    }

    #[test]
    fn validation_gate_pass() {
        let result = TrainingResult {
            learner: MlLearnerType::Fno,
            train_skill: 0.9,
            val_skill: 0.85,
            test_skill: 0.8,
            complexity: 1.0,
            conservation_error: 0.001,
        };
        let criteria = ValidationCriteria::default();
        assert!(evaluate_validation_gate(&result, 0.5, &criteria));
    }

    #[test]
    fn validation_gate_fail_conservation() {
        let result = TrainingResult {
            learner: MlLearnerType::SymbolicRegression,
            train_skill: 0.9,
            val_skill: 0.85,
            test_skill: 0.8,
            complexity: 1.0,
            conservation_error: 0.1,
        };
        let criteria = ValidationCriteria::default();
        assert!(!evaluate_validation_gate(&result, 0.5, &criteria));
    }
}
