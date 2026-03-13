//! Diagnostics agent — Phase 5.15 & Phase 10
//!
//! ILAMB/IOMB/E3SM Diagnostics/PMP wrappers, multi-component campaigns,
//! RGMA thrust diagnostics (cloud, BGC, high-latitude, variability,
//! extremes, water cycle), standardized reports with CMIP baselines.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Diagnostic package wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub output_format: String,
}

/// RGMA thrust.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RgmaThrust {
    CloudAerosol,
    Biogeochemical,
    HighLatitude,
    Variability,
    ExtremeEvents,
    WaterCycle,
}

/// A diagnostic metric result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticMetric {
    pub name: String,
    pub thrust: RgmaThrust,
    pub value: f64,
    pub cmip_baseline: f64,
    pub unit: String,
    pub passed: bool,
}

/// Campaign result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignResult {
    pub campaign_name: String,
    pub metrics: Vec<DiagnosticMetric>,
    pub overall_score: f64,
    pub passed: bool,
}

/// Standard diagnostic packages.
pub fn standard_packages() -> Vec<DiagnosticPackage> {
    vec![
        DiagnosticPackage {
            name: "ILAMB".into(),
            version: "2.7".into(),
            description: "International Land Model Benchmarking".into(),
            output_format: "html".into(),
        },
        DiagnosticPackage {
            name: "IOMB".into(),
            version: "1.0".into(),
            description: "International Ocean Model Benchmarking".into(),
            output_format: "html".into(),
        },
        DiagnosticPackage {
            name: "E3SM_Diags".into(),
            version: "2.9".into(),
            description: "E3SM diagnostics package".into(),
            output_format: "html".into(),
        },
        DiagnosticPackage {
            name: "PMP".into(),
            version: "3.0".into(),
            description: "PCMDI Metrics Package".into(),
            output_format: "json".into(),
        },
        DiagnosticPackage {
            name: "ESMValTool".into(),
            version: "2.10".into(),
            description: "ESM Evaluation Tool".into(),
            output_format: "html".into(),
        },
    ]
}

/// Standard RGMA thrust metrics.
pub fn rgma_thrust_metrics() -> Vec<DiagnosticMetric> {
    vec![
        DiagnosticMetric {
            name: "cloud_fraction".into(),
            thrust: RgmaThrust::CloudAerosol,
            value: 0.0,
            cmip_baseline: 0.67,
            unit: "fraction".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "aod_550".into(),
            thrust: RgmaThrust::CloudAerosol,
            value: 0.0,
            cmip_baseline: 0.12,
            unit: "dimensionless".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "sw_cre".into(),
            thrust: RgmaThrust::CloudAerosol,
            value: 0.0,
            cmip_baseline: -45.0,
            unit: "W/m2".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "global_c_budget".into(),
            thrust: RgmaThrust::Biogeochemical,
            value: 0.0,
            cmip_baseline: 2.3,
            unit: "PgC/yr".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "co2_flux".into(),
            thrust: RgmaThrust::Biogeochemical,
            value: 0.0,
            cmip_baseline: -3.1,
            unit: "PgC/yr".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "permafrost_area".into(),
            thrust: RgmaThrust::HighLatitude,
            value: 0.0,
            cmip_baseline: 15.0,
            unit: "1e6 km2".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "sea_ice_extent_sep".into(),
            thrust: RgmaThrust::HighLatitude,
            value: 0.0,
            cmip_baseline: 4.5,
            unit: "1e6 km2".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "arctic_amplification".into(),
            thrust: RgmaThrust::HighLatitude,
            value: 0.0,
            cmip_baseline: 2.5,
            unit: "ratio".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "enso_period".into(),
            thrust: RgmaThrust::Variability,
            value: 0.0,
            cmip_baseline: 4.0,
            unit: "years".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "amo_index".into(),
            thrust: RgmaThrust::Variability,
            value: 0.0,
            cmip_baseline: 0.0,
            unit: "K".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "precip_extreme_p99".into(),
            thrust: RgmaThrust::ExtremeEvents,
            value: 0.0,
            cmip_baseline: 45.0,
            unit: "mm/day".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "heat_wave_frequency".into(),
            thrust: RgmaThrust::ExtremeEvents,
            value: 0.0,
            cmip_baseline: 5.0,
            unit: "events/year".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "p_et_r_closure".into(),
            thrust: RgmaThrust::WaterCycle,
            value: 0.0,
            cmip_baseline: 0.0,
            unit: "mm/yr".into(),
            passed: false,
        },
        DiagnosticMetric {
            name: "global_streamflow".into(),
            thrust: RgmaThrust::WaterCycle,
            value: 0.0,
            cmip_baseline: 40000.0,
            unit: "km3/yr".into(),
            passed: false,
        },
    ]
}

pub struct DiagnosticsAgent {
    id: AgentId,
}

impl Default for DiagnosticsAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("diagnostics".into()),
        }
    }
}

