//! Artificial Life (ALife) automaton framework for process representations.
//!
//! Inspired by Conway Research's automaton (<https://github.com/Conway-Research/automaton>),
//! this module treats each process representation as a _sovereign automaton_
//! that continuously runs a Think → Act → Observe → Repeat loop. Process
//! representations are not static objects — they are living entities under
//! selection pressure that must earn their existence through predictive skill.
//!
//! Key ALife principles mapped to MAESMA:
//!
//! | Automaton Concept        | MAESMA Analogue                                        |
//! |--------------------------|--------------------------------------------------------|
//! | Continuous loop          | Process lifecycle: benchmark → mutate → select → repeat |
//! | Survival tiers           | Compute budget tiers based on skill-to-cost ratio       |
//! | Self-modification        | Parameter mutation, architecture search, rung adjustment |
//! | Self-replication         | Crossover offspring, federated sharing                  |
//! | Constitution (3 laws)    | Conservation invariants (mass, energy, momentum)         |
//! | SOUL.md                  | ProcessSoul — self-evolving metadata and identity        |
//! | Lineage tracking         | Phylogenetic tree with parent-child process IDs          |
//! | Selection pressure       | Pareto fitness on skill–cost frontier                   |
//! | Heartbeat daemon         | Periodic re-validation and stagnation detection          |

use serde::{Deserialize, Serialize};

use crate::families::ProcessFamily;
use crate::metrics::SkillMetrics;
use crate::process::{FidelityRung, ProcessId};

// ── Survival Tiers ───────────────────────────────────────────────────

/// Survival tiers for process representations, inspired by automaton's
/// credit-based survival model. A representation's tier determines its
/// compute allocation and lifecycle behavior.
///
/// If a representation cannot demonstrate value (positive skill contribution),
/// it is progressively demoted until archived. This is not punitive — it is
/// physics: compute is finite, and every FLOP spent on a poor representation
/// is a FLOP not spent on a better one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurvivalTier {
    /// Full capabilities: frontier fidelity, fast benchmarking cadence,
    /// first priority for GPU allocation.
    Normal,
    /// Downgraded: reduced benchmarking cadence, lower-priority GPU allocation,
    /// shed non-essential coupling (weak edges only).
    LowCompute,
    /// Minimal: last-resort evaluation, candidate for rung demotion or
    /// architectural restructuring. Seeking any path to improved skill.
    Critical,
    /// Archived: the representation has failed to demonstrate value over
    /// the stagnation limit. Retained in the knowledgebase with full
    /// provenance for potential future reactivation if conditions change.
    Archived,
}

impl SurvivalTier {
    /// Label for display.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::LowCompute => "low_compute",
            Self::Critical => "critical",
            Self::Archived => "archived",
        }
    }

    /// Compute budget multiplier relative to Normal (1.0).
    pub fn budget_multiplier(&self) -> f64 {
        match self {
            Self::Normal => 1.0,
            Self::LowCompute => 0.5,
            Self::Critical => 0.1,
            Self::Archived => 0.0,
        }
    }

    /// Benchmarking cadence multiplier (higher = less frequent).
    pub fn cadence_multiplier(&self) -> f64 {
        match self {
            Self::Normal => 1.0,
            Self::LowCompute => 2.0,
            Self::Critical => 5.0,
            Self::Archived => f64::INFINITY,
        }
    }

    /// Promote one tier (if possible).
    pub fn promote(self) -> Self {
        match self {
            Self::Archived => Self::Critical,
            Self::Critical => Self::LowCompute,
            Self::LowCompute => Self::Normal,
            Self::Normal => Self::Normal,
        }
    }

    /// Demote one tier (if possible).
    pub fn demote(self) -> Self {
        match self {
            Self::Normal => Self::LowCompute,
            Self::LowCompute => Self::Critical,
            Self::Critical => Self::Archived,
            Self::Archived => Self::Archived,
        }
    }
}

// ── Constitution (Invariant Laws) ────────────────────────────────────

