//! Process evolution — evolutionary operators for process representations.
//!
//! This module implements the evolutionary improvement cycle described in the
//! paper: crossover, mutation, Pareto selection, speciation, and niche
//! partitioning over the process knowledgebase. Process representations are
//! treated as evolving entities with lineage tracking.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::automaton::SurvivalTier;
use crate::families::ProcessFamily;
use crate::metrics::{SkillMetrics, pareto_dominates};
use crate::process::{FidelityRung, ProcessId};

/// Lineage identifier for tracking evolutionary ancestry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LineageId(pub Uuid);

impl LineageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for LineageId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for LineageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The evolutionary ancestry of a process representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessLineage {
    /// This representation's lineage ID.
    pub lineage_id: LineageId,
    /// Process ID of this representation.
    pub process_id: ProcessId,
    /// Generation number (0 = imported/hand-coded, 1+ = evolved).
    pub generation: u32,
    /// Parent process IDs (empty for imported, 1 for mutation, 2 for crossover).
    pub parents: Vec<ProcessId>,
    /// Which evolutionary operator produced this representation.
    pub origin_operator: Option<EvolutionaryOperator>,
    /// Species assignment (niche).
    pub species: Option<SpeciesId>,
    /// Creation timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Species identifier for niche partitioning.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpeciesId(pub String);

impl std::fmt::Display for SpeciesId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Evolutionary operators that generate new process representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionaryOperator {
    /// Crossover: combine structural elements from two parent representations.
    /// E.g., merge the canopy module from one and the soil module from another.
    Crossover,
    /// Mutation: perturb parameters, swap sub-expressions, or modify structure.
    Mutation,
    /// Selection: Pareto-based multi-objective selection over skill metrics.
    Selection,
    /// Speciation: assign to a niche based on functional similarity.
    Speciation,
    /// Immigration: import from an external source or A2A peer.
    Immigration,
}

/// Configuration for the evolutionary improvement cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    /// Maximum population size per process family per niche.
    pub max_population_per_niche: usize,
    /// Tournament size for selection.
    pub tournament_size: usize,
    /// Mutation rate [0, 1] — probability of mutating each parameter.
    pub mutation_rate: f64,
    /// Crossover rate [0, 1] — probability of crossover vs. cloning.
    pub crossover_rate: f64,
    /// Number of generations between speciation checks.
    pub speciation_interval: u32,
    /// Fitness distance threshold for speciation (compatibility distance).
    pub speciation_threshold: f64,
    /// Elitism: number of top individuals to carry forward unchanged.
    pub elitism: usize,
    /// Maximum generations before forced archival of stagnant lineages.
    pub stagnation_limit: u32,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            max_population_per_niche: 50,
            tournament_size: 5,
            mutation_rate: 0.15,
            crossover_rate: 0.4,
            speciation_interval: 10,
            speciation_threshold: 3.0,
            elitism: 2,
            stagnation_limit: 100,
        }
    }
}

/// A candidate in the evolutionary population.
///
/// Each candidate is a living automaton under selection pressure:
/// it must earn its existence through predictive skill or face
/// demotion and eventual archival.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionCandidate {
    /// Process representation ID.
    pub process_id: ProcessId,
    /// Lineage info.
    pub lineage: ProcessLineage,
    /// Process family.
    pub family: ProcessFamily,
    /// Fidelity rung.
    pub rung: FidelityRung,
    /// Current skill metrics (if evaluated).
    pub fitness: Option<SkillMetrics>,
    /// Number of generations since last fitness improvement.
    pub stagnation_counter: u32,
    /// Survival tier (ALife) — determines compute allocation.
    pub survival_tier: SurvivalTier,
    /// Cumulative compute consumed (FLOP-seconds).
    pub compute_consumed: f64,
    /// Cumulative value produced (skill × coverage).
    pub value_produced: f64,
    /// Number of offspring produced.
    pub offspring_count: u32,
}

/// The evolutionary population for a single process family.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Population {
    pub family: Option<ProcessFamily>,
    pub candidates: Vec<EvolutionCandidate>,
    pub generation: u32,
    pub species_map: std::collections::HashMap<SpeciesId, Vec<ProcessId>>,
}

impl Population {
    pub fn new(family: ProcessFamily) -> Self {
        Self {
            family: Some(family),
            candidates: Vec::new(),
            generation: 0,
            species_map: std::collections::HashMap::new(),
        }
    }

    /// Add a candidate to the population.
    pub fn add(&mut self, candidate: EvolutionCandidate) {
        if let Some(ref species) = candidate.lineage.species {
            self.species_map
                .entry(species.clone())
                .or_default()
                .push(candidate.process_id.clone());
        }
        self.candidates.push(candidate);
    }

