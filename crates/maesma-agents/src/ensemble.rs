//! Ensemble agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct EnsembleAgent {
    id: AgentId,
}

impl EnsembleAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("ensemble".into()),
        }
    }
}

#[async_trait]
impl Agent for EnsembleAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Ensemble }
    fn description(&self) -> &str { "Manages structural ensembles spanning multiple SAPG configurations" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement ensemble logic
        Ok(AgentResult::ok("EnsembleAgent executed"))
    }
}
