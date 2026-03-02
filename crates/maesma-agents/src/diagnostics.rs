//! Diagnostics agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct DiagnosticsAgent {
    id: AgentId,
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
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Diagnostics }
    fn description(&self) -> &str { "Computes emergent diagnostics and checks physical consistency of simulation output" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement diagnostics logic
        Ok(AgentResult::ok("DiagnosticsAgent executed"))
    }
}
