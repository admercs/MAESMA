//! Provenance agent — tracks lineage of all decisions and mutations.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct ProvenanceAgent {
    id: AgentId,
}

impl Default for ProvenanceAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvenanceAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("provenance".into()),
        }
    }
}

#[async_trait]
impl Agent for ProvenanceAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Provenance
    }
    fn description(&self) -> &str {
        "Tracks full provenance chain for all knowledgebase mutations and assembly decisions"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let agent = ctx
            .params
            .get("agent")
            .and_then(|v| v.as_str())
            .unwrap_or("system");
        let target = ctx
            .params
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let rationale = ctx
            .params
            .get("rationale")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let generation = ctx
            .params
            .get("generation")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let record = serde_json::json!({
            "action": action,
            "agent": agent,
            "target": target,
            "rationale": rationale,
            "generation": generation,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        info!(action, agent, target, "Provenance recorded");

        Ok(
            AgentResult::ok(format!("Provenance: {} by {} on {}", action, agent, target))
                .with_data(serde_json::json!({"record": record})),
        )
    }
}
