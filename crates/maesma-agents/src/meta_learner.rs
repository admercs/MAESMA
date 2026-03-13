//! Meta-learner agent — learns from cross-domain assembly experience.

use std::collections::HashMap;

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct MetaLearnerAgent {
    id: AgentId,
}

impl Default for MetaLearnerAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaLearnerAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("meta_learner".into()),
        }
    }
}

#[async_trait]
impl Agent for MetaLearnerAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::MetaLearner
    }
    fn description(&self) -> &str {
        "Learns across assembly attempts to improve future agent decisions"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let history = ctx
            .params
            .get("history")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if history.is_empty() {
            return Ok(AgentResult::ok(
                "No history — will learn from future assembly cycles",
            ));
        }

        let mut successes = 0u64;
        let mut _failures = 0u64;
        let mut strategy_counts: HashMap<String, (u64, u64)> = HashMap::new();

        for entry in &history {
            let success = entry
                .get("success")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let strategy = entry
                .get("strategy")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string();
            if success {
                successes += 1;
            } else {
                _failures += 1;
            }
            let counts = strategy_counts.entry(strategy).or_insert((0, 0));
            if success {
                counts.0 += 1;
            } else {
                counts.1 += 1;
            }
        }

        let best_strategy = strategy_counts
            .iter()
            .max_by(|a, b| {
                let rate_a = a.1.0 as f64 / (a.1.0 + a.1.1).max(1) as f64;
                let rate_b = b.1.0 as f64 / (b.1.0 + b.1.1).max(1) as f64;
                rate_a
                    .partial_cmp(&rate_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(name, _)| name.clone())
            .unwrap_or_else(|| "default".into());

        let recommendations: Vec<serde_json::Value> = strategy_counts
            .iter()
            .map(|(name, (s, f))| {
                let rate = *s as f64 / (*s + *f).max(1) as f64;
                serde_json::json!({
                    "strategy": name,
                    "success_rate": rate,
                    "total_attempts": s + f,
                    "recommended": rate > 0.5,
                })
            })
            .collect();

        info!(total = history.len(), successes, best = %best_strategy, "Meta-learning");

        Ok(AgentResult::ok(format!(
            "Meta-learning: {}/{} successes, best = '{}'",
            successes,
            history.len(),
            best_strategy
        ))
        .with_data(serde_json::json!({
            "success_rate": successes as f64 / history.len() as f64,
            "best_strategy": best_strategy,
            "recommendations": recommendations,
        })))
    }
}
