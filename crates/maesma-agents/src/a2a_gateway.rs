//! Agent-to-agent gateway — federation protocol for cross-instance collaboration.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct A2aGatewayAgent {
    id: AgentId,
}

impl Default for A2aGatewayAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl A2aGatewayAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("a2a_gateway".into()),
        }
    }
}

#[async_trait]
impl Agent for A2aGatewayAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::A2aGateway
    }
    fn description(&self) -> &str {
        "Handles agent-to-agent federation protocol for cross-instance collaboration"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("status");
        let peers = ctx
            .params
            .get("peers")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        match action {
            "broadcast" => {
                let payload = ctx
                    .params
                    .get("payload")
                    .cloned()
                    .unwrap_or(serde_json::json!({}));
                info!(peers = peers.len(), "Broadcasting to peers");
                Ok(
                    AgentResult::ok(format!("Broadcast queued to {} peers", peers.len()))
                        .with_data(serde_json::json!({
                            "action": "broadcast",
                            "peers_targeted": peers.len(),
                            "payload_size": payload.to_string().len(),
                        })),
                )
            }
            "request_manifests" => {
                let family = ctx
                    .params
                    .get("family")
                    .and_then(|v| v.as_str())
                    .unwrap_or("all");
                info!(
                    family,
                    peers = peers.len(),
                    "Requesting manifests from peers"
                );
                Ok(AgentResult::ok(format!(
                    "Manifest request sent to {} peers for family '{}'",
                    peers.len(),
                    family
                ))
                .with_data(serde_json::json!({
                    "action": "request_manifests",
                    "family": family,
                    "peers_queried": peers.len(),
                })))
            }
            _ => Ok(
                AgentResult::ok(format!("Gateway status: {} peers configured", peers.len()))
                    .with_data(serde_json::json!({
                        "peers": peers.len(),
                        "status": "ready",
                    })),
            ),
        }
    }
}
