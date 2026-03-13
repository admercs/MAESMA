//! Optimizer agent — multi-objective parameter tuning.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct OptimizerAgent {
    id: AgentId,
}

impl Default for OptimizerAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizerAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("optimizer".into()),
        }
    }
}

#[async_trait]
impl Agent for OptimizerAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Optimizer
    }
    fn description(&self) -> &str {
        "Tunes process parameters using multi-objective optimization"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let parameters = ctx
            .params
            .get("parameters")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let step_size = ctx
            .params
            .get("step_size")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.1);

        if parameters.is_empty() {
            return Ok(AgentResult::fail("No parameters provided for optimization"));
        }

        let mut suggestions = Vec::new();
        for (name, val) in &parameters {
            let current = val.as_f64().unwrap_or(1.0);
            let bounds = ctx
                .params
                .get("bounds")
                .and_then(|b| b.get(name))
                .and_then(|b| {
                    let lo = b.get(0).and_then(|v| v.as_f64())?;
                    let hi = b.get(1).and_then(|v| v.as_f64())?;
                    Some((lo, hi))
                });

            let delta = current.abs() * step_size;
            let (up, down) = if let Some((lo, hi)) = bounds {
                ((current + delta).min(hi), (current - delta).max(lo))
            } else {
                (current + delta, current - delta)
            };

            suggestions.push(serde_json::json!({
                "parameter": name,
                "current": current,
                "perturbed_up": up,
                "perturbed_down": down,
                "step_size": delta,
            }));
        }

        let n = suggestions.len();
        info!(
            parameters = n,
            step_size, "Optimization perturbations generated"
        );

        Ok(AgentResult::ok(format!(
            "Generated {} parameter perturbation pairs (step={})",
            n, step_size
        ))
        .with_data(serde_json::json!({
            "suggestions": suggestions,
            "n_parameters": n,
            "total_evaluations": n * 2,
        }))
        .with_next("run benchmarking with perturbed params"))
    }
}
