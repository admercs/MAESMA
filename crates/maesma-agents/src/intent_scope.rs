//! Intent & Scope Agent — Phase 5.1
//!
//! Parses user objectives into observable requirements, error bands, and
//! priority tiers.  The output drives the entire downstream assembly pipeline.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// An observable requirement extracted from user objectives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservableRequirement {
    /// Variable name (e.g., "streamflow", "soil_moisture", "LAI").
    pub variable: String,
    /// Acceptable error band.
    pub error_band: ErrorBand,
    /// Priority tier (1 = highest).
    pub priority: u8,
    /// Required spatial domain.
    pub region: Option<String>,
    /// Required temporal coverage.
    pub temporal_coverage: Option<String>,
}

/// Error band specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBand {
    /// Metric name (e.g., "rmse", "kge").
    pub metric: String,
    /// Acceptable threshold.
    pub threshold: f64,
    /// Whether lower is better.
    pub lower_is_better: bool,
}

/// Parsed user intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
    /// High-level objective text.
    pub objective: String,
    /// Extracted observable requirements.
    pub requirements: Vec<ObservableRequirement>,
    /// Computational budget tier (low/medium/high/unlimited).
    pub budget_tier: String,
    /// Whether real-time constraints apply.
    pub real_time: bool,
}

pub struct IntentScopeAgent {
    id: AgentId,
}

impl IntentScopeAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("intent_scope".into()),
        }
    }

    /// Parse a natural-language objective into structured requirements.
    pub fn parse_objective(objective: &str) -> ParsedIntent {
        let lower = objective.to_lowercase();
        let mut requirements = Vec::new();
        let mut priority = 1u8;

        // Keyword-driven requirement extraction
        let variable_keywords: &[(&str, &str, f64)] = &[
            ("streamflow", "kge", 0.7),
            ("discharge", "kge", 0.7),
            ("soil moisture", "rmse", 0.05),
            ("evapotranspiration", "rmse", 0.5),
            ("carbon", "rmse", 1.0),
            ("nee", "kge", 0.6),
            ("gpp", "rmse", 2.0),
            ("lai", "correlation", 0.7),
            ("ndvi", "correlation", 0.7),
            ("snow", "rmse", 0.1),
            ("swe", "rmse", 50.0),
            ("temperature", "rmse", 2.0),
            ("precipitation", "rmse", 1.0),
            ("fire", "rmse", 0.3),
            ("burn", "rmse", 0.3),
            ("radiation", "rmse", 10.0),
            ("albedo", "rmse", 0.05),
            ("sea surface", "rmse", 0.5),
            ("ocean", "rmse", 1.0),
            ("ice", "rmse", 0.1),
            ("permafrost", "rmse", 1.0),
            ("biodiversity", "correlation", 0.5),
            ("species", "correlation", 0.5),
        ];

        for &(keyword, metric, threshold) in variable_keywords {
            if lower.contains(keyword) {
                let lower_is_better = metric == "rmse";
                requirements.push(ObservableRequirement {
                    variable: keyword.replace(' ', "_"),
                    error_band: ErrorBand {
                        metric: metric.into(),
                        threshold,
                        lower_is_better,
                    },
                    priority,
                    region: None,
                    temporal_coverage: None,
                });
                priority = priority.saturating_add(1);
            }
        }

        // Budget detection
        let budget_tier =
            if lower.contains("fast") || lower.contains("quick") || lower.contains("screen") {
                "low"
            } else if lower.contains("high fidelity") || lower.contains("production") {
                "high"
            } else if lower.contains("unlimited") || lower.contains("full") {
                "unlimited"
            } else {
                "medium"
            };

        let real_time = lower.contains("real-time")
            || lower.contains("realtime")
            || lower.contains("operational");

        // If no specific requirements found, add a generic one
        if requirements.is_empty() {
            requirements.push(ObservableRequirement {
                variable: "general_skill".into(),
                error_band: ErrorBand {
                    metric: "kge".into(),
                    threshold: 0.5,
                    lower_is_better: false,
                },
                priority: 1,
                region: None,
                temporal_coverage: None,
            });
        }

        ParsedIntent {
            objective: objective.to_string(),
            requirements,
            budget_tier: budget_tier.into(),
            real_time,
        }
    }
}

impl Default for IntentScopeAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for IntentScopeAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn role(&self) -> AgentRole {
        AgentRole::IntentScope
    }

    fn description(&self) -> &str {
        "Parses user objectives into observable requirements, error bands, and priority tiers"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let objective = ctx
            .params
            .get("objective")
            .and_then(|v| v.as_str())
            .unwrap_or("general earth system simulation");

        let intent = Self::parse_objective(objective);

        let data = serde_json::json!({
            "objective": intent.objective,
            "requirements": intent.requirements,
            "requirement_count": intent.requirements.len(),
            "budget_tier": intent.budget_tier,
            "real_time": intent.real_time,
            "priority_variables": intent.requirements.iter()
                .map(|r| format!("{}({}≤{})", r.variable, r.error_band.metric, r.error_band.threshold))
                .collect::<Vec<_>>(),
        });

        Ok(AgentResult::ok(format!(
            "Parsed {} observable requirements from objective; budget={}",
            intent.requirements.len(),
            intent.budget_tier,
        ))
        .with_data(data)
        .with_next("kb_retrieval")
        .with_next("assembly"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_streamflow_objective() {
        let intent = IntentScopeAgent::parse_objective(
            "Simulate streamflow and soil moisture for western US watersheds",
        );
        assert_eq!(intent.requirements.len(), 2);
        assert_eq!(intent.requirements[0].variable, "streamflow");
        assert_eq!(intent.requirements[1].variable, "soil_moisture");
        assert_eq!(intent.budget_tier, "medium");
    }

    #[test]
    fn parse_fire_objective() {
        let intent = IntentScopeAgent::parse_objective(
            "High fidelity fire and carbon simulation for quick screening",
        );
        assert!(intent.requirements.iter().any(|r| r.variable == "fire"));
        assert!(intent.requirements.iter().any(|r| r.variable == "carbon"));
    }

    #[test]
    fn empty_objective_gets_default() {
        let intent = IntentScopeAgent::parse_objective("run a model");
        assert_eq!(intent.requirements.len(), 1);
        assert_eq!(intent.requirements[0].variable, "general_skill");
    }

    #[tokio::test]
    async fn execute_intent_scope() {
        let agent = IntentScopeAgent::new();
        let ctx =
            AgentContext::new().with_param("objective", serde_json::json!("streamflow prediction"));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
        assert!(result.next_actions.contains(&"kb_retrieval".to_string()));
    }
}