    /// Population size.
    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    /// Number of distinct species.
    pub fn species_count(&self) -> usize {
        self.species_map.len()
    }

    /// Get the Pareto front — candidates not dominated by any other.
    pub fn pareto_front(&self) -> Vec<&EvolutionCandidate> {
        let evaluated: Vec<&EvolutionCandidate> = self
            .candidates
            .iter()
            .filter(|c| c.fitness.is_some())
            .collect();

        evaluated
            .iter()
            .enumerate()
            .filter(|(i, a)| {
                let fitness_a = a.fitness.as_ref().unwrap();
                !evaluated.iter().enumerate().any(|(j, b)| {
                    let fitness_b = b.fitness.as_ref().unwrap();
                    i != &j && pareto_dominates(fitness_b, fitness_a)
                })
            })
            .map(|(_, c)| *c)
            .collect()
    }

    /// Tournament selection: pick the best from a random subset.
    pub fn tournament_select(&self, rng_indices: &[usize]) -> Option<&EvolutionCandidate> {
        rng_indices
            .iter()
            .filter_map(|&i| self.candidates.get(i))
            .filter(|c| c.fitness.is_some())
            .max_by(|a, b| {
                // Use KGE as primary fitness (higher is better)
                let ka = a
                    .fitness
                    .as_ref()
                    .and_then(|f| f.kge)
                    .unwrap_or(f64::NEG_INFINITY);
                let kb = b
                    .fitness
                    .as_ref()
                    .and_then(|f| f.kge)
                    .unwrap_or(f64::NEG_INFINITY);
                ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Archive stagnant candidates (demote to Archived tier).
    pub fn prune_stagnant(&mut self, stagnation_limit: u32) -> Vec<EvolutionCandidate> {
        let (keep, mut archive): (Vec<_>, Vec<_>) = self
            .candidates
            .drain(..)
            .partition(|c| c.stagnation_counter < stagnation_limit);
        // Mark archived candidates with Archived survival tier
        for c in &mut archive {
            c.survival_tier = SurvivalTier::Archived;
        }
        self.candidates = keep;
        archive
    }

    /// Evaluate survival tiers for all candidates based on skill-to-cost ratio.
    pub fn evaluate_survival(&mut self, threshold_normal: f64, threshold_low: f64) {
        for candidate in &mut self.candidates {
            let ratio = if candidate.compute_consumed > 0.0 {
                candidate.value_produced / candidate.compute_consumed
            } else {
                0.0
            };
            candidate.survival_tier = if ratio >= threshold_normal {
                SurvivalTier::Normal
            } else if ratio >= threshold_low {
                SurvivalTier::LowCompute
            } else if ratio > 0.0 {
                SurvivalTier::Critical
            } else {
                SurvivalTier::Archived
            };
        }
    }

    /// Get candidates in a specific survival tier.
    pub fn by_tier(&self, tier: SurvivalTier) -> Vec<&EvolutionCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.survival_tier == tier)
            .collect()
    }

    /// Count of alive (non-archived) candidates.
    pub fn alive_count(&self) -> usize {
        self.candidates
            .iter()
            .filter(|c| c.survival_tier != SurvivalTier::Archived)
            .count()
    }

    /// Advance generation counter.
    pub fn next_generation(&mut self) {
        self.generation += 1;
    }
}

/// Summary statistics for an evolutionary run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionSummary {
    pub family: ProcessFamily,
    pub generation: u32,
    pub population_size: usize,
    pub species_count: usize,
    pub pareto_front_size: usize,
    pub best_kge: Option<f64>,
    pub best_rmse: Option<f64>,
    pub mean_fitness: Option<f64>,
    pub archived_count: usize,
}

impl Population {
    /// Compute summary statistics.
    pub fn summary(&self) -> EvolutionSummary {
        let front = self.pareto_front();
        let best_kge = self
            .candidates
            .iter()
            .filter_map(|c| c.fitness.as_ref()?.kge)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let best_rmse = self
            .candidates
            .iter()
            .filter_map(|c| c.fitness.as_ref()?.rmse)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mean_fitness = {
            let kges: Vec<f64> = self
                .candidates
                .iter()
                .filter_map(|c| c.fitness.as_ref()?.kge)
                .collect();
            if kges.is_empty() {
                None
            } else {
                Some(kges.iter().sum::<f64>() / kges.len() as f64)
            }
        };

        EvolutionSummary {
            family: self.family.unwrap_or(ProcessFamily::Ecology),
            generation: self.generation,
            population_size: self.len(),
            species_count: self.species_count(),
            pareto_front_size: front.len(),
            best_kge,
            best_rmse,
            mean_fitness,
            archived_count: 0,
        }
    }
}
