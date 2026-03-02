//! Benchmarking agent.

use async_trait::async_trait;
use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct BenchmarkingAgent {
    id: AgentId,
}

impl BenchmarkingAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("benchmarking".into()),
        }
    }
}

#[async_trait]
impl Agent for BenchmarkingAgent {
    fn id(&self) -> &AgentId { &self.id }
    fn role(&self) -> AgentRole { AgentRole::Benchmarking }
    fn description(&self) -> &str { "Runs process representations against observational benchmarks and records skill scores" }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement benchmarking logic
        Ok(AgentResult::ok("BenchmarkingAgent executed"))
    }
}
