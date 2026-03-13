//! Assembly agent — Phase 5.3
//!
//! Build candidate SAPG from neural inference engine proposals,
//! pick rungs, declare embeddings, propose coupling cadence.

use async_trait::async_trait;
use maesma_core::families::ProcessFamily;
use maesma_core::graph::{CouplingEdge, CouplingMode, CouplingStrength, ProcessNode, Sapg};
use maesma_core::manifest::CouplingTier;
use maesma_core::process::{FidelityRung, ProcessId};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Proposed embedding for a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingProposal {
    pub process_name: String,
    pub family: String,
    pub grid_type: String,
    pub resolution_km: f64,
}

/// Proposed coupling cadence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadenceProposal {
    pub source: String,
    pub target: String,
    pub cadence_seconds: f64,
    pub reason: String,
}

/// Known coupling relationships between process families.
const COUPLING_PAIRS: &[(ProcessFamily, ProcessFamily)] = &[
    (ProcessFamily::Atmosphere, ProcessFamily::Radiation),
    (ProcessFamily::Atmosphere, ProcessFamily::Hydrology),
    (ProcessFamily::Atmosphere, ProcessFamily::Ocean),
    (ProcessFamily::Radiation, ProcessFamily::Ecology),
    (ProcessFamily::Hydrology, ProcessFamily::Ecology),
    (ProcessFamily::Ecology, ProcessFamily::Biogeochemistry),
    (ProcessFamily::Biogeochemistry, ProcessFamily::Atmosphere),
    (ProcessFamily::Fire, ProcessFamily::Ecology),
    (ProcessFamily::Fire, ProcessFamily::Atmosphere),
    (ProcessFamily::Cryosphere, ProcessFamily::Hydrology),
    (ProcessFamily::Ocean, ProcessFamily::Atmosphere),
    (ProcessFamily::TrophicDynamics, ProcessFamily::Ecology),
    (ProcessFamily::HumanSystems, ProcessFamily::Ecology),
];

fn parse_family(s: &str) -> ProcessFamily {
    serde_json::from_value(serde_json::Value::String(s.to_string()))
        .unwrap_or(ProcessFamily::Ecology)
}

fn parse_rung(s: &str) -> FidelityRung {
    match s {
        "R0" => FidelityRung::R0,
        "R2" => FidelityRung::R2,
        "R3" => FidelityRung::R3,
        _ => FidelityRung::R1,
    }
}

/// Propose default embedding for a process family.
pub fn default_embedding(family: ProcessFamily) -> EmbeddingProposal {
    let (grid, res) = match family {
        ProcessFamily::Atmosphere => ("cubed_sphere", 100.0),
        ProcessFamily::Ocean => ("tripolar", 50.0),
        ProcessFamily::Hydrology => ("unstructured", 10.0),
        ProcessFamily::Ecology => ("regular_latlon", 25.0),
        ProcessFamily::Fire => ("regular_latlon", 10.0),
        ProcessFamily::Radiation => ("cubed_sphere", 100.0),
        ProcessFamily::Cryosphere => ("tripolar", 50.0),
        _ => ("regular_latlon", 50.0),
    };
    EmbeddingProposal {
        process_name: format!("{:?}", family),
        family: format!("{:?}", family),
        grid_type: grid.into(),
        resolution_km: res,
    }
}

/// Propose coupling cadence for two families.
pub fn default_cadence(source: ProcessFamily, target: ProcessFamily) -> CadenceProposal {
    let cadence = match (source, target) {
        (ProcessFamily::Atmosphere, ProcessFamily::Radiation) => 3600.0,
        (ProcessFamily::Atmosphere, ProcessFamily::Ocean) => 86400.0,
        (ProcessFamily::Hydrology, ProcessFamily::Ecology) => 86400.0,
        (ProcessFamily::Fire, ProcessFamily::Atmosphere) => 3600.0,
        _ => 86400.0,
    };
    CadenceProposal {
        source: format!("{:?}", source),
        target: format!("{:?}", target),
        cadence_seconds: cadence,
        reason: if cadence <= 3600.0 {
            "fast coupling".into()
        } else {
            "daily exchange".into()
        },
    }
}

