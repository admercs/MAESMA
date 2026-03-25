//! Intent & Scope Agent — Phase 5.1
//!
//! Parses user objectives into observable requirements, error bands, and
//! priority tiers.  The output drives the entire downstream assembly pipeline.
//!
//! Also performs inquiry-driven selection: given a natural-language question
//! it identifies relevant process families, recommends fidelity rungs per
//! family, and suggests observation datasets for validation.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use maesma_core::families::ProcessFamily;
use maesma_core::inquiry::{
    BudgetConstraint, DatasetRecommendation, FidelityRecommendation, Inquiry, InquiryPlan,
    coupling_dependencies, dataset_keywords, family_keywords,
};

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

    /// Analyze a user inquiry and produce a full `InquiryPlan` including
    /// recommended process families, fidelity rungs, and datasets.
    pub fn analyze_inquiry(inquiry: &Inquiry) -> InquiryPlan {
        let lower = inquiry.question.to_lowercase();
        let keywords = family_keywords();
        let ds_keywords = dataset_keywords();
        let deps = coupling_dependencies();

        // 1. Identify primary families from keywords
        let mut primary_set: HashSet<ProcessFamily> = HashSet::new();
        for kw in &keywords {
            if lower.contains(kw.keyword) && kw.strong {
                primary_set.insert(kw.family);
            }
        }
        // If nothing strongly matched, try weak matches
        if primary_set.is_empty() {
            for kw in &keywords {
                if lower.contains(kw.keyword) {
                    primary_set.insert(kw.family);
                }
            }
        }
        // Final fallback: Atmosphere + Hydrology
        if primary_set.is_empty() {
            primary_set.insert(ProcessFamily::Atmosphere);
            primary_set.insert(ProcessFamily::Hydrology);
        }

        // 2. Derive supporting families from coupling dependencies
        let mut supporting_set: HashSet<ProcessFamily> = HashSet::new();
        for &(from, to) in &deps {
            if primary_set.contains(&from) && !primary_set.contains(&to) {
                supporting_set.insert(to);
            }
        }

        // 3. Determine budget
        let budget = inquiry.budget;
        let primary_rung = budget.max_rung();
        let support_rung = budget.default_rung();

        // 4. Build fidelity map
        let primary_families: Vec<ProcessFamily> = primary_set.iter().copied().collect();
        let supporting_families: Vec<ProcessFamily> = supporting_set.iter().copied().collect();

        let mut fidelity_map: Vec<FidelityRecommendation> = Vec::new();
        for &fam in &primary_families {
            fidelity_map.push(FidelityRecommendation {
                family: fam,
                recommended_rung: primary_rung,
                is_primary: true,
                reason: format!(
                    "{} directly relevant to inquiry; using {} for primary families",
                    fam.display_name(),
                    primary_rung.label()
                ),
            });
        }
        for &fam in &supporting_families {
            fidelity_map.push(FidelityRecommendation {
                family: fam,
                recommended_rung: support_rung,
                is_primary: false,
                reason: format!(
                    "{} needed as coupling dependency; using {} for supporting families",
                    fam.display_name(),
                    support_rung.label()
                ),
            });
        }

        // 5. Recommend datasets
        let mut datasets: Vec<DatasetRecommendation> = Vec::new();
        let mut seen_datasets: HashSet<String> = HashSet::new();
        for dkw in &ds_keywords {
            if lower.contains(dkw.keyword) && seen_datasets.insert(dkw.dataset_name.to_string()) {
                datasets.push(DatasetRecommendation {
                    dataset_name: dkw.dataset_name.to_string(),
                    observable: dkw.observable.to_string(),
                    relevance_score: 0.9,
                    reason: format!(
                        "Matches inquiry keyword '{}'; measures {}",
                        dkw.keyword, dkw.observable
                    ),
                });
            }
        }
        // If no direct dataset match, recommend general datasets for primary families
        if datasets.is_empty() {
            for &fam in &primary_families {
                let (ds_name, obs) = match fam {
                    ProcessFamily::Fire => ("MTBS", "burned_area"),
                    ProcessFamily::Hydrology => ("USGS_NWIS", "streamflow"),
                    ProcessFamily::Ecology => ("MODIS_LAI", "lai"),
                    ProcessFamily::Biogeochemistry => ("FLUXNET", "nee"),
                    ProcessFamily::Radiation => ("CERES", "radiation"),
                    ProcessFamily::Atmosphere => ("ERA5", "temperature"),
                    ProcessFamily::Ocean => ("Argo", "ocean_temperature"),
                    ProcessFamily::Cryosphere => ("SNODAS", "swe"),
                    ProcessFamily::Geomorphology => ("USGS_Sediment", "suspended_sediment"),
                    ProcessFamily::Geology => ("USGS_NWIS", "groundwater_level"),
                    ProcessFamily::HumanSystems => ("NLCD", "land_cover"),
                    ProcessFamily::TrophicDynamics => ("GBIF", "species_occurrence"),
                    ProcessFamily::Evolution => ("GBIF", "species_occurrence"),
                };
                if seen_datasets.insert(ds_name.to_string()) {
                    datasets.push(DatasetRecommendation {
                        dataset_name: ds_name.to_string(),
                        observable: obs.to_string(),
                        relevance_score: 0.6,
                        reason: format!("Default dataset for {} family", fam.display_name()),
                    });
                }
            }
        }

        // 6. Rationale
        let primary_names: Vec<&str> = primary_families.iter().map(|f| f.display_name()).collect();
        let support_names: Vec<&str> = supporting_families
            .iter()
            .map(|f| f.display_name())
            .collect();
        let dataset_names: Vec<&str> = datasets.iter().map(|d| d.dataset_name.as_str()).collect();

        let rationale = format!(
            "Inquiry analysis identified {} primary process families [{}] and {} supporting \
             families [{}]. Primary families will run at {} fidelity, supporting at {}. \
             {} observation datasets recommended for validation: [{}].",
            primary_families.len(),
            primary_names.join(", "),
            supporting_families.len(),
            support_names.join(", "),
            primary_rung.label(),
            support_rung.label(),
            datasets.len(),
            dataset_names.join(", "),
        );

        let confidence = if primary_families.len() >= 2 {
            0.85
        } else if primary_families.len() == 1 {
            0.75
        } else {
            0.5
        };

        InquiryPlan {
            inquiry: inquiry.clone(),
            primary_families,
            supporting_families,
            fidelity_map,
            datasets,
            manifests: Vec::new(), // populated later via KB lookup
            rationale,
            confidence,
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
        // If context contains an "inquiry" object, run full inquiry analysis
        if let Some(question) = ctx.params.get("inquiry").and_then(|v| v.as_str()) {
            let budget = ctx
                .params
                .get("budget")
                .and_then(|v| v.as_str())
                .unwrap_or("medium");
            let budget_constraint = match budget {
                "low" => BudgetConstraint::Low,
                "high" => BudgetConstraint::High,
                "unlimited" => BudgetConstraint::Unlimited,
                _ => BudgetConstraint::Medium,
            };
            let region = ctx
                .params
                .get("region")
                .and_then(|v| v.as_str())
                .map(String::from);
            let temporal_scope = ctx
                .params
                .get("temporal_scope")
                .and_then(|v| v.as_str())
                .map(String::from);

            let inquiry = Inquiry {
                question: question.to_string(),
                region,
                temporal_scope,
                budget: budget_constraint,
            };
            let plan = Self::analyze_inquiry(&inquiry);

            let data = serde_json::to_value(&plan)
                .map_err(|e| maesma_core::Error::Serialization(e.to_string()))?;

            return Ok(AgentResult::ok(format!(
                "Inquiry analysis: {} primary families, {} supporting, {} datasets; confidence={:.2}",
                plan.primary_families.len(),
                plan.supporting_families.len(),
                plan.datasets.len(),
                plan.confidence,
            ))
            .with_data(data)
            .with_next("kb_retrieval")
            .with_next("assembly"));
        }

        // Default: parse a simple objective string
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
