//! Discovery agent — proposes novel process representations from residual analysis.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct DiscoveryAgent {
    id: AgentId,
}

impl Default for DiscoveryAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("discovery".into()),
        }
    }
}

#[async_trait]
impl Agent for DiscoveryAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Discovery
    }
    fn description(&self) -> &str {
        "Proposes novel process representations via symbolic regression or neural architecture search"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let residuals = ctx
            .params
            .get("residual_stats")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        if residuals.is_empty() {
            return Ok(AgentResult::fail("No residual statistics provided"));
        }

        let mut proposals = Vec::new();
        for (field, stats) in &residuals {
            let mean_residual = stats.get("mean").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let variance = stats
                .get("variance")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let max_abs = stats.get("max_abs").and_then(|v| v.as_f64()).unwrap_or(0.0);

            // High mean residual = systematic bias = missing process
            if mean_residual.abs() > 0.1 {
                proposals.push(serde_json::json!({
                    "field": field,
                    "issue": "systematic_bias",
                    "severity": mean_residual.abs(),
                    "suggestion": format!("Add missing source/sink for '{}'", field),
                    "candidate_type": "empirical_correction",
                }));
            }

            // High variance = wrong functional form
            if variance > 0.5 {
                proposals.push(serde_json::json!({
                    "field": field,
                    "issue": "high_variance",
                    "severity": variance,
                    "suggestion": format!("Explore nonlinear representations for '{}'", field),
                    "candidate_type": "symbolic_regression",
                }));
            }

            // Large max = extreme events not captured
            if max_abs > 3.0 * variance.sqrt().max(0.01) {
                proposals.push(serde_json::json!({
                    "field": field,
                    "issue": "extreme_events",
                    "severity": max_abs,
                    "suggestion": format!("Add threshold/event-driven process for '{}'", field),
                    "candidate_type": "event_process",
                }));
            }
        }

        let n = proposals.len();
        info!(
            fields = residuals.len(),
            proposals = n,
            "Discovery analysis"
        );

        Ok(AgentResult::ok(format!(
            "Analyzed {} fields, proposed {} new process candidates",
            residuals.len(),
            n
        ))
        .with_data(serde_json::json!({
            "proposals": proposals,
            "fields_analyzed": residuals.len(),
        }))
        .with_next("evaluate proposals via benchmarking"))
    }
}