pub struct AssemblyAgent {
    id: AgentId,
}

impl Default for AssemblyAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl AssemblyAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("assembly".into()),
        }
    }
}

#[async_trait]
impl Agent for AssemblyAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Assembly
    }
    fn description(&self) -> &str {
        "Composes candidate SAPG assemblies with embeddings and coupling cadence"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("assemble");

        match action {
            "assemble" => {
                let procs = ctx
                    .params
                    .get("processes")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                if procs.is_empty() {
                    return Ok(AgentResult::fail("No processes provided")
                        .with_next("run kb_retrieval first"));
                }
                let mut sapg = Sapg::new();
                let mut ids: Vec<(ProcessId, ProcessFamily)> = Vec::new();
                let mut embeddings = Vec::new();
                let mut cadences = Vec::new();

                for p in &procs {
                    let name = p.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let family = parse_family(
                        p.get("family")
                            .and_then(|v| v.as_str())
                            .unwrap_or("ecology"),
                    );
                    let rung = parse_rung(p.get("rung").and_then(|v| v.as_str()).unwrap_or("R1"));
                    let tier = if p.get("tier").and_then(|v| v.as_str()) == Some("fast") {
                        CouplingTier::Fast
                    } else {
                        CouplingTier::Slow
                    };
                    let pid = ProcessId::new();
                    sapg.add_process(ProcessNode {
                        process_id: pid.clone(),
                        name: name.into(),
                        family,
                        rung,
                        tier,
                        cost: p.get("cost").and_then(|v| v.as_f64()).unwrap_or(1.0),
                    });
                    embeddings.push(default_embedding(family));
                    ids.push((pid, family));
                }

                for (fa, fb) in COUPLING_PAIRS {
                    for (ida, fam_a) in &ids {
                        if fam_a != fa {
                            continue;
                        }
                        for (idb, fam_b) in &ids {
                            if fam_b != fb {
                                continue;
                            }
                            let _ = sapg.add_coupling(
                                ida.clone(),
                                idb.clone(),
                                CouplingEdge {
                                    variables: vec![format!("{:?}->{:?}", fa, fb)],
                                    strength: CouplingStrength::Moderate,
                                    mode: CouplingMode::Synchronous,
                                },
                            );
                            cadences.push(default_cadence(*fa, *fb));
                        }
                    }
                }

                let summary: serde_json::Map<String, serde_json::Value> = sapg
                    .family_summary()
                    .iter()
                    .map(|(f, c)| (f.display_name().to_string(), (*c).into()))
                    .collect();
                info!(
                    nodes = sapg.node_count(),
                    edges = sapg.edge_count(),
                    "SAPG assembled"
                );
                Ok(AgentResult::ok(format!(
                    "SAPG: {} nodes, {} edges",
                    sapg.node_count(),
                    sapg.edge_count()
                ))
                .with_data(serde_json::json!({
                    "node_count": sapg.node_count(), "edge_count": sapg.edge_count(),
                    "families": summary, "embeddings": embeddings, "cadences": cadences,
                }))
                .with_next("run closure_validator"))
            }
            "embeddings" => {
                let families = vec![
                    ProcessFamily::Atmosphere,
                    ProcessFamily::Ocean,
                    ProcessFamily::Hydrology,
                    ProcessFamily::Ecology,
                    ProcessFamily::Fire,
                    ProcessFamily::Radiation,
                ];
                let embs: Vec<_> = families.into_iter().map(default_embedding).collect();
                let data = serde_json::json!({ "embeddings": embs });
                Ok(AgentResult::ok(format!("{} default embeddings", embs.len())).with_data(data))
            }
            _ => {
                let data = serde_json::json!({ "available_actions": ["assemble", "embeddings"] });
                Ok(AgentResult::ok("Assembly status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_embedding_atmosphere() {
        let e = default_embedding(ProcessFamily::Atmosphere);
        assert_eq!(e.grid_type, "cubed_sphere");
    }

    #[test]
    fn default_cadence_fast() {
        let c = default_cadence(ProcessFamily::Atmosphere, ProcessFamily::Radiation);
        assert_eq!(c.cadence_seconds, 3600.0);
    }
}
