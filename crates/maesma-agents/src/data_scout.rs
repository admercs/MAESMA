//! Data scout agent — discovers and ingests observational datasets.

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Well-known observational dataset families.
const KNOWN_DATASETS: &[(&str, &str, &str)] = &[
    (
        "FLUXNET2015",
        "ecology",
        "Global eddy-covariance flux tower network",
    ),
    ("MODIS_LAI", "ecology", "MODIS Leaf Area Index product"),
    (
        "ESA_CCI_SM",
        "hydrology",
        "ESA Climate Change Initiative soil moisture",
    ),
    (
        "GRACE_TWS",
        "hydrology",
        "GRACE total water storage anomaly",
    ),
    ("ERA5", "atmosphere", "ECMWF ERA5 reanalysis"),
    (
        "CERES_EBAF",
        "radiation",
        "CERES Energy Balanced and Filled",
    ),
    ("GFED5", "fire", "Global Fire Emissions Database v5"),
    ("ARGO", "ocean", "Argo float ocean profile database"),
    ("NSIDC_SIC", "cryosphere", "NSIDC sea ice concentration"),
    (
        "SOILGRIDS",
        "biogeochemistry",
        "SoilGrids global soil property maps",
    ),
    (
        "GBIF",
        "ecology",
        "Global Biodiversity Information Facility",
    ),
    (
        "SRTM",
        "geomorphology",
        "Shuttle Radar Topography Mission DEM",
    ),
    ("GPM_IMERG", "hydrology", "GPM IMERG precipitation product"),
    (
        "TROPOMI_NO2",
        "atmosphere",
        "Sentinel-5P TROPOMI tropospheric NO\u{2082}",
    ),
    ("ICESat2", "cryosphere", "ICESat-2 ice sheet elevation"),
];

pub struct DataScoutAgent {
    id: AgentId,
}

impl Default for DataScoutAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl DataScoutAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("data_scout".into()),
        }
    }
}

#[async_trait]
impl Agent for DataScoutAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::DataScout
    }
    fn description(&self) -> &str {
        "Discovers and ingests observational datasets for benchmarking"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let family_filter = ctx
            .params
            .get("family")
            .and_then(|v| v.as_str())
            .map(|s| s.to_lowercase());

        let datasets: Vec<serde_json::Value> = KNOWN_DATASETS
            .iter()
            .filter(|(_, fam, _)| {
                family_filter
                    .as_ref()
                    .is_none_or(|f| fam.contains(f.as_str()))
            })
            .map(|(name, family, desc)| {
                serde_json::json!({
                    "name": name, "family": family, "description": desc,
                })
            })
            .collect();

        let n = datasets.len();
        info!(count = n, "Data scout found datasets");

        Ok(
            AgentResult::ok(format!("Found {} observational datasets", n)).with_data(
                serde_json::json!({
                    "datasets": datasets,
                    "total": n,
                }),
            ),
        )
    }
}
