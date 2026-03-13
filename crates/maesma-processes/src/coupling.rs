//! Cross-family coupling declarations — defines exchange variables and
//! coupling semantics between process families.
//!
//! Each coupling declaration specifies:
//! - Source and target process families
//! - Exchanged variables (direction, units, cadence)
//! - Coupling mode (synchronous, lagged, time-accumulated)
//! - Validation datasets for the coupling pathway

use maesma_core::families::ProcessFamily;
use serde::{Deserialize, Serialize};

/// A coupling declaration between two process families.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingDeclaration {
    /// Source process family.
    pub source: ProcessFamily,
    /// Target process family.
    pub target: ProcessFamily,
    /// Variables exchanged.
    pub exchanges: Vec<CouplingExchange>,
    /// Whether the coupling is bidirectional.
    pub bidirectional: bool,
    /// Recommended coupling cadence (in seconds).
    pub cadence_seconds: f64,
    /// Validation datasets for this coupling pathway.
    pub validation_datasets: Vec<String>,
}

/// A single variable exchange in a coupling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingExchange {
    /// Variable name.
    pub variable: String,
    /// Direction of exchange.
    pub direction: ExchangeDirection,
    /// Units.
    pub units: String,
    /// Temporal aggregation applied before exchange.
    pub aggregation: ExchangeAggregation,
}

/// Direction of a coupling exchange.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExchangeDirection {
    SourceToTarget,
    TargetToSource,
    Bidirectional,
}

/// Temporal aggregation for coupling exchange.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExchangeAggregation {
    Instantaneous,
    TimeAveraged,
    Accumulated,
}

