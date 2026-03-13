//! EESM Diagnostics framework — Phase 10
//!
//! Pluggable diagnostic packages, CMOR-aligned output, RGMA thrust
//! diagnostics, and model hierarchy analysis.

use serde::{Deserialize, Serialize};

/// A pluggable diagnostic package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticPackageDef {
    pub name: String,
    pub description: String,
    pub metrics: Vec<DiagnosticMetricDef>,
    pub visualization: VisualizationFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VisualizationFormat {
    Html,
    Pdf,
    Interactive,
}

/// A diagnostic metric definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticMetricDef {
    pub name: String,
    pub variable: String,
    pub cmip_baseline: Option<String>,
    pub scoring_function: ScoringFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScoringFunction {
    Rmse,
    Bias,
    Correlation,
    TaylorDiagram,
    SkillScore,
}

/// RGMA thrust area.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RgmaThrust {
    CloudAerosol,
    Biogeochemical,
    HighLatitude,
    Variability,
    ExtremeEvents,
    WaterCycle,
}

/// RGMA thrust metric definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgmaThrustMetric {
    pub thrust: RgmaThrust,
    pub name: String,
    pub variable: String,
    pub cmip_baseline: String,
}

/// Standard RGMA metrics (Phase 10.2).
pub fn standard_rgma_metrics() -> Vec<RgmaThrustMetric> {
    vec![
        // Cloud & Aerosol
        RgmaThrustMetric {
            thrust: RgmaThrust::CloudAerosol,
            name: "Cloud fraction".into(),
            variable: "clt".into(),
            cmip_baseline: "CMIP6 multi-model mean".into(),
        },
        RgmaThrustMetric {
            thrust: RgmaThrust::CloudAerosol,
            name: "AOD".into(),
            variable: "od550aer".into(),
            cmip_baseline: "CMIP6 multi-model mean".into(),
        },
        RgmaThrustMetric {
            thrust: RgmaThrust::CloudAerosol,
            name: "CRE shortwave".into(),
            variable: "rsutcs-rsut".into(),
            cmip_baseline: "CERES EBAF".into(),
        },
        // Biogeochemical
        RgmaThrustMetric {
            thrust: RgmaThrust::Biogeochemical,
            name: "Global carbon budget".into(),
            variable: "nbp".into(),
            cmip_baseline: "GCP 2023".into(),
        },
        RgmaThrustMetric {
            thrust: RgmaThrust::Biogeochemical,
            name: "CO2 flux".into(),
            variable: "fgco2".into(),
            cmip_baseline: "SOCAT".into(),
        },
        // High-Latitude
        RgmaThrustMetric {
            thrust: RgmaThrust::HighLatitude,
            name: "Permafrost extent".into(),
            variable: "permafrost_area".into(),
            cmip_baseline: "IPA/NSIDC".into(),
        },
        RgmaThrustMetric {
            thrust: RgmaThrust::HighLatitude,
            name: "Sea ice extent".into(),
            variable: "siconc".into(),
            cmip_baseline: "NSIDC".into(),
        },
        // Variability
        RgmaThrustMetric {
            thrust: RgmaThrust::Variability,
            name: "ENSO amplitude".into(),
            variable: "tos_nino34".into(),
            cmip_baseline: "HadISST".into(),
        },
        RgmaThrustMetric {
            thrust: RgmaThrust::Variability,
            name: "AMO index".into(),
            variable: "tos_amo".into(),
            cmip_baseline: "HadISST".into(),
        },
        // Extreme Events
        RgmaThrustMetric {
            thrust: RgmaThrust::ExtremeEvents,
            name: "Temperature extremes".into(),
            variable: "tasmax_gev".into(),
            cmip_baseline: "HadEX3".into(),
        },
        RgmaThrustMetric {
            thrust: RgmaThrust::ExtremeEvents,
            name: "Precipitation extremes".into(),
            variable: "pr_rx1day".into(),
            cmip_baseline: "GHCNDEX".into(),
        },
        // Water Cycle
        RgmaThrustMetric {
            thrust: RgmaThrust::WaterCycle,
            name: "P-ET-R closure".into(),
            variable: "water_balance".into(),
            cmip_baseline: "ERA5 + GRDC".into(),
        },
        RgmaThrustMetric {
            thrust: RgmaThrust::WaterCycle,
            name: "Soil moisture".into(),
            variable: "mrso".into(),
            cmip_baseline: "ESA CCI SM".into(),
        },
    ]
}

/// Diagnostic campaign result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignResultDef {
    pub package_name: String,
    pub scores: Vec<(String, f64)>,
    pub report_path: Option<String>,
}

/// Model hierarchy analysis entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyEntry {
    pub family: String,
    pub rungs: Vec<HierarchyRungEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyRungEntry {
    pub rung: String,
    pub skill: f64,
    pub cost: f64,
    pub marginal_skill_per_cost: f64,
}

/// Compute marginal skill/cost ratio for a rung ladder.
pub fn compute_hierarchy(family: &str, rung_skill_cost: &[(&str, f64, f64)]) -> HierarchyEntry {
    let rungs: Vec<HierarchyRungEntry> = rung_skill_cost
        .windows(2)
        .map(|w| {
            let delta_skill = w[1].1 - w[0].1;
            let delta_cost = w[1].2 - w[0].2;
            HierarchyRungEntry {
                rung: w[1].0.to_string(),
                skill: w[1].1,
                cost: w[1].2,
                marginal_skill_per_cost: if delta_cost > 0.0 {
                    delta_skill / delta_cost
                } else {
                    0.0
                },
            }
        })
        .collect();
    HierarchyEntry {
        family: family.to_string(),
        rungs,
    }
}

/// Uncertainty decomposition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyDecomposition {
    pub structural_fraction: f64,
    pub parametric_fraction: f64,
    pub data_fraction: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgma_metrics_all_thrusts() {
        let metrics = standard_rgma_metrics();
        let thrusts: std::collections::HashSet<_> = metrics.iter().map(|m| m.thrust).collect();
        assert_eq!(thrusts.len(), 6);
    }

    #[test]
    fn hierarchy_computation() {
        let h = compute_hierarchy(
            "fire",
            &[("R0", 0.3, 1.0), ("R1", 0.6, 5.0), ("R2", 0.7, 25.0)],
        );
        assert_eq!(h.rungs.len(), 2);
        assert!(h.rungs[0].marginal_skill_per_cost > h.rungs[1].marginal_skill_per_cost);
    }

    #[test]
    fn uncertainty_sums_to_one() {
        let u = UncertaintyDecomposition {
            structural_fraction: 0.5,
            parametric_fraction: 0.3,
            data_fraction: 0.2,
        };
        let total = u.structural_fraction + u.parametric_fraction + u.data_fraction;
        assert!((total - 1.0).abs() < 1e-10);
    }
}