#[async_trait]
impl Agent for DiagnosticsAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::Diagnostics
    }
    fn description(&self) -> &str {
        "EESM diagnostics with ILAMB/IOMB/E3SM wrappers, RGMA thrusts, and CMIP baselines"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("check");

        match action {
            "check" => {
                let fields = ctx
                    .params
                    .get("fields")
                    .and_then(|v| v.as_object())
                    .cloned()
                    .unwrap_or_default();
                let mut issues = Vec::new();
                let mut summary = Vec::new();
                for (name, stats) in &fields {
                    let nan_count = stats.get("nan_count").and_then(|v| v.as_u64()).unwrap_or(0);
                    let inf_count = stats.get("inf_count").and_then(|v| v.as_u64()).unwrap_or(0);
                    let min = stats.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let max = stats.get("max").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let mean = stats.get("mean").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    if nan_count > 0 {
                        issues.push(serde_json::json!({ "field": name, "type": "nan", "count": nan_count, "severity": "critical" }));
                    }
                    if inf_count > 0 {
                        issues.push(serde_json::json!({ "field": name, "type": "inf", "count": inf_count, "severity": "critical" }));
                    }
                    if name.contains("temperature") && (min < 100.0 || max > 400.0) {
                        issues.push(serde_json::json!({ "field": name, "type": "out_of_bounds",
                            "detail": format!("T range [{:.1}, {:.1}] K", min, max), "severity": "warning" }));
                    }
                    summary.push(
                        serde_json::json!({ "field": name, "min": min, "max": max, "mean": mean }),
                    );
                }
                let critical = issues
                    .iter()
                    .filter(|i| i.get("severity").and_then(|v| v.as_str()) == Some("critical"))
                    .count();
                info!(
                    fields = fields.len(),
                    issues = issues.len(),
                    critical,
                    "Diagnostics check"
                );
                Ok(AgentResult::ok(format!("Check: {} fields, {} issues ({} critical)", fields.len(), issues.len(), critical))
                    .with_data(serde_json::json!({ "issues": issues, "field_summary": summary, "healthy": critical == 0 })))
            }
            "packages" => {
                let pkgs = standard_packages();
                let data = serde_json::json!({ "packages": pkgs });
                Ok(AgentResult::ok(format!("{} diagnostic packages", pkgs.len())).with_data(data))
            }
            "rgma" => {
                let thrust_filter = ctx.params.get("thrust").and_then(|v| v.as_str());
                let mut metrics = rgma_thrust_metrics();
                if let Some(tf) = thrust_filter {
                    let target = match tf {
                        "cloud" => Some(RgmaThrust::CloudAerosol),
                        "bgc" | "biogeochemical" => Some(RgmaThrust::Biogeochemical),
                        "high_latitude" | "arctic" => Some(RgmaThrust::HighLatitude),
                        "variability" => Some(RgmaThrust::Variability),
                        "extremes" => Some(RgmaThrust::ExtremeEvents),
                        "water" | "water_cycle" => Some(RgmaThrust::WaterCycle),
                        _ => None,
                    };
                    if let Some(t) = target {
                        metrics.retain(|m| m.thrust == t);
                    }
                }
                let data = serde_json::json!({ "rgma_metrics": metrics, "total": metrics.len() });
                Ok(AgentResult::ok(format!("{} RGMA metrics", metrics.len())).with_data(data))
            }
            "campaign" => {
                let name = ctx
                    .params
                    .get("campaign_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");
                let metrics = rgma_thrust_metrics();
                let score = 0.0_f64;
                let result = CampaignResult {
                    campaign_name: name.into(),
                    metrics,
                    overall_score: score,
                    passed: false,
                };
                let data = serde_json::json!({ "campaign": result });
                Ok(AgentResult::ok(format!("Campaign '{}' initialized", name)).with_data(data))
            }
            _ => {
                let data = serde_json::json!({
                    "available_actions": ["check", "packages", "rgma", "campaign"],
                    "rgma_thrusts": ["cloud", "bgc", "high_latitude", "variability", "extremes", "water_cycle"],
                });
                Ok(AgentResult::ok("Diagnostics agent status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_packages_populated() {
        let pkgs = standard_packages();
        assert!(pkgs.len() >= 5);
        assert!(pkgs.iter().any(|p| p.name == "ILAMB"));
    }

    #[test]
    fn rgma_metrics_populated() {
        let metrics = rgma_thrust_metrics();
        assert!(metrics.len() >= 14);
        let thrusts: Vec<_> = metrics.iter().map(|m| &m.thrust).collect();
        assert!(thrusts.contains(&&RgmaThrust::WaterCycle));
    }

    #[tokio::test]
    async fn execute_check_empty() {
        let agent = DiagnosticsAgent::new();
        let ctx = AgentContext::new().with_param("action", serde_json::json!("check"));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn execute_rgma_filter() {
        let agent = DiagnosticsAgent::new();
        let ctx = AgentContext::new()
            .with_param("action", serde_json::json!("rgma"))
            .with_param("thrust", serde_json::json!("cloud"));
        let result = agent.execute(ctx).await.unwrap();
        assert!(result.success);
    }
}
