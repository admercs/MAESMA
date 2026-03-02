//! Process manifest — machine-readable metadata for knowledgebase entries.

use serde::{Deserialize, Serialize};

use crate::families::ProcessFamily;
use crate::process::{FidelityRung, LifecycleStatus, ProcessId, ProcessOrigin};

/// Machine-readable process manifest (the core unit of the knowledgebase).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessManifest {
    /// Unique identifier.
    pub id: ProcessId,
    /// Human-readable name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Process family.
    pub family: ProcessFamily,
    /// Fidelity rung.
    pub rung: FidelityRung,
    /// Semantic version.
    pub version: String,
    /// I/O contract.
    pub io: IoContract,
    /// Scale envelope.
    pub scale: ScaleEnvelope,
    /// Conservation properties.
    pub conservation: Vec<ConservationProperty>,
    /// Cost model.
    pub cost: CostModel,
    /// Regime applicability tags.
    pub regime_tags: Vec<String>,
    /// Ontology relations.
    pub relations: Vec<OntologyRelation>,
    /// Provenance.
    pub origin: ProcessOrigin,
    /// Lifecycle status.
    pub lifecycle: LifecycleStatus,
    /// Backend support.
    pub backends: Vec<ComputeBackend>,
}

/// Input/output contract declaring what state variables a process reads and writes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoContract {
    /// Input variables with units.
    pub inputs: Vec<Variable>,
    /// Output variables with units.
    pub outputs: Vec<Variable>,
    /// Parameters with defaults and bounds.
    pub parameters: Vec<Parameter>,
}

/// A named variable with unit and optional description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub unit: String,
    pub description: Option<String>,
    pub dimensions: Vec<String>,
}

/// A tunable parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub unit: String,
    pub default: f64,
    pub bounds: Option<(f64, f64)>,
    pub description: Option<String>,
}

/// Scale envelope defining where a process representation is valid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleEnvelope {
    /// Minimum spatial resolution (meters).
    pub dx_min: f64,
    /// Maximum spatial resolution (meters).
    pub dx_max: f64,
    /// Minimum timestep (seconds).
    pub dt_min: f64,
    /// Maximum timestep (seconds).
    pub dt_max: f64,
    /// Coupling tier.
    pub coupling_tier: CouplingTier,
}

/// Two-tier coupling classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouplingTier {
    /// Slow processes: days–centuries (succession, soil C, management).
    Slow,
    /// Fast processes: seconds–hours (fire, plume, overland flow).
    Fast,
}

/// Conservation property a process claims to respect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationProperty {
    pub quantity: String,
    pub method: String,
}

/// Computational cost model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostModel {
    /// Estimated FLOPS per grid cell per timestep.
    pub flops_per_cell: f64,
    /// Estimated memory per grid cell (bytes).
    pub memory_per_cell: u64,
    /// Whether GPU acceleration is supported.
    pub gpu_capable: bool,
}

/// Ontology relation between processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyRelation {
    pub relation_type: RelationType,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    CompatibleWith,
    IncompatibleWith,
    RequiresCouplingWith,
    Supersedes,
    VariantOf,
}

/// Compute backend support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputeBackend {
    Cpu,
    Cuda,
    Rocm,
    MlEmulator,
}
