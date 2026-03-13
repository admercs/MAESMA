//! Scale-Aware Process Graph (SAPG) — the compositional heart of MAESMA.

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::families::ProcessFamily;
use crate::manifest::CouplingTier;
use crate::process::{FidelityRung, ProcessId};

// ── Serializable SAPG snapshot ───────────────────────────────────────

/// A fully serializable snapshot of the SAPG (nodes + edges).
/// Used for JSON/YAML export/import; the live `Sapg` struct uses
/// petgraph internals that cannot derive Serialize directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapgSnapshot {
    pub nodes: Vec<ProcessNode>,
    pub edges: Vec<SapgEdgeRecord>,
}

/// One directed edge in the serialized snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SapgEdgeRecord {
    pub from: ProcessId,
    pub to: ProcessId,
    #[serde(flatten)]
    pub edge: CouplingEdge,
}

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

// ── Serialization helpers ────────────────────────────────────────────

impl Sapg {
    /// Create a serializable snapshot capturing every node and edge.
    pub fn snapshot(&self) -> SapgSnapshot {
        let nodes: Vec<ProcessNode> = self.graph.node_weights().cloned().collect();
        let edges: Vec<SapgEdgeRecord> = self
            .graph
            .edge_indices()
            .filter_map(|ei| {
                let (a, b) = self.graph.edge_endpoints(ei)?;
                let edge = self.graph[ei].clone();
                Some(SapgEdgeRecord {
                    from: self.graph[a].process_id.clone(),
                    to: self.graph[b].process_id.clone(),
                    edge,
                })
            })
            .collect();
        SapgSnapshot { nodes, edges }
    }

    /// Reconstruct a live `Sapg` from a snapshot.
    pub fn from_snapshot(snap: &SapgSnapshot) -> crate::Result<Self> {
        let mut sapg = Self::new();
        for node in &snap.nodes {
            sapg.add_process(node.clone());
        }
        for rec in &snap.edges {
            sapg.add_coupling(rec.from.clone(), rec.to.clone(), rec.edge.clone())?;
        }
        Ok(sapg)
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> crate::Result<String> {
        serde_json::to_string_pretty(&self.snapshot())
            .map_err(|e| crate::Error::Serialization(e.to_string()))
    }

    /// Deserialize from JSON string.
    pub fn from_json(json: &str) -> crate::Result<Self> {
        let snap: SapgSnapshot =
            serde_json::from_str(json).map_err(|e| crate::Error::Serialization(e.to_string()))?;
        Self::from_snapshot(&snap)
    }

    /// Serialize to YAML string.
    pub fn to_yaml(&self) -> crate::Result<String> {
        serde_yaml::to_string(&self.snapshot())
            .map_err(|e| crate::Error::Serialization(e.to_string()))
    }

    /// Deserialize from YAML string.
    pub fn from_yaml(yaml: &str) -> crate::Result<Self> {
        let snap: SapgSnapshot =
            serde_yaml::from_str(yaml).map_err(|e| crate::Error::Serialization(e.to_string()))?;
        Self::from_snapshot(&snap)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::families::ProcessFamily;
    use crate::manifest::CouplingTier;
    use crate::process::{FidelityRung, ProcessId};

    fn make_test_sapg() -> Sapg {
        let mut sapg = Sapg::new();
        let id_a = ProcessId::new();
        let id_b = ProcessId::new();

        sapg.add_process(ProcessNode {
            process_id: id_a.clone(),
            name: "Hydro".into(),
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R0,
            tier: CouplingTier::Slow,
            cost: 1.0,
        });
        sapg.add_process(ProcessNode {
            process_id: id_b.clone(),
            name: "Fire".into(),
            family: ProcessFamily::Fire,
            rung: FidelityRung::R1,
            tier: CouplingTier::Fast,
            cost: 5.0,
        });
        sapg.add_coupling(
            id_a,
            id_b,
            CouplingEdge {
                variables: vec!["soil_moisture".into()],
                strength: CouplingStrength::Moderate,
                mode: CouplingMode::Asynchronous,
            },
        )
        .unwrap();
        sapg
    }

    #[test]
    fn snapshot_round_trip() {
        let sapg = make_test_sapg();
        let snap = sapg.snapshot();
        assert_eq!(snap.nodes.len(), 2);
        assert_eq!(snap.edges.len(), 1);

        let restored = Sapg::from_snapshot(&snap).unwrap();
        assert_eq!(restored.node_count(), 2);
        assert_eq!(restored.edge_count(), 1);
    }

    #[test]
    fn json_round_trip() {
        let sapg = make_test_sapg();
        let json = sapg.to_json().unwrap();
        assert!(json.contains("Hydro"));
        assert!(json.contains("soil_moisture"));

        let restored = Sapg::from_json(&json).unwrap();
        assert_eq!(restored.node_count(), 2);
        assert_eq!(restored.edge_count(), 1);
    }

    #[test]
    fn yaml_round_trip() {
        let sapg = make_test_sapg();
        let yaml = sapg.to_yaml().unwrap();
        assert!(yaml.contains("Hydro"));

        let restored = Sapg::from_yaml(&yaml).unwrap();
        assert_eq!(restored.node_count(), 2);
        assert_eq!(restored.edge_count(), 1);
    }

    #[test]
    fn empty_sapg_serialization() {
        let sapg = Sapg::new();
        let json = sapg.to_json().unwrap();
        let restored = Sapg::from_json(&json).unwrap();
        assert_eq!(restored.node_count(), 0);
        assert_eq!(restored.edge_count(), 0);
    }
}
