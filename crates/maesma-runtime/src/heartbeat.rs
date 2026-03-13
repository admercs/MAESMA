//! ALife heartbeat daemon — continuous survival evaluation for process automatons.
//!
//! Inspired by Conway Research's automaton: every process representation is a
//! living entity that must earn its existence. The heartbeat daemon periodically
//! re-evaluates survival tiers, enforces constitutional invariants, detects
//! stagnation, and triggers tier transitions.

use maesma_core::{HeartbeatConfig, ProcessAutomaton, SurvivalTier};
use tracing::{info, warn};

/// Outcome of a single heartbeat evaluation.
#[derive(Debug, Clone)]
pub struct HeartbeatOutcome {
    /// Process ID that was evaluated.
    pub process_id: maesma_core::ProcessId,
    /// Previous survival tier.
    pub previous_tier: SurvivalTier,
    /// New survival tier after evaluation.
    pub current_tier: SurvivalTier,
    /// Whether a tier transition occurred.
    pub tier_changed: bool,
    /// Whether any constitutional violation was detected.
    pub constitution_violated: bool,
    /// Whether the process is stagnant.
    pub is_stagnant: bool,
    /// Skill-to-cost ratio at evaluation time.
    pub skill_cost_ratio: f64,
}

/// The heartbeat daemon that monitors all process automatons.
pub struct HeartbeatDaemon {
    /// Configuration thresholds.
    config: HeartbeatConfig,
    /// Number of heartbeat cycles completed.
    cycles: u64,
    /// Total tier transitions observed.
    total_transitions: u64,
    /// Total constitutional violations observed.
    total_violations: u64,
    /// Total archival events (process deaths).
    total_archivals: u64,
}

impl HeartbeatDaemon {
    pub fn new(config: HeartbeatConfig) -> Self {
        Self {
            config,
            cycles: 0,
            total_transitions: 0,
            total_violations: 0,
            total_archivals: 0,
        }
    }

    /// Run one heartbeat cycle against a set of process automatons.
    ///
    /// This is the ALife Think → Observe loop: for every living process,
    /// we evaluate its fitness, check constitutional compliance, and
    /// determine whether it should be promoted, demoted, or archived.
    pub fn tick(&mut self, automatons: &mut [ProcessAutomaton]) -> Vec<HeartbeatOutcome> {
        self.cycles += 1;
        let mut outcomes = Vec::with_capacity(automatons.len());

        for automaton in automatons.iter_mut() {
            let previous_tier = automaton.survival_tier;

            // 1. Check conservation (constitutional law #1)
            let conservation_ok = automaton.check_conservation(0.0, 1e-6).is_ok();

            // 2. Evaluate survival tier based on skill-cost ratio
            automaton.evaluate_survival(1.0, 0.5);

            // 3. Tick the heartbeat counter
            automaton.heartbeat_tick();

            // 4. Check for stagnation
            let stagnant = automaton.is_stagnant();

            // 5. Force demotion if conservation violated
            if !conservation_ok {
                automaton.survival_tier = automaton.survival_tier.demote();
                self.total_violations += 1;
                warn!(
                    process_id = ?automaton.process_id,
                    "Constitutional violation: conservation law breach — demoting"
                );
            }

            // 6. Force demotion if stagnant
            if stagnant && automaton.survival_tier != SurvivalTier::Archived {
                automaton.survival_tier = automaton.survival_tier.demote();
                warn!(
                    process_id = ?automaton.process_id,
                    "Stagnation detected — demoting"
                );
            }

            let current_tier = automaton.survival_tier;
            let tier_changed = previous_tier != current_tier;

            if tier_changed {
                self.total_transitions += 1;
                if current_tier == SurvivalTier::Archived {
                    self.total_archivals += 1;
                    info!(
                        process_id = ?automaton.process_id,
                        "Process archived (dead) — no longer receiving compute"
                    );
                }
            }

            outcomes.push(HeartbeatOutcome {
                process_id: automaton.process_id.clone(),
                previous_tier,
                current_tier,
                tier_changed,
                constitution_violated: !conservation_ok,
                is_stagnant: stagnant,
                skill_cost_ratio: automaton.skill_cost_ratio(),
            });
        }

        if self.cycles.is_multiple_of(10) {
            info!(
                cycles = self.cycles,
                transitions = self.total_transitions,
                violations = self.total_violations,
                archivals = self.total_archivals,
                "Heartbeat daemon status"
            );
        }

        outcomes
    }

    /// Check if a specific heartbeat cycle needs full revalidation.
    pub fn needs_revalidation(&self) -> bool {
        self.cycles.is_multiple_of(self.config.revalidation_cadence)
    }

    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    pub fn total_transitions(&self) -> u64 {
        self.total_transitions
    }

    pub fn total_archivals(&self) -> u64 {
        self.total_archivals
    }

    pub fn total_violations(&self) -> u64 {
        self.total_violations
    }
}

impl Default for HeartbeatDaemon {
    fn default() -> Self {
        Self::new(HeartbeatConfig::default())
    }
}
