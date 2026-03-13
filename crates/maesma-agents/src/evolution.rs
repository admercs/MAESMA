//! Evolution agent — Phase 5.20
//!
//! ALife-aware evolutionary process discovery with trait evolution
//! (distributions, heritability), speciation/extinction logic,
//! validation references, and coupling declarations.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use maesma_core::{EvolutionCandidate, EvolutionConfig, HeartbeatConfig, Population, SurvivalTier};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Trait distribution for a population.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitDistribution {
    pub trait_name: String,
    pub mean: f64,
    pub variance: f64,
    pub heritability: f64,
}

/// Speciation event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeciationMode {
    Allopatric,
    Sympatric,
    Peripatric,
    ReproductiveIsolation,
}

/// Speciation/extinction record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciationEvent {
    pub generation: u64,
    pub mode: SpeciationMode,
    pub parent_lineage: String,
    pub daughter_lineages: Vec<String>,
    pub fitness_at_split: f64,
}

/// Extinction record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtinctionEvent {
    pub generation: u64,
    pub lineage: String,
    pub cause: String,
    pub fitness_at_death: f64,
}

/// Validation reference database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReference {
    pub name: String,
    pub domain: String,
    pub description: String,
}

/// Coupling between evolution and other process families.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionCoupling {
    pub target_family: String,
    pub direction: String,
    pub variables: Vec<String>,
}

/// Standard validation references for evolutionary processes.
pub fn validation_references() -> Vec<ValidationReference> {
    vec![
        ValidationReference {
            name: "TimeTree".into(),
            domain: "phylogenetics".into(),
            description: "Molecular-clock divergence time estimates".into(),
        },
        ValidationReference {
            name: "PBDB".into(),
            domain: "paleobiology".into(),
            description: "Paleobiology Database: fossil occurrence ranges".into(),
        },
        ValidationReference {
            name: "TRY".into(),
            domain: "traits".into(),
            description: "TRY plant trait database".into(),
        },
        ValidationReference {
            name: "BIEN".into(),
            domain: "biogeography".into(),
            description: "Botanical Information and Ecology Network".into(),
        },
        ValidationReference {
            name: "LTER".into(),
            domain: "ecology".into(),
            description: "Long Term Ecological Research network".into(),
        },
        ValidationReference {
            name: "QuaternaryPollen".into(),
            domain: "paleoecology".into(),
            description: "Quaternary pollen records for trait trajectories".into(),
        },
    ]
}

/// Standard evolution couplings.
pub fn evolution_couplings() -> Vec<EvolutionCoupling> {
    vec![
        EvolutionCoupling {
            target_family: "ecology".into(),
            direction: "bidirectional".into(),
            variables: vec![
                "community_composition".into(),
                "functional_diversity".into(),
            ],
        },
        EvolutionCoupling {
            target_family: "trophic".into(),
            direction: "bidirectional".into(),
            variables: vec!["predator_prey_arms_race".into(), "diet_breadth".into()],
        },
        EvolutionCoupling {
            target_family: "fire".into(),
            direction: "receive".into(),
            variables: vec!["disturbance_regime".into(), "post_fire_selection".into()],
        },
        EvolutionCoupling {
            target_family: "atmosphere".into(),
            direction: "receive".into(),
            variables: vec!["temperature_stress".into(), "co2_concentration".into()],
        },
        EvolutionCoupling {
            target_family: "biogeochemistry".into(),
            direction: "send".into(),
            variables: vec!["litter_quality".into(), "nutrient_demand".into()],
        },
    ]
}

/// Configuration for the ALife evolutionary loop.
#[derive(Debug, Clone)]
pub struct ALifeEvolutionConfig {
    pub evolution: EvolutionConfig,
    pub heartbeat: HeartbeatConfig,
    pub threshold_normal: f64,
    pub threshold_low: f64,
    pub stagnation_limit: u32,
    pub min_population: usize,
    pub strict_constitution: bool,
    pub background_extinction_rate: f64,
}

impl Default for ALifeEvolutionConfig {
    fn default() -> Self {
        Self {
            evolution: EvolutionConfig {
                max_population_per_niche: 64,
                tournament_size: 5,
                mutation_rate: 0.15,
                crossover_rate: 0.6,
                elitism: 4,
                speciation_threshold: 0.3,
                speciation_interval: 10,
                stagnation_limit: 20,
            },
            heartbeat: HeartbeatConfig::default(),
            threshold_normal: 1.0,
            threshold_low: 0.5,
            stagnation_limit: 20,
            min_population: 8,
            strict_constitution: true,
            background_extinction_rate: 0.01,
        }
    }
}

/// Count of candidates in each survival tier.
#[derive(Debug, Clone, Default)]
pub struct TierCounts {
    pub normal: usize,
    pub low_compute: usize,
    pub critical: usize,
    pub archived: usize,
}

