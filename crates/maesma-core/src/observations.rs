//! Observation Registry — dataset manifests, access adapters, and scoring protocols.
//!
//! Every observation product in MAESMA is described by an [`ObservationDataset`]
//! manifest that declares what observable it measures, its spatiotemporal
//! coverage, uncertainty characteristics, and how it should be used for skill
//! scoring.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::families::ProcessFamily;

// ---------------------------------------------------------------------------
// Core identifiers
// ---------------------------------------------------------------------------

/// Unique identifier for an observation dataset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObservationId(pub Uuid);

impl ObservationId {
    /// Create a deterministic ID from a namespace seed string.
    pub fn from_name(name: &str) -> Self {
        Self(Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes()))
    }
}

impl std::fmt::Display for ObservationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Observable — what is being measured
// ---------------------------------------------------------------------------

/// A geophysical or ecological observable quantity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observable {
    /// Canonical name (e.g., `streamflow`, `soil_moisture`, `lai`).
    pub name: String,
    /// SI-compatible unit string (e.g., `m3 s-1`, `m3 m-3`, `m2 m-2`).
    pub unit: String,
    /// Process families this observable is relevant to.
    pub families: Vec<ProcessFamily>,
    /// Human-readable description.
    pub description: String,
}

// ---------------------------------------------------------------------------
// Spatiotemporal coverage
// ---------------------------------------------------------------------------

/// Bounding box in decimal degrees.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub west: f64,
    pub south: f64,
    pub east: f64,
    pub north: f64,
}

impl BoundingBox {
    pub const GLOBAL: Self = Self {
        west: -180.0,
        south: -90.0,
        east: 180.0,
        north: 90.0,
    };

    pub const CONUS: Self = Self {
        west: -125.0,
        south: 24.0,
        east: -66.0,
        north: 50.0,
    };

    /// Check if a point falls within this bounding box.
    pub fn contains(&self, lon: f64, lat: f64) -> bool {
        lon >= self.west && lon <= self.east && lat >= self.south && lat <= self.north
    }

    /// Area in square degrees (approximate).
    pub fn area_deg2(&self) -> f64 {
        (self.east - self.west) * (self.north - self.south)
    }
}

/// Temporal extent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalExtent {
    /// Start year (inclusive).
    pub start_year: i32,
    /// End year (inclusive), or `None` for ongoing / NRT.
    pub end_year: Option<i32>,
}

/// Nominal cadence of observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cadence {
    /// Sub-hourly (e.g., 15-min gauges, flux towers).
    SubHourly,
    /// Hourly.
    Hourly,
    /// Daily.
    Daily,
    /// Weekly (8-day MODIS composites, etc.).
    Weekly,
    /// Monthly.
    Monthly,
    /// Annual.
    Annual,
    /// Irregular / event-based.
    Irregular,
}

/// Spatial topology of the observation network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpatialTopology {
    /// Point stations (gauges, flux towers).
    PointNetwork,
    /// Regularly gridded (satellite grids, reanalysis).
    Gridded,
    /// Swath / along-track (satellite passes).
    Swath,
    /// Polygon / plot (FIA plots, fire perimeters).
    Polygon,
    /// Profile (vertical soundings, ocean floats).
    Profile,
    /// Transect (ship tracks, flight lines).
    Transect,
}

/// Full spatiotemporal coverage descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatiotemporalCoverage {
    /// Spatial extent.
    pub bbox: BoundingBox,
    /// Temporal extent.
    pub temporal: TemporalExtent,
    /// Nominal spatial resolution (meters), if gridded.
    pub spatial_resolution_m: Option<f64>,
    /// Nominal temporal cadence.
    pub cadence: Cadence,
    /// Spatial topology.
    pub topology: SpatialTopology,
    /// Approximate station/site count (for point networks).
    pub station_count: Option<u32>,
}

// ---------------------------------------------------------------------------
// Uncertainty
// ---------------------------------------------------------------------------

/// Uncertainty characterisation for an observation product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintySpec {
    /// Typical absolute uncertainty (same unit as the observable).
    pub absolute: Option<f64>,
    /// Typical relative uncertainty (fraction, 0–1).
    pub relative: Option<f64>,
    /// Known bias description.
    pub known_biases: Vec<String>,
    /// Quality-flag variable name, if any.
    pub quality_flag: Option<String>,
}

// ---------------------------------------------------------------------------
// Access & format
// ---------------------------------------------------------------------------

