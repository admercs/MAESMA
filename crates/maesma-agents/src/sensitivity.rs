//! Sensitivity agent — perturbation-based sensitivity analysis.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct SensitivityAgent {
    id: AgentId,
}

impl Default for SensitivityAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl SensitivityAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("sensitivity".into()),
        }
    }
}

#[async_trait]
impl Agent for SensitivityAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Sensitivity
    }
    fn description(&self) -> &str {
        "Performs sensitivity analysis to identify dominant parameters and processes"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let perturbations = ctx
            .params
            .get("perturbations")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let base_skill = ctx
            .params
            .get("base_skill")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        if perturbations.is_empty() {
            return Ok(AgentResult::fail("No perturbation results provided"));
        }

        let mut indices: Vec<serde_json::Value> = perturbations
            .iter()
            .map(|p| {
                let param = p.get("parameter").and_then(|v| v.as_str()).unwrap_or("?");
                let delta = p.get("delta").and_then(|v| v.as_f64()).unwrap_or(0.1);
                let new_skill = p
                    .get("new_skill")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(base_skill);
                let sensitivity = if delta.abs() > 1e-10 {
                    (new_skill - base_skill).abs() / delta.abs()
                } else {
                    0.0
                };
                serde_json::json!({
                    "parameter": param,
                    "sensitivity_index": sensitivity,
                    "delta_skill": new_skill - base_skill,
                    "direction": if new_skill > base_skill { "improves" } else { "degrades" },
                })
            })
            .collect();

        indices.sort_by(|a, b| {
            let sa = a
                .get("sensitivity_index")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let sb = b
                .get("sensitivity_index")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            sb.partial_cmp(&sa).unwrap_or(std::cmp::Ordering::Equal)
        });

        let dominant = indices
            .first()
            .and_then(|v| v.get("parameter"))
            .and_then(|v| v.as_str())
            .unwrap_or("none");

        info!(
            params = perturbations.len(),
            dominant, "Sensitivity analysis"
        );

        Ok(AgentResult::ok(format!(
            "Sensitivity: {} parameters, dominant = '{}'",
            perturbations.len(),
            dominant
        ))
        .with_data(serde_json::json!({
            "indices": indices,
            "dominant_parameter": dominant,
        })))
    }
}
