//! Inquiry-driven selection — automatic dataset and fidelity selection
//! based on user questions.
//!
//! When a user poses a scientific question (e.g., "How will wildfire risk
//! change in the Western US under 2°C warming?"), the inquiry module
//! determines which process families are relevant, what fidelity rung each
//! family should run at, and which observation datasets are needed for
//! validation.

use serde::{Deserialize, Serialize};

use crate::families::ProcessFamily;
use crate::process::FidelityRung;

// ---------------------------------------------------------------------------
// Inquiry input
// ---------------------------------------------------------------------------

/// A user inquiry that drives automatic dataset and fidelity selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inquiry {
    /// The natural-language question or objective.
    pub question: String,
    /// Optional spatial constraint (region name or bounding box description).
    pub region: Option<String>,
    /// Optional temporal scope (e.g., "2000-2020", "historical", "future").
    pub temporal_scope: Option<String>,
    /// Computational budget constraint.
    pub budget: BudgetConstraint,
}

/// Computational budget constraint for inquiry-driven selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetConstraint {
    /// Quick screening — prefer R0/R1.
    Low,
    /// Standard research — R1/R2.
    Medium,
    /// High-fidelity production — R2/R3.
    High,
    /// No constraint — use best available.
    Unlimited,
}

impl BudgetConstraint {
    /// Maximum fidelity rung permitted by this budget.
    pub fn max_rung(&self) -> FidelityRung {
        match self {
            Self::Low => FidelityRung::R1,
            Self::Medium => FidelityRung::R2,
            Self::High => FidelityRung::R3,
            Self::Unlimited => FidelityRung::R3,
        }
    }

    /// Preferred default rung for families that are not the primary focus.
    pub fn default_rung(&self) -> FidelityRung {
        match self {
            Self::Low => FidelityRung::R0,
            Self::Medium => FidelityRung::R1,
            Self::High => FidelityRung::R2,
            Self::Unlimited => FidelityRung::R2,
        }
    }
}

impl Default for BudgetConstraint {
    fn default() -> Self {
        Self::Medium
    }
}

// ---------------------------------------------------------------------------
// Inquiry plan output
// ---------------------------------------------------------------------------

/// The output plan: recommended datasets, fidelities, and manifests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InquiryPlan {
    /// Original inquiry.
    pub inquiry: Inquiry,
    /// Identified primary process families (directly relevant to the question).
    pub primary_families: Vec<ProcessFamily>,
    /// Supporting process families (needed for coupling / boundary conditions).
    pub supporting_families: Vec<ProcessFamily>,
    /// Per-family fidelity recommendation.
    pub fidelity_map: Vec<FidelityRecommendation>,
    /// Recommended observation datasets for validation.
    pub datasets: Vec<DatasetRecommendation>,
    /// Matching knowledgebase manifests.
    pub manifests: Vec<ManifestMatch>,
    /// Human-readable rationale.
    pub rationale: String,
    /// Confidence in the plan (0–1).
    pub confidence: f64,
}

/// Per-family fidelity recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidelityRecommendation {
    /// Process family.
    pub family: ProcessFamily,
    /// Recommended fidelity rung.
    pub recommended_rung: FidelityRung,
    /// Whether this is a primary or supporting family.
    pub is_primary: bool,
    /// Reason for this recommendation.
    pub reason: String,
}

/// A recommended observation dataset for validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetRecommendation {
    /// Dataset short name.
    pub dataset_name: String,
    /// What observable it measures.
    pub observable: String,
    /// Relevance score (0–1).
    pub relevance_score: f64,
    /// Why this dataset is recommended.
    pub reason: String,
}

/// A matching manifest from the knowledgebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMatch {
    /// Manifest identifier.
    pub manifest_id: String,
    /// Process name.
    pub name: String,
    /// Process family.
    pub family: ProcessFamily,
    /// Fidelity rung.
    pub rung: FidelityRung,
    /// Relevance score (0–1).
    pub relevance_score: f64,
}

// ---------------------------------------------------------------------------
// Inquiry analysis helpers
// ---------------------------------------------------------------------------

/// Keyword-to-family mapping used by the inquiry analyzer.
pub struct FamilyKeyword {
    pub keyword: &'static str,
    pub family: ProcessFamily,
    /// If true, this keyword always implies the family is primary.
    pub strong: bool,
}

