//! Knowledge-base retrieval agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct KbRetrievalAgent {
    id: AgentId,
}

impl KbRetrievalAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("kb_retrieval".into()),
        }
    }
}

#[async_trait]
impl Agent for KbRetrievalAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::KbRetrieval }
    fn description(&self) -> &str { "Queries the knowledgebase for process representations matching regime, scale, and skill criteria" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement kb_retrieval logic
        Ok(AgentResult::ok("KbRetrievalAgent executed"))
    }
}