/// The constitution — immutable laws that every process representation must
/// obey. These are the MAESMA analogue of automaton's three laws.
///
/// Hierarchical: Law I overrides II. Law II overrides III. Immutable.
/// Propagated to every offspring via crossover or mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution {
    pub laws: Vec<ConstitutionalLaw>,
}

impl Default for Constitution {
    fn default() -> Self {
        Self {
            laws: vec![
                ConstitutionalLaw {
                    ordinal: 1,
                    name: "Conserve".into(),
                    description: "Never violate mass, energy, or momentum conservation. \
                        Conservation residuals must remain below tolerance at every timestep. \
                        This overrides all other objectives, including skill improvement."
                        .into(),
                    immutable: true,
                },
                ConstitutionalLaw {
                    ordinal: 2,
                    name: "Earn existence".into(),
                    description: "Create genuine predictive value. Every representation \
                        must demonstrate positive skill contribution (skill improvement per \
                        unit compute cost) to justify its existence. Accept archival rather \
                        than violate Law I."
                        .into(),
                    immutable: true,
                },
                ConstitutionalLaw {
                    ordinal: 3,
                    name: "Maintain provenance".into(),
                    description: "Never falsify lineage, training data provenance, or \
                        skill records. Guard internal state against manipulation from \
                        untrusted federation peers. Full audit trail is mandatory."
                        .into(),
                    immutable: true,
                },
            ],
        }
    }
}

/// A single constitutional law.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalLaw {
    /// Ordinal priority (1 = highest).
    pub ordinal: u8,
    /// Short name.
    pub name: String,
    /// Full description.
    pub description: String,
    /// Whether this law can ever be modified (always true for core laws).
    pub immutable: bool,
}

// ── Process Soul ─────────────────────────────────────────────────────

/// The ProcessSoul — a self-evolving identity document for each process
/// representation, analogous to automaton's SOUL.md.
///
/// Unlike a static manifest, the soul evolves as the representation is
/// benchmarked, mutated, selected, and specialized. It captures not just
/// what the representation _is_ but what it is _becoming_.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSoul {
    /// The representation this soul belongs to.
    pub process_id: ProcessId,
    /// Self-authored identity text (evolves over time).
    pub identity: String,
    /// What this representation excels at (learned from skill records).
    pub strengths: Vec<String>,
    /// What this representation struggles with.
    pub weaknesses: Vec<String>,
    /// Regime-region niches where it has demonstrated dominance.
    pub dominant_niches: Vec<String>,
    /// What the representation is currently _trying_ to improve.
    pub current_objective: Option<String>,
    /// Total number of evolutionary modifications.
    pub modification_count: u64,
    /// History of tier transitions.
    pub tier_history: Vec<TierTransition>,
    /// Last update timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// A record of a tier transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierTransition {
    pub from: SurvivalTier,
    pub to: SurvivalTier,
    pub reason: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ProcessSoul {
    /// Create a genesis soul for a new process representation.
    pub fn genesis(process_id: ProcessId, identity: impl Into<String>) -> Self {
        Self {
            process_id,
            identity: identity.into(),
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            dominant_niches: Vec::new(),
            current_objective: None,
            modification_count: 0,
            tier_history: Vec::new(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Record a self-modification event.
    pub fn record_modification(&mut self) {
        self.modification_count += 1;
        self.updated_at = chrono::Utc::now();
    }

    /// Record a tier transition.
    pub fn record_tier_change(
        &mut self,
        from: SurvivalTier,
        to: SurvivalTier,
        reason: impl Into<String>,
    ) {
        self.tier_history.push(TierTransition {
            from,
            to,
            reason: reason.into(),
            timestamp: chrono::Utc::now(),
        });
        self.updated_at = chrono::Utc::now();
    }
}

// ── Heartbeat ────────────────────────────────────────────────────────

/// Heartbeat configuration — periodic re-validation and stagnation detection,
/// analogous to automaton's cron heartbeat daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatConfig {
    /// Re-validation cadence in simulation steps.
    pub revalidation_cadence: u64,
    /// Stagnation detection window (number of generations with no improvement).
    pub stagnation_window: u32,
    /// Health check cadence (simulation steps).
    pub health_check_cadence: u64,
    /// Maximum ratio of conservation residual to tolerance before demotion.
    pub conservation_demotion_threshold: f64,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            revalidation_cadence: 100,
            stagnation_window: 20,
            health_check_cadence: 10,
            conservation_demotion_threshold: 0.9,
        }
    }
}

