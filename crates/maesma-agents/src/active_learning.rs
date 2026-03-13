//! Active Learning Agent — Phase 5.8
//!
//! Identifies high-uncertainty configurations, under-observed regimes, and
//! sensitivity frontiers.  Proposes transferability tests and outputs a
//! prioritized experiment queue ranked by expected information gain.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// A proposed experiment for the active learning queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentProposal {
    /// Description of what to test.
    pub description: String,
    /// Expected information gain (higher = more valuable).
    pub expected_gain: f64,
    /// Kind of experiment.
    pub kind: ExperimentKind,
    /// Configuration parameters to test.
    pub config: serde_json::Value,
    /// Estimated cost (wall-time, compute).
    pub estimated_cost: f64,
}

/// Kinds of experiments the active learner can propose.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentKind {
    /// Test a different rung for a specific family.
    RungSwap {
        family: String,
        from: String,
        to: String,
    },
    /// Cross-region validation (transferability test).
    CrossRegion {
        source_region: String,
        target_region: String,
    },
    /// Parameter sensitivity exploration.
    ParameterSweep {
        parameter: String,
        range: (f64, f64),
    },
    /// Test under a specific regime.
    RegimeTest { regime: String },
    /// Fill a data gap.
    DataGap { variable: String, region: String },
}

pub struct ActiveLearningAgent {
    id: AgentId,
}

impl ActiveLearningAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("active_learning".into()),
        }
    }

    /// Compute information gain score for a skill gap.
    fn information_gain(uncertainty: f64, coverage: f64, cost: f64) -> f64 {
        // High uncertainty + low coverage = high gain; penalize by cost
        let raw = uncertainty * (1.0 - coverage);
        if cost > 0.0 { raw / cost.sqrt() } else { raw }
    }
}

impl Default for ActiveLearningAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for ActiveLearningAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn role(&self) -> AgentRole {
        AgentRole::ActiveLearning
    }

    fn description(&self) -> &str {
        "Identifies high-uncertainty configurations and proposes targeted experiments"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let mut proposals = Vec::new();

        // Analyze skill gaps from context
        if let Some(gaps) = ctx.params.get("skill_gaps").and_then(|v| v.as_array()) {
            for gap in gaps {
                let family = gap
                    .get("family")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let uncertainty = gap
                    .get("uncertainty")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5);
                let coverage = gap.get("coverage").and_then(|v| v.as_f64()).unwrap_or(0.5);
                let current_rung = gap.get("rung").and_then(|v| v.as_str()).unwrap_or("R0");

                let gain = Self::information_gain(uncertainty, coverage, 1.0);

                // Propose rung upgrade if uncertainty is high
                if uncertainty > 0.3 {
                    let next_rung = match current_rung {
                        "R0" => "R1",
                        "R1" => "R2",
                        _ => "R3",
                    };
                    proposals.push(ExperimentProposal {
                        description: format!(
                            "Upgrade {} from {} to {}",
                            family, current_rung, next_rung
                        ),
                        expected_gain: gain * 1.2,
                        kind: ExperimentKind::RungSwap {
                            family: family.into(),
                            from: current_rung.into(),
                            to: next_rung.into(),
                        },
                        config: serde_json::json!({"family": family, "rung": next_rung}),
                        estimated_cost: 2.0,
                    });
                }

                // Propose regime test if coverage is low
                if coverage < 0.4 {
                    proposals.push(ExperimentProposal {
                        description: format!("Test {} under under-observed regimes", family),
                        expected_gain: gain,
                        kind: ExperimentKind::RegimeTest {
                            regime: format!("{}_stress", family),
                        },
                        config: serde_json::json!({"family": family, "regime": "stress"}),
                        estimated_cost: 1.5,
                    });
                }
            }
        }

        // Analyze transferability from context
        if let Some(regions) = ctx.params.get("tested_regions").and_then(|v| v.as_array()) {
            let tested: Vec<&str> = regions.iter().filter_map(|v| v.as_str()).collect();
            let all_regions = [
                "CONUS",
                "Europe",
                "Amazon",
                "Sahel",
                "Boreal",
                "Arctic",
                "Australia",
                "Southeast_Asia",
                "Central_Africa",
            ];
            for region in &all_regions {
                if !tested.iter().any(|t| t == region) {
                    if let Some(source) = tested.first() {
                        proposals.push(ExperimentProposal {
                            description: format!("Cross-region transfer: {} → {}", source, region),
                            expected_gain: 0.6,
                            kind: ExperimentKind::CrossRegion {
                                source_region: source.to_string(),
                                target_region: region.to_string(),
                            },
                            config: serde_json::json!({"source": source, "target": region}),
                            estimated_cost: 1.0,
                        });
                    }
                }
            }
        }

        // Sort by expected gain (descending)
        proposals.sort_by(|a, b| {
            b.expected_gain
                .partial_cmp(&a.expected_gain)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let count = proposals.len();
        let top_proposals: Vec<_> = proposals.iter().take(10).collect();

        let data = serde_json::json!({
            "total_proposals": count,
            "top_proposals": top_proposals,
            "queue": proposals,
        });

        Ok(AgentResult::ok(format!(
            "Generated {} experiment proposals ranked by information gain",
            count,
        ))
        .with_data(data)
        .with_next("experiment_orchestrator"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn information_gain_calculation() {
        let gain = ActiveLearningAgent::information_gain(0.8, 0.2, 1.0);
        assert!(gain > 0.5);
        let low_gain = ActiveLearningAgent::information_gain(0.1, 0.9, 1.0);
        assert!(gain > low_gain);
    }

    #[tokio::test]
    async fn execute_with_gaps() {
        let agent = ActiveLearningAgent::new();
        let ctx = AgentContext::new().with_param(
            "skill_gaps",
            serde_json::json!([
                {"family": "Hydrology", "uncertainty": 0.7, "coverage": 0.3, "rung": "R0"},
                {"family": "Fire", "uncertainty": 0.9, "coverage": 0.1, "rung": "R0"},
            ]),
        );
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
        let data = result.data.unwrap();
        assert!(data["total_proposals"].as_u64().unwrap() >= 4);
    }
}
