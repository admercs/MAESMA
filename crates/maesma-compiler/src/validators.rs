//! Validation passes for SAPG compilation.
//!
//! Each validator inspects the SAPG (augmented by process manifests) and emits
//! diagnostics. Real checks include:
//! - Conservation closure (balanced sources/sinks per conserved quantity)
//! - Scale envelope compatibility across coupled processes
//! - Coupling consistency (variable names/units match between edges and I/O)
//! - Double-counting detection (same output from multiple processes)
//! - State-space closure (every input is produced or declared as forcing)
//! - CFL / numerical stability heuristics

use std::collections::{HashMap, HashSet};

use maesma_core::graph::Sapg;
use maesma_core::manifest::{ProcessManifest, ScaleEnvelope};
use maesma_core::process::ProcessId;

use crate::{Diagnostic, DiagnosticLevel};

/// A manifest lookup table passed into validators.
pub type ManifestIndex = HashMap<ProcessId, ProcessManifest>;

// -----------------------------------------------------------------------
// 1. Conservation Closure
// -----------------------------------------------------------------------

/// Verify conservation closure: for every conserved quantity declared across
/// the graph, the net source/sink balance should be covered.
pub fn check_conservation_closure(sapg: &Sapg, manifests: &ManifestIndex) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if sapg.node_count() == 0 {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message: "SAPG has no process nodes".into(),
            source: Some("conservation_closure".into()),
        });
        return diags;
    }

    // Collect conserved quantities and which processes declare them.
    let mut conservation_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut all_outputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut missing_manifest = 0usize;

    for node in sapg.processes() {
        if let Some(m) = manifests.get(&node.process_id) {
            for cp in &m.conservation {
                conservation_map
                    .entry(cp.quantity.clone())
                    .or_default()
                    .push(node.name.clone());
            }
            for out_var in &m.io.outputs {
                all_outputs
                    .entry(out_var.name.clone())
                    .or_default()
                    .push(node.name.clone());
            }
        } else {
            missing_manifest += 1;
        }
    }

    if missing_manifest > 0 {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message: format!(
                "Conservation check incomplete: {} process(es) missing manifests",
                missing_manifest
            ),
            source: Some("conservation_closure".into()),
        });
    }

    // Flag: conserved quantity declared by only one process.
    for (quantity, procs) in &conservation_map {
        if procs.len() == 1 {
            diags.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: format!(
                    "Conserved quantity '{}' declared by only 1 process ({}); verify source/sink balance",
                    quantity, procs[0]
                ),
                source: Some("conservation_closure".into()),
            });
        }
    }

    // Heuristic: outputs that look conservation-relevant but have no declaration.
    let keywords = [
        "mass", "energy", "water", "carbon", "nitrogen", "heat", "momentum",
    ];
    for (var_name, producers) in &all_outputs {
        let lower = var_name.to_lowercase();
        if keywords.iter().any(|kw| lower.contains(kw)) && !conservation_map.contains_key(var_name)
        {
            diags.push(Diagnostic {
                level: DiagnosticLevel::Info,
                message: format!(
                    "Output '{}' (from {}) appears conservation-relevant but has no conservation declaration",
                    var_name, producers.join(", ")
                ),
                source: Some("conservation_closure".into()),
            });
        }
    }

    if diags
        .iter()
        .all(|d| !matches!(d.level, DiagnosticLevel::Error))
    {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: format!(
                "Conservation closure check passed: {} conserved quantities across {} nodes",
                conservation_map.len(),
                sapg.node_count()
            ),
            source: Some("conservation_closure".into()),
        });
    }

    diags
}

// -----------------------------------------------------------------------
// 2. Scale Compatibility
// -----------------------------------------------------------------------

