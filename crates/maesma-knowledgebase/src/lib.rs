//! Process Knowledgebase — the versioned, content-addressed store at the heart of MAESMA.
//!
//! The knowledgebase stores process manifests, skill records, ontology relations,
//! and provenance metadata. It is backed by SQLite for persistence and uses
//! BLAKE3 hashing for content addressing and tamper detection.

pub mod observation_seed;
pub mod ontology_seed;
pub mod query;
pub mod seed;
pub mod seed_extended;
pub mod store;

pub use observation_seed::generate_seed_observations;
pub use ontology_seed::{RelationStats, generate_seed_relations, relation_statistics};
pub use query::QueryBuilder;
pub use seed::{SourceModel, generate_seed_manifests};
pub use seed_extended::generate_extended_manifests;
pub use store::{ClosureReport, KnowledgebaseStore};