/// Returns the canonical set of keyword → ProcessFamily mappings.
pub fn family_keywords() -> Vec<FamilyKeyword> {
    vec![
        // Fire
        FamilyKeyword {
            keyword: "fire",
            family: ProcessFamily::Fire,
            strong: true,
        },
        FamilyKeyword {
            keyword: "wildfire",
            family: ProcessFamily::Fire,
            strong: true,
        },
        FamilyKeyword {
            keyword: "burn",
            family: ProcessFamily::Fire,
            strong: true,
        },
        FamilyKeyword {
            keyword: "combustion",
            family: ProcessFamily::Fire,
            strong: true,
        },
        FamilyKeyword {
            keyword: "ignition",
            family: ProcessFamily::Fire,
            strong: true,
        },
        FamilyKeyword {
            keyword: "ember",
            family: ProcessFamily::Fire,
            strong: true,
        },
        // Hydrology
        FamilyKeyword {
            keyword: "streamflow",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "discharge",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "runoff",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "flood",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "drought",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "groundwater",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "soil moisture",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "infiltration",
            family: ProcessFamily::Hydrology,
            strong: false,
        },
        FamilyKeyword {
            keyword: "evapotranspiration",
            family: ProcessFamily::Hydrology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "water",
            family: ProcessFamily::Hydrology,
            strong: false,
        },
        // Ecology
        FamilyKeyword {
            keyword: "vegetation",
            family: ProcessFamily::Ecology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "lai",
            family: ProcessFamily::Ecology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "ndvi",
            family: ProcessFamily::Ecology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "phenology",
            family: ProcessFamily::Ecology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "succession",
            family: ProcessFamily::Ecology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "species",
            family: ProcessFamily::Ecology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "biodiversity",
            family: ProcessFamily::Ecology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "forest",
            family: ProcessFamily::Ecology,
            strong: false,
        },
        FamilyKeyword {
            keyword: "plant",
            family: ProcessFamily::Ecology,
            strong: false,
        },
        // Biogeochemistry
        FamilyKeyword {
            keyword: "carbon",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        FamilyKeyword {
            keyword: "nitrogen",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        FamilyKeyword {
            keyword: "phosphorus",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        FamilyKeyword {
            keyword: "nee",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        FamilyKeyword {
            keyword: "gpp",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        FamilyKeyword {
            keyword: "decomposition",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        FamilyKeyword {
            keyword: "nutrient",
            family: ProcessFamily::Biogeochemistry,
            strong: false,
        },
        FamilyKeyword {
            keyword: "methane",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        FamilyKeyword {
            keyword: "co2",
            family: ProcessFamily::Biogeochemistry,
            strong: true,
        },
        // Radiation
        FamilyKeyword {
            keyword: "radiation",
            family: ProcessFamily::Radiation,
            strong: true,
        },
        FamilyKeyword {
            keyword: "albedo",
            family: ProcessFamily::Radiation,
            strong: true,
        },
        FamilyKeyword {
            keyword: "shortwave",
            family: ProcessFamily::Radiation,
            strong: true,
        },
        FamilyKeyword {
            keyword: "longwave",
            family: ProcessFamily::Radiation,
            strong: true,
        },
        FamilyKeyword {
            keyword: "solar",
            family: ProcessFamily::Radiation,
            strong: false,
        },
        // Atmosphere
        FamilyKeyword {
            keyword: "temperature",
            family: ProcessFamily::Atmosphere,
            strong: false,
        },
        FamilyKeyword {
            keyword: "precipitation",
            family: ProcessFamily::Atmosphere,
            strong: false,
        },
        FamilyKeyword {
            keyword: "wind",
            family: ProcessFamily::Atmosphere,
            strong: false,
        },
        FamilyKeyword {
            keyword: "weather",
            family: ProcessFamily::Atmosphere,
            strong: true,
        },
        FamilyKeyword {
            keyword: "climate",
            family: ProcessFamily::Atmosphere,
            strong: false,
        },
        FamilyKeyword {
            keyword: "convection",
            family: ProcessFamily::Atmosphere,
            strong: true,
        },
        FamilyKeyword {
            keyword: "aerosol",
            family: ProcessFamily::Atmosphere,
            strong: true,
        },
        // Ocean
        FamilyKeyword {
            keyword: "ocean",
            family: ProcessFamily::Ocean,
            strong: true,
        },
        FamilyKeyword {
            keyword: "sea surface",
            family: ProcessFamily::Ocean,
            strong: true,
        },
        FamilyKeyword {
            keyword: "marine",
            family: ProcessFamily::Ocean,
            strong: true,
        },
        FamilyKeyword {
            keyword: "wave",
            family: ProcessFamily::Ocean,
            strong: false,
        },
        FamilyKeyword {
            keyword: "salinity",
            family: ProcessFamily::Ocean,
            strong: true,
        },
        FamilyKeyword {
            keyword: "thermohaline",
            family: ProcessFamily::Ocean,
            strong: true,
        },
        // Cryosphere
        FamilyKeyword {
            keyword: "snow",
            family: ProcessFamily::Cryosphere,
            strong: true,
        },
        FamilyKeyword {
            keyword: "ice",
            family: ProcessFamily::Cryosphere,
            strong: true,
        },
        FamilyKeyword {
            keyword: "glacier",
            family: ProcessFamily::Cryosphere,
            strong: true,
        },
        FamilyKeyword {
            keyword: "permafrost",
            family: ProcessFamily::Cryosphere,
            strong: true,
        },
        FamilyKeyword {
            keyword: "swe",
            family: ProcessFamily::Cryosphere,
            strong: true,
        },
        FamilyKeyword {
            keyword: "sea ice",
            family: ProcessFamily::Cryosphere,
            strong: true,
        },
        // Geomorphology
        FamilyKeyword {
            keyword: "erosion",
            family: ProcessFamily::Geomorphology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "sediment",
            family: ProcessFamily::Geomorphology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "landscape",
            family: ProcessFamily::Geomorphology,
            strong: false,
        },
        FamilyKeyword {
            keyword: "landslide",
            family: ProcessFamily::Geomorphology,
            strong: true,
        },
        // Geology
        FamilyKeyword {
            keyword: "subsurface",
            family: ProcessFamily::Geology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "tectonic",
            family: ProcessFamily::Geology,
            strong: true,
        },
        FamilyKeyword {
            keyword: "reactive transport",
            family: ProcessFamily::Geology,
            strong: true,
        },
        // Human systems
        FamilyKeyword {
            keyword: "land use",
            family: ProcessFamily::HumanSystems,
            strong: true,
        },
        FamilyKeyword {
            keyword: "agriculture",
            family: ProcessFamily::HumanSystems,
            strong: true,
        },
        FamilyKeyword {
            keyword: "crop",
            family: ProcessFamily::HumanSystems,
            strong: true,
        },
        FamilyKeyword {
            keyword: "urban",
            family: ProcessFamily::HumanSystems,
            strong: true,
        },
        FamilyKeyword {
            keyword: "infrastructure",
            family: ProcessFamily::HumanSystems,
            strong: true,
        },
        FamilyKeyword {
            keyword: "irrigation",
            family: ProcessFamily::HumanSystems,
            strong: true,
        },
        // Trophic dynamics
        FamilyKeyword {
            keyword: "food web",
            family: ProcessFamily::TrophicDynamics,
            strong: true,
        },
        FamilyKeyword {
            keyword: "predator",
            family: ProcessFamily::TrophicDynamics,
            strong: true,
        },
        FamilyKeyword {
            keyword: "prey",
            family: ProcessFamily::TrophicDynamics,
            strong: true,
        },
        FamilyKeyword {
            keyword: "trophic",
            family: ProcessFamily::TrophicDynamics,
            strong: true,
        },
        FamilyKeyword {
            keyword: "herbivore",
            family: ProcessFamily::TrophicDynamics,
            strong: true,
        },
        // Evolution
        FamilyKeyword {
            keyword: "evolution",
            family: ProcessFamily::Evolution,
            strong: true,
        },
        FamilyKeyword {
            keyword: "speciation",
            family: ProcessFamily::Evolution,
            strong: true,
        },
        FamilyKeyword {
            keyword: "gene flow",
            family: ProcessFamily::Evolution,
            strong: true,
        },
        FamilyKeyword {
            keyword: "phylo",
            family: ProcessFamily::Evolution,
            strong: true,
        },
    ]
}

/// Well-known coupling dependencies between families.
///
/// If a primary family is selected, its coupling dependencies become
/// supporting families (unless they are already primary).
pub fn coupling_dependencies() -> Vec<(ProcessFamily, ProcessFamily)> {
    vec![
        // Fire needs vegetation fuel load (Ecology) and weather (Atmosphere)
        (ProcessFamily::Fire, ProcessFamily::Ecology),
        (ProcessFamily::Fire, ProcessFamily::Atmosphere),
        // Hydrology needs radiation for ET and atmosphere for precipitation
        (ProcessFamily::Hydrology, ProcessFamily::Radiation),
        (ProcessFamily::Hydrology, ProcessFamily::Atmosphere),
        // Ecology needs hydrology (water supply) and biogeochemistry (nutrients)
        (ProcessFamily::Ecology, ProcessFamily::Hydrology),
        (ProcessFamily::Ecology, ProcessFamily::Biogeochemistry),
        // Biogeochemistry needs ecology (litter input) and hydrology (transport)
        (ProcessFamily::Biogeochemistry, ProcessFamily::Ecology),
        (ProcessFamily::Biogeochemistry, ProcessFamily::Hydrology),
        // Cryosphere needs radiation (energy balance) and atmosphere (forcing)
        (ProcessFamily::Cryosphere, ProcessFamily::Radiation),
        (ProcessFamily::Cryosphere, ProcessFamily::Atmosphere),
        // Geomorphology needs hydrology (overland flow) and ecology (vegetation cover)
        (ProcessFamily::Geomorphology, ProcessFamily::Hydrology),
        (ProcessFamily::Geomorphology, ProcessFamily::Ecology),
        // Ocean needs atmosphere (air-sea fluxes)
        (ProcessFamily::Ocean, ProcessFamily::Atmosphere),
    ]
}

/// Keyword-to-dataset mapping for common observational products.
pub struct DatasetKeyword {
    pub keyword: &'static str,
    pub dataset_name: &'static str,
    pub observable: &'static str,
}

/// Returns canonical dataset recommendations based on variable keywords.
pub fn dataset_keywords() -> Vec<DatasetKeyword> {
    vec![
        DatasetKeyword {
            keyword: "streamflow",
            dataset_name: "USGS_NWIS",
            observable: "streamflow",
        },
        DatasetKeyword {
            keyword: "discharge",
            dataset_name: "USGS_NWIS",
            observable: "streamflow",
        },
        DatasetKeyword {
            keyword: "soil moisture",
            dataset_name: "SMAP_L3",
            observable: "soil_moisture",
        },
        DatasetKeyword {
            keyword: "evapotranspiration",
            dataset_name: "FLUXNET",
            observable: "evapotranspiration",
        },
        DatasetKeyword {
            keyword: "gpp",
            dataset_name: "FLUXNET",
            observable: "gpp",
        },
        DatasetKeyword {
            keyword: "nee",
            dataset_name: "FLUXNET",
            observable: "nee",
        },
        DatasetKeyword {
            keyword: "carbon",
            dataset_name: "FLUXNET",
            observable: "nee",
        },
        DatasetKeyword {
            keyword: "lai",
            dataset_name: "MODIS_LAI",
            observable: "lai",
        },
        DatasetKeyword {
            keyword: "ndvi",
            dataset_name: "MODIS_NDVI",
            observable: "ndvi",
        },
        DatasetKeyword {
            keyword: "snow",
            dataset_name: "SNODAS",
            observable: "swe",
        },
        DatasetKeyword {
            keyword: "swe",
            dataset_name: "SNODAS",
            observable: "swe",
        },
        DatasetKeyword {
            keyword: "temperature",
            dataset_name: "ERA5",
            observable: "temperature",
        },
        DatasetKeyword {
            keyword: "precipitation",
            dataset_name: "PRISM",
            observable: "precipitation",
        },
        DatasetKeyword {
            keyword: "fire",
            dataset_name: "MTBS",
            observable: "burned_area",
        },
        DatasetKeyword {
            keyword: "burn",
            dataset_name: "MTBS",
            observable: "burned_area",
        },
        DatasetKeyword {
            keyword: "wildfire",
            dataset_name: "NIFC",
            observable: "fire_occurrence",
        },
        DatasetKeyword {
            keyword: "radiation",
            dataset_name: "CERES",
            observable: "radiation",
        },
        DatasetKeyword {
            keyword: "albedo",
            dataset_name: "MODIS_Albedo",
            observable: "albedo",
        },
        DatasetKeyword {
            keyword: "sea surface",
            dataset_name: "OISST",
            observable: "sst",
        },
        DatasetKeyword {
            keyword: "ocean",
            dataset_name: "Argo",
            observable: "ocean_temperature",
        },
        DatasetKeyword {
            keyword: "ice",
            dataset_name: "NSIDC_SIC",
            observable: "sea_ice_concentration",
        },
        DatasetKeyword {
            keyword: "glacier",
            dataset_name: "RGI",
            observable: "glacier_area",
        },
        DatasetKeyword {
            keyword: "permafrost",
            dataset_name: "GTN-P",
            observable: "ground_temperature",
        },
        DatasetKeyword {
            keyword: "biodiversity",
            dataset_name: "GBIF",
            observable: "species_occurrence",
        },
        DatasetKeyword {
            keyword: "land use",
            dataset_name: "NLCD",
            observable: "land_cover",
        },
        DatasetKeyword {
            keyword: "agriculture",
            dataset_name: "USDA_NASS",
            observable: "crop_yield",
        },
        DatasetKeyword {
            keyword: "erosion",
            dataset_name: "USLE_DB",
            observable: "soil_loss",
        },
        DatasetKeyword {
            keyword: "sediment",
            dataset_name: "USGS_Sediment",
            observable: "suspended_sediment",
        },
    ]
}
