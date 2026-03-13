//! Geoengineering agent — evaluates intervention scenarios.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

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
        let scenario = ctx
            .params
            .get("scenario")
            .and_then(|v| v.as_str())
            .unwrap_or("none");
        let magnitude = ctx
            .params
            .get("magnitude")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        let assessment = match scenario {
            "stratospheric_aerosol" => serde_json::json!({
                "scenario": scenario,
                "description": "Stratospheric aerosol injection (SAI)",
                "estimated_forcing_wm2": -magnitude * 2.0,
                "affected_families": ["radiation", "atmosphere", "hydrology"],
                "risks": ["precipitation pattern changes", "ozone depletion", "termination shock"],
                "reversibility": "low",
            }),
            "marine_cloud_brightening" => serde_json::json!({
                "scenario": scenario,
                "description": "Marine cloud brightening (MCB)",
                "estimated_forcing_wm2": -magnitude * 0.5,
                "affected_families": ["radiation", "ocean", "atmosphere"],
                "risks": ["regional precipitation shifts", "marine ecosystem impacts"],
                "reversibility": "moderate",
            }),
            "ocean_alkalinity" => serde_json::json!({
                "scenario": scenario,
                "description": "Ocean alkalinity enhancement (OAE)",
                "estimated_co2_removal_gt_yr": magnitude * 0.1,
                "affected_families": ["ocean", "biogeochemistry"],
                "risks": ["local pH changes", "marine organism impacts"],
                "reversibility": "low",
            }),
            _ => serde_json::json!({
                "scenario": "unrecognized",
                "available_scenarios": ["stratospheric_aerosol", "marine_cloud_brightening", "ocean_alkalinity"],
            }),
        };

        info!(scenario, magnitude, "Geoengineering assessment");

        Ok(AgentResult::ok(format!(
            "Geoengineering: {} (magnitude {:.1})",
            scenario, magnitude
        ))
        .with_data(assessment))
    }
}
