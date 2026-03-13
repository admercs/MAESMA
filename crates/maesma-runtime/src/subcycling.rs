//! Async subcycling and device assignment for the MAESMA runtime.
//!
//! Provides per-process independent subcycling within a stage, allowing
//! fast processes (e.g., fire, overland flow) to take many small steps
//! while slow processes (e.g., ecology, BGC) take one large step.
//! Also manages device (CPU/GPU) pinning for process runners.

use std::collections::HashMap;

use maesma_core::process::ProcessId;
use serde::{Deserialize, Serialize};

/// Device assignment for a process runner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceAssignment {
    /// Run on a specific CPU core range.
    Cpu { affinity: Option<u32> },
    /// Run on a specific GPU.
    Gpu { device_id: u32 },
}

impl Default for DeviceAssignment {
    fn default() -> Self {
        DeviceAssignment::Cpu { affinity: None }
    }
}

/// Subcycle configuration for a single process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubcycleConfig {
    /// Process identifier.
    pub process_id: ProcessId,
    /// Internal timestep for this process (seconds).
    pub dt_internal: f64,
    /// Number of internal steps per stage step.
    pub n_substeps: usize,
    /// Device assignment.
    pub device: DeviceAssignment,
    /// Whether this process can run concurrently with others in the same stage.
    pub concurrent: bool,
}

/// A subcycling plan for an entire stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubcyclePlan {
    /// Stage name.
    pub stage_name: String,
    /// Stage-level timestep (seconds).
    pub dt_stage: f64,
    /// Per-process subcycle configurations.
    pub configs: Vec<SubcycleConfig>,
    /// Concurrency groups — processes in the same group can run in parallel.
    pub concurrency_groups: Vec<Vec<ProcessId>>,
}

/// Device inventory for the runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInventory {
    /// Number of CPU cores available.
    pub n_cpus: u32,
    /// GPU devices available (device_id → memory in MiB).
    pub gpus: HashMap<u32, u64>,
}

impl DeviceInventory {
    /// Detect available devices (stub: returns 8 CPUs, no GPUs).
    pub fn detect() -> Self {
        Self {
            n_cpus: 8,
            gpus: HashMap::new(),
        }
    }

    /// Create an inventory with explicit GPU configuration.
    pub fn with_gpus(n_cpus: u32, gpus: Vec<(u32, u64)>) -> Self {
        Self {
            n_cpus,
            gpus: gpus.into_iter().collect(),
        }
    }

    /// Whether any GPU is available.
    pub fn has_gpu(&self) -> bool {
        !self.gpus.is_empty()
    }

    /// Total GPU memory across all devices.
    pub fn total_gpu_memory(&self) -> u64 {
        self.gpus.values().sum()
    }
}

/// Build a subcycle plan from stage metadata and process manifests.
///
/// `dt_stage` is the stage-level timestep.
/// `process_dts` maps process_id → desired internal dt (from discretization plan).
/// `gpu_capable` maps process_id → whether the process supports GPU.
/// `memory_per_cell` maps process_id → memory per cell (bytes).
pub fn build_subcycle_plan(
    stage_name: &str,
    dt_stage: f64,
    process_dts: &HashMap<ProcessId, f64>,
    gpu_capable: &HashMap<ProcessId, bool>,
    memory_per_cell: &HashMap<ProcessId, u64>,
    n_cells: usize,
    inventory: &DeviceInventory,
) -> SubcyclePlan {
    let mut configs = Vec::new();
    let mut gpu_assignments: HashMap<u32, u64> = HashMap::new();

    for (pid, &dt_desired) in process_dts {
        let n_substeps = (dt_stage / dt_desired).ceil() as usize;
        let dt_internal = dt_stage / n_substeps as f64;

        // Assign GPU if capable and memory fits.
        let device = if gpu_capable.get(pid).copied().unwrap_or(false) && inventory.has_gpu() {
            let mem_needed = memory_per_cell.get(pid).copied().unwrap_or(0) * n_cells as u64;
            // Find a GPU with enough free memory.
            let mut assigned = None;
            for (&dev_id, &total_mem) in &inventory.gpus {
                let used = gpu_assignments.get(&dev_id).copied().unwrap_or(0);
                if used + mem_needed <= total_mem {
                    *gpu_assignments.entry(dev_id).or_default() += mem_needed;
                    assigned = Some(DeviceAssignment::Gpu { device_id: dev_id });
                    break;
                }
            }
            assigned.unwrap_or_default()
        } else {
            DeviceAssignment::Cpu { affinity: None }
        };

        configs.push(SubcycleConfig {
            process_id: pid.clone(),
            dt_internal,
            n_substeps,
            device,
            concurrent: true, // Default: all processes can run concurrently.
        });
    }

    // Build concurrency groups: GPU processes in one group, CPU in another.
    let mut gpu_group = Vec::new();
    let mut cpu_group = Vec::new();
    for cfg in &configs {
        match cfg.device {
            DeviceAssignment::Gpu { .. } => gpu_group.push(cfg.process_id.clone()),
            DeviceAssignment::Cpu { .. } => cpu_group.push(cfg.process_id.clone()),
        }
    }
    let mut groups = Vec::new();
    if !gpu_group.is_empty() {
        groups.push(gpu_group);
    }
    if !cpu_group.is_empty() {
        groups.push(cpu_group);
    }

    SubcyclePlan {
        stage_name: stage_name.to_string(),
        dt_stage,
        configs,
        concurrency_groups: groups,
    }
}

