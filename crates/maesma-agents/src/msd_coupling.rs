//! MSD Coupling Agent — Phase 5.13
//!
//! Bidirectional coupling between human and natural systems for Multi-Sector
//! Dynamics.  Handles coupling frequency negotiation, cascading impact
//! propagation, and MSD scenario parameter space exploration.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// A cascading impact chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeChain {
    /// Ordered list of sectors affected.
    pub sectors: Vec<String>,
    /// Trigger event.
    pub trigger: String,
    /// Estimated propagation time (years).
    pub propagation_time: f64,
    /// Severity multiplier at each stage.
    pub severity_multipliers: Vec<f64>,
}

/// MSD scenario parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsdScenario {
    /// Shared Socioeconomic Pathway (SSP1–SSP5).
    pub ssp: String,
    /// Representative Concentration Pathway.
    pub rcp: String,
    /// Technology trajectory label.
    pub technology: String,
    /// Policy trajectory label.
    pub policy: String,
}

/// Coupling exchange between human and natural systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsdExchange {
    /// Source sector.
    pub from: String,
    /// Target sector.
    pub to: String,
    /// Variable being exchanged.
    pub variable: String,
    /// Coupling direction.
    pub direction: String,
    /// Update frequency (timesteps).
    pub frequency: u32,
}

pub struct MsdCouplingAgent {
    id: AgentId,
}

impl MsdCouplingAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("msd_coupling".into()),
        }
    }

    /// Define standard MSD coupling exchanges.
    pub fn standard_exchanges() -> Vec<MsdExchange> {
        vec![
            MsdExchange {
                from: "climate".into(),
                to: "water_systems".into(),
                variable: "precipitation_change".into(),
                direction: "natural_to_human".into(),
                frequency: 1,
            },
            MsdExchange {
                from: "climate".into(),
                to: "energy_systems".into(),
                variable: "temperature_extremes".into(),
                direction: "natural_to_human".into(),
                frequency: 1,
            },
            MsdExchange {
                from: "climate".into(),
                to: "agriculture".into(),
                variable: "growing_season_length".into(),
                direction: "natural_to_human".into(),
                frequency: 12,
            },
            MsdExchange {
                from: "water_systems".into(),
                to: "hydrology".into(),
                variable: "water_extraction".into(),
                direction: "human_to_natural".into(),
                frequency: 1,
            },
            MsdExchange {
                from: "energy_systems".into(),
                to: "atmosphere".into(),
                variable: "co2_emissions".into(),
                direction: "human_to_natural".into(),
                frequency: 12,
            },
            MsdExchange {
                from: "land_use".into(),
                to: "ecology".into(),
                variable: "land_cover_change".into(),
                direction: "human_to_natural".into(),
                frequency: 12,
            },
            MsdExchange {
                from: "infrastructure".into(),
                to: "hydrology".into(),
                variable: "impervious_fraction".into(),
                direction: "human_to_natural".into(),
                frequency: 120,
            },
            MsdExchange {
                from: "agriculture".into(),
                to: "biogeochemistry".into(),
                variable: "fertilizer_application".into(),
                direction: "human_to_natural".into(),
                frequency: 12,
            },
        ]
    }

    /// Build standard cascading impact chains.
    pub fn standard_cascades() -> Vec<CascadeChain> {
        vec![
            CascadeChain {
                trigger: "drought".into(),
                sectors: vec![
                    "water".into(),
                    "energy".into(),
                    "agriculture".into(),
                    "economy".into(),
                ],
                propagation_time: 0.5,
                severity_multipliers: vec![1.0, 0.8, 1.2, 0.6],
            },
            CascadeChain {
                trigger: "flood".into(),
                sectors: vec![
                    "infrastructure".into(),
                    "water".into(),
                    "health".into(),
                    "economy".into(),
                ],
                propagation_time: 0.1,
                severity_multipliers: vec![1.0, 0.9, 0.7, 0.5],
            },
            CascadeChain {
                trigger: "heatwave".into(),
                sectors: vec![
                    "energy".into(),
                    "health".into(),
                    "agriculture".into(),
                    "water".into(),
                ],
                propagation_time: 0.02,
                severity_multipliers: vec![1.0, 1.1, 0.8, 0.7],
            },
            CascadeChain {
                trigger: "wildfire".into(),
                sectors: vec![
                    "ecology".into(),
                    "air_quality".into(),
                    "infrastructure".into(),
                    "economy".into(),
                ],
                propagation_time: 0.05,
                severity_multipliers: vec![1.0, 0.9, 0.6, 0.4],
            },
        ]
    }
}

