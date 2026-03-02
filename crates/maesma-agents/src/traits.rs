//! Core agent trait and supporting types.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique agent identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The role an agent serves in the MAESMA workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    KbRetrieval,
    Assembly,
    ClosureValidator,
    Benchmarking,
    Selection,
    Optimizer,
    Discovery,
    DataScout,
    A2aGateway,
    RegimeDetector,
    ScaleNegotiator,
    Provenance,
    SalientDynamics,
    Ensemble,
    Diagnostics,
    Sensitivity,
    Hypothesis,
    Geoengineering,
    PlanetaryDefense,
    Trophic,
    Evolution,
    MetaLearner,
    RuntimeSentinel,
    FoundationModel,
    AutonomousObservation,
}

/// Context passed to agents on each invocation.
pub struct AgentContext {
    /// Arbitrary key-value parameters.
    pub params: HashMap<String, serde_json::Value>,
}

impl AgentContext {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn with_param(mut self, key: impl Into<String>, val: serde_json::Value) -> Self {
        self.params.insert(key.into(), val);
        self
    }
}

impl Default for AgentContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Result returned by an agent after execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    /// Whether the agent succeeded.
    pub success: bool,
    /// Human-readable summary.
    pub summary: String,
    /// Structured output data.
    pub data: Option<serde_json::Value>,
    /// Recommended next actions.
    pub next_actions: Vec<String>,
}

impl AgentResult {
    pub fn ok(summary: impl Into<String>) -> Self {
        Self {
            success: true,
            summary: summary.into(),
            data: None,
            next_actions: Vec::new(),
        }
    }

    pub fn fail(summary: impl Into<String>) -> Self {
        Self {
            success: false,
            summary: summary.into(),
            data: None,
            next_actions: Vec::new(),
        }
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_next(mut self, action: impl Into<String>) -> Self {
        self.next_actions.push(action.into());
        self
    }
}

/// The core agent trait. All 25 agents implement this.
#[async_trait]
pub trait Agent: Send + Sync {
    /// Unique identifier for this agent instance.
    fn id(&self) -> &AgentId;

    /// Agent role.
    fn role(&self) -> AgentRole;

    /// Human-readable description.
    fn description(&self) -> &str;

    /// Execute the agent's task.
    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult>;

    /// Whether the agent is ready to execute.
    fn is_ready(&self) -> bool {
        true
    }
}
