//! Comparison protocols — standardized rules for comparing simulated output
//! against observations, per observation type and process family.

use serde::{Deserialize, Serialize};

use crate::families::ProcessFamily;
use crate::observations::{Cadence, ExtractionMethod, SpatialTopology};

/// A comparison protocol defining how to evaluate a process family against
/// a specific observation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonProtocol {
    /// Protocol name (e.g., "streamflow_daily_kge").
    pub name: String,
    /// Target process family.
    pub family: ProcessFamily,
    /// Observable name this protocol evaluates.
    pub observable: String,
    /// Spatial topology of the observation (determines extraction method).
    pub topology: SpatialTopology,
    /// Required temporal cadence for comparison.
    pub cadence: Cadence,
    /// How to extract simulated values.
    pub extraction: ExtractionMethod,
    /// Temporal aggregation before comparison.
    pub temporal_aggregation: TemporalAggregation,
    /// Primary metric for this comparison.
    pub primary_metric: String,
    /// Secondary metrics to also compute.
    pub secondary_metrics: Vec<String>,
    /// Pass/fail thresholds.
    pub thresholds: MetricThresholds,
    /// Minimum data overlap required (years).
    pub min_overlap_years: f64,
    /// Spin-up to discard (years).
    pub spinup_years: f64,
    /// Whether to compute per-site/cell metrics and aggregate.
    pub per_site_aggregation: bool,
}

/// Temporal aggregation strategy before computing metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemporalAggregation {
    /// Compare at native resolution (no aggregation).
    None,
    /// Aggregate to daily means.
    Daily,
    /// Aggregate to monthly means.
    Monthly,
    /// Aggregate to annual means.
    Annual,
    /// Aggregate to seasonal means (DJF, MAM, JJA, SON).
    Seasonal,
    /// Compute climatological means (multi-year monthly means).
    Climatology,
}

/// Pass/fail metric thresholds for automated validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricThresholds {
    /// KGE threshold (higher better; typical pass: > 0.5).
    pub kge_pass: Option<f64>,
    /// NSE threshold (higher better; typical pass: > 0.5).
    pub nse_pass: Option<f64>,
    /// RMSE threshold (lower better; absolute, dataset-specific).
    pub rmse_max: Option<f64>,
    /// Bias ratio acceptable range (e.g., 0.8–1.2).
    pub bias_range: Option<(f64, f64)>,
    /// Correlation minimum (e.g., > 0.7).
    pub correlation_min: Option<f64>,
    /// Conservation residual maximum.
    pub conservation_max: Option<f64>,
}

impl MetricThresholds {
    pub fn default_strict() -> Self {
        Self {
            kge_pass: Some(0.7),
            nse_pass: Some(0.7),
            rmse_max: None,
            bias_range: Some((0.9, 1.1)),
            correlation_min: Some(0.8),
            conservation_max: Some(1e-6),
        }
    }

    pub fn default_lenient() -> Self {
        Self {
            kge_pass: Some(0.3),
            nse_pass: Some(0.3),
            rmse_max: None,
            bias_range: Some((0.5, 2.0)),
            correlation_min: Some(0.5),
            conservation_max: Some(1e-3),
        }
    }
}

/// Registry of comparison protocols.
#[derive(Debug, Default)]
pub struct ProtocolRegistry {
    protocols: Vec<ComparisonProtocol>,
}