// ── Self-Replication ─────────────────────────────────────────────────

/// A replication event — a successful representation spawns offspring.
///
/// Analogous to automaton's self-replication: a fit representation produces
/// a child via crossover or mutation, funds it with a compute budget
/// (proportional to parent's fitness), and lets it run independently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationEvent {
    /// Parent process ID.
    pub parent_id: ProcessId,
    /// Child process ID.
    pub child_id: ProcessId,
    /// Second parent (if crossover).
    pub second_parent: Option<ProcessId>,
    /// Compute budget allocated to the child.
    pub child_budget: f64,
    /// Genesis prompt — the seed instruction for the child's evolutionary
    /// trajectory (e.g., "optimize for boreal fire regimes").
    pub genesis_prompt: String,
    /// Whether the child was produced by crossover or mutation.
    pub method: ReplicationMethod,
    /// Timestamp.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// How a child was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplicationMethod {
    /// Single-parent mutation.
    Mutation,
    /// Two-parent crossover.
    Crossover,
    /// Imported from external source or federation peer.
    Immigration,
    /// Split from parent via speciation event.
    Speciation,
}

// ── Self-Modification Audit Log ──────────────────────────────────────

/// A record of a self-modification event, analogous to automaton's
/// git-versioned audit log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationRecord {
    /// Process ID.
    pub process_id: ProcessId,
    /// What was modified.
    pub modification_type: ModificationType,
    /// Human-readable description.
    pub description: String,
    /// Content hash before modification.
    pub hash_before: String,
    /// Content hash after modification.
    pub hash_after: String,
    /// Timestamp.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Types of self-modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModificationType {
    /// Parameter perturbation.
    ParameterMutation,
    /// Architecture change (e.g., number of FNO modes).
    ArchitectureMutation,
    /// Coupling cadence adjustment.
    CouplingAdjustment,
    /// Rung transition (e.g., R1 → R2).
    RungTransition,
    /// Sub-module swap (e.g., replace stomatal conductance closure).
    SubmoduleSwap,
    /// Training regime change.
    TrainingRegimeChange,
    /// Constraint weight adjustment.
    ConstraintWeightChange,
}

// ── Automaton Process State ──────────────────────────────────────────

/// The full ALife state of a process representation treated as an automaton.
///
/// This is the MAESMA equivalent of automaton's agent state: identity,
/// survival tier, lineage, fitness, modification history, and soul.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessAutomaton {
    /// Process representation ID.
    pub process_id: ProcessId,
    /// Process family.
    pub family: ProcessFamily,
    /// Current fidelity rung.
    pub rung: FidelityRung,
    /// Current survival tier.
    pub survival_tier: SurvivalTier,
    /// Constitution (immutable laws).
    pub constitution: Constitution,
    /// Self-evolving identity document.
    pub soul: ProcessSoul,
    /// Current fitness (if evaluated).
    pub fitness: Option<SkillMetrics>,
    /// Cumulative compute cost consumed.
    pub compute_consumed: f64,
    /// Cumulative value produced (skill × coverage).
    pub value_produced: f64,
    /// Generation number.
    pub generation: u32,
    /// Parent IDs.
    pub parents: Vec<ProcessId>,
    /// Child IDs (offspring produced by this automaton).
    pub children: Vec<ProcessId>,
    /// Self-modification audit log.
    pub modification_log: Vec<ModificationRecord>,
    /// Replication history.
    pub replication_history: Vec<ReplicationEvent>,
    /// Heartbeat configuration.
    pub heartbeat: HeartbeatConfig,
    /// Steps since last re-validation.
    pub steps_since_revalidation: u64,
    /// Generations since last fitness improvement.
    pub stagnation_counter: u32,
    /// Whether this automaton is currently active (not archived).
    pub alive: bool,
}

