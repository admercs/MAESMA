//! Task scheduler — executes the compiled SAPG schedule.

use std::collections::HashMap;

use maesma_compiler::schedule::ExecutionSchedule;
use maesma_core::process::{ProcessId, ProcessRunner};
use tracing::info;

use crate::events::EventBus;
use crate::health::HealthMonitor;
use crate::state::{ProcessStateAdapter, SimulationState};

/// The simulation scheduler.
pub struct Scheduler {
    schedule: ExecutionSchedule,
    current_step: u64,
    current_time: f64,
    health: HealthMonitor,
    /// Registered process runners, keyed by ProcessId.
    runners: HashMap<ProcessId, Box<dyn ProcessRunner>>,
}

impl Scheduler {
    pub fn new(schedule: ExecutionSchedule) -> Self {
        Self {
            schedule,
            current_step: 0,
            current_time: 0.0,
            health: HealthMonitor::new(),
            runners: HashMap::new(),
        }
    }

    /// Register a process runner for a given process ID.
    pub fn register_runner(&mut self, id: ProcessId, runner: Box<dyn ProcessRunner>) {
        self.runners.insert(id, runner);
    }

    /// Advance the simulation by one global timestep.
    pub fn step(
        &mut self,
        state: &mut SimulationState,
        event_bus: &mut EventBus,
    ) -> maesma_core::Result<()> {
        self.current_step += 1;
        self.current_time += self.schedule.dt_global;
        state.advance_time(self.schedule.dt_global);

        info!(
            step = self.current_step,
            time = self.current_time,
            "Global step"
        );

        // Clone stage metadata so we can borrow runners mutably inside the loop.
        let stages: Vec<_> = self
            .schedule
            .stages
            .iter()
            .map(|s| (s.dt, s.sub_steps, s.processes.clone()))
            .collect();

        for (dt, sub_steps, pids) in &stages {
            for _sub in 0..*sub_steps {
                for pid in pids {
                    if let Some(runner) = self.runners.get_mut(pid) {
                        let mut adapter = ProcessStateAdapter::new(state);
                        runner.step(&mut adapter, *dt)?;
                        adapter.sync_back();
                    }
                }
            }
        }

        // Process any pending events
        while let Some(event) = event_bus.poll() {
            info!(event = ?event.kind, "Processing event");
        }

        // Health check
        self.health.check(state)?;

        Ok(())
    }

    /// Run the simulation for N global steps.
    pub fn run(
        &mut self,
        state: &mut SimulationState,
        event_bus: &mut EventBus,
        n_steps: u64,
    ) -> maesma_core::Result<()> {
        for _ in 0..n_steps {
            self.step(state, event_bus)?;
        }
        info!(total_steps = self.current_step, "Simulation run complete");
        Ok(())
    }

    pub fn current_step(&self) -> u64 {
        self.current_step
    }

    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    /// Number of registered runners.
    pub fn runner_count(&self) -> usize {
        self.runners.len()
    }
}
