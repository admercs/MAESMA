//! Data plane types — Phase 6
//!
//! Data agents (requirements, discovery, acquisition, preprocessing, QA/QC),
//! data governance (allowlists, credentials, checksums, license enforcement),
//! and initial data product definitions.

use serde::{Deserialize, Serialize};

/// Data pipeline stage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataPipelineStage {
    Requirements,
    Discovery,
    Acquisition,
    Preprocessing,
    QaQc,
    Deposit,
}

/// Data requirement from an IR (intermediate representation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRequirement {
    pub variable: String,
    pub family: String,
    pub min_resolution_km: f64,
    pub min_temporal_resolution_s: f64,
    pub required_coverage: f64,
    pub priority: u8,
}

/// Acquisition plan entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionPlan {
    pub dataset_name: String,
    pub source_url: String,
    pub protocol: String,
    pub estimated_size_gb: f64,
    pub priority: u8,
    pub stage: DataPipelineStage,
}

/// Data governance rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRule {
    pub rule_type: GovernanceRuleType,
    pub description: String,
    pub enforced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovernanceRuleType {
    AllowlistedProvider,
    LicenseCheck,
    ChecksumVerification,
    RateLimiting,
    CredentialVault,
    ContentHashing,
    VersionedRecipe,
}

/// Preprocessing recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingRecipe {
    pub name: String,
    pub steps: Vec<PreprocessingStep>,
    pub output_format: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PreprocessingStep {
    Reproject { target_crs: String },
    Resample { resolution_km: u32 },
    Tile { tile_size: u32 },
    CloudMask,
    GapFill,
    UnitConvert { from: String, to: String },
}

/// QA/QC check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaQcResult {
    pub dataset: String,
    pub completeness: f64,
    pub outlier_fraction: f64,
    pub temporal_gap_count: u32,
    pub uncertainty_available: bool,
    pub passed: bool,
}

/// An initial data product definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialDataProduct {
    pub name: String,
    pub source: String,
    pub variable: String,
    pub resolution: String,
    pub coverage: String,
}

/// Standard initial data products (Phase 6.5).
pub fn initial_data_products() -> Vec<InitialDataProduct> {
    vec![
        InitialDataProduct {
            name: "Copernicus DEM".into(),
            source: "Copernicus/SRTM".into(),
            variable: "elevation".into(),
            resolution: "30m".into(),
            coverage: "global".into(),
        },
        InitialDataProduct {
            name: "Sentinel-2 L2A".into(),
            source: "Copernicus".into(),
            variable: "surface_reflectance".into(),
            resolution: "10m".into(),
            coverage: "global land".into(),
        },
        InitialDataProduct {
            name: "VIIRS Active Fire".into(),
            source: "NASA FIRMS".into(),
            variable: "fire_detection".into(),
            resolution: "375m".into(),
            coverage: "global NRT".into(),
        },
        InitialDataProduct {
            name: "GPM IMERG".into(),
            source: "NASA".into(),
            variable: "precipitation".into(),
            resolution: "0.1deg".into(),
            coverage: "global 60N-60S".into(),
        },
        InitialDataProduct {
            name: "MODIS LAI/NDVI".into(),
            source: "NASA LP DAAC".into(),
            variable: "lai_ndvi".into(),
            resolution: "500m".into(),
            coverage: "global land".into(),
        },
    ]
}

/// Standard governance rules (Phase 6.4).
pub fn standard_governance_rules() -> Vec<GovernanceRule> {
    vec![
        GovernanceRule {
            rule_type: GovernanceRuleType::AllowlistedProvider,
            description: "Only fetch from approved data providers".into(),
            enforced: true,
        },
        GovernanceRule {
            rule_type: GovernanceRuleType::LicenseCheck,
            description: "Verify data license is compatible".into(),
            enforced: true,
        },
        GovernanceRule {
            rule_type: GovernanceRuleType::ChecksumVerification,
            description: "Verify SHA-256 checksums on all downloads".into(),
            enforced: true,
        },
        GovernanceRule {
            rule_type: GovernanceRuleType::RateLimiting,
            description: "Respect provider rate limits".into(),
            enforced: true,
        },
        GovernanceRule {
            rule_type: GovernanceRuleType::CredentialVault,
            description: "Store credentials securely, never in code".into(),
            enforced: true,
        },
        GovernanceRule {
            rule_type: GovernanceRuleType::ContentHashing,
            description: "Content-addressed storage for deduplication".into(),
            enforced: false,
        },
        GovernanceRule {
            rule_type: GovernanceRuleType::VersionedRecipe,
            description: "All preprocessing uses versioned, deterministic recipes".into(),
            enforced: true,
        },
    ]
}

/// Derive requirements from a list of process families and their needed variables.
pub fn derive_requirements(needs: &[(&str, &str, f64)]) -> Vec<DataRequirement> {
    needs
        .iter()
        .enumerate()
        .map(|(i, (var, fam, res_km))| DataRequirement {
            variable: var.to_string(),
            family: fam.to_string(),
            min_resolution_km: *res_km,
            min_temporal_resolution_s: 86400.0,
            required_coverage: 0.8,
            priority: (i as u8).min(9),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_products_populated() {
        let products = initial_data_products();
        assert_eq!(products.len(), 5);
        assert!(products.iter().any(|p| p.variable == "precipitation"));
    }

    #[test]
    fn governance_rules_populated() {
        let rules = standard_governance_rules();
        assert!(rules.len() >= 7);
        assert!(rules.iter().filter(|r| r.enforced).count() >= 5);
    }

    #[test]
    fn derive_requirements_basic() {
        let reqs = derive_requirements(&[
            ("temperature", "atmosphere", 100.0),
            ("precipitation", "hydrology", 10.0),
        ]);
        assert_eq!(reqs.len(), 2);
        assert_eq!(reqs[0].priority, 0);
    }

    #[test]
    fn qa_qc_result_passed() {
        let r = QaQcResult {
            dataset: "test".into(),
            completeness: 0.95,
            outlier_fraction: 0.01,
            temporal_gap_count: 2,
            uncertainty_available: true,
            passed: true,
        };
        assert!(r.passed);
    }
}
