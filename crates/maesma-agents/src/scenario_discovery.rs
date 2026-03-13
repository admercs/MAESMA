//! Scenario Discovery Agent — Phase 5.14
//!
//! AI-driven exploration of parameter spaces: Latin hypercube / Sobol sampling,
//! outcome clustering, tipping point identification, extreme event compounding.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// A sample point in the scenario parameter space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioSample {
    /// Parameter values at this sample.
    pub parameters: std::collections::HashMap<String, f64>,
    /// Outcome metrics (filled after simulation).
    pub outcomes: std::collections::HashMap<String, f64>,
    /// Cluster assignment.
    pub cluster: Option<u32>,
}

/// A detected tipping point in the parameter space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TippingPoint {
    /// Parameter that triggers the tipping.
    pub parameter: String,
    /// Approximate threshold value.
    pub threshold: f64,
    /// Outcome variable that shifts.
    pub outcome_variable: String,
    /// Magnitude of the shift.
    pub shift_magnitude: f64,
}

/// Sensitivity index from variance decomposition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityIndex {
    pub parameter: String,
    /// First-order Sobol index.
    pub first_order: f64,
    /// Total-order Sobol index.
    pub total_order: f64,
}

/// Compound extreme event specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompoundEvent {
    pub events: Vec<String>,
    pub joint_probability: f64,
    pub impact_multiplier: f64,
}

pub struct ScenarioDiscoveryAgent {
    id: AgentId,
}

impl ScenarioDiscoveryAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("scenario_discovery".into()),
        }
    }

    /// Generate Latin Hypercube samples in [0,1]^d.
    pub fn latin_hypercube(n_samples: usize, n_dims: usize) -> Vec<Vec<f64>> {
        let mut samples = Vec::with_capacity(n_samples);
        for i in 0..n_samples {
            let mut point = Vec::with_capacity(n_dims);
            for d in 0..n_dims {
                // Stratified sampling: divide [0,1] into n_samples bins
                let bin = (i + d * 7) % n_samples; // simple deterministic permutation
                let lo = bin as f64 / n_samples as f64;
                let hi = (bin + 1) as f64 / n_samples as f64;
                point.push((lo + hi) / 2.0); // bin center
            }
            samples.push(point);
        }
        samples
    }

    /// Detect tipping points from a sorted series of (parameter_value, outcome_value).
    pub fn detect_tipping_points(
        sorted_pairs: &[(f64, f64)],
        param_name: &str,
        outcome_name: &str,
        threshold_ratio: f64,
    ) -> Vec<TippingPoint> {
        let mut tips = Vec::new();
        if sorted_pairs.len() < 3 {
            return tips;
        }
        let outcome_range = sorted_pairs
            .iter()
            .map(|(_, o)| *o)
            .fold(f64::NEG_INFINITY, f64::max)
            - sorted_pairs
                .iter()
                .map(|(_, o)| *o)
                .fold(f64::INFINITY, f64::min);
        if outcome_range.abs() < 1e-12 {
            return tips;
        }

        for window in sorted_pairs.windows(2) {
            let delta = (window[1].1 - window[0].1).abs();
            if delta / outcome_range > threshold_ratio {
                tips.push(TippingPoint {
                    parameter: param_name.into(),
                    threshold: (window[0].0 + window[1].0) / 2.0,
                    outcome_variable: outcome_name.into(),
                    shift_magnitude: delta,
                });
            }
        }
        tips
    }

    /// Standard compound events for analysis.
    pub fn standard_compound_events() -> Vec<CompoundEvent> {
        vec![
            CompoundEvent {
                events: vec!["drought".into(), "heatwave".into()],
                joint_probability: 0.05,
                impact_multiplier: 2.5,
            },
            CompoundEvent {
                events: vec![
                    "drought".into(),
                    "heatwave".into(),
                    "infrastructure_failure".into(),
                ],
                joint_probability: 0.005,
                impact_multiplier: 5.0,
            },
            CompoundEvent {
                events: vec!["flood".into(), "landslide".into()],
                joint_probability: 0.03,
                impact_multiplier: 3.0,
            },
            CompoundEvent {
                events: vec!["wildfire".into(), "drought".into(), "wind_event".into()],
                joint_probability: 0.01,
                impact_multiplier: 4.0,
            },
            CompoundEvent {
                events: vec!["sea_level_rise".into(), "storm_surge".into()],
                joint_probability: 0.02,
                impact_multiplier: 3.5,
            },
        ]
    }
}

