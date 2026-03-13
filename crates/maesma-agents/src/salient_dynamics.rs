//! Salient dynamics agent — identifies the most impactful processes.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct SalientDynamicsAgent {
    id: AgentId,
}

impl Default for SalientDynamicsAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl SalientDynamicsAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("salient_dynamics".into()),
        }
    }
}

#[async_trait]
impl Agent for SalientDynamicsAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::SalientDynamics
    }
    fn description(&self) -> &str {
        "Prioritizes processes with greatest effect on system state evolution"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let field_variances = ctx
            .params
            .get("field_variances")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        if field_variances.is_empty() {
            return Ok(AgentResult::fail("No field variance data provided"));
        }

        let total_variance: f64 = field_variances.values().filter_map(|v| v.as_f64()).sum();

        let mut rankings: Vec<serde_json::Value> = field_variances
            .iter()
            .map(|(field, var)| {
                let v = var.as_f64().unwrap_or(0.0);
                let fraction = if total_variance > 0.0 {
                    v / total_variance
                } else {
                    0.0
                };
                serde_json::json!({
                    "field": field,
                    "variance": v,
                    "fraction_of_total": fraction,
                    "salient": fraction > 0.1,
                })
            })
            .collect();

        rankings.sort_by(|a, b| {
            let va = a.get("variance").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let vb = b.get("variance").and_then(|v| v.as_f64()).unwrap_or(0.0);
            vb.partial_cmp(&va).unwrap_or(std::cmp::Ordering::Equal)
        });

        let salient_count = rankings
            .iter()
            .filter(|r| r.get("salient").and_then(|v| v.as_bool()).unwrap_or(false))
            .count();

        info!(
            total_fields = field_variances.len(),
            salient = salient_count,
            "Salience analysis"
        );

        Ok(AgentResult::ok(format!(
            "{} of {} fields are dynamically salient (>10% variance)",
            salient_count,
            field_variances.len()
        ))
        .with_data(serde_json::json!({
            "rankings": rankings,
            "total_variance": total_variance,
            "salient_count": salient_count,
        })))
    }
}
