//! Planetary defense agent — evaluates planetary-scale threats and responses.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

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
        "Simulates planetary defense scenarios including impact and climate response"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let threat = ctx
            .params
            .get("threat")
            .and_then(|v| v.as_str())
            .unwrap_or("none");
        let severity = ctx
            .params
            .get("severity")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let assessment = match threat {
            "asteroid_impact" => {
                let energy_mt = severity;
                serde_json::json!({
                    "threat": "asteroid_impact",
                    "energy_mt": energy_mt,
                    "climate_effects": {
                        "dust_loading_tg": energy_mt * 0.5,
                        "temperature_change_k": -(energy_mt / 100.0).min(10.0),
                        "duration_years": (energy_mt / 10.0).min(5.0),
                    },
                    "affected_families": ["atmosphere", "radiation", "ecology", "fire"],
                    "sapg_modifications": ["activate fire cascade", "add dust aerosol forcing"],
                })
            }
            "supervolcano" => serde_json::json!({
                "threat": "supervolcano",
                "vei": severity,
                "climate_effects": {
                    "aerosol_forcing_wm2": -severity * 2.0,
                    "temperature_change_k": -severity * 0.8,
                    "duration_years": severity * 0.5,
                },
                "affected_families": ["atmosphere", "radiation", "biogeochemistry"],
            }),
            "solar_storm" => serde_json::json!({
                "threat": "solar_storm",
                "class": if severity > 5.0 { "X-class" } else { "M-class" },
                "effects": ["ionospheric disruption", "geomagnetic storm"],
                "affected_families": ["atmosphere"],
            }),
            _ => serde_json::json!({
                "threat": "none",
                "available_threats": ["asteroid_impact", "supervolcano", "solar_storm"],
            }),
        };

        info!(threat, severity, "Planetary defense assessment");

        Ok(
            AgentResult::ok(format!("Threat: {} (severity {:.1})", threat, severity))
                .with_data(assessment),
        )
    }
}