impl Default for MsdCouplingAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for MsdCouplingAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn role(&self) -> AgentRole {
        AgentRole::MsdCoupling
    }

    fn description(&self) -> &str {
        "Couples human and natural systems for multi-sector dynamics analysis"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("status");

        match action {
            "cascade" => {
                let trigger = ctx
                    .params
                    .get("trigger")
                    .and_then(|v| v.as_str())
                    .unwrap_or("drought");
                let severity = ctx
                    .params
                    .get("severity")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);

                let cascades = Self::standard_cascades();
                let matching: Vec<_> = cascades.iter().filter(|c| c.trigger == trigger).collect();

                let impacts: Vec<serde_json::Value> = matching
                    .iter()
                    .flat_map(|c| {
                        c.sectors.iter().zip(c.severity_multipliers.iter()).map(
                            move |(sector, mult)| {
                                serde_json::json!({
                                    "sector": sector,
                                    "impact_severity": severity * mult,
                                    "propagation_time_years": c.propagation_time,
                                })
                            },
                        )
                    })
                    .collect();

                let data = serde_json::json!({
                    "trigger": trigger,
                    "base_severity": severity,
                    "cascading_impacts": impacts,
                });
                Ok(AgentResult::ok(format!(
                    "Cascade analysis for '{}': {} sectors affected",
                    trigger,
                    impacts.len()
                ))
                .with_data(data))
            }

            "scenario" => {
                let ssp = ctx
                    .params
                    .get("ssp")
                    .and_then(|v| v.as_str())
                    .unwrap_or("SSP2");
                let rcp = ctx
                    .params
                    .get("rcp")
                    .and_then(|v| v.as_str())
                    .unwrap_or("4.5");
                let exchanges = Self::standard_exchanges();
                let scenario = MsdScenario {
                    ssp: ssp.into(),
                    rcp: rcp.into(),
                    technology: "reference".into(),
                    policy: "current_policies".into(),
                };
                let data = serde_json::json!({
                    "scenario": scenario,
                    "exchanges": exchanges,
                    "exchange_count": exchanges.len(),
                });
                Ok(AgentResult::ok(format!(
                    "MSD scenario {}-{}: {} exchanges configured",
                    ssp,
                    rcp,
                    exchanges.len()
                ))
                .with_data(data))
            }

            _ => {
                let exchanges = Self::standard_exchanges();
                let cascades = Self::standard_cascades();
                let data = serde_json::json!({
                    "exchange_count": exchanges.len(),
                    "cascade_count": cascades.len(),
                    "exchanges": exchanges,
                    "supported_triggers": cascades.iter().map(|c| &c.trigger).collect::<Vec<_>>(),
                });
                Ok(AgentResult::ok(format!(
                    "MSD coupling: {} exchanges, {} cascade templates",
                    exchanges.len(),
                    cascades.len()
                ))
                .with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_exchanges_populated() {
        let exchanges = MsdCouplingAgent::standard_exchanges();
        assert!(exchanges.len() >= 8);
        assert!(exchanges.iter().any(|e| e.direction == "human_to_natural"));
        assert!(exchanges.iter().any(|e| e.direction == "natural_to_human"));
    }

    #[test]
    fn standard_cascades_populated() {
        let cascades = MsdCouplingAgent::standard_cascades();
        assert!(cascades.len() >= 4);
    }

    #[tokio::test]
    async fn execute_cascade() {
        let agent = MsdCouplingAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("cascade"))
            .with_param("trigger", serde_json::json!("drought"))
            .with_param("severity", serde_json::json!(2.0));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
        let data = result.data.unwrap();
        assert!(data["cascading_impacts"].as_array().unwrap().len() >= 4);
    }
}
