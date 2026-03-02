//! Event system — event-driven process triggers.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// An event that can trigger process activation or reconfiguration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event kind.
    pub kind: EventKind,
    /// Simulation time at which the event was raised.
    pub time: f64,
    /// Spatial location (grid cell indices), if applicable.
    pub location: Option<(usize, usize)>,
    /// Additional payload.
    pub payload: Option<serde_json::Value>,
}

/// Known event kinds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// Lightning strike → potential fire ignition.
    LightningIgnition,
    /// Anthropogenic fire ignition.
    AnthropogenicIgnition,
    /// Regime shift detected.
    RegimeShift,
    /// Threshold exceedance (e.g., soil saturation → overland flow).
    ThresholdExceedance,
    /// Process hot-swap request.
    HotSwap,
    /// External forcing update.
    ForcingUpdate,
    /// Simulation checkpoint.
    Checkpoint,
    /// Foundation model ensemble forecast available.
    FoundationModelForecast,
    /// Observation scene classified and accepted by on-board AI.
    ObservationAccepted,
    /// Observation scene rejected by on-board AI filter.
    ObservationRejected,
    /// Active sensor tasking request generated.
    TaskingRequest,
    /// Process evolution generation completed.
    EvolutionGeneration,
    /// Process speciation event.
    Speciation,
    /// Pareto front updated.
    ParetoFrontUpdate,
    // ── ALife / automaton events ──────────────────────────────
    /// Process survival tier changed (promotion or demotion).
    SurvivalTierChange,
    /// Process self-modification recorded (parameter/architecture mutation).
    SelfModification,
    /// Process self-replication (offspring spawned).
    Replication,
    /// Constitutional invariant violated (conservation, provenance, etc.).
    ConstitutionViolation,
    /// Heartbeat daemon health check completed.
    HeartbeatCheck,
    /// Process stagnation detected (no skill improvement).
    StagnationDetected,
    /// Process archived (dead — no longer receiving compute).
    ProcessArchived,
    /// Immigration event (external process introduced to population).
    Immigration,
    /// Custom event.
    Custom(String),
}

/// A simple event bus (FIFO queue).
pub struct EventBus {
    queue: VecDeque<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Push an event onto the bus.
    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    /// Poll the next event.
    pub fn poll(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }

    /// Peek at the next event without removing it.
    pub fn peek(&self) -> Option<&Event> {
        self.queue.front()
    }

    /// Number of pending events.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
