//! Sensitivity agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct SensitivityAgent {
    id: AgentId,
}

impl SensitivityAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("sensitivity".into()),
        }
    }
}

#[async_trait]
impl Agent for SensitivityAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Sensitivity }
    fn description(&self) -> &str { "Performs sensitivity analysis to identify dominant parameters and processes" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement sensitivity logic
        Ok(AgentResult::ok("SensitivityAgent executed"))
    }
}
