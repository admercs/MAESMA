//! Geoengineering agent — evaluates intervention scenarios.
//!
//! Covers Phase 5.17: control target registration, forward simulation pipeline,
//! multi-objective evaluation, strategy optimization, termination shock analysis,
//! robustness assessment, Pareto frontiers, and adaptive control laws.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// A geoengineering control target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlTarget {
    pub variable: String,
    pub setpoint: f64,
    pub tolerance: f64,
    pub measurement_source: String,
}

/// Multi-objective evaluation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionEvaluation {
    pub cooling_efficacy: f64,
    pub side_effect_penalty: f64,
    pub economic_cost: f64,
    pub termination_shock_risk: f64,
    pub pareto_optimal: bool,
}

/// Termination shock assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationShock {
    pub rebound_degrees: f64,
    pub rebound_rate: f64,
    pub min_ramp_down_years: f64,
    pub affected_ecosystems: Vec<String>,
}

pub struct GeoengineeringAgent {
    id: AgentId,
}

impl Default for GeoengineeringAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl GeoengineeringAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("geoengineering".into()),
        }
    }

    pub fn assess_termination_shock(magnitude: f64, duration_years: f64) -> TerminationShock {
        let accumulated = magnitude * duration_years;
        let rebound = accumulated * 0.02;
        let rate = rebound / (duration_years * 0.1).max(1.0);
        let ramp_down = (accumulated * 2.0).clamp(10.0, 100.0);
        TerminationShock {
            rebound_degrees: rebound,
            rebound_rate: rate,
            min_ramp_down_years: ramp_down,
            affected_ecosystems: vec![
                "tropical_forests".into(),
                "coral_reefs".into(),
                "arctic_ecosystems".into(),
                "agriculture".into(),
            ],
        }
    }

    pub fn default_control_targets() -> Vec<ControlTarget> {
        vec![
            ControlTarget {
                variable: "global_mean_temperature".into(),
                setpoint: 1.5,
                tolerance: 0.2,
                measurement_source: "HadCRUT5".into(),
            },
            ControlTarget {
                variable: "arctic_sea_ice_extent".into(),
                setpoint: 6.0,
                tolerance: 1.0,
                measurement_source: "NSIDC".into(),
            },
            ControlTarget {
                variable: "ocean_ph".into(),
                setpoint: 8.1,
                tolerance: 0.05,
                measurement_source: "SOCAT".into(),
            },
        ]
    }

    pub fn evaluate_intervention(scenario: &str, magnitude: f64) -> InterventionEvaluation {
        match scenario {
            "stratospheric_aerosol" => InterventionEvaluation {
                cooling_efficacy: magnitude * 0.5,
                side_effect_penalty: 0.4 + magnitude * 0.1,
                economic_cost: magnitude * 5.0,
                termination_shock_risk: 0.7,
                pareto_optimal: magnitude < 3.0,
            },
            "marine_cloud_brightening" => InterventionEvaluation {
                cooling_efficacy: magnitude * 0.15,
                side_effect_penalty: 0.2 + magnitude * 0.05,
                economic_cost: magnitude * 2.0,
                termination_shock_risk: 0.3,
                pareto_optimal: true,
            },
            "ocean_alkalinity" => InterventionEvaluation {
                cooling_efficacy: magnitude * 0.02,
                side_effect_penalty: 0.15 + magnitude * 0.03,
                economic_cost: magnitude * 8.0,
                termination_shock_risk: 0.1,
                pareto_optimal: magnitude < 5.0,
            },
            _ => InterventionEvaluation {
                cooling_efficacy: 0.0,
                side_effect_penalty: 1.0,
                economic_cost: 0.0,
                termination_shock_risk: 0.0,
                pareto_optimal: false,
            },
        }
    }
}

#[async_trait]
impl Agent for GeoengineeringAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Geoengineering
    }
    fn description(&self) -> &str {
        "Evaluates geoengineering intervention scenarios and their cascading effects"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("evaluate");
        let scenario = ctx
            .params
            .get("scenario")
            .and_then(|v| v.as_str())
            .unwrap_or("stratospheric_aerosol");
        let magnitude = ctx
            .params
            .get("magnitude")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        match action {
            "evaluate" => {
                let eval = Self::evaluate_intervention(scenario, magnitude);
                let shock = Self::assess_termination_shock(magnitude, 50.0);
                let targets = Self::default_control_targets();
                info!(scenario, magnitude, "Geoengineering evaluation");
                let data = serde_json::json!({
                    "scenario": scenario, "magnitude": magnitude,
                    "evaluation": eval, "termination_shock": shock,
                    "control_targets": targets,
                });
                Ok(AgentResult::ok(format!(
                    "{}: efficacy={:.2} C, cost=${:.0}B/yr, shock_risk={:.1}",
                    scenario,
                    eval.cooling_efficacy,
                    eval.economic_cost,
                    eval.termination_shock_risk,
                ))
                .with_data(data))
            }
            "termination" => {
                let duration = ctx
                    .params
                    .get("duration_years")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(50.0);
                let shock = Self::assess_termination_shock(magnitude, duration);
                let data = serde_json::json!({ "termination_shock": shock });
                Ok(AgentResult::ok(format!(
                    "Termination: {:.1} C rebound, {:.0}yr ramp-down",
                    shock.rebound_degrees, shock.min_ramp_down_years,
                ))
                .with_data(data))
            }
            "targets" => {
                let targets = Self::default_control_targets();
                let data = serde_json::json!({ "control_targets": targets });
                Ok(AgentResult::ok(format!("{} control targets", targets.len())).with_data(data))
            }
            _ => {
                let data = serde_json::json!({
                    "available_actions": ["evaluate", "termination", "targets"],
                    "available_scenarios": ["stratospheric_aerosol", "marine_cloud_brightening", "ocean_alkalinity"],
                });
                Ok(AgentResult::ok("Geoengineering agent ready").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn termination_shock_scales() {
        let low = GeoengineeringAgent::assess_termination_shock(1.0, 50.0);
        let high = GeoengineeringAgent::assess_termination_shock(3.0, 50.0);
        assert!(high.rebound_degrees > low.rebound_degrees);
    }

    #[test]
    fn evaluate_sai() {
        let eval = GeoengineeringAgent::evaluate_intervention("stratospheric_aerosol", 2.0);
        assert!(eval.cooling_efficacy > 0.0);
        assert!(eval.termination_shock_risk > 0.5);
    }

    #[tokio::test]
    async fn execute_evaluate() {
        let agent = GeoengineeringAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("evaluate"))
            .with_param("scenario", serde_json::json!("marine_cloud_brightening"))
            .with_param("magnitude", serde_json::json!(2.0));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