/// Execution statistics from a subcycled stage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubcycleStats {
    /// Total wall-clock time for the stage (microseconds).
    pub wall_time_us: u64,
    /// Per-process step counts.
    pub step_counts: HashMap<String, usize>,
    /// Per-process cumulative wall time (microseconds).
    pub process_times: HashMap<String, u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(_name: &str) -> ProcessId {
        ProcessId::new()
    }

    #[test]
    fn subcycle_plan_computes_substeps() {
        let fire_id = pid("fire");
        let eco_id = pid("ecology");

        let mut process_dts = HashMap::new();
        process_dts.insert(fire_id.clone(), 60.0); // 1-minute fire steps
        process_dts.insert(eco_id.clone(), 86400.0); // daily ecology

        let gpu_cap: HashMap<ProcessId, bool> = HashMap::new();
        let mem: HashMap<ProcessId, u64> = HashMap::new();
        let inventory = DeviceInventory::detect();

        let plan = build_subcycle_plan(
            "mixed_physics",
            86400.0,
            &process_dts,
            &gpu_cap,
            &mem,
            1000,
            &inventory,
        );

        assert_eq!(plan.stage_name, "mixed_physics");
        let fire_cfg = plan
            .configs
            .iter()
            .find(|c| c.process_id == fire_id)
            .unwrap();
        assert_eq!(fire_cfg.n_substeps, 1440); // 86400/60
        assert!((fire_cfg.dt_internal - 60.0).abs() < 1e-6);

        let eco_cfg = plan
            .configs
            .iter()
            .find(|c| c.process_id == eco_id)
            .unwrap();
        assert_eq!(eco_cfg.n_substeps, 1);
    }

    #[test]
    fn gpu_assignment_respects_memory() {
        let mut process_dts = HashMap::new();
        let p1 = pid("gpu_proc");
        let p2 = pid("cpu_proc");
        process_dts.insert(p1.clone(), 60.0);
        process_dts.insert(p2.clone(), 60.0);

        let mut gpu_cap = HashMap::new();
        gpu_cap.insert(p1.clone(), true);
        gpu_cap.insert(p2.clone(), false);

        let mut mem = HashMap::new();
        mem.insert(p1.clone(), 100); // 100 bytes/cell
        mem.insert(p2.clone(), 100);

        let inventory = DeviceInventory::with_gpus(8, vec![(0, 1_000_000)]); // 1MB GPU

        let plan = build_subcycle_plan(
            "test",
            3600.0,
            &process_dts,
            &gpu_cap,
            &mem,
            1000,
            &inventory,
        );

        let gpu_cfg = plan.configs.iter().find(|c| c.process_id == p1).unwrap();
        assert!(matches!(
            gpu_cfg.device,
            DeviceAssignment::Gpu { device_id: 0 }
        ));

        let cpu_cfg = plan.configs.iter().find(|c| c.process_id == p2).unwrap();
        assert!(matches!(cpu_cfg.device, DeviceAssignment::Cpu { .. }));

        // Should have 2 concurrency groups.
        assert_eq!(plan.concurrency_groups.len(), 2);
    }

    #[test]
    fn gpu_overflow_falls_back_to_cpu() {
        let p = pid("big_proc");
        let mut process_dts = HashMap::new();
        process_dts.insert(p.clone(), 60.0);

        let mut gpu_cap = HashMap::new();
        gpu_cap.insert(p.clone(), true);

        let mut mem = HashMap::new();
        mem.insert(p.clone(), 1_000_000); // 1MB/cell

        // GPU only has 1KB.
        let inventory = DeviceInventory::with_gpus(8, vec![(0, 1024)]);

        let plan = build_subcycle_plan(
            "test",
            3600.0,
            &process_dts,
            &gpu_cap,
            &mem,
            1000,
            &inventory,
        );

        let cfg = plan.configs.iter().find(|c| c.process_id == p).unwrap();
        assert!(matches!(cfg.device, DeviceAssignment::Cpu { .. }));
    }

    #[test]
    fn device_inventory_detection() {
        let inv = DeviceInventory::detect();
        assert_eq!(inv.n_cpus, 8);
        assert!(!inv.has_gpu());

        let inv2 = DeviceInventory::with_gpus(16, vec![(0, 8192), (1, 8192)]);
        assert!(inv2.has_gpu());
        assert_eq!(inv2.total_gpu_memory(), 16384);
    }
}
