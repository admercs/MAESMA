//! Task scheduler — executes the compiled SAPG schedule.

use maesma_compiler::schedule::ExecutionSchedule;
use tracing::info;

use crate::events::EventBus;
use crate::health::HealthMonitor;
use crate::state::SimulationState;

/// The simulation scheduler.
pub struct Scheduler {
    schedule: ExecutionSchedule,
    current_step: u64,
    current_time: f64,
    health: HealthMonitor,
}

impl Scheduler {
    pub fn new(schedule: ExecutionSchedule) -> Self {
        Self {
            schedule,
            current_step: 0,
            current_time: 0.0,
            health: HealthMonitor::new(),
        }
    }

    /// Advance the simulation by one global timestep.
    pub fn step(
        &mut self,
        state: &mut SimulationState,
        event_bus: &mut EventBus,
    ) -> maesma_core::Result<()> {
        self.current_step += 1;
        self.current_time += self.schedule.dt_global;

        info!(step = self.current_step, time = self.current_time, "Global step");

        for stage in &self.schedule.stages {
            for _sub in 0..stage.sub_steps {
                for _pid in &stage.processes {
                    // TODO: dispatch to actual ProcessRunner implementations
                    // For now, this is a scheduling skeleton.
                }
            }
        }

        // Process any pending events
        while let Some(event) = event_bus.poll() {
            info!(event = ?event.kind, "Processing event");
            // TODO: dispatch event to relevant process runners
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
}
