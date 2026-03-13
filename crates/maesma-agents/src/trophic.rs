//! Trophic dynamics agent — manages food web interactions.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

pub struct TrophicAgent {
    id: AgentId,
}

impl Default for TrophicAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl TrophicAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("trophic".into()),
        }
    }
}

#[async_trait]
impl Agent for TrophicAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Trophic
    }
    fn description(&self) -> &str {
        "Manages trophic dynamics and food-web process representations"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let species = ctx
            .params
            .get("species")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if species.is_empty() {
            return Ok(AgentResult::ok("No species data provided"));
        }

        let dt = ctx.params.get("dt").and_then(|v| v.as_f64()).unwrap_or(1.0);

        // Logistic growth for each species
        let mut results = Vec::new();
        for sp in &species {
            let name = sp.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let abundance = sp
                .get("abundance")
                .and_then(|v| v.as_f64())
                .unwrap_or(100.0);
            let growth_rate = sp
                .get("growth_rate")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.1);
            let carrying_capacity = sp
                .get("carrying_capacity")
                .and_then(|v| v.as_f64())
                .unwrap_or(1000.0);

            let dn = growth_rate * abundance * (1.0 - abundance / carrying_capacity) * dt;
            let new_abundance = (abundance + dn).max(0.0);

            results.push(serde_json::json!({
                "name": name,
                "abundance": new_abundance,
                "growth_rate": growth_rate,
                "delta": dn,
            }));
        }

        info!(species = species.len(), dt, "Trophic dynamics step");

        Ok(AgentResult::ok(format!(
            "Trophic step: {} species (dt={:.1})",
            species.len(),
            dt
        ))
        .with_data(serde_json::json!({"species": results, "dt": dt})))
    }
}
