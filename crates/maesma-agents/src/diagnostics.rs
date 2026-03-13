//! Diagnostics agent — physical consistency checks on simulation output.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct DiagnosticsAgent {
    id: AgentId,
}

impl Default for DiagnosticsAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("diagnostics".into()),
        }
    }
}

#[async_trait]
impl Agent for DiagnosticsAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Diagnostics
    }
    fn description(&self) -> &str {
        "Computes emergent diagnostics and checks physical consistency of simulation output"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let fields = ctx
            .params
            .get("fields")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let mut issues = Vec::new();
        let mut summary = Vec::new();

        for (name, stats) in &fields {
            let nan_count = stats.get("nan_count").and_then(|v| v.as_u64()).unwrap_or(0);
            let inf_count = stats.get("inf_count").and_then(|v| v.as_u64()).unwrap_or(0);
            let min = stats.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let max = stats.get("max").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let mean = stats.get("mean").and_then(|v| v.as_f64()).unwrap_or(0.0);

            if nan_count > 0 {
                issues.push(serde_json::json!({
                    "field": name, "type": "nan", "count": nan_count, "severity": "critical"
                }));
            }
            if inf_count > 0 {
                issues.push(serde_json::json!({
                    "field": name, "type": "inf", "count": inf_count, "severity": "critical"
                }));
            }
            if name.contains("temperature") && (min < 100.0 || max > 400.0) {
                issues.push(serde_json::json!({
                    "field": name, "type": "out_of_bounds",
                    "detail": format!("T range [{:.1}, {:.1}] K", min, max),
                    "severity": "warning"
                }));
            }
            if name.contains("moisture") && (min < 0.0 || max > 1.0) {
                issues.push(serde_json::json!({
                    "field": name, "type": "out_of_bounds",
                    "detail": format!("Moisture range [{:.3}, {:.3}]", min, max),
                    "severity": "warning"
                }));
            }
            summary.push(serde_json::json!({
                "field": name, "min": min, "max": max, "mean": mean,
            }));
        }

        let critical = issues
            .iter()
            .filter(|i| i.get("severity").and_then(|v| v.as_str()) == Some("critical"))
            .count();

        info!(
            fields = fields.len(),
            issues = issues.len(),
            critical,
            "Diagnostics complete"
        );

        Ok(AgentResult::ok(format!(
            "Diagnostics: {} fields, {} issues ({} critical)",
            fields.len(),
            issues.len(),
            critical
        ))
        .with_data(serde_json::json!({
            "issues": issues,
            "field_summary": summary,
            "healthy": critical == 0,
        })))
    }
}
