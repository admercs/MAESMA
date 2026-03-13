//! Benchmarking agent — evaluates process skill against observational data.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

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

        if observed.is_empty() || predicted.is_empty() || observed.len() != predicted.len() {
            return Ok(AgentResult::fail(
                "Need equal-length observed and predicted arrays",
            ));
        }

        let n = observed.len() as f64;

        // RMSE
        let mse: f64 = observed
            .iter()
            .zip(&predicted)
            .map(|(o, p)| (o - p).powi(2))
            .sum::<f64>()
            / n;
        let rmse = mse.sqrt();

        // Bias
        let bias = predicted.iter().sum::<f64>() / n - observed.iter().sum::<f64>() / n;

        // Correlation
        let mean_o = observed.iter().sum::<f64>() / n;
        let mean_p = predicted.iter().sum::<f64>() / n;
        let cov: f64 = observed
            .iter()
            .zip(&predicted)
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

        // NSE
        let ss_res: f64 = observed
            .iter()
            .zip(&predicted)
            .map(|(o, p)| (o - p).powi(2))
            .sum();
        let ss_tot: f64 = observed.iter().map(|o| (o - mean_o).powi(2)).sum();
        let nse = if ss_tot > 0.0 {
            1.0 - ss_res / ss_tot
        } else {
            0.0
        };

        // KGE
        let alpha = if std_o > 0.0 { std_p / std_o } else { 1.0 };
        let beta = if mean_o != 0.0 { mean_p / mean_o } else { 1.0 };
        let kge = 1.0
            - ((correlation - 1.0).powi(2) + (alpha - 1.0).powi(2) + (beta - 1.0).powi(2)).sqrt();

        info!(rmse, kge, nse, bias, "Benchmark metrics computed");

        Ok(AgentResult::ok(format!(
            "Benchmark: RMSE={:.4}, KGE={:.4}, NSE={:.4}, r={:.4}",
            rmse, kge, nse, correlation
        ))
        .with_data(serde_json::json!({
            "rmse": rmse,
            "kge": kge,
            "nse": nse,
            "bias": bias,
            "correlation": correlation,
            "n_samples": observed.len(),
        }))
        .with_next("run selection"))
    }
}