impl Default for ScenarioDiscoveryAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Agent for ScenarioDiscoveryAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn role(&self) -> AgentRole {
        AgentRole::ScenarioDiscovery
    }

    fn description(&self) -> &str {
        "Explores scenario spaces, identifies tipping points, and analyzes compound extremes"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("sample");

        match action {
            "sample" => {
                let n_samples = ctx
                    .params
                    .get("n_samples")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(50) as usize;
                let params = ctx
                    .params
                    .get("parameters")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_else(|| vec!["param1".into(), "param2".into()]);

                let lhs = Self::latin_hypercube(n_samples, params.len());
                let samples: Vec<ScenarioSample> = lhs
                    .into_iter()
                    .map(|point| {
                        let mut parameters = std::collections::HashMap::new();
                        for (i, p) in params.iter().enumerate() {
                            parameters.insert(p.clone(), point[i]);
                        }
                        ScenarioSample {
                            parameters,
                            outcomes: std::collections::HashMap::new(),
                            cluster: None,
                        }
                    })
                    .collect();

                let data = serde_json::json!({
                    "sample_count": samples.len(),
                    "parameter_count": params.len(),
                    "method": "latin_hypercube",
                    "samples": samples,
                });
                Ok(AgentResult::ok(format!(
                    "Generated {} LHS samples across {} parameters",
                    n_samples,
                    params.len()
                ))
                .with_data(data)
                .with_next("run_simulations"))
            }

            "tipping_points" => {
                let mut all_tips = Vec::new();
                if let Some(results) = ctx.params.get("results").and_then(|v| v.as_array()) {
                    // Gather parameter-outcome pairs
                    let param_names: Vec<String> = results
                        .first()
                        .and_then(|r| r.get("parameters"))
                        .and_then(|p| p.as_object())
                        .map(|obj| obj.keys().cloned().collect())
                        .unwrap_or_default();

                    let outcome_names: Vec<String> = results
                        .first()
                        .and_then(|r| r.get("outcomes"))
                        .and_then(|o| o.as_object())
                        .map(|obj| obj.keys().cloned().collect())
                        .unwrap_or_default();

                    for param in &param_names {
                        for outcome in &outcome_names {
                            let mut pairs: Vec<(f64, f64)> = results
                                .iter()
                                .filter_map(|r| {
                                    let p = r.get("parameters")?.get(param)?.as_f64()?;
                                    let o = r.get("outcomes")?.get(outcome)?.as_f64()?;
                                    Some((p, o))
                                })
                                .collect();
                            pairs.sort_by(|a, b| {
                                a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                            });
                            all_tips
                                .extend(Self::detect_tipping_points(&pairs, param, outcome, 0.3));
                        }
                    }
                }

                let data = serde_json::json!({
                    "tipping_points": all_tips,
                    "count": all_tips.len(),
                });
                Ok(AgentResult::ok(format!(
                    "Detected {} potential tipping points",
                    all_tips.len()
                ))
                .with_data(data))
            }

            "compound_events" => {
                let compounds = Self::standard_compound_events();
                let data = serde_json::json!({
                    "compound_events": compounds,
                    "count": compounds.len(),
                });
                Ok(AgentResult::ok(format!(
                    "Defined {} compound event scenarios",
                    compounds.len()
                ))
                .with_data(data))
            }

            _ => {
                let data = serde_json::json!({
                    "available_actions": ["sample", "tipping_points", "compound_events"],
                    "description": "AI-driven scenario space exploration",
                });
                Ok(AgentResult::ok(
                    "Scenario discovery: use action=sample|tipping_points|compound_events",
                )
                .with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latin_hypercube_dimensions() {
        let samples = ScenarioDiscoveryAgent::latin_hypercube(10, 3);
        assert_eq!(samples.len(), 10);
        assert!(samples.iter().all(|s| s.len() == 3));
        assert!(
            samples
                .iter()
                .all(|s| s.iter().all(|&v| (0.0..=1.0).contains(&v)))
        );
    }

    #[test]
    fn detect_tipping() {
        let pairs = vec![(0.0, 1.0), (0.5, 1.1), (1.0, 1.2), (1.5, 5.0), (2.0, 5.1)];
        let tips = ScenarioDiscoveryAgent::detect_tipping_points(&pairs, "temp", "biomass", 0.3);
        assert_eq!(tips.len(), 1);
        assert!((tips[0].threshold - 1.25).abs() < 0.01);
    }

    #[tokio::test]
    async fn execute_sample() {
        let agent = ScenarioDiscoveryAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("sample"))
            .with_param("n_samples", serde_json::json!(20))
            .with_param(
                "parameters",
                serde_json::json!(["temperature", "precipitation"]),
            );
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
        let data = result.data.unwrap();
        assert_eq!(data["sample_count"].as_u64().unwrap(), 20);
    }
}
