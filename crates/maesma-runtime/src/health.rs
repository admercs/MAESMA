//! Health monitoring — runtime sentinel for NaN/blow-up detection.

use tracing::error;

use crate::state::SimulationState;

/// Runtime health monitor.
pub struct HealthMonitor {
    /// Number of consecutive healthy steps.
    healthy_steps: u64,
    /// Total NaN events detected.
    nan_events: u64,
    /// Total blow-up events detected.
    blowup_events: u64,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            healthy_steps: 0,
            nan_events: 0,
            blowup_events: 0,
        }
    }

    /// Check simulation state for numerical issues.
    pub fn check(&mut self, state: &SimulationState) -> maesma_core::Result<()> {
        if state.has_nan() {
            self.nan_events += 1;
            self.healthy_steps = 0;
            error!(
                nan_events = self.nan_events,
                "NaN detected in simulation state"
            );
            return Err(maesma_core::Error::Runtime(
                "NaN detected in simulation state — hot-swap or rollback required".into(),
            ));
        }

        if state.has_inf() {
            self.blowup_events += 1;
            self.healthy_steps = 0;
            error!(
                blowup_events = self.blowup_events,
                "Infinite values detected (numerical blow-up)"
            );
            return Err(maesma_core::Error::Runtime(
                "Numerical blow-up detected — hot-swap or rollback required".into(),
            ));
        }

        self.healthy_steps += 1;
        Ok(())
    }

    pub fn healthy_steps(&self) -> u64 {
        self.healthy_steps
    }

    pub fn nan_events(&self) -> u64 {
        self.nan_events
    }

    pub fn blowup_events(&self) -> u64 {
        self.blowup_events
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}
