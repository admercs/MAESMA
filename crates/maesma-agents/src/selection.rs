//! Selection agent — Phase 5.7
//!
//! Neural inference + Bayesian posteriors, marginal likelihoods,
//! online Bayesian updating, BMA ensemble weights, Pareto selection.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Bayesian model evidence for a candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEvidence {
    pub model_id: String,
    pub log_marginal_likelihood: f64,
    pub prior_weight: f64,
    pub posterior_weight: f64,
    pub bic: f64,
}

/// BMA (Bayesian Model Averaging) ensemble.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BmaEnsemble {
    pub members: Vec<ModelEvidence>,
    pub effective_n: f64,
}

/// Compute BMA posterior weights from log marginal likelihoods.
pub fn bma_weights(log_ml: &[f64], priors: &[f64]) -> Vec<f64> {
    if log_ml.is_empty() {
        return vec![];
    }
    let max_ll = log_ml.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let unnorm: Vec<f64> = log_ml
        .iter()
        .zip(priors)
        .map(|(ll, p)| (ll - max_ll).exp() * p)
        .collect();
    let total: f64 = unnorm.iter().sum();
    if total > 0.0 {
        unnorm.iter().map(|u| u / total).collect()
    } else {
        vec![1.0 / log_ml.len() as f64; log_ml.len()]
    }
}

/// Compute effective ensemble size from weights.
pub fn effective_ensemble_size(weights: &[f64]) -> f64 {
    let sum_sq: f64 = weights.iter().map(|w| w * w).sum();
    if sum_sq > 0.0 { 1.0 / sum_sq } else { 0.0 }
}

/// Online Bayesian update: new_posterior ∝ likelihood * prior.
pub fn bayesian_update(prior: &[f64], log_likelihoods: &[f64]) -> Vec<f64> {
    if prior.len() != log_likelihoods.len() || prior.is_empty() {
        return vec![];
    }
    let max_ll = log_likelihoods
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let unnorm: Vec<f64> = prior
        .iter()
        .zip(log_likelihoods)
        .map(|(p, ll)| p * (ll - max_ll).exp())
        .collect();
    let total: f64 = unnorm.iter().sum();
    if total > 0.0 {
        unnorm.iter().map(|u| u / total).collect()
    } else {
        prior.to_vec()
    }
}

/// Check if candidate `a` Pareto-dominates candidate `b`.
fn dominates(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    let a_rmse = a.get("rmse").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let b_rmse = b.get("rmse").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let a_cost = a.get("cost").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let b_cost = b.get("cost").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let a_kge = a.get("kge").and_then(|v| v.as_f64()).unwrap_or(f64::MIN);
    let b_kge = b.get("kge").and_then(|v| v.as_f64()).unwrap_or(f64::MIN);
    (a_rmse <= b_rmse && a_cost <= b_cost && a_kge >= b_kge)
        && (a_rmse < b_rmse || a_cost < b_cost || a_kge > b_kge)
}

pub struct SelectionAgent {
    id: AgentId,
}

impl Default for SelectionAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("selection".into()),
        }
    }
}

#[async_trait]
impl Agent for SelectionAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Selection
    }
    fn description(&self) -> &str {
        "Bayesian model selection with BMA ensemble weights and Pareto-front analysis"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("pareto");

        match action {
            "pareto" => {
                let candidates = ctx
                    .params
                    .get("candidates")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                if candidates.is_empty() {
                    return Ok(AgentResult::fail("No candidates"));
                }
                let mut pareto: Vec<&serde_json::Value> = Vec::new();
                for c in &candidates {
                    if !candidates.iter().any(|other| dominates(other, c)) {
                        pareto.push(c);
                    }
                }
                let selected: Vec<serde_json::Value> =
                    pareto.iter().map(|v| (*v).clone()).collect();
                let n = selected.len();
                info!(total = candidates.len(), selected = n, "Pareto selection");
                Ok(
                    AgentResult::ok(format!("{} of {} on Pareto front", n, candidates.len()))
                        .with_data(serde_json::json!({ "selected": selected, "pareto_size": n }))
                        .with_next("run optimizer"),
                )
            }
            "bma" => {
                let log_mls: Vec<f64> = ctx
                    .params
                    .get("log_marginal_likelihoods")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                let priors: Vec<f64> = ctx
                    .params
                    .get("priors")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_else(|| vec![1.0 / log_mls.len().max(1) as f64; log_mls.len()]);
                let weights = bma_weights(&log_mls, &priors);
                let eff_n = effective_ensemble_size(&weights);
                let data = serde_json::json!({ "weights": weights, "effective_n": eff_n });
                Ok(
                    AgentResult::ok(format!("BMA weights computed, eff_N={:.1}", eff_n))
                        .with_data(data),
                )
            }
            "update" => {
                let prior: Vec<f64> = ctx
                    .params
                    .get("prior")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                let lls: Vec<f64> = ctx
                    .params
                    .get("log_likelihoods")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                let posterior = bayesian_update(&prior, &lls);
                let data = serde_json::json!({ "posterior": posterior });
                Ok(AgentResult::ok("Bayesian update complete").with_data(data))
            }
            _ => {
                let data = serde_json::json!({ "available_actions": ["pareto", "bma", "update"] });
                Ok(AgentResult::ok("Selection status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bma_weights_uniform() {
        let w = bma_weights(&[0.0, 0.0, 0.0], &[1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]);
        assert!((w.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        assert!((w[0] - w[1]).abs() < 1e-10);
    }

    #[test]
    fn bma_weights_skewed() {
        let w = bma_weights(&[0.0, -10.0], &[0.5, 0.5]);
        assert!(w[0] > 0.99);
    }

    #[test]
    fn effective_n_one() {
        let n = effective_ensemble_size(&[1.0, 0.0, 0.0]);
        assert!((n - 1.0).abs() < 1e-10);
    }

    #[test]
    fn bayesian_update_concentrates() {
        let prior = vec![0.5, 0.5];
        let lls = vec![0.0, -10.0];
        let post = bayesian_update(&prior, &lls);
        assert!(post[0] > 0.99);
    }

    #[tokio::test]
    async fn execute_pareto() {
        let agent = SelectionAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("pareto"))
            .with_param(
                "candidates",
                serde_json::json!([
                    {"rmse": 0.1, "cost": 10.0, "kge": 0.9},
                    {"rmse": 0.5, "cost": 20.0, "kge": 0.6},
                ]),
            );
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
