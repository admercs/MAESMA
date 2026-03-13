//! Skill Librarian Agent — Phase 5.9
//!
//! Manages the Skill Score Store lifecycle: writes, queries, versioning, GC.
//! Aggregates skill across dimensions, detects skill drift, and generates
//! trend reports.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Skill drift detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    pub family: String,
    pub metric: String,
    pub trend: f64,
    pub is_drifting: bool,
    pub direction: String,
}

/// Aggregated skill summary across a dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedSkill {
    pub dimension: String,
    pub value: String,
    pub mean_kge: f64,
    pub mean_rmse: f64,
    pub record_count: usize,
}

pub struct SkillLibrarianAgent {
    id: AgentId,
}

impl SkillLibrarianAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("skill_librarian".into()),
        }
    }

    /// Detect drift in a time-series of skill values.
    pub fn detect_drift(values: &[f64], threshold: f64) -> Option<DriftReport> {
        if values.len() < 3 {
            return None;
        }
        let n = values.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean: f64 = values.iter().sum::<f64>() / n;

        let mut num = 0.0;
        let mut den = 0.0;
        for (i, &y) in values.iter().enumerate() {
            let x = i as f64;
            num += (x - x_mean) * (y - y_mean);
            den += (x - x_mean) * (x - x_mean);
        }
        let slope = if den.abs() > 1e-12 { num / den } else { 0.0 };

        let is_drifting = slope.abs() > threshold;
        let direction = if slope > 0.0 {
            "improving"
        } else {
            "degrading"
        };

        Some(DriftReport {
            family: String::new(),
            metric: String::new(),
            trend: slope,
            is_drifting,
            direction: direction.into(),
        })
    }
}

impl Default for SkillLibrarianAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for SkillLibrarianAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn role(&self) -> AgentRole {
        AgentRole::SkillLibrarian
    }

    fn description(&self) -> &str {
        "Manages skill score lifecycle, drift detection, and trend reporting"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("report");

        match action {
            "drift_check" => {
                let mut reports = Vec::new();
                if let Some(series) = ctx.params.get("skill_series").and_then(|v| v.as_object()) {
                    let threshold = ctx
                        .params
                        .get("threshold")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.01);
                    for (family, values) in series {
                        if let Some(arr) = values.as_array() {
                            let vals: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();
                            if let Some(mut report) = Self::detect_drift(&vals, threshold) {
                                report.family = family.clone();
                                report.metric = "kge".into();
                                reports.push(report);
                            }
                        }
                    }
                }
                let drifting: Vec<_> = reports.iter().filter(|r| r.is_drifting).collect();
                let data = serde_json::json!({
                    "drift_reports": reports,
                    "drifting_count": drifting.len(),
                    "total_checked": reports.len(),
                });
                Ok(AgentResult::ok(format!(
                    "Drift check: {}/{} families drifting",
                    drifting.len(),
                    reports.len()
                ))
                .with_data(data))
            }

            "aggregate" => {
                let mut summaries = Vec::new();
                if let Some(records) = ctx.params.get("records").and_then(|v| v.as_array()) {
                    let dimension = ctx
                        .params
                        .get("dimension")
                        .and_then(|v| v.as_str())
                        .unwrap_or("region");

                    let mut groups: std::collections::HashMap<String, Vec<(f64, f64)>> =
                        std::collections::HashMap::new();
                    for record in records {
                        let key = record
                            .get(dimension)
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let kge = record.get("kge").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let rmse = record.get("rmse").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        groups.entry(key.to_string()).or_default().push((kge, rmse));
                    }
                    for (value, metrics) in &groups {
                        let n = metrics.len();
                        let mean_kge = metrics.iter().map(|(k, _)| k).sum::<f64>() / n as f64;
                        let mean_rmse = metrics.iter().map(|(_, r)| r).sum::<f64>() / n as f64;
                        summaries.push(AggregatedSkill {
                            dimension: dimension.into(),
                            value: value.clone(),
                            mean_kge,
                            mean_rmse,
                            record_count: n,
                        });
                    }
                }
                let data = serde_json::json!({ "aggregated": summaries });
                Ok(
                    AgentResult::ok(format!("Aggregated {} groups", summaries.len()))
                        .with_data(data),
                )
            }

            "gc" => {
                let max_age_days = ctx
                    .params
                    .get("max_age_days")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(365);
                let keep_pareto = ctx
                    .params
                    .get("keep_pareto")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                let data = serde_json::json!({
                    "action": "garbage_collection",
                    "max_age_days": max_age_days,
                    "keep_pareto_front": keep_pareto,
                    "status": "scheduled",
                });
                Ok(AgentResult::ok("Skill store GC scheduled").with_data(data))
            }

            _ => {
                // Default: summary report
                let total_records = ctx
                    .params
                    .get("total_records")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                let families_covered = ctx
                    .params
                    .get("families_covered")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                let data = serde_json::json!({
                    "total_records": total_records,
                    "families_covered": families_covered,
                    "status": "healthy",
                });
                Ok(AgentResult::ok(format!(
                    "Skill store: {} records across {} families",
                    total_records, families_covered
                ))
                .with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_improvement_drift() {
        let values = vec![0.5, 0.55, 0.6, 0.65, 0.7, 0.75];
        let report = SkillLibrarianAgent::detect_drift(&values, 0.01).unwrap();
        assert!(report.is_drifting);
        assert_eq!(report.direction, "improving");
    }

    #[test]
    fn detect_no_drift() {
        let values = vec![0.5, 0.50, 0.50, 0.50];
        let report = SkillLibrarianAgent::detect_drift(&values, 0.01).unwrap();
        assert!(!report.is_drifting);
    }

    #[tokio::test]
    async fn execute_drift_check() {
        let agent = SkillLibrarianAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("drift_check"))
            .with_param(
                "skill_series",
                serde_json::json!({
                    "Hydrology": [0.5, 0.55, 0.6, 0.65, 0.7],
                    "Fire": [0.4, 0.4, 0.4, 0.4, 0.4],
                }),
            );
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
