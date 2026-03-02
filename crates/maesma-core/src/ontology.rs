//! Ontology — domain-specific relations between process representations.

use serde::{Deserialize, Serialize};

use crate::manifest::RelationType;
use crate::process::ProcessId;

/// An ontology relation between two processes in the knowledgebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub source: ProcessId,
    pub relation: RelationType,
    pub target: ProcessId,
    pub justification: Option<String>,
}

/// Collection of ontology relations for reasoning about compatibility.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OntologyGraph {
    relations: Vec<Relation>,
}

impl OntologyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, relation: Relation) {
        self.relations.push(relation);
    }

    /// Find all relations where `id` is the source.
    pub fn relations_from(&self, id: ProcessId) -> Vec<&Relation> {
        self.relations.iter().filter(|r| r.source == id).collect()
    }

    /// Find all relations where `id` is the target.
    pub fn relations_to(&self, id: ProcessId) -> Vec<&Relation> {
        self.relations.iter().filter(|r| r.target == id).collect()
    }

    /// Check whether two processes are explicitly marked incompatible.
    pub fn are_incompatible(&self, a: ProcessId, b: ProcessId) -> bool {
        self.relations.iter().any(|r| {
            matches!(r.relation, RelationType::IncompatibleWith)
                && ((r.source == a && r.target == b) || (r.source == b && r.target == a))
        })
    }

    /// Check whether `a` requires coupling with `b`.
    pub fn requires_coupling(&self, a: ProcessId, b: ProcessId) -> bool {
        self.relations.iter().any(|r| {
            matches!(r.relation, RelationType::RequiresCouplingWith)
                && r.source == a
                && r.target == b
        })
    }

    /// Get all processes that the given process supersedes.
    pub fn supersedes(&self, id: ProcessId) -> Vec<ProcessId> {
        self.relations
            .iter()
            .filter(|r| r.source == id && matches!(r.relation, RelationType::Supersedes))
            .map(|r| r.target.clone())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.relations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.relations.is_empty()
    }
}
