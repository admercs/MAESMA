//! Selection agent — Pareto-optimal process selection.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

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

/// Check if candidate `a` Pareto-dominates candidate `b`.
/// Uses (rmse ↓, cost ↓, kge ↑) as objectives.
fn dominates(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    let a_rmse = a.get("rmse").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let b_rmse = b.get("rmse").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let a_cost = a.get("cost").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let b_cost = b.get("cost").and_then(|v| v.as_f64()).unwrap_or(f64::MAX);
    let a_kge = a.get("kge").and_then(|v| v.as_f64()).unwrap_or(f64::MIN);
    let b_kge = b.get("kge").and_then(|v| v.as_f64()).unwrap_or(f64::MIN);

    let all_ok = a_rmse <= b_rmse && a_cost <= b_cost && a_kge >= b_kge;
    let some_better = a_rmse < b_rmse || a_cost < b_cost || a_kge > b_kge;
    all_ok && some_better
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
        "Selects optimal process representation per slot using Pareto-front analysis"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let candidates = ctx
            .params
            .get("candidates")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if candidates.is_empty() {
            return Ok(AgentResult::fail("No candidates for selection"));
        }

        let mut pareto: Vec<&serde_json::Value> = Vec::new();
        for c in &candidates {
            let dominated = candidates.iter().any(|other| dominates(other, c));
            if !dominated {
                pareto.push(c);
            }
        }

        let selected: Vec<serde_json::Value> = pareto.iter().map(|v| (*v).clone()).collect();
        let n_selected = selected.len();

        info!(
            total = candidates.len(),
            selected = n_selected,
            "Pareto selection"
        );

        Ok(AgentResult::ok(format!(
            "Selected {} of {} candidates on Pareto front",
            n_selected,
            candidates.len()
        ))
        .with_data(serde_json::json!({
            "selected": selected,
            "total_candidates": candidates.len(),
            "pareto_size": n_selected,
        }))
        .with_next("run optimizer"))
    }
}