/// How the dataset can be accessed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMethod {
    /// HTTPS / S3 download with a URL pattern.
    HttpDownload { url_pattern: String },
    /// OPeNDAP endpoint.
    Opendap { endpoint: String },
    /// STAC catalog.
    Stac {
        catalog_url: String,
        collection: String,
    },
    /// NASA Earthdata / CMR.
    NasaCmr { short_name: String, version: String },
    /// API endpoint (REST / GraphQL).
    Api { base_url: String },
    /// Local file path pattern.
    LocalFile { path_pattern: String },
}

/// Data format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataFormat {
    NetCdf,
    Zarr,
    GeoTiff,
    Csv,
    Parquet,
    Hdf5,
    GeoJson,
    Shapefile,
    Grib2,
}

/// Data licence classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum License {
    PublicDomain,
    CcBy4,
    CcBySa4,
    UsGov,
    OpenData,
    RestrictedAcademic,
    Commercial,
    Custom(String),
}

/// Latency classification for NRT considerations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyClass {
    /// Near-real-time (minutes to hours).
    Nrt,
    /// Rapid (1–7 days).
    Rapid,
    /// Standard (weeks to months).
    Standard,
    /// Archive only (no ongoing production).
    Archive,
}

// ---------------------------------------------------------------------------
// Scoring protocol
// ---------------------------------------------------------------------------

/// How this dataset should be used for skill scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringProtocol {
    /// Primary metric name (e.g., `kge`, `rmse`, `crps`).
    pub primary_metric: String,
    /// Extraction method.
    pub extraction: ExtractionMethod,
    /// Minimum time overlap required (years).
    pub min_overlap_years: f64,
    /// Spin-up to discard (years).
    pub spinup_discard_years: f64,
}

/// How simulated values are extracted to compare with observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionMethod {
    /// Point extraction at station coordinates.
    PointExtraction,
    /// Spatial averaging over grid cells.
    SpatialAverage,
    /// Area-weighted aggregation to basin / polygon.
    AreaWeighted,
    /// Profile extraction (vertical).
    ProfileExtraction,
    /// Temporal alignment only (gridded vs gridded).
    TemporalAlignment,
}

// ---------------------------------------------------------------------------
// Observation Dataset — the main manifest
// ---------------------------------------------------------------------------

/// Full manifest for an observation dataset / product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationDataset {
    /// Unique identifier.
    pub id: ObservationId,
    /// Short name (e.g., `USGS_NWIS`, `SMAP_L3`, `MODIS_LAI`).
    pub name: String,
    /// Long descriptive name.
    pub description: String,
    /// Provider / institution.
    pub provider: String,
    /// What this dataset measures.
    pub observable: Observable,
    /// Spatiotemporal coverage.
    pub coverage: SpatiotemporalCoverage,
    /// Uncertainty characterisation.
    pub uncertainty: UncertaintySpec,
    /// Access methods (may have multiple).
    pub access: Vec<AccessMethod>,
    /// Data format(s).
    pub formats: Vec<DataFormat>,
    /// Licence.
    pub license: License,
    /// Latency classification.
    pub latency: LatencyClass,
    /// Scoring protocol for skill evaluation.
    pub scoring: ScoringProtocol,
    /// Process families this dataset can evaluate.
    pub evaluates_families: Vec<ProcessFamily>,
    /// Tags for regime / region filtering.
    pub tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// Observation Registry — in-memory index
// ---------------------------------------------------------------------------

/// In-memory observation registry backed by a `Vec<ObservationDataset>`.
#[derive(Debug, Default)]
pub struct ObservationRegistry {
    datasets: Vec<ObservationDataset>,
}

