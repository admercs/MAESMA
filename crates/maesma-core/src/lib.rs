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
pub mod regime;
pub mod skills;
pub mod spatial;
pub mod units;

pub use automaton::{Constitution, HeartbeatConfig, ProcessAutomaton, ProcessSoul, SurvivalTier};
pub use error::{Error, Result};
pub use evolution::{EvolutionCandidate, EvolutionConfig, Population, ProcessLineage};
pub use families::ProcessFamily;
pub use manifest::ProcessManifest;
pub use observations::{ObservationDataset, ObservationId, ObservationRegistry};
pub use process::{FidelityRung, ProcessId, ProcessRepresentation};
pub use regime::{Regime, RegimeTag};
pub use skills::SkillRecord;
pub use spatial::SpatialDomain;
