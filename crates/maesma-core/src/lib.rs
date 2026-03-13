//! # maesma-core
//!
//! Core traits, types, and domain primitives for the Modular Agentic Earth
//! System Modeling Arena (MAESMA). This crate defines the foundational
//! abstractions that all other crates depend on.

pub mod automaton;
pub mod error;
pub mod evolution;
pub mod families;
pub mod graph;
pub mod manifest;
pub mod metrics;
pub mod observations;
pub mod ontology;
pub mod process;
pub mod protocols;
pub mod regime;
pub mod skills;
pub mod spatial;
pub mod topology;
pub mod units;
pub mod variables;

pub use automaton::{Constitution, HeartbeatConfig, ProcessAutomaton, ProcessSoul, SurvivalTier};
pub use error::{Error, Result};
pub use evolution::{EvolutionCandidate, EvolutionConfig, Population, ProcessLineage};
pub use families::ProcessFamily;
pub use graph::{Sapg, SapgEdgeRecord, SapgSnapshot};
pub use manifest::ProcessManifest;
pub use observations::{ObservationDataset, ObservationId, ObservationRegistry};
pub use process::{FidelityRung, ProcessId, ProcessRepresentation};
pub use protocols::{ComparisonProtocol, MetricThresholds, ProtocolRegistry, TemporalAggregation};
pub use regime::{Regime, RegimeTag};
pub use skills::SkillRecord;
pub use spatial::SpatialDomain;
pub use topology::{ConservationError, PatchMosaic, RemapInfoLoss, RemapOperator, RemapWeight};
pub use variables::{BoundsResult, VariableCategory, VariableDescriptor, VariableRegistry};
