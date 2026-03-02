//! MAESMA Runtime — task scheduler, event-driven embedding, and health monitoring.
//!
//! The runtime executes compiled SAPG schedules, manages state arrays,
//! handles event-driven process triggers (e.g., fire ignition), and
//! provides real-time health monitoring via the runtime sentinel.

pub mod events;
pub mod health;
pub mod heartbeat;
pub mod scheduler;
pub mod state;

pub use events::{Event, EventBus};
pub use health::HealthMonitor;
pub use heartbeat::{HeartbeatDaemon, HeartbeatOutcome};
pub use scheduler::Scheduler;
pub use state::SimulationState;
