//! Planetary defense agent — Phase 5.18
//!
//! NEO ingest, threat assessment (Palermo scale), impact cascade modeling,
//! deflection strategy optimization, mass extinction calibration, and
//! observation campaign recommendations.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// A near-Earth object threat assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoThreat {
    pub designation: String,
    pub diameter_m: f64,
    pub impact_probability: f64,
    pub palermo_scale: f64,
    pub impact_energy_mt: f64,
    pub warning_time_years: f64,
}

/// Deflection strategy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeflectionStrategy {
    pub method: String,
    pub delta_v_required_cm_s: f64,
    pub mission_lead_time_years: f64,
    pub success_probability: f64,
    pub cost_estimate_b: f64,
}

/// Impact cascade assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactCascade {
    pub impact_type: String,
    pub dust_loading_tg: f64,
    pub temperature_change_k: f64,
    pub duration_years: f64,
    pub affected_families: Vec<String>,
    pub food_web_disruption: f64,
}

/// Mass extinction calibration reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtinctionReference {
    pub event: String,
    pub age_mya: f64,
    pub species_loss_pct: f64,
    pub recovery_time_myr: f64,
    pub primary_cause: String,
}

pub struct PlanetaryDefenseAgent {
    id: AgentId,
}

impl Default for PlanetaryDefenseAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl PlanetaryDefenseAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("planetary_defense".into()),
        }
    }

    pub fn assess_impact(diameter_m: f64) -> ImpactCascade {
        let energy_mt = 0.001 * diameter_m.powi(3) * 0.5;
        let (dust, temp_change, duration) = if energy_mt > 1e6 {
            (1e4, -10.0, 10.0)
        } else if energy_mt > 1e3 {
            (1e2, -3.0, 2.0)
        } else if energy_mt > 1.0 {
            (1.0, -0.1, 0.1)
        } else {
            (0.01, 0.0, 0.0)
        };
        ImpactCascade {
            impact_type: if diameter_m > 100.0 {
                "global".into()
            } else {
                "regional".into()
            },
            dust_loading_tg: dust,
            temperature_change_k: temp_change,
            duration_years: duration,
            affected_families: vec![
                "atmosphere".into(),
                "radiation".into(),
                "ecology".into(),
                "ocean".into(),
                "biogeochemistry".into(),
                "trophic_dynamics".into(),
            ],
            food_web_disruption: (energy_mt / 1e6).min(1.0),
        }
    }

    pub fn deflection_strategies(warning_years: f64, diameter_m: f64) -> Vec<DeflectionStrategy> {
        let delta_v = 0.01 * diameter_m / warning_years.max(1.0);
        vec![
            DeflectionStrategy {
                method: "kinetic_impactor".into(),
                delta_v_required_cm_s: delta_v,
                mission_lead_time_years: 2.0,
                success_probability: if warning_years > 5.0 { 0.85 } else { 0.4 },
                cost_estimate_b: 1.0 + diameter_m * 0.001,
            },
            DeflectionStrategy {
                method: "gravity_tractor".into(),
                delta_v_required_cm_s: delta_v * 0.5,
                mission_lead_time_years: 10.0,
                success_probability: if warning_years > 15.0 { 0.7 } else { 0.1 },
                cost_estimate_b: 5.0,
            },
            DeflectionStrategy {
                method: "ion_beam".into(),
                delta_v_required_cm_s: delta_v * 0.3,
                mission_lead_time_years: 8.0,
                success_probability: if warning_years > 10.0 { 0.6 } else { 0.15 },
                cost_estimate_b: 3.0,
            },
            DeflectionStrategy {
                method: "nuclear_standoff".into(),
                delta_v_required_cm_s: delta_v * 2.0,
                mission_lead_time_years: 1.0,
                success_probability: if warning_years > 1.0 { 0.9 } else { 0.6 },
                cost_estimate_b: 2.0,
            },
        ]
    }

    pub fn extinction_references() -> Vec<ExtinctionReference> {
        vec![
            ExtinctionReference {
                event: "K-Pg (Chicxulub)".into(),
                age_mya: 66.0,
                species_loss_pct: 76.0,
                recovery_time_myr: 10.0,
                primary_cause: "asteroid_impact".into(),
            },
            ExtinctionReference {
                event: "P-T (Great Dying)".into(),
                age_mya: 252.0,
                species_loss_pct: 96.0,
                recovery_time_myr: 30.0,
                primary_cause: "volcanism_ocean_anoxia".into(),
            },
            ExtinctionReference {
                event: "Late Devonian".into(),
                age_mya: 375.0,
                species_loss_pct: 75.0,
                recovery_time_myr: 15.0,
                primary_cause: "unknown_multiple".into(),
            },
            ExtinctionReference {
                event: "End-Triassic".into(),
                age_mya: 201.0,
                species_loss_pct: 80.0,
                recovery_time_myr: 10.0,
                primary_cause: "volcanism_co2".into(),
            },
        ]
    }

    pub fn neo_data_sources() -> Vec<&'static str> {
        vec![
            "CNEOS_Sentry",
            "CNEOS_Scout",
            "JPL_Horizons",
            "MPC",
            "USAF_SSN",
            "USSF_18SDS",
            "ESA_NEOCC",
        ]
    }
}

