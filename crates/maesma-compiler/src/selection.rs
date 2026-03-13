//! Rung selection and coupling strategy — choose fidelity rungs per family
//! and determine discretization / coupling approaches.
//!
//! Given the target spatial resolution, regime context, compute budget, and
//! available representations in the knowledgebase, this module selects the
//! best-fit fidelity rung for each process family and determines coupling
//! strategies (operator splitting vs. monolithic, coupling frequency).

use std::collections::HashMap;

use maesma_core::families::ProcessFamily;
use maesma_core::manifest::{CouplingTier, ProcessManifest, ScaleEnvelope};
use maesma_core::process::FidelityRung;
use maesma_core::regime::RegimeTag;
use serde::{Deserialize, Serialize};

/// Constraints for rung selection.
#[derive(Debug, Clone)]
pub struct SelectionConstraints {
    /// Target spatial resolution (meters).
    pub target_dx: f64,
    /// Target timestep (seconds).
    pub target_dt: f64,
    /// Maximum total FLOPS budget per global timestep.
    pub flops_budget: f64,
    /// Number of grid cells.
    pub n_cells: usize,
    /// Active regime tags (e.g., drought, fire-prone).
    pub regime_tags: Vec<RegimeTag>,
    /// GPU available.
    pub gpu_available: bool,
}

/// Result of rung selection for one family.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RungSelection {
    /// The selected family.
    pub family: ProcessFamily,
    /// The chosen rung.
    pub rung: FidelityRung,
    /// The manifest that was selected.
    pub manifest_name: String,
    /// Why this rung was chosen.
    pub rationale: String,
    /// Estimated cost (FLOPS for the whole domain per timestep).
    pub estimated_cost: f64,
}

/// Coupling strategy for the assembled model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CouplingStrategy {
    /// Godunov-type operator splitting: processes execute sequentially.
    OperatorSplitting {
        /// Splitting order (first-order Lie or second-order Strang).
        order: SplittingOrder,
    },
    /// Monolithic: all processes coupled implicitly in one system.
    Monolithic,
}

/// Operator splitting order.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SplittingOrder {
    /// First-order Lie splitting: A → B → C.
    Lie,
    /// Second-order Strang splitting: A/2 → B → A/2.
    Strang,
}

/// Discretization plan for a given process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscretizationPlan {
    /// Selected timestep for this process (seconds).
    pub dt: f64,
    /// Number of sub-steps per global timestep.
    pub sub_steps: usize,
    /// Coupling frequency with other tiers (in global timesteps).
    pub coupling_frequency: usize,
    /// Device preference.
    pub device: DevicePreference,
}

/// Device preference for execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DevicePreference {
    Cpu,
    Gpu(u32),
    Any,
}

/// Full assembly plan: rung selections + coupling + discretization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyPlan {
    /// Selected rungs per family.
    pub selections: Vec<RungSelection>,
    /// Coupling strategy.
    pub coupling: CouplingStrategy,
    /// Per-family discretization plans.
    pub discretization: HashMap<String, DiscretizationPlan>,
    /// Total estimated FLOPS per global timestep.
    pub total_cost: f64,
    /// Whether the plan is within budget.
    pub within_budget: bool,
}

