//! Salient dynamics agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct SalientDynamicsAgent {
    id: AgentId,
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
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::SalientDynamics }
    fn description(&self) -> &str { "Prioritizes processes with greatest effect on system state evolution" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement salient_dynamics logic
        Ok(AgentResult::ok("SalientDynamicsAgent executed"))
    }
}