#[async_trait]
impl Agent for PlanetaryDefenseAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::PlanetaryDefense
    }
    fn description(&self) -> &str {
        "Monitors and assesses planetary-scale environmental threats"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("assess");

        match action {
            "assess" => {
                let diameter = ctx
                    .params
                    .get("diameter_m")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(100.0);
                let warning = ctx
                    .params
                    .get("warning_years")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(10.0);
                let impact = Self::assess_impact(diameter);
                let strategies = Self::deflection_strategies(warning, diameter);
                let best = strategies.iter().max_by(|a, b| {
                    a.success_probability
                        .partial_cmp(&b.success_probability)
                        .unwrap()
                });
                let data = serde_json::json!({
                    "impact_assessment": impact,
                    "deflection_strategies": strategies,
                    "recommended": best.map(|s| &s.method),
                    "data_sources": Self::neo_data_sources(),
                });
                Ok(AgentResult::ok(format!(
                    "NEO {}m: {} impact, recommend {}",
                    diameter,
                    impact.impact_type,
                    best.map(|s| s.method.as_str()).unwrap_or("none"),
                ))
                .with_data(data))
            }
            "calibrate" => {
                let refs = Self::extinction_references();
                let data = serde_json::json!({ "extinction_references": refs });
                Ok(
                    AgentResult::ok(format!("{} extinction references loaded", refs.len()))
                        .with_data(data),
                )
            }
            _ => {
                let data = serde_json::json!({
                    "available_actions": ["assess", "calibrate"],
                    "data_sources": Self::neo_data_sources(),
                });
                Ok(AgentResult::ok("Planetary defense ready").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impact_scales_with_diameter() {
        let small = PlanetaryDefenseAgent::assess_impact(10.0);
        let large = PlanetaryDefenseAgent::assess_impact(1000.0);
        assert!(large.dust_loading_tg > small.dust_loading_tg);
    }

    #[test]
    fn deflection_strategies_populated() {
        let strats = PlanetaryDefenseAgent::deflection_strategies(10.0, 300.0);
        assert_eq!(strats.len(), 4);
    }

    #[test]
    fn extinction_references_populated() {
        let refs = PlanetaryDefenseAgent::extinction_references();
        assert!(refs.len() >= 4);
    }

    #[tokio::test]
    async fn execute_assess() {
        let agent = PlanetaryDefenseAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("assess"))
            .with_param("diameter_m", serde_json::json!(500.0))
            .with_param("warning_years", serde_json::json!(15.0));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