/// Select the best fidelity rung for each family given constraints.
///
/// Strategy: for each family, try highest rung first and fall back to
/// lower rungs if the scale envelope or budget is exceeded.
pub fn select_rungs(
    manifests: &[ProcessManifest],
    constraints: &SelectionConstraints,
) -> Vec<RungSelection> {
    // Group manifests by family.
    let mut by_family: HashMap<ProcessFamily, Vec<&ProcessManifest>> = HashMap::new();
    for m in manifests {
        by_family.entry(m.family).or_default().push(m);
    }

    let mut selections = Vec::new();
    let per_family_budget = constraints.flops_budget / by_family.len().max(1) as f64;

    for (family, candidates) in &by_family {
        // Sort by rung descending (prefer highest fidelity).
        let mut sorted: Vec<&&ProcessManifest> = candidates.iter().collect();
        sorted.sort_by(|a, b| b.rung.cmp(&a.rung));

        let mut selected = None;

        for manifest in &sorted {
            // Check scale envelope compatibility.
            if !scale_compatible(
                &manifest.scale,
                constraints.target_dx,
                constraints.target_dt,
            ) {
                continue;
            }

            // Check regime compatibility.
            if !constraints.regime_tags.is_empty()
                && !manifest.regime_tags.is_empty()
                && !manifest.regime_tags.iter().any(|t| {
                    constraints
                        .regime_tags
                        .iter()
                        .any(|r| format!("{r:?}") == *t)
                })
            {
                continue;
            }

            // Check GPU requirement.
            if !constraints.gpu_available
                && manifest.backends.iter().all(|b| {
                    matches!(
                        b,
                        maesma_core::manifest::ComputeBackend::Cuda
                            | maesma_core::manifest::ComputeBackend::Rocm
                    )
                })
            {
                continue;
            }

            // Estimate cost.
            let cost = manifest.cost.flops_per_cell * constraints.n_cells as f64;
            if cost > per_family_budget {
                continue;
            }

            selected = Some((*manifest, cost));
            break;
        }

        if let Some((manifest, cost)) = selected {
            selections.push(RungSelection {
                family: *family,
                rung: manifest.rung,
                manifest_name: manifest.name.clone(),
                rationale: format!(
                    "Highest compatible rung within budget ({:.0} FLOPS, limit {:.0})",
                    cost, per_family_budget
                ),
                estimated_cost: cost,
            });
        } else if let Some(fallback) = sorted.last() {
            // Fallback to lowest rung even if over budget.
            let cost = fallback.cost.flops_per_cell * constraints.n_cells as f64;
            selections.push(RungSelection {
                family: *family,
                rung: fallback.rung,
                manifest_name: fallback.name.clone(),
                rationale: format!(
                    "Fallback to lowest rung {:?} — no higher rung fits constraints",
                    fallback.rung
                ),
                estimated_cost: cost,
            });
        }
    }

    // Sort by family for deterministic output.
    selections.sort_by_key(|s| format!("{:?}", s.family));
    selections
}

/// Choose coupling strategy based on the process mix.
pub fn choose_coupling_strategy(selections: &[RungSelection]) -> CouplingStrategy {
    let has_fast = selections.iter().any(|s| {
        matches!(
            s.family,
            ProcessFamily::Fire | ProcessFamily::Atmosphere | ProcessFamily::Radiation
        )
    });
    let has_slow = selections.iter().any(|s| {
        matches!(
            s.family,
            ProcessFamily::Ecology | ProcessFamily::Biogeochemistry | ProcessFamily::Geology
        )
    });

    if has_fast && has_slow {
        // Mixed timescales → Strang splitting for second-order accuracy.
        CouplingStrategy::OperatorSplitting {
            order: SplittingOrder::Strang,
        }
    } else {
        // Homogeneous timescales → Lie splitting is simpler.
        CouplingStrategy::OperatorSplitting {
            order: SplittingOrder::Lie,
        }
    }
}

/// Build discretization plans for each selected process.
pub fn build_discretization(
    selections: &[RungSelection],
    manifests: &[ProcessManifest],
    constraints: &SelectionConstraints,
) -> HashMap<String, DiscretizationPlan> {
    let mut plans = HashMap::new();

    for sel in selections {
        let manifest = manifests.iter().find(|m| m.name == sel.manifest_name);
        let (dt, sub_steps, coupling_freq) = if let Some(m) = manifest {
            compute_timestep(&m.scale, constraints.target_dt)
        } else {
            (constraints.target_dt, 1, 1)
        };

        let device = if let Some(m) = manifest {
            if constraints.gpu_available && m.cost.gpu_capable {
                DevicePreference::Gpu(0)
            } else {
                DevicePreference::Cpu
            }
        } else {
            DevicePreference::Cpu
        };

        plans.insert(
            sel.manifest_name.clone(),
            DiscretizationPlan {
                dt,
                sub_steps,
                coupling_frequency: coupling_freq,
                device,
            },
        );
    }

    plans
}

