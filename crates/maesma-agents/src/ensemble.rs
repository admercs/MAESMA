//! Ensemble agent — manages structural ensembles across SAPG configurations.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct EnsembleAgent {
    id: AgentId,
}

impl Default for EnsembleAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl EnsembleAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("ensemble".into()),
        }
    }
}

#[async_trait]
impl Agent for EnsembleAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Ensemble
    }
    fn description(&self) -> &str {
        "Manages structural ensembles spanning multiple SAPG configurations"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let members = ctx
            .params
            .get("members")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if members.is_empty() {
            return Ok(AgentResult::fail("No ensemble members provided"));
        }

        let weights: Vec<f64> = ctx
            .params
            .get("weights")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_f64()).collect())
            .unwrap_or_else(|| vec![1.0 / members.len() as f64; members.len()]);

        let skills: Vec<f64> = members
            .iter()
            .filter_map(|m| m.get("skill").and_then(|v| v.as_f64()))
            .collect();

        let ensemble_mean = if !skills.is_empty() {
            skills.iter().zip(&weights).map(|(s, w)| s * w).sum::<f64>()
        } else {
            0.0
        };

        let ensemble_spread = if skills.len() > 1 {
            let var: f64 = skills
                .iter()
                .map(|s| (s - ensemble_mean).powi(2))
                .sum::<f64>()
                / (skills.len() - 1) as f64;
            var.sqrt()
        } else {
            0.0
        };

        info!(
            members = members.len(),
            mean = ensemble_mean,
            spread = ensemble_spread,
            "Ensemble stats"
        );

        Ok(AgentResult::ok(format!(
            "Ensemble: {} members, mean={:.4}, spread={:.4}",
            members.len(),
            ensemble_mean,
            ensemble_spread
        ))
        .with_data(serde_json::json!({
            "n_members": members.len(),
            "ensemble_mean": ensemble_mean,
            "ensemble_spread": ensemble_spread,
            "weights": weights,
        })))
    }
}
