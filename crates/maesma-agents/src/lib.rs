//! Agent framework — the 25-agent swarm that autonomously evolves SAPG assemblies.
//!
//! Each agent implements the [`Agent`] trait and performs a specific role in
//! the knowledgebase-centric workflow: retrieval, assembly, validation,
//! benchmarking, optimization, discovery, evolution, and federation.

pub mod registry;
pub mod traits;

// Agent implementations — one module per agent role.
pub mod a2a_gateway;
pub mod assembly;
pub mod autonomous_observation;
pub mod benchmarking;
pub mod closure;
pub mod data_scout;
pub mod diagnostics;
pub mod discovery;
pub mod ensemble;
pub mod evolution;
pub mod foundation_model;
pub mod geoengineering;
pub mod hypothesis;
pub mod kb_retrieval;
pub mod meta_learner;
pub mod optimizer;
pub mod planetary_defense;
pub mod provenance;
pub mod regime_detector;
pub mod runtime_sentinel;
pub mod salient_dynamics;
pub mod scale_negotiator;
pub mod selection;
pub mod sensitivity;
pub mod trophic;

// New agents — Phase 5 gap fills.
pub mod active_learning;
pub mod intent_scope;
pub mod msd_coupling;
pub mod scenario_discovery;
pub mod skill_librarian;

pub use registry::AgentRegistry;
pub use traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};