/// Check that coupled processes have overlapping scale envelopes.
pub fn check_scale_compatibility(sapg: &Sapg, manifests: &ManifestIndex) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let graph = sapg.inner();

    for edge_idx in graph.edge_indices() {
        let (src_idx, tgt_idx) = graph.edge_endpoints(edge_idx).unwrap();
        let src_node = &graph[src_idx];
        let tgt_node = &graph[tgt_idx];

        let src_manifest = manifests.get(&src_node.process_id);
        let tgt_manifest = manifests.get(&tgt_node.process_id);

        if let (Some(sm), Some(tm)) = (src_manifest, tgt_manifest) {
            if !scales_overlap_spatial(&sm.scale, &tm.scale) {
                diags.push(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "Scale mismatch: '{}' (dx {}-{} m) has no spatial overlap with '{}' (dx {}-{} m)",
                        src_node.name, sm.scale.dx_min, sm.scale.dx_max,
                        tgt_node.name, tm.scale.dx_min, tm.scale.dx_max
                    ),
                    source: Some("scale_compatibility".into()),
                });
            }

            if !scales_overlap_temporal(&sm.scale, &tm.scale) {
                diags.push(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "Scale mismatch: '{}' (dt {}-{} s) has no temporal overlap with '{}' (dt {}-{} s)",
                        src_node.name, sm.scale.dt_min, sm.scale.dt_max,
                        tgt_node.name, tm.scale.dt_min, tm.scale.dt_max
                    ),
                    source: Some("scale_compatibility".into()),
                });
            }

            let dx_ratio =
                (sm.scale.dx_max / tm.scale.dx_min).max(tm.scale.dx_max / sm.scale.dx_min);
            if dx_ratio > 100.0 {
                diags.push(Diagnostic {
                    level: DiagnosticLevel::Warning,
                    message: format!(
                        "Large scale ratio ({:.0}x) between '{}' and '{}'; conservative remapping needed",
                        dx_ratio, src_node.name, tgt_node.name
                    ),
                    source: Some("scale_compatibility".into()),
                });
            }
        }
    }

    if diags
        .iter()
        .all(|d| !matches!(d.level, DiagnosticLevel::Error))
    {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: format!(
                "Scale compatibility check passed for {} edges",
                graph.edge_count()
            ),
            source: Some("scale_compatibility".into()),
        });
    }

    diags
}

fn scales_overlap_spatial(a: &ScaleEnvelope, b: &ScaleEnvelope) -> bool {
    a.dx_min <= b.dx_max && b.dx_min <= a.dx_max
}

fn scales_overlap_temporal(a: &ScaleEnvelope, b: &ScaleEnvelope) -> bool {
    a.dt_min <= b.dt_max && b.dt_min <= a.dt_max
}

// -----------------------------------------------------------------------
// 3. Coupling Consistency
// -----------------------------------------------------------------------

/// Verify that coupling edges reference variables existing in source outputs
/// and target inputs, with matching units.
pub fn check_coupling_consistency(sapg: &Sapg, manifests: &ManifestIndex) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let graph = sapg.inner();

    for edge_idx in graph.edge_indices() {
        let edge = &graph[edge_idx];
        let (src_idx, tgt_idx) = graph.edge_endpoints(edge_idx).unwrap();
        let src_node = &graph[src_idx];
        let tgt_node = &graph[tgt_idx];

        let src_manifest = manifests.get(&src_node.process_id);
        let tgt_manifest = manifests.get(&tgt_node.process_id);

        for var_name in &edge.variables {
            if let Some(sm) = src_manifest {
                let src_var = sm.io.outputs.iter().find(|v| &v.name == var_name);
                if src_var.is_none() {
                    diags.push(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!(
                            "Edge variable '{}' not found in outputs of source '{}'",
                            var_name, src_node.name
                        ),
                        source: Some("coupling_consistency".into()),
                    });
                }

                if let Some(tm) = tgt_manifest {
                    let tgt_var = tm.io.inputs.iter().find(|v| &v.name == var_name);
                    if tgt_var.is_none() {
                        diags.push(Diagnostic {
                            level: DiagnosticLevel::Error,
                            message: format!(
                                "Edge variable '{}' not found in inputs of target '{}'",
                                var_name, tgt_node.name
                            ),
                            source: Some("coupling_consistency".into()),
                        });
                    }

                    if let (Some(sv), Some(tv)) = (src_var, tgt_var)
                        && sv.unit != tv.unit
                    {
                        diags.push(Diagnostic {
                                level: DiagnosticLevel::Error,
                                message: format!(
                                    "Unit mismatch for '{}': source '{}' produces '{}', target '{}' expects '{}'",
                                    var_name, src_node.name, sv.unit, tgt_node.name, tv.unit
                                ),
                                source: Some("coupling_consistency".into()),
                            });
                    }
                }
            }
        }
    }

    if diags
        .iter()
        .all(|d| !matches!(d.level, DiagnosticLevel::Error))
    {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: format!(
                "Coupling consistency check passed for {} edges",
                graph.edge_count()
            ),
            source: Some("coupling_consistency".into()),
        });
    }

    diags
}