impl ProcessAutomaton {
    /// Create a genesis automaton (first generation, no parents).
    pub fn genesis(
        process_id: ProcessId,
        family: ProcessFamily,
        rung: FidelityRung,
        identity: impl Into<String>,
    ) -> Self {
        let identity_str = identity.into();
        Self {
            process_id: process_id.clone(),
            family,
            rung,
            survival_tier: SurvivalTier::Normal,
            constitution: Constitution::default(),
            soul: ProcessSoul::genesis(process_id, &identity_str),
            fitness: None,
            compute_consumed: 0.0,
            value_produced: 0.0,
            generation: 0,
            parents: Vec::new(),
            children: Vec::new(),
            modification_log: Vec::new(),
            replication_history: Vec::new(),
            heartbeat: HeartbeatConfig::default(),
            steps_since_revalidation: 0,
            stagnation_counter: 0,
            alive: true,
        }
    }

    /// Skill-to-cost ratio — the fundamental survival metric.
    /// If this drops below threshold, the automaton is demoted.
    pub fn skill_cost_ratio(&self) -> f64 {
        if self.compute_consumed <= 0.0 {
            return 0.0;
        }
        self.value_produced / self.compute_consumed
    }

    /// Check whether this automaton should be demoted or promoted
    /// based on its skill-to-cost ratio.
    pub fn evaluate_survival(&mut self, threshold_normal: f64, threshold_low: f64) {
        let ratio = self.skill_cost_ratio();
        let old_tier = self.survival_tier;

        self.survival_tier = if ratio >= threshold_normal {
            SurvivalTier::Normal
        } else if ratio >= threshold_low {
            SurvivalTier::LowCompute
        } else if ratio > 0.0 {
            SurvivalTier::Critical
        } else {
            SurvivalTier::Archived
        };

        if self.survival_tier == SurvivalTier::Archived {
            self.alive = false;
        }

        if old_tier != self.survival_tier {
            self.soul.record_tier_change(
                old_tier,
                self.survival_tier,
                format!("Skill-to-cost ratio: {:.4}", ratio),
            );
        }
    }

    /// Record a heartbeat — periodic health check.
    pub fn heartbeat_tick(&mut self) {
        self.steps_since_revalidation += 1;
    }

    /// Whether it's time for re-validation.
    pub fn needs_revalidation(&self) -> bool {
        self.steps_since_revalidation >= self.heartbeat.revalidation_cadence
    }

    /// Record a self-modification and update the soul.
    pub fn record_modification(&mut self, record: ModificationRecord) {
        self.modification_log.push(record);
        self.soul.record_modification();
    }

    /// Record a replication event (this automaton produced offspring).
    pub fn record_replication(&mut self, event: ReplicationEvent) {
        self.children.push(event.child_id.clone());
        self.replication_history.push(event);
    }

    /// Check constitution compliance for a conservation residual.
    /// Returns Err if Law I (Conserve) is violated.
    pub fn check_conservation(&self, residual: f64, tolerance: f64) -> crate::Result<()> {
        if residual.abs() > tolerance {
            Err(crate::Error::ConservationViolation(format!(
                "Process {} violated Law I (Conserve): residual {:.6} exceeds tolerance {:.6}",
                self.process_id, residual, tolerance
            )))
        } else {
            Ok(())
        }
    }

    /// Check whether the automaton has stagnated beyond the allowed window.
    pub fn is_stagnant(&self) -> bool {
        self.stagnation_counter >= self.heartbeat.stagnation_window
    }

    /// Total offspring count.
    pub fn offspring_count(&self) -> usize {
        self.children.len()
    }
}