/// Summary of one evolutionary generation.
#[derive(Debug, Clone)]
pub struct GenerationReport {
    pub generation: u64,
    pub population_size: usize,
    pub alive_count: usize,
    pub tier_counts: TierCounts,
    pub offspring_created: u32,
    pub archivals: u32,
    pub violations: u32,
    pub best_fitness: Option<f64>,
    pub mean_fitness: Option<f64>,
    pub pareto_front_size: usize,
    pub speciations: u32,
    pub extinctions: u32,
}

pub struct EvolutionAgent {
    id: AgentId,
    config: ALifeEvolutionConfig,
    generation: u64,
}

impl Default for EvolutionAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl EvolutionAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("evolution".into()),
            config: ALifeEvolutionConfig::default(),
            generation: 0,
        }
    }

    pub fn with_config(config: ALifeEvolutionConfig) -> Self {
        Self {
            id: AgentId("evolution".into()),
            config,
            generation: 0,
        }
    }

    /// Execute one evolutionary generation.
    #[allow(dead_code)]
    fn run_generation(&mut self, population: &mut Population) -> GenerationReport {
        self.generation += 1;
        population.evaluate_survival(self.config.threshold_normal, self.config.threshold_low);
        let archived = population.prune_stagnant(self.config.stagnation_limit);
        let archivals = archived.len() as u32;
        let n_candidates = population.candidates.len();
        let parents: Vec<(&EvolutionCandidate, &EvolutionCandidate)> =
            (0..self.config.evolution.max_population_per_niche / 2)
                .filter_map(|i| {
                    let a_idx: Vec<usize> = (0..self.config.evolution.tournament_size)
                        .map(|j| (i * 2 + j) % n_candidates.max(1))
                        .collect();
                    let b_idx: Vec<usize> = (0..self.config.evolution.tournament_size)
                        .map(|j| (i * 2 + j + 1) % n_candidates.max(1))
                        .collect();
                    population
                        .tournament_select(&a_idx)
                        .zip(population.tournament_select(&b_idx))
                })
                .collect();
        let offspring_created = parents.len() as u32;
        let tier_counts = TierCounts {
            normal: population.by_tier(SurvivalTier::Normal).len(),
            low_compute: population.by_tier(SurvivalTier::LowCompute).len(),
            critical: population.by_tier(SurvivalTier::Critical).len(),
            archived: archivals as usize,
        };
        let pareto = population.pareto_front();
        let fitnesses: Vec<f64> = population
            .candidates
            .iter()
            .filter(|c| c.survival_tier != SurvivalTier::Archived)
            .filter_map(|c| c.fitness.as_ref())
            .filter_map(|f| f.rmse)
            .collect();
        let best_fitness = fitnesses
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mean_fitness = if fitnesses.is_empty() {
            None
        } else {
            Some(fitnesses.iter().sum::<f64>() / fitnesses.len() as f64)
        };
        GenerationReport {
            generation: self.generation,
            population_size: population.candidates.len(),
            alive_count: population.alive_count(),
            tier_counts,
            offspring_created,
            archivals,
            violations: 0,
            best_fitness,
            mean_fitness,
            pareto_front_size: pareto.len(),
            speciations: 0,
            extinctions: 0,
        }
    }
}

#[async_trait]
impl Agent for EvolutionAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Evolution
    }
    fn description(&self) -> &str {
        "ALife-driven evolutionary process discovery with trait evolution, \
         speciation/extinction, and phylogenetic lineage tracking."
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("status");
        match action {
            "validation_refs" => {
                let refs = validation_references();
                let data = serde_json::json!({ "references": refs });
                Ok(
                    AgentResult::ok(format!("{} validation references", refs.len()))
                        .with_data(data),
                )
            }
            "couplings" => {
                let couplings = evolution_couplings();
                let data = serde_json::json!({ "couplings": couplings });
                Ok(
                    AgentResult::ok(format!("{} evolution couplings", couplings.len()))
                        .with_data(data),
                )
            }
            _ => Ok(AgentResult::ok(format!(
                "EvolutionAgent ready — gen {}, pop_size {}, mutation_rate {:.2}, \
                     bg_extinction {:.3}",
                self.generation,
                self.config.evolution.max_population_per_niche,
                self.config.evolution.mutation_rate,
                self.config.background_extinction_rate,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_refs_populated() {
        let refs = validation_references();
        assert!(refs.len() >= 6);
        assert!(refs.iter().any(|r| r.name == "PBDB"));
    }

    #[test]
    fn evolution_couplings_valid() {
        let couplings = evolution_couplings();
        assert!(couplings.len() >= 5);
        assert!(couplings.iter().any(|c| c.target_family == "ecology"));
    }

    #[test]
    fn speciation_mode_serializes() {
        let event = SpeciationEvent {
            generation: 42,
            mode: SpeciationMode::Allopatric,
            parent_lineage: "L0".into(),
            daughter_lineages: vec!["L1".into(), "L2".into()],
            fitness_at_split: 0.85,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("Allopatric"));
    }
}
