//! Provenance agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct ProvenanceAgent {
    id: AgentId,
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
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Provenance }
    fn description(&self) -> &str { "Tracks full provenance chain for all knowledgebase mutations and assembly decisions" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement provenance logic
        Ok(AgentResult::ok("ProvenanceAgent executed"))
    }
}