/// Build the complete set of cross-family coupling declarations.
pub fn all_coupling_declarations() -> Vec<CouplingDeclaration> {
    vec![
        // Atmosphere → Radiation, Hydrology, Fire
        CouplingDeclaration {
            source: ProcessFamily::Atmosphere,
            target: ProcessFamily::Radiation,
            exchanges: vec![
                CouplingExchange {
                    variable: "sw_down".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "W/m²".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "lw_down".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "W/m²".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "cloud_fraction".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "fraction".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: false,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["CERES_EBAF".into(), "ARM_sites".into()],
        },
        CouplingDeclaration {
            source: ProcessFamily::Atmosphere,
            target: ProcessFamily::Hydrology,
            exchanges: vec![
                CouplingExchange {
                    variable: "precipitation".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "kg/m²/s".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
                CouplingExchange {
                    variable: "air_temperature".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "K".into(),
                    aggregation: ExchangeAggregation::TimeAveraged,
                },
                CouplingExchange {
                    variable: "evapotranspiration".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "kg/m²/s".into(),
                    aggregation: ExchangeAggregation::TimeAveraged,
                },
            ],
            bidirectional: true,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["USGS_gauges".into(), "FLUXNET".into()],
        },
        CouplingDeclaration {
            source: ProcessFamily::Atmosphere,
            target: ProcessFamily::Fire,
            exchanges: vec![
                CouplingExchange {
                    variable: "wind_speed".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "m/s".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "relative_humidity".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "fraction".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "heat_flux".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "W/m²".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: true,
            cadence_seconds: 60.0,
            validation_datasets: vec!["GFED5".into(), "MTBS".into()],
        },
        // Radiation → Ecology (PAR)
        CouplingDeclaration {
            source: ProcessFamily::Radiation,
            target: ProcessFamily::Ecology,
            exchanges: vec![
                CouplingExchange {
                    variable: "par".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "µmol/m²/s".into(),
                    aggregation: ExchangeAggregation::TimeAveraged,
                },
                CouplingExchange {
                    variable: "lai".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "m²/m²".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: true,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["MODIS_LAI".into(), "FLUXNET".into()],
        },
        // Hydrology → Ecology (soil moisture)
        CouplingDeclaration {
            source: ProcessFamily::Hydrology,
            target: ProcessFamily::Ecology,
            exchanges: vec![
                CouplingExchange {
                    variable: "soil_moisture".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "m³/m³".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "root_water_uptake".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "kg/m²/s".into(),
                    aggregation: ExchangeAggregation::TimeAveraged,
                },
            ],
            bidirectional: true,
            cadence_seconds: 86400.0,
            validation_datasets: vec!["SMAP".into(), "FLUXNET".into()],
        },
        // Ecology → Biogeochemistry (litter, NPP)
        CouplingDeclaration {
            source: ProcessFamily::Ecology,
            target: ProcessFamily::Biogeochemistry,
            exchanges: vec![
                CouplingExchange {
                    variable: "litter_flux".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "gC/m²/day".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
                CouplingExchange {
                    variable: "npp".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "gC/m²/day".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
                CouplingExchange {
                    variable: "nitrogen_availability".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "gN/m²".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: true,
            cadence_seconds: 86400.0,
            validation_datasets: vec!["FLUXNET".into(), "SOILGRIDS".into()],
        },
        // Biogeochemistry → Atmosphere (CO₂ emissions)
        CouplingDeclaration {
            source: ProcessFamily::Biogeochemistry,
            target: ProcessFamily::Atmosphere,
            exchanges: vec![CouplingExchange {
                variable: "co2_flux".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "µmol/m²/s".into(),
                aggregation: ExchangeAggregation::TimeAveraged,
            }],
            bidirectional: false,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["FLUXNET".into()],
        },
        // Fire → Ecology (mortality, fuel transitions)
        CouplingDeclaration {
            source: ProcessFamily::Fire,
            target: ProcessFamily::Ecology,
            exchanges: vec![
                CouplingExchange {
                    variable: "burn_severity".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "fraction".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "mortality_fraction".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "fraction".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "fuel_load".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "kg/m²".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: true,
            cadence_seconds: 60.0,
            validation_datasets: vec!["MTBS".into(), "RAVG".into(), "FIA".into()],
        },
        // Fire → Biogeochemistry (combustion emissions, char)
        CouplingDeclaration {
            source: ProcessFamily::Fire,
            target: ProcessFamily::Biogeochemistry,
            exchanges: vec![
                CouplingExchange {
                    variable: "combustion_c_emissions".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "gC/m²".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
                CouplingExchange {
                    variable: "char_fraction".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "fraction".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "ash_nutrient_pulse".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "gN/m²".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
            ],
            bidirectional: false,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["GFED5".into()],
        },
        // Cryosphere → Hydrology (snowmelt, glacier runoff)
        CouplingDeclaration {
            source: ProcessFamily::Cryosphere,
            target: ProcessFamily::Hydrology,
            exchanges: vec![CouplingExchange {
                variable: "snowmelt".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "kg/m²/s".into(),
                aggregation: ExchangeAggregation::Accumulated,
            }],
            bidirectional: false,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["NSIDC_SIC".into()],
        },
        // Ocean ↔ Atmosphere (SST, heat flux)
        CouplingDeclaration {
            source: ProcessFamily::Ocean,
            target: ProcessFamily::Atmosphere,
            exchanges: vec![
                CouplingExchange {
                    variable: "sst".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "K".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "ocean_heat_flux".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "W/m²".into(),
                    aggregation: ExchangeAggregation::TimeAveraged,
                },
                CouplingExchange {
                    variable: "wind_stress".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "N/m²".into(),
                    aggregation: ExchangeAggregation::TimeAveraged,
                },
            ],
            bidirectional: true,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["ARGO".into(), "CERES_EBAF".into()],
        },
        // Ocean ↔ Cryosphere (sea ice, freshwater flux)
        CouplingDeclaration {
            source: ProcessFamily::Ocean,
            target: ProcessFamily::Cryosphere,
            exchanges: vec![CouplingExchange {
                variable: "ocean_temperature".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "K".into(),
                aggregation: ExchangeAggregation::Instantaneous,
            }],
            bidirectional: true,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["NSIDC_SIC".into(), "ARGO".into()],
        },
        // Cryosphere → Atmosphere (albedo)
        CouplingDeclaration {
            source: ProcessFamily::Cryosphere,
            target: ProcessFamily::Atmosphere,
            exchanges: vec![CouplingExchange {
                variable: "surface_albedo".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "fraction".into(),
                aggregation: ExchangeAggregation::Instantaneous,
            }],
            bidirectional: false,
            cadence_seconds: 3600.0,
            validation_datasets: vec!["CERES_EBAF".into()],
        },
        // Trophic ↔ Ecology (herbivory, dispersal)
        CouplingDeclaration {
            source: ProcessFamily::TrophicDynamics,
            target: ProcessFamily::Ecology,
            exchanges: vec![
                CouplingExchange {
                    variable: "herbivory_rate".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "gC/m²/day".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
                CouplingExchange {
                    variable: "seed_dispersal".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "seeds/m²".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
                CouplingExchange {
                    variable: "vegetation_biomass".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "gC/m²".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: true,
            cadence_seconds: 86400.0,
            validation_datasets: vec!["GBIF".into(), "NEON_surveys".into()],
        },
        // Trophic → Biogeochemistry (nutrient cycling)
        CouplingDeclaration {
            source: ProcessFamily::TrophicDynamics,
            target: ProcessFamily::Biogeochemistry,
            exchanges: vec![CouplingExchange {
                variable: "nutrient_recycling".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "gN/m²/day".into(),
                aggregation: ExchangeAggregation::Accumulated,
            }],
            bidirectional: false,
            cadence_seconds: 86400.0,
            validation_datasets: vec!["LTER".into()],
        },
        // Evolution ↔ Ecology (trait shifts, community assembly)
        CouplingDeclaration {
            source: ProcessFamily::Evolution,
            target: ProcessFamily::Ecology,
            exchanges: vec![
                CouplingExchange {
                    variable: "trait_mean_sla".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "m²/kg".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
                CouplingExchange {
                    variable: "fitness_landscape".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "dimensionless".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: true,
            cadence_seconds: 86400.0 * 365.0, // annual
            validation_datasets: vec!["TRY".into(), "BIEN".into()],
        },
        // Evolution ↔ Trophic (body-size structure, speciation/extinction)
        CouplingDeclaration {
            source: ProcessFamily::Evolution,
            target: ProcessFamily::TrophicDynamics,
            exchanges: vec![CouplingExchange {
                variable: "body_size_distribution".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "kg".into(),
                aggregation: ExchangeAggregation::Instantaneous,
            }],
            bidirectional: true,
            cadence_seconds: 86400.0 * 365.0,
            validation_datasets: vec!["PBDB".into(), "TimeTree".into()],
        },
        // Human Systems → Ecology (land-use change)
        CouplingDeclaration {
            source: ProcessFamily::HumanSystems,
            target: ProcessFamily::Ecology,
            exchanges: vec![CouplingExchange {
                variable: "land_use_fraction".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "fraction".into(),
                aggregation: ExchangeAggregation::Instantaneous,
            }],
            bidirectional: false,
            cadence_seconds: 86400.0 * 30.0, // monthly
            validation_datasets: vec!["ESA_CCI_LC".into()],
        },
        // Human Systems ↔ Hydrology (water extraction)
        CouplingDeclaration {
            source: ProcessFamily::HumanSystems,
            target: ProcessFamily::Hydrology,
            exchanges: vec![
                CouplingExchange {
                    variable: "water_extraction".into(),
                    direction: ExchangeDirection::SourceToTarget,
                    units: "m³/s".into(),
                    aggregation: ExchangeAggregation::Accumulated,
                },
                CouplingExchange {
                    variable: "water_availability".into(),
                    direction: ExchangeDirection::TargetToSource,
                    units: "m³".into(),
                    aggregation: ExchangeAggregation::Instantaneous,
                },
            ],
            bidirectional: true,
            cadence_seconds: 86400.0,
            validation_datasets: vec!["USGS_gauges".into()],
        },
        // Human Systems ↔ Atmosphere (emissions)
        CouplingDeclaration {
            source: ProcessFamily::HumanSystems,
            target: ProcessFamily::Atmosphere,
            exchanges: vec![CouplingExchange {
                variable: "anthropogenic_emissions".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "kgCO2/m²/s".into(),
                aggregation: ExchangeAggregation::Accumulated,
            }],
            bidirectional: false,
            cadence_seconds: 86400.0 * 30.0,
            validation_datasets: vec!["EIA_energy".into()],
        },
        // Trophic ↔ Ocean (marine food webs)
        CouplingDeclaration {
            source: ProcessFamily::TrophicDynamics,
            target: ProcessFamily::Ocean,
            exchanges: vec![CouplingExchange {
                variable: "marine_biomass".into(),
                direction: ExchangeDirection::Bidirectional,
                units: "gC/m²".into(),
                aggregation: ExchangeAggregation::Instantaneous,
            }],
            bidirectional: true,
            cadence_seconds: 86400.0,
            validation_datasets: vec!["FAO_fisheries".into()],
        },
        // Trophic ↔ Hydrology (aquatic food webs)
        CouplingDeclaration {
            source: ProcessFamily::TrophicDynamics,
            target: ProcessFamily::Hydrology,
            exchanges: vec![CouplingExchange {
                variable: "stream_temperature".into(),
                direction: ExchangeDirection::TargetToSource,
                units: "K".into(),
                aggregation: ExchangeAggregation::Instantaneous,
            }],
            bidirectional: true,
            cadence_seconds: 86400.0,
            validation_datasets: vec!["USGS_gauges".into()],
        },
        // Evolution → Disturbance / Biogeochem (fire tolerance, decomposition)
        CouplingDeclaration {
            source: ProcessFamily::Evolution,
            target: ProcessFamily::Biogeochemistry,
            exchanges: vec![CouplingExchange {
                variable: "decomposition_trait".into(),
                direction: ExchangeDirection::SourceToTarget,
                units: "dimensionless".into(),
                aggregation: ExchangeAggregation::Instantaneous,
            }],
            bidirectional: false,
            cadence_seconds: 86400.0 * 365.0,
            validation_datasets: vec!["TRY".into(), "LTER".into()],
        },
    ]
}

/// Compute the post-fire runoff/erosion risk index.
///
/// Based on burn severity → soil hydrophobicity + reduced interception:
///   risk = severity × (1 + slope_factor) × rain_intensity_factor
///
/// Returns risk ∈ [0, 1] per cell.
pub fn post_fire_runoff_erosion_risk(
    burn_severity: &[f64],
    slope_degrees: &[f64],
    rain_intensity_mm_hr: f64,
) -> Vec<f64> {
    burn_severity
        .iter()
        .zip(slope_degrees.iter())
        .map(|(&sev, &slope)| {
            let slope_factor = (slope / 45.0).min(1.0);
            let rain_factor = (rain_intensity_mm_hr / 50.0).min(1.0);
            (sev * (1.0 + slope_factor) * rain_factor).clamp(0.0, 1.0)
        })
        .collect()
}

/// Derive fuel strata from stand structure.
///
/// Classifies vegetation into surface, ladder, and canopy fuel layers based
/// on canopy height, canopy base height, and canopy bulk density.
///
/// Returns (surface_fuel_load, canopy_fuel_load, ladder_fuel_load) in kg/m².
pub fn derive_fuel_strata(
    biomass_kg_m2: f64,
    canopy_height_m: f64,
    canopy_base_height_m: f64,
    canopy_bulk_density_kg_m3: f64,
) -> (f64, f64, f64) {
    // Surface fuels: litter + low vegetation
    let surface_fraction = if canopy_height_m > 0.0 {
        (1.0 - canopy_base_height_m / canopy_height_m).clamp(0.1, 0.5)
    } else {
        0.8
    };

    // Canopy fuels: proportional to CBD × canopy depth
    let canopy_depth = (canopy_height_m - canopy_base_height_m).max(0.0);
    let canopy_fuel = canopy_bulk_density_kg_m3 * canopy_depth;

    // Ladder fuels: gap between surface and canopy
    let ladder_fraction = if canopy_base_height_m > 2.0 {
        0.05
    } else {
        0.15
    };

    let surface_fuel = biomass_kg_m2 * surface_fraction;
    let ladder_fuel = biomass_kg_m2 * ladder_fraction;

    (surface_fuel, canopy_fuel, ladder_fuel)
}

/// Check if crown fire can initiate based on Rothermel/Van Wagner criteria.
///
/// Crown fire initiation requires surface fire intensity > critical value:
///   I_crit = (0.010 × CBH × (460 + 25.9 × FMC))^1.5
///
/// where CBH = canopy base height (m), FMC = foliar moisture content (%).
pub fn crown_fire_threshold(
    surface_fire_intensity_kw_m: f64,
    canopy_base_height_m: f64,
    foliar_moisture_pct: f64,
) -> bool {
    let i_crit = (0.010 * canopy_base_height_m * (460.0 + 25.9 * foliar_moisture_pct)).powf(1.5);
    surface_fire_intensity_kw_m > i_crit
}

/// Compute combustion emissions from fire.
///
/// Returns (C emitted, N emitted, char produced) in g/m².
pub fn combustion_emissions(
    biomass_consumed_g_m2: f64,
    combustion_completeness: f64,
    carbon_fraction: f64,
    nitrogen_fraction: f64,
    char_fraction: f64,
) -> (f64, f64, f64) {
    let c_emitted = biomass_consumed_g_m2 * combustion_completeness * carbon_fraction;
    let n_emitted = biomass_consumed_g_m2 * combustion_completeness * nitrogen_fraction;
    let char_produced = biomass_consumed_g_m2 * char_fraction * (1.0 - combustion_completeness);
    (c_emitted, n_emitted, char_produced)
}

/// Post-fire mineralization pulse — nutrients released from ash.
///
/// Returns available N pulse (gN/m²) from burned organic matter.
pub fn post_fire_mineralization(
    burned_organic_n_g_m2: f64,
    severity: f64,
    soil_moisture_fraction: f64,
) -> f64 {
    // Higher severity → more mineral N released
    // Higher soil moisture → faster mineralization
    let moisture_factor = (soil_moisture_fraction / 0.3).min(1.0);
    burned_organic_n_g_m2 * severity * 0.6 * moisture_factor
}

// ───────────────────────────────────────────────────────────────────
// Tests
// ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_couplings_registered() {
        let decls = all_coupling_declarations();
        assert!(decls.len() >= 20, "Expected 20+ coupling declarations");
        // Check known pairs exist
        let has_atm_rad = decls
            .iter()
            .any(|d| d.source == ProcessFamily::Atmosphere && d.target == ProcessFamily::Radiation);
        assert!(has_atm_rad, "Missing Atmosphere→Radiation coupling");
    }

    #[test]
    fn coupling_exchange_variables() {
        let decls = all_coupling_declarations();
        for d in &decls {
            assert!(
                !d.exchanges.is_empty(),
                "Coupling {:?}→{:?} has no exchanges",
                d.source,
                d.target
            );
            assert!(d.cadence_seconds > 0.0);
        }
    }

    #[test]
    fn post_fire_runoff_risk() {
        let risk = post_fire_runoff_erosion_risk(&[0.8, 0.0, 0.5], &[30.0, 10.0, 45.0], 40.0);
        assert!(risk[0] > risk[1]); // burned > unburned
        assert!(risk[0] > 0.5);
        assert!((risk[1]).abs() < 1e-10); // zero severity → zero risk
    }

    #[test]
    fn fuel_strata_derivation() {
        let (surface, canopy, ladder) = derive_fuel_strata(5.0, 20.0, 10.0, 0.1);
        assert!(surface > 0.0);
        assert!(canopy > 0.0);
        assert!(ladder > 0.0);
        assert!(canopy > ladder); // typically more canopy than ladder fuel
    }

    #[test]
    fn crown_fire_threshold_physics() {
        // Low intensity → no crown fire
        assert!(!crown_fire_threshold(50.0, 10.0, 100.0));
        // Very high intensity → crown fire
        assert!(crown_fire_threshold(50000.0, 2.0, 80.0));
    }

    #[test]
    fn combustion_emissions_conservation() {
        let (c, n, char) = combustion_emissions(1000.0, 0.8, 0.45, 0.01, 0.05);
        assert!((c - 360.0).abs() < 1e-6); // 1000 * 0.8 * 0.45
        assert!((n - 8.0).abs() < 1e-6); // 1000 * 0.8 * 0.01
        assert!(char > 0.0);
    }

    #[test]
    fn post_fire_mineralization_scales() {
        let low = post_fire_mineralization(10.0, 0.3, 0.2);
        let high = post_fire_mineralization(10.0, 0.9, 0.5);
        assert!(high > low); // higher severity + moisture → more mineralization
    }
}
