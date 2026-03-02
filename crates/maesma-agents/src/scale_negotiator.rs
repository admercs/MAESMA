//! Scale negotiator agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct ScaleNegotiatorAgent {
    id: AgentId,
}

impl ScaleNegotiatorAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("scale_negotiator".into()),
        }
    }
}

#[async_trait]
impl Agent for ScaleNegotiatorAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::ScaleNegotiator }
    fn description(&self) -> &str { "Negotiates spatial and temporal scale compatibility between coupled processes" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement scale_negotiator logic
        Ok(AgentResult::ok("ScaleNegotiatorAgent executed"))
    }
}
