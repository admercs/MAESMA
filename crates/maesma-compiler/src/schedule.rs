//! Execution schedule — the compiled operator-splitting plan.

use maesma_core::graph::{CouplingMode, Sapg};
use maesma_core::manifest::CouplingTier;
use maesma_core::process::ProcessId;
use serde::{Deserialize, Serialize};

/// A compiled execution schedule for the SAPG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSchedule {
    /// Ordered stages of execution within one global timestep.
    pub stages: Vec<Stage>,
    /// Global timestep in seconds.
    pub dt_global: f64,
}

/// A stage groups processes that execute together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    /// Stage name.
    pub name: String,
    /// Coupling tier.
    pub tier: CouplingTier,
    /// Sub-timestep within the global timestep.
    pub dt: f64,
    /// Number of sub-steps per global step.
    pub sub_steps: usize,
    /// Processes that execute in this stage (potentially parallel).
    pub processes: Vec<ProcessId>,
    /// Coupling mode for this stage.
    pub mode: CouplingMode,
}

/// Generate an execution schedule from a compiled SAPG.
pub fn generate_schedule(sapg: &Sapg) -> maesma_core::Result<ExecutionSchedule> {
    let mut fast_processes = Vec::new();
    let mut slow_processes = Vec::new();

    for proc in sapg.processes() {
        match proc.tier {
            CouplingTier::Fast => fast_processes.push(proc.process_id.clone()),
            CouplingTier::Slow => slow_processes.push(proc.process_id.clone()),
        }
    }

    let mut stages = Vec::new();

    // Fast-tier stage: sub-stepped within global dt
    if !fast_processes.is_empty() {
        stages.push(Stage {
            name: "fast_physics".into(),
            tier: CouplingTier::Fast,
            dt: 60.0,        // 1-minute sub-steps
            sub_steps: 1440, // one day's worth at 1-min
            processes: fast_processes,
            mode: CouplingMode::Synchronous,
        });
    }

    // Slow-tier stage: once per global dt
    if !slow_processes.is_empty() {
        stages.push(Stage {
            name: "slow_biogeochemistry".into(),
            tier: CouplingTier::Slow,
            dt: 86400.0, // daily
            sub_steps: 1,
            processes: slow_processes,
            mode: CouplingMode::Asynchronous,
        });
    }

    Ok(ExecutionSchedule {
        stages,
        dt_global: 86400.0, // 1 day
    })
}
