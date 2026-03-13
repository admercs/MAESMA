//! Assembly agent — builds SAPG configurations from process manifests.

use async_trait::async_trait;
use maesma_core::families::ProcessFamily;
use maesma_core::graph::{CouplingEdge, CouplingMode, CouplingStrength, ProcessNode, Sapg};
use maesma_core::manifest::CouplingTier;
use maesma_core::process::{FidelityRung, ProcessId};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

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
        "Composes candidate SAPG assemblies from retrieved process representations"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let procs = ctx
            .params
            .get("processes")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if procs.is_empty() {
            return Ok(AgentResult::fail("No processes provided for assembly")
                .with_next("run kb_retrieval first"));
        }

        let mut sapg = Sapg::new();
        let mut ids: Vec<(ProcessId, ProcessFamily)> = Vec::new();

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
            ids.push((pid, family));
        }

        // Auto-generate coupling edges between interacting families
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
            "Assembled SAPG: {} nodes, {} edges",
            sapg.node_count(),
            sapg.edge_count()
        ))
        .with_data(serde_json::json!({
            "node_count": sapg.node_count(),
            "edge_count": sapg.edge_count(),
            "families": summary,
        }))
        .with_next("run closure_validator"))
    }
}
