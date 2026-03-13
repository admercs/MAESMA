//! Event store and streaming — Phases 14-15
//!
//! Agent event schema, event bus, event store (append-only log),
//! retention policies, and streaming/NRT types.

use serde::{Deserialize, Serialize};

/// Agent event schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub agent_id: String,
    pub event_type: AgentEventType,
    pub timestamp: String,
    pub payload: serde_json::Value,
    pub cycle_id: Option<u64>,
    pub correlation_id: Option<String>,
}

/// Event types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentEventType {
    TaskStarted,
    TaskCompleted,
    TaskFailed,
    SkillRecorded,
    DataDiscovered,
    DataIngested,
    ProcessLearned,
    RungSwapped,
    ParetoUpdated,
    RegimeDetected,
    A2aExchange,
    ObjectiveChanged,
}

/// Retention policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub hot_days: u32,
    pub warm_days: u32,
    pub cold_archive: bool,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            hot_days: 30,
            warm_days: 365,
            cold_archive: true,
        }
    }
}

/// NRT data update event (Phase 14.3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataUpdateEvent {
    pub variable: String,
    pub tiles: Vec<String>,
    pub time_range: (String, String),
    pub quality_delta: f64,
}

/// NRT trigger type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NrtTrigger {
    ActiveFire,
    PrecipRadar,
    BurnSeverity,
    SeismicAlert,
}

/// Streaming pipeline configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub enabled: bool,
    pub triggers: Vec<NrtTrigger>,
    pub tile_first: bool,
    pub max_latency_s: u32,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            triggers: vec![NrtTrigger::ActiveFire, NrtTrigger::PrecipRadar],
            tile_first: true,
            max_latency_s: 300,
        }
    }
}

/// Runtime sentinel metrics (Phase 14.1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelMetrics {
    pub process_family: String,
    pub region: String,
    pub wall_time_s: f64,
    pub memory_mb: f64,
    pub info_loss: f64,
    pub triggered_upgrade: bool,
    pub triggered_downgrade: bool,
}

/// Hysteresis band for rung switching decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HysteresisBand {
    pub upgrade_threshold: f64,
    pub downgrade_threshold: f64,
    pub cooldown_cycles: u32,
}

impl Default for HysteresisBand {
    fn default() -> Self {
        Self {
            upgrade_threshold: 0.8,
            downgrade_threshold: 0.3,
            cooldown_cycles: 5,
        }
    }
}

/// Check if a rung upgrade should be triggered.
pub fn should_upgrade(info_loss: f64, band: &HysteresisBand) -> bool {
    info_loss > band.upgrade_threshold
}

/// Check if a rung downgrade should be triggered.
pub fn should_downgrade(info_loss: f64, band: &HysteresisBand) -> bool {
    info_loss < band.downgrade_threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_creation() {
        let event = AgentEvent {
            agent_id: "optimizer_1".into(),
            event_type: AgentEventType::ParetoUpdated,
            timestamp: "2025-01-01T00:00:00Z".into(),
            payload: serde_json::json!({"frontier_size": 5}),
            cycle_id: Some(42),
            correlation_id: Some("corr_abc".into()),
        };
        assert_eq!(event.event_type, AgentEventType::ParetoUpdated);
    }

    #[test]
    fn retention_defaults() {
        let r = RetentionPolicy::default();
        assert_eq!(r.hot_days, 30);
        assert_eq!(r.warm_days, 365);
        assert!(r.cold_archive);
    }

    #[test]
    fn hysteresis_upgrade_downgrade() {
        let band = HysteresisBand::default();
        assert!(should_upgrade(0.9, &band));
        assert!(!should_upgrade(0.5, &band));
        assert!(should_downgrade(0.2, &band));
        assert!(!should_downgrade(0.5, &band));
    }

    #[test]
    fn streaming_config_defaults() {
        let c = StreamingConfig::default();
        assert!(c.tile_first);
        assert!(!c.enabled);
    }
}
