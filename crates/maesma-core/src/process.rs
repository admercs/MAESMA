//! Process identity, representation, and fidelity rungs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::families::ProcessFamily;

/// Unique identifier for a process representation in the knowledgebase.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessId(pub Uuid);

impl ProcessId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ProcessId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ProcessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Fidelity rung on the representation ladder (R0–R3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FidelityRung {
    /// R0: Regional / coarse / empirical.
    R0,
    /// R1: Landscape / intermediate fidelity.
    R1,
    /// R2: Event / local / high fidelity.
    R2,
    /// R3: Research / maximum fidelity.
    R3,
}

impl FidelityRung {
    pub fn label(&self) -> &'static str {
        match self {
            Self::R0 => "Regional",
            Self::R1 => "Landscape",
            Self::R2 => "Event/Local",
            Self::R3 => "Research",
        }
    }
}

impl std::fmt::Display for FidelityRung {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self, self.label())
    }
}

/// Origin provenance for a process representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessOrigin {
    /// Hand-coded by a human developer.
    HandCoded,
    /// Discovered from observational data by the process discovery pipeline.
    Discovered {
        /// Hash of the training data used.
        data_fingerprint: String,
        /// ID of the residual analysis that motivated discovery.
        motivating_residual: Option<String>,
    },
    /// Imported from an external model or library.
    Imported { source: String, version: String },
    /// Federated — received from an A2A peer.
    Federated { peer_id: String },
}

/// A process representation in the knowledgebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRepresentation {
    /// Unique identifier.
    pub id: ProcessId,
    /// Human-readable name.
    pub name: String,
    /// Description of what this process represents.
    pub description: String,
    /// Which process family this belongs to.
    pub family: ProcessFamily,
    /// Fidelity rung on the representation ladder.
    pub rung: FidelityRung,
    /// Provenance / origin.
    pub origin: ProcessOrigin,
    /// Semantic version of this representation.
    pub version: String,
    /// Tags for regime applicability.
    pub regime_tags: Vec<String>,
    /// Whether this representation is currently active/usable.
    pub active: bool,
    /// Lifecycle status.
    pub lifecycle: LifecycleStatus,
    /// Creation timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Lifecycle status for discovered representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStatus {
    /// Just submitted, not yet validated.
    Candidate,
    /// Passed initial validation gates.
    Provisional,
    /// Passed full multi-criteria validation.
    Validated,
    /// In active use in production configurations.
    Production,
    /// Archived — superseded or failed re-validation.
    Archived,
}

/// The trait that all runnable process implementations must satisfy.
pub trait ProcessRunner: Send + Sync {
    /// Returns the process family.
    fn family(&self) -> ProcessFamily;

    /// Returns the fidelity rung.
    fn rung(&self) -> FidelityRung;

    /// Returns input variable names this process reads.
    fn inputs(&self) -> Vec<String>;

    /// Returns output variable names this process writes.
    fn outputs(&self) -> Vec<String>;

    /// Returns the set of conserved quantities this process respects.
    fn conserved_quantities(&self) -> Vec<String>;

    /// Advance the process by one timestep (dt in seconds).
    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> crate::Result<()>;
}

/// Trait for accessing simulation state that process runners read/write.
pub trait ProcessState {
    /// Get a scalar field by name.
    fn get_field(&self, name: &str) -> Option<&ndarray::ArrayD<f64>>;

    /// Get a mutable scalar field by name.
    fn get_field_mut(&mut self, name: &str) -> Option<&mut ndarray::ArrayD<f64>>;

    /// Get a scalar parameter by name.
    fn get_param(&self, name: &str) -> Option<f64>;

    /// Current simulation time.
    fn time(&self) -> chrono::DateTime<chrono::Utc>;
}
