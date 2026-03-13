//! Regime detector agent — detects environmental regime shifts.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct RegimeDetectorAgent {
    id: AgentId,
}

impl Default for RegimeDetectorAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl RegimeDetectorAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("regime_detector".into()),
        }
    }
}

#[async_trait]
impl Agent for RegimeDetectorAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::RegimeDetector
    }
    fn description(&self) -> &str {
        "Detects environmental regime shifts and triggers SAPG reconfiguration"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let field_stats = ctx
            .params
            .get("field_stats")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let shift_threshold = ctx
            .params
            .get("threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(2.0);

        let mut detected_shifts = Vec::new();
        for (field, stats) in &field_stats {
            let mean = stats.get("mean").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let std = stats.get("std").and_then(|v| v.as_f64()).unwrap_or(1.0);
            let trend = stats.get("trend").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let baseline = stats
                .get("baseline_mean")
                .and_then(|v| v.as_f64())
                .unwrap_or(mean);

            let z_score = if std > 0.0 {
                (mean - baseline).abs() / std
            } else {
                0.0
            };
            let trend_significant = trend.abs() > shift_threshold * std;

            if z_score > shift_threshold || trend_significant {
                detected_shifts.push(serde_json::json!({
                    "field": field,
                    "z_score": z_score,
                    "trend": trend,
                    "regime": if mean > baseline { "above_normal" } else { "below_normal" },
                    "confidence": (1.0 - (-z_score).exp()).min(0.99),
                }));
            }
        }

        let n = detected_shifts.len();
        info!(shifts = n, fields = field_stats.len(), "Regime detection");

        if n > 0 {
            Ok(AgentResult::ok(format!("Detected {} regime shifts", n))
                .with_data(serde_json::json!({"shifts": detected_shifts, "reconfigure": true}))
                .with_next("trigger SAPG reconfiguration"))
        } else {
            Ok(AgentResult::ok("No regime shifts detected")
                .with_data(serde_json::json!({"shifts": [], "reconfigure": false})))
        }
    }
}
