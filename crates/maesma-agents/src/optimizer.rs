//! Optimizer agent — Phase 5.10
//!
//! Fitness function: F(r,g,l) = Σ w_m·S_m(r,g,l) - λ·C(r)
//! Per-region, per-regime loop, convergence detection, budget-aware mode,
//! Pareto frontier, provenance logging.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Fitness evaluation for a candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessEvaluation {
    pub candidate_id: String,
    pub region: String,
    pub regime: String,
    pub weighted_skill: f64,
    pub cost_penalty: f64,
    pub fitness: f64,
}

/// Convergence status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceStatus {
    pub iteration: u64,
    pub best_fitness: f64,
    pub fitness_history: Vec<f64>,
    pub converged: bool,
    pub relative_change: f64,
}

/// Budget status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetStatus {
    pub total_budget: f64,
    pub spent: f64,
    pub remaining: f64,
    pub evaluations_remaining: u64,
}

/// Compute fitness: F = Σ w_m * S_m - λ * C.
pub fn compute_fitness(skill_scores: &[f64], weights: &[f64], cost: f64, cost_penalty: f64) -> f64 {
    let weighted_skill: f64 = skill_scores.iter().zip(weights).map(|(s, w)| s * w).sum();
    weighted_skill - cost_penalty * cost
}

/// Detect convergence from fitness history.
pub fn detect_convergence(history: &[f64], tolerance: f64, window: usize) -> bool {
    if history.len() < window + 1 {
        return false;
    }
    let recent = &history[history.len() - window..];
    let best = recent.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let worst = recent.iter().copied().fold(f64::INFINITY, f64::min);
    let range = (best - worst).abs();
    range < tolerance * best.abs().max(1e-10)
}

pub struct OptimizerAgent {
    id: AgentId,
}

impl Default for OptimizerAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizerAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("optimizer".into()),
        }
    }
}

#[async_trait]
impl Agent for OptimizerAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Optimizer
    }
    fn description(&self) -> &str {
        "Autonomous fitness-driven optimization with convergence detection and budget awareness"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("perturb");

        match action {
            "perturb" => {
                let parameters = ctx
                    .params
                    .get("parameters")
                    .and_then(|v| v.as_object())
                    .cloned()
                    .unwrap_or_default();
                let step_size = ctx
                    .params
                    .get("step_size")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.1);
                if parameters.is_empty() {
                    return Ok(AgentResult::fail("No parameters provided"));
                }
                let mut suggestions = Vec::new();
                for (name, val) in &parameters {
                    let current = val.as_f64().unwrap_or(1.0);
                    let bounds = ctx
                        .params
                        .get("bounds")
                        .and_then(|b| b.get(name))
                        .and_then(|b| Some((b.get(0)?.as_f64()?, b.get(1)?.as_f64()?)));
                    let delta = current.abs() * step_size;
                    let (up, down) = if let Some((lo, hi)) = bounds {
                        ((current + delta).min(hi), (current - delta).max(lo))
                    } else {
                        (current + delta, current - delta)
                    };
                    suggestions.push(serde_json::json!({
                        "parameter": name, "current": current,
                        "perturbed_up": up, "perturbed_down": down, "step_size": delta,
                    }));
                }
                let n = suggestions.len();
                info!(parameters = n, step_size, "Perturbations generated");
                Ok(AgentResult::ok(format!("{} perturbation pairs", n))
                    .with_data(serde_json::json!({ "suggestions": suggestions, "n_parameters": n }))
                    .with_next("run benchmarking with perturbed params"))
            }
            "fitness" => {
                let skills: Vec<f64> = ctx
                    .params
                    .get("skill_scores")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                let weights: Vec<f64> = ctx
                    .params
                    .get("weights")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_else(|| vec![1.0 / skills.len().max(1) as f64; skills.len()]);
                let cost = ctx
                    .params
                    .get("cost")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let lam = ctx
                    .params
                    .get("cost_penalty")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.01);
                let f = compute_fitness(&skills, &weights, cost, lam);
                let region = ctx
                    .params
                    .get("region")
                    .and_then(|v| v.as_str())
                    .unwrap_or("global");
                let regime = ctx
                    .params
                    .get("regime")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");
                let eval = FitnessEvaluation {
                    candidate_id: ctx
                        .params
                        .get("candidate_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .into(),
                    region: region.into(),
                    regime: regime.into(),
                    weighted_skill: skills.iter().zip(&weights).map(|(s, w)| s * w).sum(),
                    cost_penalty: lam * cost,
                    fitness: f,
                };
                let data = serde_json::json!({ "evaluation": eval });
                Ok(
                    AgentResult::ok(format!("Fitness={:.4} ({}/{})", f, region, regime))
                        .with_data(data),
                )
            }
            "convergence" => {
                let history: Vec<f64> = ctx
                    .params
                    .get("history")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                let tol = ctx
                    .params
                    .get("tolerance")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.001);
                let window = ctx
                    .params
                    .get("window")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(5) as usize;
                let converged = detect_convergence(&history, tol, window);
                let best = history.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let rel_change = if history.len() >= 2 {
                    let last = history[history.len() - 1];
                    let prev = history[history.len() - 2];
                    (last - prev).abs() / prev.abs().max(1e-10)
                } else {
                    1.0
                };
                let status = ConvergenceStatus {
                    iteration: history.len() as u64,
                    best_fitness: best,
                    fitness_history: history,
                    converged,
                    relative_change: rel_change,
                };
                let data = serde_json::json!({ "convergence": status });
                Ok(AgentResult::ok(format!("Converged={}", converged)).with_data(data))
            }
            _ => {
                let data = serde_json::json!({
                    "available_actions": ["perturb", "fitness", "convergence"],
                    "fitness_formula": "F(r,g,l) = sum(w_m * S_m) - lambda * C(r)",
                });
                Ok(AgentResult::ok("Optimizer status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitness_basic() {
        let f = compute_fitness(&[0.9, 0.8], &[0.5, 0.5], 10.0, 0.01);
        assert!((f - (0.45 + 0.40 - 0.1)).abs() < 1e-10);
    }

    #[test]
    fn convergence_not_enough_data() {
        assert!(!detect_convergence(&[1.0, 1.0], 0.001, 5));
    }

    #[test]
    fn convergence_detected() {
        let h = vec![0.5, 0.6, 0.7, 0.8, 0.85, 0.85, 0.85, 0.85, 0.85, 0.85];
        assert!(detect_convergence(&h, 0.01, 5));
    }

    #[test]
    fn convergence_not_detected() {
        let h = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        assert!(!detect_convergence(&h, 0.001, 5));
    }

    #[tokio::test]
    async fn execute_fitness() {
        let agent = OptimizerAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("fitness"))
            .with_param("skill_scores", serde_json::json!([0.9, 0.8]))
            .with_param("weights", serde_json::json!([0.5, 0.5]))
            .with_param("cost", serde_json::json!(10.0));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
