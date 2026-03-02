//! Trophic agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct TrophicAgent {
    id: AgentId,
}

impl TrophicAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("trophic".into()),
        }
    }
}

#[async_trait]
impl Agent for TrophicAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Trophic }
    fn description(&self) -> &str { "Manages trophic dynamics and food-web process representations" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement trophic logic
        Ok(AgentResult::ok("TrophicAgent executed"))
    }
}
