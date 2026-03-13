//! Scale negotiator agent — negotiates spatial/temporal resolution compatibility.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct ScaleNegotiatorAgent {
    id: AgentId,
}

impl Default for ScaleNegotiatorAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl ScaleNegotiatorAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("scale_negotiator".into()),
        }
    }
}

#[async_trait]
impl Agent for ScaleNegotiatorAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::ScaleNegotiator
    }
    fn description(&self) -> &str {
        "Negotiates spatial and temporal scale compatibility between coupled processes"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let processes = ctx
            .params
            .get("processes")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if processes.len() < 2 {
            return Ok(AgentResult::ok(
                "Need \u{2265}2 processes for scale negotiation",
            ));
        }

        let mut conflicts = Vec::new();
        let mut negotiated_dx = f64::MIN;
        let mut negotiated_dt = f64::MAX;

        for p in &processes {
            let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let dx_min = p.get("dx_min").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let dx_max = p
                .get("dx_max")
                .and_then(|v| v.as_f64())
                .unwrap_or(100_000.0);
            let dt_min = p.get("dt_min").and_then(|v| v.as_f64()).unwrap_or(1.0);
            let dt_max = p.get("dt_max").and_then(|v| v.as_f64()).unwrap_or(86400.0);

            if dx_min > negotiated_dx {
                negotiated_dx = dx_min;
            }
            if dt_max < negotiated_dt {
                negotiated_dt = dt_max;
            }

            if dx_min > dx_max {
                conflicts.push(serde_json::json!({
                    "process": name,
                    "issue": "invalid_spatial_range",
                    "dx_min": dx_min, "dx_max": dx_max,
                }));
            }
            if dt_min > dt_max {
                conflicts.push(serde_json::json!({
                    "process": name,
                    "issue": "invalid_temporal_range",
                    "dt_min": dt_min, "dt_max": dt_max,
                }));
            }
        }

        info!(
            dx = negotiated_dx,
            dt = negotiated_dt,
            conflicts = conflicts.len(),
            "Scale negotiation"
        );

        Ok(AgentResult::ok(format!(
            "Scale negotiation: dx={:.0}m, dt={:.0}s, {} conflicts",
            negotiated_dx,
            negotiated_dt,
            conflicts.len()
        ))
        .with_data(serde_json::json!({
            "negotiated_dx_m": negotiated_dx,
            "negotiated_dt_s": negotiated_dt,
            "conflicts": conflicts,
            "compatible": conflicts.is_empty(),
        })))
    }
}
