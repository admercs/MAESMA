//! Knowledge-base retrieval agent.

use async_trait::async_trait;
use maesma_knowledgebase::KnowledgebaseStore;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

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
        "Queries the knowledgebase for process representations matching regime, scale, and skill criteria"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
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
        let data: Vec<serde_json::Value> = filtered
            .into_iter()
            .map(|(id, name, family)| serde_json::json!({"id": id, "name": name, "family": family}))
            .collect();

        info!(count, "KB retrieval complete");

        Ok(AgentResult::ok(format!("Retrieved {} manifests", count))
            .with_data(serde_json::json!({"manifests": data, "total": count})))
    }
}
