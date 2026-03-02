//! Scale-Aware Process Graph (SAPG) — the compositional heart of MAESMA.

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::families::ProcessFamily;
use crate::manifest::CouplingTier;
use crate::process::{FidelityRung, ProcessId};

// ── Node ─────────────────────────────────────────────────────────────

/// A node in the SAPG represents an active process representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessNode {
    pub process_id: ProcessId,
    pub name: String,
    pub family: ProcessFamily,
    pub rung: FidelityRung,
    pub tier: CouplingTier,
    /// Current computational cost estimate (FLOPS/cell/step).
    pub cost: f64,
}

// ── Edge ─────────────────────────────────────────────────────────────

/// An edge in the SAPG represents a coupling between two process nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingEdge {
    /// Variables passed along this edge.
    pub variables: Vec<String>,
    /// Coupling strength category.
    pub strength: CouplingStrength,
    /// Temporal coupling mode.
    pub mode: CouplingMode,
}

/// Qualitative coupling strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouplingStrength {
    Strong,
    Moderate,
    Weak,
}

/// Temporal coupling mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouplingMode {
    /// Two-way operator splitting within a single timestep.
    Synchronous,
    /// One-way flux handoff every N steps.
    Asynchronous,
    /// Event-driven trigger.
    EventDriven,
}

// ── Graph ────────────────────────────────────────────────────────────

/// The Scale-Aware Process Graph.
pub struct Sapg {
    /// The directed graph.
    graph: DiGraph<ProcessNode, CouplingEdge>,
    /// Lookup from ProcessId to NodeIndex.
    index: HashMap<ProcessId, NodeIndex>,
}

impl Sapg {
    /// Create an empty SAPG.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index: HashMap::new(),
        }
    }

    /// Add a process node; returns the graph index.
    pub fn add_process(&mut self, node: ProcessNode) -> NodeIndex {
        let id = node.process_id.clone();
        let idx = self.graph.add_node(node);
        self.index.insert(id, idx);
        idx
    }

    /// Add a coupling edge between two process nodes.
    pub fn add_coupling(
        &mut self,
        from: ProcessId,
        to: ProcessId,
        edge: CouplingEdge,
    ) -> crate::Result<()> {
        let a = *self
            .index
            .get(&from)
            .ok_or_else(|| crate::Error::ProcessNotFound(from.to_string()))?;
        let b = *self
            .index
            .get(&to)
            .ok_or_else(|| crate::Error::ProcessNotFound(to.to_string()))?;
        self.graph.add_edge(a, b, edge);
        Ok(())
    }

    /// Return the number of process nodes.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Return the number of coupling edges.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Iterate over all process nodes.
    pub fn processes(&self) -> impl Iterator<Item = &ProcessNode> {
        self.graph.node_weights()
    }

    /// Get a process node by id.
    pub fn get_process(&self, id: ProcessId) -> Option<&ProcessNode> {
        self.index.get(&id).map(|&idx| &self.graph[idx])
    }

    /// Hot-swap: replace a process node in-place, preserving edges.
    pub fn swap_process(&mut self, old_id: ProcessId, new_node: ProcessNode) -> crate::Result<()> {
        let idx = *self
            .index
            .get(&old_id)
            .ok_or_else(|| crate::Error::ProcessNotFound(old_id.to_string()))?;
        let new_id = new_node.process_id.clone();
        self.graph[idx] = new_node;
        self.index.remove(&old_id);
        self.index.insert(new_id, idx);
        Ok(())
    }

    /// Return families and their node counts.
    pub fn family_summary(&self) -> HashMap<ProcessFamily, usize> {
        let mut counts = HashMap::new();
        for node in self.graph.node_weights() {
            *counts.entry(node.family).or_insert(0) += 1;
        }
        counts
    }

    /// Get a reference to the inner petgraph.
    pub fn inner(&self) -> &DiGraph<ProcessNode, CouplingEdge> {
        &self.graph
    }
}

impl Default for Sapg {
    fn default() -> Self {
        Self::new()
    }
}
