//! Hypothesis agent — generates and tests scientific hypotheses as competing configs.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct HypothesisAgent {
    id: AgentId,
}

impl Default for HypothesisAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl HypothesisAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("hypothesis".into()),
        }
    }
}

#[async_trait]
impl Agent for HypothesisAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Hypothesis
    }
    fn description(&self) -> &str {
        "Formulates and tests scientific hypotheses as competing SAPG configurations"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let current_config = ctx
            .params
            .get("current_config")
            .cloned()
            .unwrap_or(serde_json::json!({}));
        let residual_pattern = ctx
            .params
            .get("residual_pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let target_field = ctx
            .params
            .get("target_field")
            .and_then(|v| v.as_str())
            .unwrap_or("state");

        let hypotheses = vec![
            serde_json::json!({
                "id": "H1",
                "description": format!("Missing process: add higher-rung representation for {}", target_field),
                "action": "upgrade_rung",
                "expected_improvement": "reduced systematic bias",
                "testable": true,
            }),
            serde_json::json!({
                "id": "H2",
                "description": format!("Wrong coupling: {} pattern suggests missing feedback loop", residual_pattern),
                "action": "add_coupling",
                "expected_improvement": "reduced variance in residuals",
                "testable": true,
            }),
            serde_json::json!({
                "id": "H3",
                "description": "Parameter calibration: current parameters may be suboptimal for this regime",
                "action": "recalibrate",
                "expected_improvement": "improved KGE",
                "testable": true,
            }),
        ];

        info!(
            hypotheses = hypotheses.len(),
            target = target_field,
            "Hypotheses generated"
        );

        Ok(AgentResult::ok(format!(
            "Generated {} competing hypotheses for '{}'",
            hypotheses.len(),
            target_field
        ))
        .with_data(serde_json::json!({
            "hypotheses": hypotheses,
            "current_config": current_config,
        }))
        .with_next("test hypotheses via parallel benchmarking"))
    }
}
