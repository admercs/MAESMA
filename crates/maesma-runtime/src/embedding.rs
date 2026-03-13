//! Event-driven embedding — dynamically spawns, upgrades, or downgrades
//! process solvers in response to runtime events.
//!
//! Examples:
//! - Lightning ignition → spawn F1 fire solver in a bounding box
//! - Extreme rain → upgrade H0→H1 in affected basins
//! - Drought → upgrade radiation locally
//! - Hysteresis timers prevent rapid toggling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use crate::events::{EventBus, EventKind};
use crate::state::SimulationState;
use maesma_core::families::ProcessFamily;
use maesma_core::process::FidelityRung;

/// A rule that maps an event pattern to an embedding action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRule {
    /// Human-readable name.
    pub name: String,
    /// The event kind that triggers this rule.
    pub trigger: EventKind,
    /// Which process family is affected.
    pub family: ProcessFamily,
    /// Target rung to upgrade to (or spawn).
    pub target_rung: FidelityRung,
    /// Minimum hold time (seconds) before downgrading back.
    pub hold_seconds: f64,
    /// Optional bounding-box radius (grid cells) around the event location.
    pub radius: Option<usize>,
}

/// Tracks an active embedding: a region where a higher-fidelity solver
/// has been activated in response to an event.
#[derive(Debug, Clone)]
pub struct ActiveEmbedding {
    pub rule_name: String,
    pub family: ProcessFamily,
    pub rung: FidelityRung,
    pub center: (usize, usize),
    pub radius: usize,
    pub activated_at: f64,
    pub hold_until: f64,
}

/// The embedding engine manages event-driven process upgrades.
pub struct EmbeddingEngine {
    rules: Vec<EmbeddingRule>,
    active: Vec<ActiveEmbedding>,
    /// Tracks how many times each rule has fired.
    fire_counts: HashMap<String, u64>,
}

impl EmbeddingEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            active: Vec::new(),
            fire_counts: HashMap::new(),
        }
    }

    /// Create an engine pre-loaded with the default MAESMA embedding rules.
    pub fn with_defaults() -> Self {
        let mut engine = Self::new();

        engine.add_rule(EmbeddingRule {
            name: "fire_ignition".into(),
            trigger: EventKind::LightningIgnition,
            family: ProcessFamily::Fire,
            target_rung: FidelityRung::R1,
            hold_seconds: 86400.0, // 1 day
            radius: Some(5),
        });

        engine.add_rule(EmbeddingRule {
            name: "anthropogenic_fire".into(),
            trigger: EventKind::AnthropogenicIgnition,
            family: ProcessFamily::Fire,
            target_rung: FidelityRung::R1,
            hold_seconds: 86400.0,
            radius: Some(3),
        });

        engine.add_rule(EmbeddingRule {
            name: "extreme_rain_hydro_upgrade".into(),
            trigger: EventKind::ThresholdExceedance,
            family: ProcessFamily::Hydrology,
            target_rung: FidelityRung::R1,
            hold_seconds: 3600.0, // 1 hour
            radius: Some(10),
        });

        engine.add_rule(EmbeddingRule {
            name: "regime_shift_radiation".into(),
            trigger: EventKind::RegimeShift,
            family: ProcessFamily::Radiation,
            target_rung: FidelityRung::R1,
            hold_seconds: 7200.0, // 2 hours
            radius: None,
        });

        engine
    }

    /// Register an embedding rule.
    pub fn add_rule(&mut self, rule: EmbeddingRule) {
        self.rules.push(rule);
    }

    /// Process all pending events and activate embeddings as needed.
    /// Returns the list of newly activated embeddings.
    pub fn process_events(
        &mut self,
        event_bus: &mut EventBus,
        current_time: f64,
        state: &SimulationState,
    ) -> Vec<ActiveEmbedding> {
        let mut activated = Vec::new();
        let mut events = Vec::new();

        // Drain events
        while let Some(ev) = event_bus.poll() {
            events.push(ev);
        }

        for event in &events {
            for rule in &self.rules {
                if Self::event_matches(&event.kind, &rule.trigger) {
                    let center = event.location.unwrap_or((state.nx / 2, state.ny / 2));
                    let radius = rule.radius.unwrap_or(0);

                    let embedding = ActiveEmbedding {
                        rule_name: rule.name.clone(),
                        family: rule.family,
                        rung: rule.target_rung,
                        center,
                        radius,
                        activated_at: current_time,
                        hold_until: current_time + rule.hold_seconds,
                    };

                    info!(
                        rule = %rule.name,
                        family = ?rule.family,
                        rung = ?rule.target_rung,
                        center = ?center,
                        radius,
                        "Embedding activated"
                    );

                    *self.fire_counts.entry(rule.name.clone()).or_insert(0) += 1;
                    activated.push(embedding.clone());
                    self.active.push(embedding);
                }
            }
        }

        activated
    }

    /// Expire embeddings whose hold timer has elapsed.
    /// Returns the list of expired embeddings (for downgrade actions).
    pub fn expire(&mut self, current_time: f64) -> Vec<ActiveEmbedding> {
        let (expired, remaining): (Vec<_>, Vec<_>) = self
            .active
            .drain(..)
            .partition(|e| current_time >= e.hold_until);

        self.active = remaining;

        for e in &expired {
            info!(
                rule = %e.rule_name,
                family = ?e.family,
                "Embedding expired — graceful downshift"
            );
        }

        expired
    }

    /// Currently active embeddings.
    pub fn active_embeddings(&self) -> &[ActiveEmbedding] {
        &self.active
    }

    /// Number of currently active embeddings.
    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    /// Total fire counts per rule.
    pub fn fire_counts(&self) -> &HashMap<String, u64> {
        &self.fire_counts
    }

    /// Number of registered rules.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Check if an event kind matches a rule trigger.
    fn event_matches(event: &EventKind, trigger: &EventKind) -> bool {
        std::mem::discriminant(event) == std::mem::discriminant(trigger)
    }
}

