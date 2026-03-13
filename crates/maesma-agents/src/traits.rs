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
    IntentScope,
    ActiveLearning,
    SkillLibrarian,
    MsdCoupling,
    ScenarioDiscovery,
}

impl AgentRole {
    /// All agent roles.
    pub fn all() -> &'static [Self] {
        &[
            Self::KbRetrieval,
            Self::Assembly,
            Self::ClosureValidator,
            Self::Benchmarking,
            Self::Selection,
            Self::Optimizer,
            Self::Discovery,
            Self::DataScout,
            Self::A2aGateway,
            Self::RegimeDetector,
            Self::ScaleNegotiator,
            Self::Provenance,
            Self::SalientDynamics,
            Self::Ensemble,
            Self::Diagnostics,
            Self::Sensitivity,
            Self::Hypothesis,
            Self::Geoengineering,
            Self::PlanetaryDefense,
            Self::Trophic,
            Self::Evolution,
            Self::MetaLearner,
            Self::RuntimeSentinel,
            Self::FoundationModel,
            Self::AutonomousObservation,
            Self::IntentScope,
            Self::ActiveLearning,
            Self::SkillLibrarian,
            Self::MsdCoupling,
            Self::ScenarioDiscovery,
        ]
    }

    /// Human-readable description of the role.
    pub fn description(&self) -> &'static str {
        match self {
            Self::KbRetrieval => "Retrieves process representations from the knowledgebase",
            Self::Assembly => "Assembles SAPG configurations from available processes",
            Self::ClosureValidator => "Validates conservation and closure properties",
            Self::Benchmarking => "Benchmarks process representations against observations",
            Self::Selection => "Selects optimal representations via multi-criteria analysis",
            Self::Optimizer => "Tunes process parameters to improve skill metrics",
            Self::Discovery => "Discovers new process representations from residuals",
            Self::DataScout => "Locates and ingests observational datasets",
            Self::A2aGateway => "Manages agent-to-agent federation with peers",
            Self::RegimeDetector => "Detects environmental regime shifts",
            Self::ScaleNegotiator => "Negotiates spatial/temporal scale compatibility",
            Self::Provenance => "Tracks full lineage and provenance of decisions",
            Self::SalientDynamics => "Identifies dynamically important processes",
            Self::Ensemble => "Manages ensemble runs and uncertainty quantification",
            Self::Diagnostics => "Generates diagnostic reports and visualizations",
            Self::Sensitivity => "Performs sensitivity analysis on SAPG configurations",
            Self::Hypothesis => "Generates and tests scientific hypotheses",
            Self::Geoengineering => "Evaluates geoengineering intervention scenarios",
            Self::PlanetaryDefense => "Monitors planetary-scale environmental threats",
            Self::Trophic => "Manages trophic web interactions and food chains",
            Self::Evolution => "Drives ALife-based evolution of process populations",
            Self::MetaLearner => "Learns meta-strategies from cross-domain experience",
            Self::RuntimeSentinel => "Monitors runtime health and detects anomalies",
            Self::FoundationModel => "Orchestrates foundation model inference",
            Self::AutonomousObservation => "Controls autonomous observation platforms",
            Self::IntentScope => "Parses user objectives into observable requirements",
            Self::ActiveLearning => "Identifies high-uncertainty configurations for experiments",
            Self::SkillLibrarian => "Manages skill score lifecycle and drift detection",
            Self::MsdCoupling => "Couples human and natural systems for MSD analysis",
            Self::ScenarioDiscovery => "Explores scenario spaces to find tipping points",
        }
    }
}
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