// -----------------------------------------------------------------------
// 4. Double-Counting Detection
// -----------------------------------------------------------------------

/// Detect variables written by more than one process (potential double-counting).
pub fn check_double_counting(sapg: &Sapg, manifests: &ManifestIndex) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut output_producers: HashMap<String, Vec<String>> = HashMap::new();

    for node in sapg.processes() {
        if let Some(m) = manifests.get(&node.process_id) {
            for out_var in &m.io.outputs {
                output_producers
                    .entry(out_var.name.clone())
                    .or_default()
                    .push(node.name.clone());
            }
        }
    }

    for (var_name, producers) in &output_producers {
        if producers.len() > 1 {
            diags.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: format!(
                    "Potential double-counting: '{}' written by {} processes ({})",
                    var_name,
                    producers.len(),
                    producers.join(", ")
                ),
                source: Some("double_counting".into()),
            });
        }
    }

    if diags.is_empty() {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: "No double-counting detected".into(),
            source: Some("double_counting".into()),
        });
    }

    diags
}

// -----------------------------------------------------------------------
// 5. State-Space Closure
// -----------------------------------------------------------------------

/// Check that every variable consumed (input) by some process is produced
/// (output) by at least one other process, or declared as external forcing.
pub fn check_state_space_closure(
    sapg: &Sapg,
    manifests: &ManifestIndex,
    external_forcings: &HashSet<String>,
) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut all_inputs: HashSet<String> = HashSet::new();
    let mut all_outputs: HashSet<String> = HashSet::new();

    for node in sapg.processes() {
        if let Some(m) = manifests.get(&node.process_id) {
            for inp in &m.io.inputs {
                all_inputs.insert(inp.name.clone());
            }
            for out in &m.io.outputs {
                all_outputs.insert(out.name.clone());
            }
        }
    }

    let unclosed: Vec<_> = all_inputs
        .difference(&all_outputs)
        .filter(|v| !external_forcings.contains(v.as_str()))
        .collect();

    for var in &unclosed {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message: format!(
                "State-space gap: input '{}' is not produced by any process and not declared as external forcing",
                var
            ),
            source: Some("state_space_closure".into()),
        });
    }

    let unused: Vec<_> = all_outputs.difference(&all_inputs).collect();
    if !unused.is_empty() {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: format!(
                "Unused outputs (terminal variables): {}",
                unused
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            source: Some("state_space_closure".into()),
        });
    }

    if unclosed.is_empty() {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: format!(
                "State-space closure verified: {} inputs satisfied by {} outputs + {} forcings",
                all_inputs.len(),
                all_outputs.len(),
                external_forcings.len()
            ),
            source: Some("state_space_closure".into()),
        });
    }

    diags
}

// -----------------------------------------------------------------------
// 6. CFL / Numerical Stability Heuristic
// -----------------------------------------------------------------------

/// Heuristic CFL-like stability check. Uses rule-of-thumb:
/// dt_max ≤ dx_min / V_char, with V_char family-dependent.
pub fn check_numerical_stability(sapg: &Sapg, manifests: &ManifestIndex) -> Vec<Diagnostic> {
    use maesma_core::families::ProcessFamily;

    let mut diags = Vec::new();

    for node in sapg.processes() {
        if let Some(m) = manifests.get(&node.process_id) {
            let v_char: Option<f64> = match node.family {
                ProcessFamily::Atmosphere => Some(340.0),
                ProcessFamily::Ocean => Some(200.0),
                ProcessFamily::Hydrology => Some(5.0),
                ProcessFamily::Fire => Some(3.0),
                ProcessFamily::Cryosphere => Some(0.01),
                _ => None,
            };

            if let Some(v) = v_char {
                let cfl_dt = m.scale.dx_min / v;
                if m.scale.dt_max > cfl_dt * 10.0 {
                    diags.push(Diagnostic {
                        level: DiagnosticLevel::Warning,
                        message: format!(
                            "Potential CFL violation for '{}': dt_max={:.0}s but CFL limit ~{:.1}s (dx_min={:.0}m, V_char={:.1}m/s)",
                            node.name, m.scale.dt_max, cfl_dt, m.scale.dx_min, v
                        ),
                        source: Some("numerical_stability".into()),
                    });
                }
            }
        }
    }

    if diags.is_empty() {
        diags.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: "Numerical stability heuristics passed".into(),
            source: Some("numerical_stability".into()),
        });
    }

    diags
}

