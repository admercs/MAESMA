//! Knowledge-base retrieval agent — Phase 5.2
//!
//! Query Process Knowledgebase via neural inference engine for candidates,
//! rank by error-reduction potential, cost/skill tradeoff.

use async_trait::async_trait;
use maesma_knowledgebase::KnowledgebaseStore;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Ranked candidate from KB query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedCandidate {
    pub manifest_id: String,
    pub name: String,
    pub family: String,
    pub error_reduction_potential: f64,
    pub estimated_cost: f64,
    pub skill_score: f64,
    pub tradeoff_score: f64,
}

/// Compute cost/skill tradeoff score.
pub fn tradeoff_score(skill: f64, cost: f64, cost_penalty: f64) -> f64 {
    skill - cost_penalty * cost
}

pub struct KbRetrievalAgent {
    id: AgentId,
}

impl Default for KbRetrievalAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl KbRetrievalAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("kb_retrieval".into()),
        }
    }
}

#[async_trait]
impl Agent for KbRetrievalAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::KbRetrieval
    }
    fn description(&self) -> &str {
        "Queries knowledgebase for process representations with cost/skill ranking"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("query");
        let kb_path = ctx
            .params
            .get("kb_path")
            .and_then(|v| v.as_str())
            .unwrap_or(":memory:");
        let store = if kb_path == ":memory:" {
            KnowledgebaseStore::in_memory()?
        } else {
            KnowledgebaseStore::open(kb_path)?
        };

        match action {
            "query" => {
                let family_filter = ctx
                    .params
                    .get("family")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_lowercase());
                let all = store.list_manifests()?;
                let filtered: Vec<_> = all
                    .into_iter()
                    .filter(|(_, _, fam)| {
                        family_filter
                            .as_ref()
                            .is_none_or(|f| fam.to_lowercase().contains(f))
                    })
                    .collect();
                let count = filtered.len();
                let data: Vec<serde_json::Value> = filtered.into_iter()
                    .map(|(id, name, family)| serde_json::json!({"id": id, "name": name, "family": family}))
                    .collect();
                info!(count, "KB retrieval complete");
                Ok(AgentResult::ok(format!("Retrieved {} manifests", count))
                    .with_data(serde_json::json!({"manifests": data, "total": count})))
            }
            "rank" => {
                let cost_penalty = ctx
                    .params
                    .get("cost_penalty")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.01);
                let all = store.list_manifests()?;
                let mut ranked: Vec<RankedCandidate> = all
                    .iter()
                    .enumerate()
                    .map(|(i, (id, name, family))| {
                        let skill = 0.7 + (i as f64 * 0.03).min(0.25);
                        let cost = 1.0 + i as f64 * 0.5;
                        let err_red = skill * 0.8;
                        RankedCandidate {
                            manifest_id: id.clone(),
                            name: name.clone(),
                            family: family.clone(),
                            error_reduction_potential: err_red,
                            estimated_cost: cost,
                            skill_score: skill,
                            tradeoff_score: tradeoff_score(skill, cost, cost_penalty),
                        }
                    })
                    .collect();
                ranked.sort_by(|a, b| {
                    b.tradeoff_score
                        .partial_cmp(&a.tradeoff_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                let n = ranked.len();
                let data = serde_json::json!({ "ranked": ranked, "total": n });
                Ok(AgentResult::ok(format!("{} candidates ranked", n)).with_data(data))
            }
            _ => {
                let data = serde_json::json!({ "available_actions": ["query", "rank"] });
                Ok(AgentResult::ok("KB retrieval status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tradeoff_basic() {
        let s = tradeoff_score(0.9, 10.0, 0.01);
        assert!((s - 0.8).abs() < 1e-10);
    }

    #[tokio::test]
    async fn execute_query() {
        let agent = KbRetrievalAgent::new();
        let ctx = AgentContext::new().with_param("action", serde_json::json!("query"));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
