//! Runtime sentinel agent — monitors simulation health.

use async_trait::async_trait;
use tracing::{info, warn};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct RuntimeSentinelAgent {
    id: AgentId,
}

impl Default for RuntimeSentinelAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeSentinelAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("runtime_sentinel".into()),
        }
    }
}

#[async_trait]
impl Agent for RuntimeSentinelAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::RuntimeSentinel
    }
    fn description(&self) -> &str {
        "Monitors runtime health, detects NaN/blow-up, and triggers hot-swap recovery"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let health = ctx
            .params
            .get("health")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let mut alerts = Vec::new();
        let mut actions = Vec::new();

        for (field, stats) in &health {
            let nan_count = stats.get("nan_count").and_then(|v| v.as_u64()).unwrap_or(0);
            let inf_count = stats.get("inf_count").and_then(|v| v.as_u64()).unwrap_or(0);
            let max_val = stats
                .get("max_value")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let growth_rate = stats
                .get("growth_rate")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

            if nan_count > 0 {
                warn!(field, nan_count, "NaN detected");
                alerts.push(serde_json::json!({
                    "field": field, "type": "nan", "count": nan_count, "severity": "critical",
                }));
                actions.push(format!("hot-swap process for '{}'", field));
            }
            if inf_count > 0 {
                warn!(field, inf_count, "Inf detected");
                alerts.push(serde_json::json!({
                    "field": field, "type": "inf", "count": inf_count, "severity": "critical",
                }));
                actions.push(format!("reduce timestep for '{}'", field));
            }
            if max_val.abs() > 1e15 {
                alerts.push(serde_json::json!({
                    "field": field, "type": "blowup", "max_value": max_val, "severity": "warning",
                }));
                actions.push(format!("check CFL for '{}'", field));
            }
            if growth_rate > 10.0 {
                alerts.push(serde_json::json!({
                    "field": field, "type": "exponential_growth", "rate": growth_rate, "severity": "warning",
                }));
            }
        }

        let healthy = alerts.is_empty();
        info!(
            fields = health.len(),
            alerts = alerts.len(),
            healthy,
            "Health check"
        );

        Ok(AgentResult::ok(if healthy {
            "Runtime healthy — no anomalies detected".to_string()
        } else {
            format!(
                "ALERT: {} issues, {} actions recommended",
                alerts.len(),
                actions.len()
            )
        })
        .with_data(serde_json::json!({
            "healthy": healthy,
            "alerts": alerts,
            "recommended_actions": actions,
        })))
    }
}