impl ProtocolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load default protocols for standard observation types.
    pub fn with_defaults() -> Self {
        let mut reg = Self::new();

        // Streamflow — daily KGE at gauge locations.
        reg.register(ComparisonProtocol {
            name: "streamflow_daily_kge".into(),
            family: ProcessFamily::Hydrology,
            observable: "streamflow".into(),
            topology: SpatialTopology::PointNetwork,
            cadence: Cadence::Daily,
            extraction: ExtractionMethod::PointExtraction,
            temporal_aggregation: TemporalAggregation::None,
            primary_metric: "kge".into(),
            secondary_metrics: vec!["nse".into(), "bias".into(), "correlation".into()],
            thresholds: MetricThresholds {
                kge_pass: Some(0.5),
                nse_pass: Some(0.5),
                rmse_max: None,
                bias_range: Some((0.8, 1.2)),
                correlation_min: Some(0.7),
                conservation_max: None,
            },
            min_overlap_years: 5.0,
            spinup_years: 1.0,
            per_site_aggregation: true,
        });

        // Soil moisture — daily RMSE from SMAP gridded.
        reg.register(ComparisonProtocol {
            name: "soil_moisture_daily_rmse".into(),
            family: ProcessFamily::Hydrology,
            observable: "soil_moisture".into(),
            topology: SpatialTopology::Gridded,
            cadence: Cadence::Daily,
            extraction: ExtractionMethod::SpatialAverage,
            temporal_aggregation: TemporalAggregation::None,
            primary_metric: "rmse".into(),
            secondary_metrics: vec!["correlation".into(), "bias".into()],
            thresholds: MetricThresholds {
                kge_pass: None,
                nse_pass: None,
                rmse_max: Some(0.05), // m3/m3
                bias_range: Some((0.8, 1.2)),
                correlation_min: Some(0.6),
                conservation_max: None,
            },
            min_overlap_years: 3.0,
            spinup_years: 0.5,
            per_site_aggregation: false,
        });

        // LAI — 8-day MODIS composite correlation.
        reg.register(ComparisonProtocol {
            name: "lai_8day_correlation".into(),
            family: ProcessFamily::Ecology,
            observable: "lai".into(),
            topology: SpatialTopology::Gridded,
            cadence: Cadence::Weekly,
            extraction: ExtractionMethod::SpatialAverage,
            temporal_aggregation: TemporalAggregation::Monthly,
            primary_metric: "correlation".into(),
            secondary_metrics: vec!["rmse".into(), "bias".into()],
            thresholds: MetricThresholds {
                kge_pass: None,
                nse_pass: None,
                rmse_max: Some(1.0), // m2/m2
                bias_range: Some((0.7, 1.3)),
                correlation_min: Some(0.6),
                conservation_max: None,
            },
            min_overlap_years: 5.0,
            spinup_years: 2.0,
            per_site_aggregation: false,
        });

        // Flux tower energy/carbon — half-hourly KGE.
        reg.register(ComparisonProtocol {
            name: "fluxnet_nee_kge".into(),
            family: ProcessFamily::Biogeochemistry,
            observable: "nee".into(),
            topology: SpatialTopology::PointNetwork,
            cadence: Cadence::SubHourly,
            extraction: ExtractionMethod::PointExtraction,
            temporal_aggregation: TemporalAggregation::Daily,
            primary_metric: "kge".into(),
            secondary_metrics: vec!["rmse".into(), "correlation".into(), "bias".into()],
            thresholds: MetricThresholds {
                kge_pass: Some(0.4),
                nse_pass: None,
                rmse_max: None,
                bias_range: Some((0.7, 1.3)),
                correlation_min: Some(0.5),
                conservation_max: None,
            },
            min_overlap_years: 2.0,
            spinup_years: 0.5,
            per_site_aggregation: true,
        });

        // Burn severity — MTBS/RAVG spatial RMSE.
        reg.register(ComparisonProtocol {
            name: "burn_severity_rmse".into(),
            family: ProcessFamily::Fire,
            observable: "burn_severity".into(),
            topology: SpatialTopology::Polygon,
            cadence: Cadence::Irregular,
            extraction: ExtractionMethod::AreaWeighted,
            temporal_aggregation: TemporalAggregation::None,
            primary_metric: "rmse".into(),
            secondary_metrics: vec!["correlation".into()],
            thresholds: MetricThresholds {
                kge_pass: None,
                nse_pass: None,
                rmse_max: Some(0.3), // severity class
                bias_range: None,
                correlation_min: Some(0.5),
                conservation_max: None,
            },
            min_overlap_years: 5.0,
            spinup_years: 0.0,
            per_site_aggregation: false,
        });

        // Snow water equivalent — daily RMSE.
        reg.register(ComparisonProtocol {
            name: "swe_daily_rmse".into(),
            family: ProcessFamily::Cryosphere,
            observable: "swe".into(),
            topology: SpatialTopology::PointNetwork,
            cadence: Cadence::Daily,
            extraction: ExtractionMethod::PointExtraction,
            temporal_aggregation: TemporalAggregation::None,
            primary_metric: "rmse".into(),
            secondary_metrics: vec!["bias".into(), "correlation".into()],
            thresholds: MetricThresholds {
                kge_pass: None,
                nse_pass: Some(0.5),
                rmse_max: Some(50.0), // mm
                bias_range: Some((0.8, 1.2)),
                correlation_min: Some(0.7),
                conservation_max: None,
            },
            min_overlap_years: 5.0,
            spinup_years: 1.0,
            per_site_aggregation: true,
        });

        // TOA radiation — monthly RMSE.
        reg.register(ComparisonProtocol {
            name: "toa_radiation_monthly".into(),
            family: ProcessFamily::Radiation,
            observable: "toa_net_radiation".into(),
            topology: SpatialTopology::Gridded,
            cadence: Cadence::Monthly,
            extraction: ExtractionMethod::SpatialAverage,
            temporal_aggregation: TemporalAggregation::Climatology,
            primary_metric: "rmse".into(),
            secondary_metrics: vec!["bias".into(), "correlation".into()],
            thresholds: MetricThresholds {
                kge_pass: None,
                nse_pass: None,
                rmse_max: Some(5.0), // W/m2
                bias_range: Some((0.95, 1.05)),
                correlation_min: Some(0.9),
                conservation_max: Some(1.0), // W/m2
            },
            min_overlap_years: 10.0,
            spinup_years: 2.0,
            per_site_aggregation: false,
        });

        // SST — monthly ocean.
        reg.register(ComparisonProtocol {
            name: "sst_monthly_rmse".into(),
            family: ProcessFamily::Ocean,
            observable: "sst".into(),
            topology: SpatialTopology::Gridded,
            cadence: Cadence::Monthly,
            extraction: ExtractionMethod::SpatialAverage,
            temporal_aggregation: TemporalAggregation::Monthly,
            primary_metric: "rmse".into(),
            secondary_metrics: vec!["bias".into(), "correlation".into()],
            thresholds: MetricThresholds {
                kge_pass: None,
                nse_pass: None,
                rmse_max: Some(1.0), // °C
                bias_range: Some((0.95, 1.05)),
                correlation_min: Some(0.9),
                conservation_max: None,
            },
            min_overlap_years: 10.0,
            spinup_years: 5.0,
            per_site_aggregation: false,
        });

        reg
    }

    /// Register a protocol.
    pub fn register(&mut self, protocol: ComparisonProtocol) {
        self.protocols.push(protocol);
    }

    /// Get protocols for a process family.
    pub fn for_family(&self, family: ProcessFamily) -> Vec<&ComparisonProtocol> {
        self.protocols
            .iter()
            .filter(|p| p.family == family)
            .collect()
    }

    /// Get protocols for an observable.
    pub fn for_observable(&self, obs: &str) -> Vec<&ComparisonProtocol> {
        self.protocols
            .iter()
            .filter(|p| p.observable == obs)
            .collect()
    }

    /// Get protocol by name.
    pub fn by_name(&self, name: &str) -> Option<&ComparisonProtocol> {
        self.protocols.iter().find(|p| p.name == name)
    }

    /// All protocols.
    pub fn all(&self) -> &[ComparisonProtocol] {
        &self.protocols
    }

    /// Count.
    pub fn len(&self) -> usize {
        self.protocols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.protocols.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_protocols_loaded() {
        let reg = ProtocolRegistry::with_defaults();
        assert_eq!(reg.len(), 8);
    }

    #[test]
    fn filter_by_family() {
        let reg = ProtocolRegistry::with_defaults();
        let hydro = reg.for_family(ProcessFamily::Hydrology);
        assert_eq!(hydro.len(), 2); // streamflow + soil_moisture
    }

    #[test]
    fn filter_by_observable() {
        let reg = ProtocolRegistry::with_defaults();
        let sm = reg.for_observable("soil_moisture");
        assert_eq!(sm.len(), 1);
        assert_eq!(sm[0].primary_metric, "rmse");
    }

    #[test]
    fn lookup_by_name() {
        let reg = ProtocolRegistry::with_defaults();
        let p = reg.by_name("streamflow_daily_kge").unwrap();
        assert_eq!(p.primary_metric, "kge");
        assert_eq!(p.extraction, ExtractionMethod::PointExtraction);
    }

    #[test]
    fn strict_thresholds_stricter_than_lenient() {
        let strict = MetricThresholds::default_strict();
        let lenient = MetricThresholds::default_lenient();
        assert!(strict.kge_pass.unwrap() > lenient.kge_pass.unwrap());
        assert!(strict.correlation_min.unwrap() > lenient.correlation_min.unwrap());
    }
}
