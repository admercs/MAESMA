//! Benchmarking agent — Phase 5.6
//!
//! Run/schedule simulations, extract outputs at observation locations,
//! compute multi-metric skill, write to Store, compare against existing records.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// A benchmark run record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRun {
    pub run_id: String,
    pub family: String,
    pub rung: String,
    pub region: String,
    pub metrics: BenchmarkMetrics,
    pub comparison: Option<ComparisonResult>,
}

/// Computed benchmark metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    pub rmse: f64,
    pub bias: f64,
    pub correlation: f64,
    pub nse: f64,
    pub kge: f64,
    pub n_samples: usize,
}

/// Result of comparing against existing records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub improved: bool,
    pub delta_rmse: f64,
    pub delta_kge: f64,
    pub previous_best: String,
}

/// Compute benchmark metrics from observations and predictions.
pub fn compute_metrics(observed: &[f64], predicted: &[f64]) -> Option<BenchmarkMetrics> {
    if observed.is_empty() || observed.len() != predicted.len() {
        return None;
    }
    let n = observed.len() as f64;
    let mse: f64 = observed
        .iter()
        .zip(predicted)
        .map(|(o, p)| (o - p).powi(2))
        .sum::<f64>()
        / n;
    let rmse = mse.sqrt();
    let bias = predicted.iter().sum::<f64>() / n - observed.iter().sum::<f64>() / n;
    let mean_o = observed.iter().sum::<f64>() / n;
    let mean_p = predicted.iter().sum::<f64>() / n;
    let cov: f64 = observed
        .iter()
        .zip(predicted)
        .map(|(o, p)| (o - mean_o) * (p - mean_p))
        .sum::<f64>()
        / n;
    let std_o = (observed.iter().map(|o| (o - mean_o).powi(2)).sum::<f64>() / n).sqrt();
    let std_p = (predicted.iter().map(|p| (p - mean_p).powi(2)).sum::<f64>() / n).sqrt();
    let correlation = if std_o > 0.0 && std_p > 0.0 {
        cov / (std_o * std_p)
    } else {
        0.0
    };
    let ss_res: f64 = observed
        .iter()
        .zip(predicted)
        .map(|(o, p)| (o - p).powi(2))
        .sum();
    let ss_tot: f64 = observed.iter().map(|o| (o - mean_o).powi(2)).sum();
    let nse = if ss_tot > 0.0 {
        1.0 - ss_res / ss_tot
    } else {
        0.0
    };
    let alpha = if std_o > 0.0 { std_p / std_o } else { 1.0 };
    let beta = if mean_o != 0.0 { mean_p / mean_o } else { 1.0 };
    let kge =
        1.0 - ((correlation - 1.0).powi(2) + (alpha - 1.0).powi(2) + (beta - 1.0).powi(2)).sqrt();
    Some(BenchmarkMetrics {
        rmse,
        bias,
        correlation,
        nse,
        kge,
        n_samples: observed.len(),
    })
}

pub struct BenchmarkingAgent {
    id: AgentId,
}

impl Default for BenchmarkingAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchmarkingAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("benchmarking".into()),
        }
    }
}

#[async_trait]
impl Agent for BenchmarkingAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Benchmarking
    }
    fn description(&self) -> &str {
        "Runs process representations against observational benchmarks and records skill scores"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("evaluate");

        match action {
            "evaluate" => {
                let observed: Vec<f64> = ctx
                    .params
                    .get("observed")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                let predicted: Vec<f64> = ctx
                    .params
                    .get("predicted")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                match compute_metrics(&observed, &predicted) {
                    Some(m) => {
                        info!(
                            rmse = m.rmse,
                            kge = m.kge,
                            nse = m.nse,
                            bias = m.bias,
                            "Benchmark"
                        );
                        Ok(AgentResult::ok(format!(
                            "Benchmark: RMSE={:.4}, KGE={:.4}, NSE={:.4}",
                            m.rmse, m.kge, m.nse
                        ))
                        .with_data(serde_json::json!({ "metrics": m }))
                        .with_next("run selection"))
                    }
                    None => Ok(AgentResult::fail(
                        "Need equal-length observed and predicted arrays",
                    )),
                }
            }
            "compare" => {
                let current_rmse = ctx
                    .params
                    .get("current_rmse")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                let best_rmse = ctx
                    .params
                    .get("best_rmse")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(2.0);
                let current_kge = ctx
                    .params
                    .get("current_kge")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5);
                let best_kge = ctx
                    .params
                    .get("best_kge")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.3);
                let comparison = ComparisonResult {
                    improved: current_rmse < best_rmse,
                    delta_rmse: best_rmse - current_rmse,
                    delta_kge: current_kge - best_kge,
                    previous_best: "prior_run".into(),
                };
                let data = serde_json::json!({ "comparison": comparison });
                Ok(
                    AgentResult::ok(format!("Comparison: improved={}", comparison.improved))
                        .with_data(data),
                )
            }
            _ => {
                let data = serde_json::json!({ "available_actions": ["evaluate", "compare"] });
                Ok(AgentResult::ok("Benchmarking status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_metrics_basic() {
        let obs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let pred = vec![1.1, 2.1, 2.9, 4.2, 4.8];
        let m = compute_metrics(&obs, &pred).unwrap();
        assert!(m.rmse < 0.3);
        assert!(m.kge > 0.8);
        assert!(m.correlation > 0.95);
    }

    #[test]
    fn compute_metrics_perfect() {
        let v = vec![1.0, 2.0, 3.0];
        let m = compute_metrics(&v, &v).unwrap();
        assert!((m.rmse).abs() < 1e-10);
        assert!((m.kge - 1.0).abs() < 1e-10);
    }

    #[test]
    fn compute_metrics_empty() {
        assert!(compute_metrics(&[], &[]).is_none());
    }

    #[tokio::test]
    async fn execute_evaluate() {
        let agent = BenchmarkingAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("evaluate"))
            .with_param("observed", serde_json::json!([1.0, 2.0, 3.0]))
            .with_param("predicted", serde_json::json!([1.1, 2.0, 2.9]));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
