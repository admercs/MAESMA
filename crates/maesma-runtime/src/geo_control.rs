//! Geoengineering feedback control — Phase 12
//!
//! Control targets, actuator interface, control loop, strategy discovery,
//! termination shock analysis, and safety governance.

use serde::{Deserialize, Serialize};

/// A control target (setpoint + tolerance + measurement source).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlTarget {
    pub variable: String,
    pub setpoint: f64,
    pub tolerance: f64,
    pub measurement_source: String,
}

/// Intervention type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InterventionType {
    StratosphericAerosol,
    MarineCloudBrightening,
    DirectAirCapture,
    OceanAlkalinityEnhancement,
    IronFertilization,
    SurfaceAlbedo,
}

/// Actuator interface for a geoengineering intervention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actuator {
    pub intervention: InterventionType,
    pub control_variable: String,
    pub min_value: f64,
    pub max_value: f64,
    pub ramp_rate_per_year: f64,
}

/// A time-varying control schedule entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlScheduleEntry {
    pub year: i32,
    pub intervention: InterventionType,
    pub magnitude: f64,
}

/// Control loop state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ControlLoopPhase {
    Observe,
    ComputeError,
    Predict,
    Intervene,
    Simulate,
    Verify,
    Update,
}

/// Error computation from control target.
pub fn compute_control_error(target: &ControlTarget, observed: f64) -> f64 {
    observed - target.setpoint
}

/// Check if observation is within tolerance.
pub fn within_tolerance(target: &ControlTarget, observed: f64) -> bool {
    (observed - target.setpoint).abs() <= target.tolerance
}

/// Strategy optimization method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationMethod {
    ModelPredictiveControl,
    ReinforcementLearning,
    PortfolioOptimization,
}

/// Multi-objective evaluation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoEvaluation {
    pub cooling_efficacy: f64,
    pub side_effect_penalty: f64,
    pub economic_cost: f64,
    pub termination_shock_risk: f64,
}

/// Termination shock analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationShockAnalysis {
    pub cessation_year: i32,
    pub rebound_rate_k_per_decade: f64,
    pub overshoot_k: f64,
    pub tipping_proximity: f64,
    pub min_safe_rampdown_years: f64,
}

/// Robustness scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobustnessScenario {
    pub ecs: f64,
    pub ssp: String,
    pub actuator_failure: bool,
    pub tipping_point: Option<String>,
}

/// Side-effect constraint check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConstraint {
    pub variable: String,
    pub limit: f64,
    pub actual: f64,
    pub passed: bool,
}

/// Distributional equity analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityAnalysis {
    pub region: String,
    pub temperature_change_k: f64,
    pub precipitation_change_pct: f64,
    pub classification: EquityClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EquityClassification {
    Winner,
    Neutral,
    Loser,
}

/// Standard actuators.
pub fn standard_actuators() -> Vec<Actuator> {
    vec![
        Actuator {
            intervention: InterventionType::StratosphericAerosol,
            control_variable: "so2_injection_tg_yr".into(),
            min_value: 0.0,
            max_value: 20.0,
            ramp_rate_per_year: 2.0,
        },
        Actuator {
            intervention: InterventionType::DirectAirCapture,
            control_variable: "co2_removal_gt_yr".into(),
            min_value: 0.0,
            max_value: 10.0,
            ramp_rate_per_year: 1.0,
        },
        Actuator {
            intervention: InterventionType::MarineCloudBrightening,
            control_variable: "sea_salt_emission_tg_yr".into(),
            min_value: 0.0,
            max_value: 50.0,
            ramp_rate_per_year: 10.0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_error_computation() {
        let target = ControlTarget {
            variable: "global_mean_temp".into(),
            setpoint: 1.5,
            tolerance: 0.1,
            measurement_source: "era5".into(),
        };
        let error = compute_control_error(&target, 2.0);
        assert!((error - 0.5).abs() < 1e-10);
        assert!(!within_tolerance(&target, 2.0));
        assert!(within_tolerance(&target, 1.55));
    }

    #[test]
    fn standard_actuators_populated() {
        let a = standard_actuators();
        assert!(a.len() >= 3);
    }

    #[test]
    fn termination_analysis() {
        let t = TerminationShockAnalysis {
            cessation_year: 2080,
            rebound_rate_k_per_decade: 0.5,
            overshoot_k: 1.2,
            tipping_proximity: 0.7,
            min_safe_rampdown_years: 30.0,
        };
        assert!(t.min_safe_rampdown_years > 0.0);
    }
}