impl Default for EmbeddingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{Event, EventBus, EventKind};
    use crate::state::SimulationState;

    #[test]
    fn default_rules_loaded() {
        let engine = EmbeddingEngine::with_defaults();
        assert!(engine.rule_count() >= 4);
        assert_eq!(engine.active_count(), 0);
    }

    #[test]
    fn ignition_triggers_embedding() {
        let mut engine = EmbeddingEngine::with_defaults();
        let mut bus = EventBus::new();
        let state = SimulationState::new(20, 20);

        bus.push(Event {
            kind: EventKind::LightningIgnition,
            time: 100.0,
            location: Some((5, 5)),
            payload: None,
        });

        let activated = engine.process_events(&mut bus, 100.0, &state);
        assert_eq!(activated.len(), 1);
        assert_eq!(activated[0].family, ProcessFamily::Fire);
        assert_eq!(activated[0].rung, FidelityRung::R1);
        assert_eq!(engine.active_count(), 1);
    }

    #[test]
    fn embedding_expires_after_hold() {
        let mut engine = EmbeddingEngine::with_defaults();
        let mut bus = EventBus::new();
        let state = SimulationState::new(10, 10);

        bus.push(Event {
            kind: EventKind::LightningIgnition,
            time: 0.0,
            location: Some((5, 5)),
            payload: None,
        });
        engine.process_events(&mut bus, 0.0, &state);
        assert_eq!(engine.active_count(), 1);

        // Before hold expires
        let expired = engine.expire(1000.0);
        assert_eq!(expired.len(), 0);
        assert_eq!(engine.active_count(), 1);

        // After hold expires (86400 seconds = 1 day)
        let expired = engine.expire(90000.0);
        assert_eq!(expired.len(), 1);
        assert_eq!(engine.active_count(), 0);
    }

    #[test]
    fn threshold_triggers_hydro_upgrade() {
        let mut engine = EmbeddingEngine::with_defaults();
        let mut bus = EventBus::new();
        let state = SimulationState::new(10, 10);

        bus.push(Event {
            kind: EventKind::ThresholdExceedance,
            time: 50.0,
            location: Some((3, 7)),
            payload: None,
        });

        let activated = engine.process_events(&mut bus, 50.0, &state);
        assert_eq!(activated.len(), 1);
        assert_eq!(activated[0].family, ProcessFamily::Hydrology);
    }

    #[test]
    fn fire_counts_tracked() {
        let mut engine = EmbeddingEngine::with_defaults();
        let mut bus = EventBus::new();
        let state = SimulationState::new(10, 10);

        for i in 0..3 {
            bus.push(Event {
                kind: EventKind::LightningIgnition,
                time: i as f64 * 100.0,
                location: Some((5, 5)),
                payload: None,
            });
        }
        engine.process_events(&mut bus, 0.0, &state);
        assert_eq!(engine.fire_counts()["fire_ignition"], 3);
    }

    #[test]
    fn no_events_no_activation() {
        let mut engine = EmbeddingEngine::with_defaults();
        let mut bus = EventBus::new();
        let state = SimulationState::new(10, 10);

        let activated = engine.process_events(&mut bus, 0.0, &state);
        assert_eq!(activated.len(), 0);
        assert_eq!(engine.active_count(), 0);
    }
}
