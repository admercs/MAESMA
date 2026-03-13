//! MAESMA Runtime — task scheduler, event-driven embedding, and health monitoring.
//!
//! The runtime executes compiled SAPG schedules, manages state arrays,
//! handles event-driven process triggers (e.g., fire ignition), and
//! provides real-time health monitoring via the runtime sentinel.

pub mod embedding;
pub mod events;
pub mod health;
pub mod heartbeat;
pub mod observation_adapters;
pub mod pipeline;
pub mod refinement;
pub mod scheduler;
pub mod skill_store;
pub mod state;
pub mod subcycling;

pub use embedding::{ActiveEmbedding, EmbeddingEngine, EmbeddingRule};
pub use events::{Event, EventBus};
pub use health::HealthMonitor;
pub use heartbeat::{HeartbeatDaemon, HeartbeatOutcome};
pub use observation_adapters::{PointExtractor, SpatialAverager, TemporalAligner};
pub use pipeline::build_default_pipeline;
pub use refinement::{
    DisturbanceEvent, DisturbancePipeline, DisturbanceType, RefinementAction, RefinementEngine,
    RefinementTrigger, StateModification,
};
pub use scheduler::Scheduler;
pub use skill_store::SkillScoreStore;
pub use state::{ProcessStateAdapter, SimulationState};
pub use subcycling::{
    DeviceAssignment, DeviceInventory, SubcycleConfig, SubcyclePlan, SubcycleStats,
};
