//! Assembly agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct AssemblyAgent {
    id: AgentId,
}

impl AssemblyAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("assembly".into()),
        }
    }
}

#[async_trait]
impl Agent for AssemblyAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Assembly }
    fn description(&self) -> &str { "Composes candidate SAPG assemblies from retrieved process representations" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement assembly logic
        Ok(AgentResult::ok("AssemblyAgent executed"))
    }
}
