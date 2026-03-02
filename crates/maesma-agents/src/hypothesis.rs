//! Hypothesis agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct HypothesisAgent {
    id: AgentId,
}

impl HypothesisAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("hypothesis".into()),
        }
    }
}

#[async_trait]
impl Agent for HypothesisAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Hypothesis }
    fn description(&self) -> &str { "Formulates and tests scientific hypotheses as competing SAPG configurations" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement hypothesis logic
        Ok(AgentResult::ok("HypothesisAgent executed"))
    }
}
