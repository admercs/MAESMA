//! Data scout agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct DataScoutAgent {
    id: AgentId,
}

impl DataScoutAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("data_scout".into()),
        }
    }
}

#[async_trait]
impl Agent for DataScoutAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::DataScout }
    fn description(&self) -> &str { "Discovers and ingests observational datasets for benchmarking" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement data_scout logic
        Ok(AgentResult::ok("DataScoutAgent executed"))
    }
}