/// Build a full assembly plan.
pub fn build_assembly_plan(
    manifests: &[ProcessManifest],
    constraints: &SelectionConstraints,
) -> AssemblyPlan {
    let selections = select_rungs(manifests, constraints);
    let coupling = choose_coupling_strategy(&selections);
    let discretization = build_discretization(&selections, manifests, constraints);
    let total_cost: f64 = selections.iter().map(|s| s.estimated_cost).sum();
    let within_budget = total_cost <= constraints.flops_budget;

    AssemblyPlan {
        selections,
        coupling,
        discretization,
        total_cost,
        within_budget,
    }
}

/// Check whether a scale envelope is compatible with target resolution.
fn scale_compatible(envelope: &ScaleEnvelope, dx: f64, dt: f64) -> bool {
    dx >= envelope.dx_min && dx <= envelope.dx_max && dt >= envelope.dt_min && dt <= envelope.dt_max
}

/// Compute appropriate timestep and sub-stepping for a process.
fn compute_timestep(envelope: &ScaleEnvelope, global_dt: f64) -> (f64, usize, usize) {
    let ideal_dt = envelope.dt_min.max(envelope.dt_max.min(global_dt));
    let sub_steps = (global_dt / ideal_dt).ceil() as usize;
    let actual_dt = global_dt / sub_steps as f64;

    // Coupling frequency: fast processes couple every step, slow every N.
    let coupling_freq = match envelope.coupling_tier {
        CouplingTier::Fast => 1,
        CouplingTier::Slow => 1,
    };

    (actual_dt, sub_steps, coupling_freq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use maesma_core::manifest::*;
    use maesma_core::process::{LifecycleStatus, ProcessId, ProcessOrigin};

    fn make_manifest(
        family: ProcessFamily,
        rung: FidelityRung,
        name: &str,
        dx_range: (f64, f64),
        dt_range: (f64, f64),
        flops: f64,
    ) -> ProcessManifest {
        ProcessManifest {
            id: ProcessId::new(),
            name: name.into(),
            description: String::new(),
            family,
            rung,
            version: "1.0".into(),
            io: IoContract {
                inputs: vec![Variable {
                    name: "temperature".into(),
                    unit: "K".into(),
                    description: None,
                    dimensions: vec![],
                }],
                outputs: vec![Variable {
                    name: "heat_flux".into(),
                    unit: "W/m2".into(),
                    description: None,
                    dimensions: vec![],
                }],
                parameters: vec![],
            },
            scale: ScaleEnvelope {
                dx_min: dx_range.0,
                dx_max: dx_range.1,
                dt_min: dt_range.0,
                dt_max: dt_range.1,
                coupling_tier: CouplingTier::Fast,
            },
            conservation: vec![],
            cost: CostModel {
                flops_per_cell: flops,
                memory_per_cell: 100,
                gpu_capable: false,
            },
            regime_tags: vec![],
            relations: vec![],
            origin: ProcessOrigin::HandCoded,
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        }
    }

    #[test]
    fn selects_highest_compatible_rung() {
        let manifests = vec![
            make_manifest(
                ProcessFamily::Fire,
                FidelityRung::R0,
                "F0",
                (100.0, 10000.0),
                (1.0, 86400.0),
                100.0,
            ),
            make_manifest(
                ProcessFamily::Fire,
                FidelityRung::R1,
                "F1",
                (10.0, 1000.0),
                (1.0, 600.0),
                1000.0,
            ),
            make_manifest(
                ProcessFamily::Fire,
                FidelityRung::R2,
                "F2",
                (1.0, 100.0),
                (0.1, 60.0),
                10000.0,
            ),
        ];
        let constraints = SelectionConstraints {
            target_dx: 500.0,
            target_dt: 300.0,
            flops_budget: 1e9,
            n_cells: 10000,
            regime_tags: vec![],
            gpu_available: false,
        };
        let selections = select_rungs(&manifests, &constraints);
        assert_eq!(selections.len(), 1);
        assert_eq!(selections[0].rung, FidelityRung::R1);
    }

    #[test]
    fn falls_back_to_lower_rung_on_budget() {
        let manifests = vec![
            make_manifest(
                ProcessFamily::Hydrology,
                FidelityRung::R0,
                "H0",
                (100.0, 10000.0),
                (60.0, 86400.0),
                10.0,
            ),
            make_manifest(
                ProcessFamily::Hydrology,
                FidelityRung::R1,
                "H1",
                (30.0, 1000.0),
                (5.0, 3600.0),
                1e8,
            ),
        ];
        let constraints = SelectionConstraints {
            target_dx: 500.0,
            target_dt: 3600.0,
            flops_budget: 1e6, // Very tight budget
            n_cells: 10000,
            regime_tags: vec![],
            gpu_available: false,
        };
        let selections = select_rungs(&manifests, &constraints);
        assert_eq!(selections[0].rung, FidelityRung::R0);
    }

    #[test]
    fn strang_splitting_for_mixed_timescales() {
        let selections = vec![
            RungSelection {
                family: ProcessFamily::Fire,
                rung: FidelityRung::R1,
                manifest_name: "F1".into(),
                rationale: String::new(),
                estimated_cost: 0.0,
            },
            RungSelection {
                family: ProcessFamily::Ecology,
                rung: FidelityRung::R0,
                manifest_name: "E0".into(),
                rationale: String::new(),
                estimated_cost: 0.0,
            },
        ];
        match choose_coupling_strategy(&selections) {
            CouplingStrategy::OperatorSplitting {
                order: SplittingOrder::Strang,
            } => {}
            other => panic!("Expected Strang splitting, got {:?}", other),
        }
    }

    #[test]
    fn assembly_plan_tracks_budget() {
        let manifests = vec![
            make_manifest(
                ProcessFamily::Fire,
                FidelityRung::R0,
                "F0",
                (100.0, 10000.0),
                (1.0, 86400.0),
                100.0,
            ),
            make_manifest(
                ProcessFamily::Hydrology,
                FidelityRung::R0,
                "H0",
                (100.0, 10000.0),
                (60.0, 86400.0),
                50.0,
            ),
        ];
        let constraints = SelectionConstraints {
            target_dx: 1000.0,
            target_dt: 86400.0,
            flops_budget: 1e9,
            n_cells: 1000,
            regime_tags: vec![],
            gpu_available: false,
        };
        let plan = build_assembly_plan(&manifests, &constraints);
        assert_eq!(plan.selections.len(), 2);
        assert!(plan.within_budget);
        assert!(plan.total_cost > 0.0);
    }

    #[test]
    fn discretization_computes_sub_steps() {
        let manifests = vec![make_manifest(
            ProcessFamily::Fire,
            FidelityRung::R1,
            "F1",
            (10.0, 1000.0),
            (1.0, 60.0),
            100.0,
        )];
        let constraints = SelectionConstraints {
            target_dx: 100.0,
            target_dt: 3600.0,
            flops_budget: 1e9,
            n_cells: 100,
            regime_tags: vec![],
            gpu_available: false,
        };
        let selections = select_rungs(&manifests, &constraints);
        let disc = build_discretization(&selections, &manifests, &constraints);
        let plan = disc.get("F1").unwrap();
        // 3600s global / 60s max → 60 sub-steps
        assert_eq!(plan.sub_steps, 60);
        assert!((plan.dt - 60.0).abs() < 1e-6);
    }
}