impl ObservationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a dataset.
    pub fn register(&mut self, ds: ObservationDataset) {
        self.datasets.push(ds);
    }

    /// Register many datasets at once.
    pub fn register_all(&mut self, ds: Vec<ObservationDataset>) {
        self.datasets.extend(ds);
    }

    /// Total count.
    pub fn len(&self) -> usize {
        self.datasets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.datasets.is_empty()
    }

    /// Get by ID.
    pub fn get(&self, id: ObservationId) -> Option<&ObservationDataset> {
        self.datasets.iter().find(|d| d.id == id)
    }

    /// Get by name.
    pub fn by_name(&self, name: &str) -> Option<&ObservationDataset> {
        self.datasets.iter().find(|d| d.name == name)
    }

    /// Filter by process family.
    pub fn for_family(&self, family: ProcessFamily) -> Vec<&ObservationDataset> {
        self.datasets
            .iter()
            .filter(|d| d.evaluates_families.contains(&family))
            .collect()
    }

    /// Filter by observable name.
    pub fn for_observable(&self, obs_name: &str) -> Vec<&ObservationDataset> {
        self.datasets
            .iter()
            .filter(|d| d.observable.name == obs_name)
            .collect()
    }

    /// Filter by spatial containment.
    pub fn covering(&self, lon: f64, lat: f64) -> Vec<&ObservationDataset> {
        self.datasets
            .iter()
            .filter(|d| d.coverage.bbox.contains(lon, lat))
            .collect()
    }

    /// Filter by latency class.
    pub fn by_latency(&self, latency: LatencyClass) -> Vec<&ObservationDataset> {
        self.datasets
            .iter()
            .filter(|d| d.latency == latency)
            .collect()
    }

    /// Filter by tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&ObservationDataset> {
        self.datasets
            .iter()
            .filter(|d| d.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// All datasets as a slice.
    pub fn all(&self) -> &[ObservationDataset] {
        &self.datasets
    }

    /// Summary statistics.
    pub fn summary(&self) -> RegistrySummary {
        let mut families = std::collections::HashSet::new();
        let mut observables = std::collections::HashSet::new();
        let mut providers = std::collections::HashSet::new();
        for ds in &self.datasets {
            for f in &ds.evaluates_families {
                families.insert(*f);
            }
            observables.insert(ds.observable.name.clone());
            providers.insert(ds.provider.clone());
        }
        RegistrySummary {
            total_datasets: self.datasets.len(),
            unique_observables: observables.len(),
            unique_providers: providers.len(),
            families_covered: families.len(),
        }
    }
}

/// Summary of registry contents.
#[derive(Debug, Clone)]
pub struct RegistrySummary {
    pub total_datasets: usize,
    pub unique_observables: usize,
    pub unique_providers: usize,
    pub families_covered: usize,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::families::ProcessFamily;

    fn sample_dataset(name: &str, family: ProcessFamily) -> ObservationDataset {
        ObservationDataset {
            id: ObservationId::from_name(name),
            name: name.to_string(),
            description: format!("Test dataset {name}"),
            provider: "Test".into(),
            observable: Observable {
                name: "test_var".into(),
                unit: "m".into(),
                families: vec![family],
                description: "test".into(),
            },
            coverage: SpatiotemporalCoverage {
                bbox: BoundingBox::GLOBAL,
                temporal: TemporalExtent {
                    start_year: 2000,
                    end_year: Some(2023),
                },
                spatial_resolution_m: Some(1000.0),
                cadence: Cadence::Daily,
                topology: SpatialTopology::Gridded,
                station_count: None,
            },
            uncertainty: UncertaintySpec {
                absolute: Some(0.1),
                relative: None,
                known_biases: vec![],
                quality_flag: None,
            },
            access: vec![],
            formats: vec![DataFormat::NetCdf],
            license: License::PublicDomain,
            latency: LatencyClass::Standard,
            scoring: ScoringProtocol {
                primary_metric: "rmse".into(),
                extraction: ExtractionMethod::PointExtraction,
                min_overlap_years: 2.0,
                spinup_discard_years: 1.0,
            },
            evaluates_families: vec![family],
            tags: vec!["test".into()],
        }
    }

    #[test]
    fn test_registry_basic_ops() {
        let mut reg = ObservationRegistry::new();
        assert!(reg.is_empty());

        let ds = sample_dataset("TEST_DS", ProcessFamily::Hydrology);
        let id = ds.id;
        reg.register(ds);

        assert_eq!(reg.len(), 1);
        assert!(reg.get(id).is_some());
        assert!(reg.by_name("TEST_DS").is_some());
    }

    #[test]
    fn test_registry_family_filter() {
        let mut reg = ObservationRegistry::new();
        reg.register(sample_dataset("HYDRO1", ProcessFamily::Hydrology));
        reg.register(sample_dataset("FIRE1", ProcessFamily::Fire));
        reg.register(sample_dataset("HYDRO2", ProcessFamily::Hydrology));

        assert_eq!(reg.for_family(ProcessFamily::Hydrology).len(), 2);
        assert_eq!(reg.for_family(ProcessFamily::Fire).len(), 1);
        assert_eq!(reg.for_family(ProcessFamily::Ocean).len(), 0);
    }

    #[test]
    fn test_deterministic_ids() {
        let id1 = ObservationId::from_name("USGS_NWIS");
        let id2 = ObservationId::from_name("USGS_NWIS");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_bounding_box() {
        let conus = BoundingBox::CONUS;
        assert!(conus.contains(-100.0, 40.0));
        assert!(!conus.contains(10.0, 50.0));
    }
}