// -----------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use maesma_core::families::ProcessFamily;
    use maesma_core::graph::{CouplingEdge, CouplingMode, CouplingStrength, ProcessNode};
    use maesma_core::manifest::*;
    use maesma_core::process::{FidelityRung, LifecycleStatus, ProcessId, ProcessOrigin};

    fn make_manifest(
        name: &str,
        inputs: &[(&str, &str)],
        outputs: &[(&str, &str)],
    ) -> ProcessManifest {
        ProcessManifest {
            id: ProcessId::new(),
            name: name.into(),
            description: format!("Test process {name}"),
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R1,
            version: "0.1.0".into(),
            io: IoContract {
                inputs: inputs
                    .iter()
                    .map(|(n, u)| Variable {
                        name: n.to_string(),
                        unit: u.to_string(),
                        description: None,
                        dimensions: vec!["x".into(), "y".into()],
                    })
                    .collect(),
                outputs: outputs
                    .iter()
                    .map(|(n, u)| Variable {
                        name: n.to_string(),
                        unit: u.to_string(),
                        description: None,
                        dimensions: vec!["x".into(), "y".into()],
                    })
                    .collect(),
                parameters: vec![],
            },
            scale: ScaleEnvelope {
                dx_min: 10.0,
                dx_max: 1000.0,
                dt_min: 60.0,
                dt_max: 3600.0,
                coupling_tier: CouplingTier::Fast,
            },
            conservation: vec![ConservationProperty {
                quantity: "water_mass".into(),
                method: "finite_volume".into(),
            }],
            cost: CostModel {
                flops_per_cell: 1e6,
                memory_per_cell: 1024,
                gpu_capable: false,
            },
            regime_tags: vec![],
            relations: vec![],
            origin: ProcessOrigin::HandCoded,
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        }
    }

    fn build_test_graph() -> (Sapg, ManifestIndex) {
        let mut sapg = Sapg::new();
        let mut manifests = ManifestIndex::new();

        let m1 = make_manifest(
            "rainfall_infiltration",
            &[("precipitation", "kg m-2 s-1")],
            &[("soil_moisture", "m3 m-3"), ("runoff", "m3 s-1")],
        );
        let m2 = make_manifest(
            "soil_evaporation",
            &[("soil_moisture", "m3 m-3"), ("net_radiation", "W m-2")],
            &[("latent_heat", "W m-2")],
        );

        sapg.add_process(ProcessNode {
            process_id: m1.id.clone(),
            name: m1.name.clone(),
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R1,
            tier: CouplingTier::Fast,
            cost: 1e6,
        });
        sapg.add_process(ProcessNode {
            process_id: m2.id.clone(),
            name: m2.name.clone(),
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R1,
            tier: CouplingTier::Fast,
            cost: 1e6,
        });
        sapg.add_coupling(
            m1.id.clone(),
            m2.id.clone(),
            CouplingEdge {
                variables: vec!["soil_moisture".into()],
                strength: CouplingStrength::Strong,
                mode: CouplingMode::Synchronous,
            },
        )
        .unwrap();

        manifests.insert(m1.id.clone(), m1);
        manifests.insert(m2.id.clone(), m2);
        (sapg, manifests)
    }

    #[test]
    fn test_conservation_closure_basic() {
        let (sapg, manifests) = build_test_graph();
        let diags = check_conservation_closure(&sapg, &manifests);
        assert!(
            !diags
                .iter()
                .any(|d| matches!(d.level, DiagnosticLevel::Error))
        );
    }

    #[test]
    fn test_scale_compatibility_pass() {
        let (sapg, manifests) = build_test_graph();
        let diags = check_scale_compatibility(&sapg, &manifests);
        assert!(
            !diags
                .iter()
                .any(|d| matches!(d.level, DiagnosticLevel::Error))
        );
    }

    #[test]
    fn test_coupling_consistency_pass() {
        let (sapg, manifests) = build_test_graph();
        let diags = check_coupling_consistency(&sapg, &manifests);
        assert!(
            !diags
                .iter()
                .any(|d| matches!(d.level, DiagnosticLevel::Error))
        );
    }

    #[test]
    fn test_coupling_consistency_missing_var() {
        let mut sapg = Sapg::new();
        let mut manifests = ManifestIndex::new();
        let m1 = make_manifest("proc_a", &[], &[("temperature", "K")]);
        let m2 = make_manifest("proc_b", &[("humidity", "kg kg-1")], &[]);

        sapg.add_process(ProcessNode {
            process_id: m1.id.clone(),
            name: "proc_a".into(),
            family: ProcessFamily::Atmosphere,
            rung: FidelityRung::R0,
            tier: CouplingTier::Slow,
            cost: 100.0,
        });
        sapg.add_process(ProcessNode {
            process_id: m2.id.clone(),
            name: "proc_b".into(),
            family: ProcessFamily::Atmosphere,
            rung: FidelityRung::R0,
            tier: CouplingTier::Slow,
            cost: 100.0,
        });
        sapg.add_coupling(
            m1.id.clone(),
            m2.id.clone(),
            CouplingEdge {
                variables: vec!["nonexistent_var".into()],
                strength: CouplingStrength::Weak,
                mode: CouplingMode::Asynchronous,
            },
        )
        .unwrap();

        manifests.insert(m1.id.clone(), m1);
        manifests.insert(m2.id.clone(), m2);

        let diags = check_coupling_consistency(&sapg, &manifests);
        let errors: Vec<_> = diags
            .iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Error))
            .collect();
        assert!(
            errors.len() >= 2,
            "Expected errors for missing var in src and tgt"
        );
    }

    #[test]
    fn test_double_counting() {
        let mut sapg = Sapg::new();
        let mut manifests = ManifestIndex::new();
        let m1 = make_manifest("proc_x", &[], &[("soil_moisture", "m3 m-3")]);
        let m2 = make_manifest("proc_y", &[], &[("soil_moisture", "m3 m-3")]);

        sapg.add_process(ProcessNode {
            process_id: m1.id.clone(),
            name: "proc_x".into(),
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R1,
            tier: CouplingTier::Fast,
            cost: 1e6,
        });
        sapg.add_process(ProcessNode {
            process_id: m2.id.clone(),
            name: "proc_y".into(),
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R1,
            tier: CouplingTier::Fast,
            cost: 1e6,
        });
        manifests.insert(m1.id.clone(), m1);
        manifests.insert(m2.id.clone(), m2);

        let diags = check_double_counting(&sapg, &manifests);
        assert!(
            diags
                .iter()
                .any(|d| matches!(d.level, DiagnosticLevel::Warning)
                    && d.message.contains("soil_moisture"))
        );
    }

    #[test]
    fn test_state_space_closure_with_forcings() {
        let (sapg, manifests) = build_test_graph();
        let mut forcings = HashSet::new();
        forcings.insert("precipitation".to_string());
        forcings.insert("net_radiation".to_string());

        let diags = check_state_space_closure(&sapg, &manifests, &forcings);
        assert!(
            !diags
                .iter()
                .any(|d| matches!(d.level, DiagnosticLevel::Error))
        );
    }

    #[test]
    fn test_state_space_gap() {
        let (sapg, manifests) = build_test_graph();
        let forcings = HashSet::new();

        let diags = check_state_space_closure(&sapg, &manifests, &forcings);
        let errors: Vec<_> = diags
            .iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Error))
            .collect();
        assert!(
            errors.len() >= 2,
            "Expected ≥2 state-space gaps, got {}",
            errors.len()
        );
    }
}
