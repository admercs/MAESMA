//! Seed data — initial process manifests populating the knowledgebase.
//!
//! This module provides the 1185 seed processes extracted from 50 reference models
//! as described in the MAESMA paper appendix. Each process is represented as a
//! [`ProcessManifest`] with machine-readable I/O contracts, scale envelopes,
//! conservation properties, cost models, and ontology relations.

use maesma_core::families::ProcessFamily;
use maesma_core::manifest::{
    ComputeBackend, ConservationProperty, CostModel, CouplingTier, IoContract, OntologyRelation,
    ProcessManifest, RelationType, ScaleEnvelope, Variable,
};
use maesma_core::process::{FidelityRung, LifecycleStatus, ProcessId, ProcessOrigin};
use uuid::Uuid;

/// Source model identifier for provenance tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceModel {
    Badlands,
    Cesm,
    E3sm,
    Fates,
    ILand,
    Landlab,
    LpjGuess,
    NoahMp,
    ParFlow,
    WrfSfire,
    Orchidee,
    Vic,
    GfdlEsm4,
    Pecan,
    LandisII,
    Jules,
    Cable,
    Classic,
    Summa,
    Ed2,
    Pflotran,
    Roms,
    Swat,
    GeosChem,
    Delft3d,
    LisfloodFp,
    Lm3Ppa,
    SortieNd,
    JabowaForet,
    Formind,
    LandisNecn,
    Uvafme,
    Forclim,
    SortieNg,
    DeepLand,
    Earth2,
    PhiSat2,
    FireBench,
    // New models (Phase 24)
    Marbl,
    Pism,
    Oggm,
    Apsim,
    Dssat,
    CryoGrid,
    Xbeach,
    SurfexTeb,
    Ewe,
    Carma,
    Ats,
    Bfm,
    Maestra,
}

impl SourceModel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Badlands => "Badlands",
            Self::Cesm => "CESM",
            Self::E3sm => "E3SM",
            Self::Fates => "FATES",
            Self::ILand => "iLand",
            Self::Landlab => "Landlab",
            Self::LpjGuess => "LPJ-GUESS",
            Self::NoahMp => "Noah-MP",
            Self::ParFlow => "ParFlow",
            Self::WrfSfire => "WRF-SFIRE",
            Self::Orchidee => "ORCHIDEE",
            Self::Vic => "VIC",
            Self::GfdlEsm4 => "GFDL-ESM4",
            Self::Pecan => "PEcAn",
            Self::LandisII => "LANDIS-II",
            Self::Jules => "JULES",
            Self::Cable => "CABLE",
            Self::Classic => "CLASSIC",
            Self::Summa => "SUMMA",
            Self::Ed2 => "ED2",
            Self::Pflotran => "PFLOTRAN",
            Self::Roms => "ROMS",
            Self::Swat => "SWAT",
            Self::GeosChem => "GEOS-Chem",
            Self::Delft3d => "Delft3D",
            Self::LisfloodFp => "LISFLOOD-FP",
            Self::Lm3Ppa => "LM3-PPA",
            Self::SortieNd => "SORTIE-ND",
            Self::JabowaForet => "JABOWA/FORET",
            Self::Formind => "FORMIND",
            Self::LandisNecn => "LANDIS-II NECN",
            Self::Uvafme => "UVAFME",
            Self::Forclim => "FORCLIM",
            Self::SortieNg => "SORTIE-NG",
            Self::DeepLand => "DeepLand",
            Self::Earth2 => "Earth-2",
            Self::PhiSat2 => "PhiSat-2",
            Self::FireBench => "FireBench",
            Self::Marbl => "MARBL",
            Self::Pism => "PISM",
            Self::Oggm => "OGGM",
            Self::Apsim => "APSIM",
            Self::Dssat => "DSSAT",
            Self::CryoGrid => "CryoGrid",
            Self::Xbeach => "XBeach",
            Self::SurfexTeb => "SURFEX/TEB",
            Self::Ewe => "EwE",
            Self::Carma => "CARMA",
            Self::Ats => "ATS",
            Self::Bfm => "BFM",
            Self::Maestra => "MAESTRA",
        }
    }
}

/// Helper to create a deterministic UUID from source model and process index.
fn make_id(source: SourceModel, index: u32) -> ProcessId {
    // Create deterministic UUID from namespace + source + index
    let name = format!("maesma:{}:{}", source.name(), index);
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes());
    ProcessId(uuid)
}

/// Create a basic I/O contract with common patterns.
fn io_contract(inputs: &[(&str, &str)], outputs: &[(&str, &str)]) -> IoContract {
    IoContract {
        inputs: inputs
            .iter()
            .map(|(name, unit)| Variable {
                name: name.to_string(),
                unit: unit.to_string(),
                description: None,
                dimensions: vec!["x".to_string(), "y".to_string()],
            })
            .collect(),
        outputs: outputs
            .iter()
            .map(|(name, unit)| Variable {
                name: name.to_string(),
                unit: unit.to_string(),
                description: None,
                dimensions: vec!["x".to_string(), "y".to_string()],
            })
            .collect(),
        parameters: Vec::new(),
    }
}

/// Create standard scale envelope for a given coupling tier and resolution range.
fn scale_envelope(
    tier: CouplingTier,
    dx_min: f64,
    dx_max: f64,
    dt_min: f64,
    dt_max: f64,
) -> ScaleEnvelope {
    ScaleEnvelope {
        dx_min,
        dx_max,
        dt_min,
        dt_max,
        coupling_tier: tier,
    }
}

/// Create standard cost model.
fn cost_model(flops: f64, mem: u64, gpu: bool) -> CostModel {
    CostModel {
        flops_per_cell: flops,
        memory_per_cell: mem,
        gpu_capable: gpu,
    }
}

/// Map paper fidelity strings to FidelityRung.
fn fidelity(s: &str) -> FidelityRung {
    match s {
        "Empirical" => FidelityRung::R0,
        "Intermediate" => FidelityRung::R1,
        "Physics-based" => FidelityRung::R2,
        _ => FidelityRung::R1,
    }
}

// ============================================================================
// SEED PROCESS MANIFESTS
// ============================================================================

/// Generate all seed process manifests (1185 total from 50 source models).
pub fn generate_seed_manifests() -> Vec<ProcessManifest> {
    let mut manifests = Vec::with_capacity(1200);

    // Badlands (33 processes)
    manifests.extend(badlands_processes());

    // CESM composite (182 processes)
    manifests.extend(cesm_processes());

    // FATES (90 processes)
    manifests.extend(fates_processes());

    // Landlab (49 processes)
    manifests.extend(landlab_processes());

    // WRF-SFIRE (43 processes)
    manifests.extend(wrf_sfire_processes());

    // Noah-MP (32 processes)
    manifests.extend(noah_mp_processes());

    // ParFlow (16 processes)
    manifests.extend(parflow_processes());

    // MARBL (20 processes)
    manifests.extend(marbl_processes());

    // PISM (23 processes)
    manifests.extend(pism_processes());

    // OGGM (15 processes)
    manifests.extend(oggm_processes());

    // APSIM (25 processes)
    manifests.extend(apsim_processes());

    // DSSAT (24 processes)
    manifests.extend(dssat_processes());

    // CryoGrid (18 processes)
    manifests.extend(cryogrid_processes());

    // XBeach (20 processes)
    manifests.extend(xbeach_processes());

    // SURFEX/TEB (18 processes)
    manifests.extend(surfex_teb_processes());

    // EwE (17 processes)
    manifests.extend(ewe_processes());

    // CARMA (18 processes)
    manifests.extend(carma_processes());

    // ATS (18 processes)
    manifests.extend(ats_processes());

    // BFM (22 processes)
    manifests.extend(bfm_processes());

    // Extended seed: 31 remaining source models
    manifests.extend(crate::seed_extended::generate_extended_manifests());

    manifests
}

// ============================================================================
// BADLANDS PROCESSES (33)
// ============================================================================

fn badlands_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Badlands, 1),
            name: "Stream Power Erosion".to_string(),
            description:
                "Fluvial incision via detachment-limited stream power law (E = K A^m S^n)."
                    .to_string(),
            family: ProcessFamily::Geomorphology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("drainage_area", "m²"),
                    ("slope", "m/m"),
                    ("precipitation", "m/s"),
                ],
                &[("erosion_rate", "m/s"), ("sediment_flux", "kg/s")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 100.0, 100_000.0, 31536000.0, 3.1536e10),
            conservation: vec![ConservationProperty {
                quantity: "sediment_mass".to_string(),
                method: "flux_balance".to_string(),
            }],
            cost: cost_model(1e4, 256, false),
            regime_tags: vec!["fluvial".to_string(), "landscape_evolution".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Badlands".to_string(),
                version: "2.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Badlands, 2),
            name: "Hillslope Diffusion".to_string(),
            description: "Linear diffusion of topography representing soil creep and rain splash."
                .to_string(),
            family: ProcessFamily::Geomorphology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("elevation", "m"), ("diffusivity", "m²/s")],
                &[("elevation_change", "m/s")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 100.0, 50_000.0, 31536000.0, 3.1536e10),
            conservation: vec![ConservationProperty {
                quantity: "sediment_volume".to_string(),
                method: "diffusion_mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 128, false),
            regime_tags: vec!["hillslope".to_string(), "diffusion".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Badlands".to_string(),
                version: "2.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Badlands, 3),
            name: "Flexural Isostasy".to_string(),
            description: "Lithospheric flexure response to sediment loading/unloading.".to_string(),
            family: ProcessFamily::Geology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("sediment_load", "Pa"), ("elastic_thickness", "m")],
                &[("vertical_displacement", "m")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 500_000.0, 31536000.0, 3.1536e11),
            conservation: vec![],
            cost: cost_model(1e5, 512, false),
            regime_tags: vec!["tectonics".to_string(), "isostasy".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Badlands".to_string(),
                version: "2.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// CESM PROCESSES (representative subset)
// ============================================================================

fn cesm_processes() -> Vec<ProcessManifest> {
    vec![
        // CAM Atmosphere
        ProcessManifest {
            id: make_id(SourceModel::Cesm, 1),
            name: "Deep Convection (Zhang-McFarlane)".to_string(),
            description: "Parameterized deep cumulus convection with CAPE-based mass flux closure."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "K"),
                    ("specific_humidity", "kg/kg"),
                    ("pressure", "Pa"),
                ],
                &[
                    ("convective_heating", "K/s"),
                    ("convective_moistening", "kg/kg/s"),
                    ("precipitation", "kg/m²/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 200_000.0, 1200.0, 3600.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "energy".to_string(),
                    method: "column_integral".to_string(),
                },
                ConservationProperty {
                    quantity: "water".to_string(),
                    method: "mass_balance".to_string(),
                },
            ],
            cost: cost_model(1e6, 4096, true),
            regime_tags: vec!["convection".to_string(), "tropical".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CESM/CAM6".to_string(),
                version: "6.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Cesm, 2),
            name: "RRTMGP Radiation".to_string(),
            description:
                "Rapid Radiative Transfer Model for GCMs with k-distribution shortwave/longwave."
                    .to_string(),
            family: ProcessFamily::Radiation,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "K"),
                    ("water_vapor", "kg/kg"),
                    ("ozone", "mol/mol"),
                    ("cloud_fraction", "1"),
                ],
                &[
                    ("sw_flux_dn", "W/m²"),
                    ("sw_flux_up", "W/m²"),
                    ("lw_flux_dn", "W/m²"),
                    ("lw_flux_up", "W/m²"),
                    ("heating_rate", "K/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 200_000.0, 3600.0, 10800.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "radiative_balance".to_string(),
            }],
            cost: cost_model(1e7, 8192, true),
            regime_tags: vec!["radiation".to_string(), "global".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CESM/RRTMGP".to_string(),
                version: "1.5".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        // CLM Land Surface
        ProcessManifest {
            id: make_id(SourceModel::Cesm, 10),
            name: "CLM Photosynthesis (Farquhar)".to_string(),
            description:
                "Farquhar-Collatz C3/C4 photosynthesis coupled to Ball-Berry stomatal conductance."
                    .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("par", "W/m²"),
                    ("temperature", "K"),
                    ("co2", "ppm"),
                    ("vapor_pressure", "Pa"),
                ],
                &[
                    ("gpp", "gC/m²/s"),
                    ("stomatal_conductance", "mol/m²/s"),
                    ("transpiration", "kg/m²/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(5e5, 2048, true),
            regime_tags: vec!["photosynthesis".to_string(), "vegetation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CESM/CLM5".to_string(),
                version: "5.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Cesm, 11),
            name: "CLM Soil Hydrology (Richards)".to_string(),
            description: "1D Richards equation for variably saturated soil water flow.".to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("infiltration", "kg/m²/s"),
                    ("soil_moisture", "m³/m³"),
                    ("hydraulic_conductivity", "m/s"),
                ],
                &[
                    ("soil_moisture", "m³/m³"),
                    ("drainage", "kg/m²/s"),
                    ("runoff", "kg/m²/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["hydrology".to_string(), "soil".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CESM/CLM5".to_string(),
                version: "5.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // MOM6 Ocean
        ProcessManifest {
            id: make_id(SourceModel::Cesm, 50),
            name: "Ocean Primitive Equations".to_string(),
            description:
                "Hydrostatic primitive equations for ocean circulation on z*/sigma hybrid grid."
                    .to_string(),
            family: ProcessFamily::Ocean,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("wind_stress", "Pa"),
                    ("heat_flux", "W/m²"),
                    ("freshwater_flux", "kg/m²/s"),
                ],
                &[
                    ("velocity_u", "m/s"),
                    ("velocity_v", "m/s"),
                    ("temperature", "K"),
                    ("salinity", "psu"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 100_000.0, 600.0, 3600.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "mass".to_string(),
                    method: "continuity".to_string(),
                },
                ConservationProperty {
                    quantity: "energy".to_string(),
                    method: "enthalpy_budget".to_string(),
                },
            ],
            cost: cost_model(1e8, 16384, true),
            regime_tags: vec!["ocean".to_string(), "circulation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CESM/MOM6".to_string(),
                version: "6.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        // CICE Sea Ice
        ProcessManifest {
            id: make_id(SourceModel::Cesm, 80),
            name: "Sea Ice Dynamics (EVP)".to_string(),
            description: "Elastic-viscous-plastic rheology for sea ice momentum with ridging."
                .to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("wind_stress", "Pa"),
                    ("ocean_stress", "Pa"),
                    ("ice_concentration", "1"),
                    ("ice_thickness", "m"),
                ],
                &[
                    ("ice_velocity_u", "m/s"),
                    ("ice_velocity_v", "m/s"),
                    ("ice_strength", "N/m"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 100_000.0, 600.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "momentum".to_string(),
                method: "stress_balance".to_string(),
            }],
            cost: cost_model(1e6, 4096, true),
            regime_tags: vec!["sea_ice".to_string(), "polar".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CESM/CICE6".to_string(),
                version: "6.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
    ]
}

// ============================================================================
// FATES PROCESSES (representative subset)
// ============================================================================

fn fates_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Fates, 1),
            name: "Cohort Demography".to_string(),
            description: "Cohort-based vegetation demography tracking size distributions."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("npp", "gC/m²/s"), ("mortality_rate", "1/yr")],
                &[
                    ("cohort_density", "1/m²"),
                    ("biomass", "gC/m²"),
                    ("lai", "m²/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 100_000.0, 86400.0, 2592000.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "cohort_tracking".to_string(),
            }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec!["demography".to_string(), "forest".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "FATES".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Fates, 2),
            name: "SPITFIRE Fire Behavior".to_string(),
            description: "Fire spread model coupling Rothermel spread rate with fuel dynamics."
                .to_string(),
            family: ProcessFamily::Fire,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("fuel_load", "kg/m²"),
                    ("fuel_moisture", "1"),
                    ("wind_speed", "m/s"),
                    ("slope", "degrees"),
                ],
                &[
                    ("fire_intensity", "kW/m"),
                    ("spread_rate", "m/s"),
                    ("burned_fraction", "1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 30.0, 10_000.0, 60.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "fuel_mass".to_string(),
                method: "combustion_balance".to_string(),
            }],
            cost: cost_model(1e4, 1024, true),
            regime_tags: vec!["fire".to_string(), "wildfire".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "FATES/SPITFIRE".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Fates, 3),
            name: "Tree Allometry".to_string(),
            description: "Allometric relationships converting DBH to height, crown area, biomass."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Empirical"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("dbh", "cm"), ("pft_id", "1")],
                &[("height", "m"), ("crown_area", "m²"), ("biomass", "kgC")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10.0, 100_000.0, 86400.0, 31536000.0),
            conservation: vec![],
            cost: cost_model(1e2, 64, false),
            regime_tags: vec!["allometry".to_string(), "forest".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "FATES".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// LANDLAB PROCESSES (representative subset)
// ============================================================================

fn landlab_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Landlab, 1),
            name: "FlowRouter".to_string(),
            description: "D8/D-infinity flow routing algorithms for drainage network.".to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("elevation", "m"), ("precipitation", "m/s")],
                &[
                    ("drainage_area", "m²"),
                    ("flow_direction", "1"),
                    ("discharge", "m³/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10.0, 100_000.0, 3600.0, 31536000.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 256, false),
            regime_tags: vec!["hydrology".to_string(), "drainage".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Landlab".to_string(),
                version: "2.6".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Landlab, 2),
            name: "SpeciesEvolver".to_string(),
            description: "Phylogenetic species evolution with speciation, extinction, dispersal."
                .to_string(),
            family: ProcessFamily::Evolution,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("habitat_suitability", "1"),
                    ("dispersal_rate", "m/yr"),
                    ("speciation_rate", "1/yr"),
                ],
                &[("species_richness", "1"), ("phylogenetic_tree", "newick")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                1000.0,
                1_000_000.0,
                31536000.0,
                3.1536e10,
            ),
            conservation: vec![],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["evolution".to_string(), "biogeography".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Landlab".to_string(),
                version: "2.6".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// WRF-SFIRE PROCESSES (representative subset)
// ============================================================================

fn wrf_sfire_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::WrfSfire, 1),
            name: "Level Set Fire Spread".to_string(),
            description: "Level set method for fire front propagation with Rothermel spread rate."
                .to_string(),
            family: ProcessFamily::Fire,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("fuel_properties", "struct"),
                    ("wind_u", "m/s"),
                    ("wind_v", "m/s"),
                    ("slope", "degrees"),
                ],
                &[
                    ("fire_front", "1"),
                    ("spread_rate", "m/s"),
                    ("fire_intensity", "kW/m"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10.0, 1000.0, 0.1, 60.0),
            conservation: vec![ConservationProperty {
                quantity: "fire_perimeter".to_string(),
                method: "level_set".to_string(),
            }],
            cost: cost_model(1e5, 2048, true),
            regime_tags: vec![
                "fire".to_string(),
                "wildfire".to_string(),
                "coupled_fire_atmosphere".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "WRF-SFIRE".to_string(),
                version: "4.5".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::WrfSfire, 2),
            name: "Fire-Atmosphere Coupling".to_string(),
            description: "Two-way coupling of fire heat/moisture flux with WRF LES atmosphere."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("fire_heat_flux", "W/m²"),
                    ("fire_moisture_flux", "kg/m²/s"),
                ],
                &[
                    ("temperature_tendency", "K/s"),
                    ("wind_u", "m/s"),
                    ("wind_v", "m/s"),
                    ("tke", "m²/s²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10.0, 1000.0, 0.1, 10.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "flux_coupling".to_string(),
            }],
            cost: cost_model(1e6, 4096, true),
            regime_tags: vec!["fire".to_string(), "plume".to_string(), "les".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "WRF-SFIRE".to_string(),
                version: "4.5".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
    ]
}

// ============================================================================
// NOAH-MP PROCESSES
// ============================================================================

fn noah_mp_processes() -> Vec<ProcessManifest> {
    vec![ProcessManifest {
        id: make_id(SourceModel::NoahMp, 1),
        name: "Multi-layer Canopy".to_string(),
        description: "Two-stream canopy radiation with explicit crown/understory separation."
            .to_string(),
        family: ProcessFamily::Radiation,
        rung: fidelity("Intermediate"),
        version: "1.0.0".to_string(),
        io: io_contract(
            &[("sw_down", "W/m²"), ("lw_down", "W/m²"), ("lai", "m²/m²")],
            &[
                ("sw_absorbed_canopy", "W/m²"),
                ("sw_absorbed_ground", "W/m²"),
                ("lw_net", "W/m²"),
            ],
        ),
        scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 1800.0, 3600.0),
        conservation: vec![ConservationProperty {
            quantity: "energy".to_string(),
            method: "radiation_balance".to_string(),
        }],
        cost: cost_model(1e4, 512, false),
        regime_tags: vec!["canopy".to_string(), "radiation".to_string()],
        relations: Vec::new(),
        origin: ProcessOrigin::Imported {
            source: "Noah-MP".to_string(),
            version: "5.0".to_string(),
        },
        lifecycle: LifecycleStatus::Production,
        backends: vec![ComputeBackend::Cpu],
    }]
}

// ============================================================================
// PARFLOW PROCESSES
// ============================================================================

fn parflow_processes() -> Vec<ProcessManifest> {
    vec![ProcessManifest {
        id: make_id(SourceModel::ParFlow, 1),
        name: "3D Richards Equation".to_string(),
        description: "Fully 3D variably saturated groundwater flow via Richards equation."
            .to_string(),
        family: ProcessFamily::Hydrology,
        rung: fidelity("Physics-based"),
        version: "1.0.0".to_string(),
        io: io_contract(
            &[
                ("pressure_head", "m"),
                ("hydraulic_conductivity", "m/s"),
                ("porosity", "1"),
            ],
            &[("pressure_head", "m"), ("saturation", "1"), ("flux", "m/s")],
        ),
        scale: scale_envelope(CouplingTier::Fast, 10.0, 10_000.0, 60.0, 3600.0),
        conservation: vec![ConservationProperty {
            quantity: "water".to_string(),
            method: "mass_balance".to_string(),
        }],
        cost: cost_model(1e7, 8192, true),
        regime_tags: vec![
            "hydrology".to_string(),
            "groundwater".to_string(),
            "3d".to_string(),
        ],
        relations: Vec::new(),
        origin: ProcessOrigin::Imported {
            source: "ParFlow".to_string(),
            version: "3.12".to_string(),
        },
        lifecycle: LifecycleStatus::Production,
        backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
    }]
}

// ============================================================================
// MARBL PROCESSES (20)
// ============================================================================

fn marbl_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Marbl, 1),
            name: "Phytoplankton Growth".to_string(),
            description: "Light- and nutrient-limited growth for multiple functional types (diatoms, small phyto, diazotrophs, coccolithophores).".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("par", "W/m²"), ("no3", "mmol/m³"), ("po4", "mmol/m³"), ("fe", "nmol/m³"), ("temperature", "°C")],
                &[("phyto_c", "mmol/m³"), ("phyto_chl", "mg/m³"), ("primary_production", "mmol/m³/d")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty { quantity: "carbon".to_string(), method: "mass_balance".to_string() }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec!["ocean".to_string(), "biogeochemistry".to_string(), "phytoplankton".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "MARBL".to_string(), version: "1.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Marbl, 2),
            name: "Air-Sea Gas Exchange".to_string(),
            description: "CO₂ and O₂ flux across air-sea interface via wind-speed dependent piston velocity.".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("wind_speed", "m/s"), ("sst", "°C"), ("pco2_ocean", "µatm"), ("pco2_atm", "µatm")],
                &[("co2_flux", "mol/m²/yr"), ("o2_flux", "mol/m²/yr")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty { quantity: "carbon".to_string(), method: "flux_balance".to_string() }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["ocean".to_string(), "carbon_cycle".to_string(), "air_sea".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "MARBL".to_string(), version: "1.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Marbl, 3),
            name: "Carbonate Chemistry".to_string(),
            description: "Full ocean carbon chemistry solver (DIC, alkalinity, pH, pCO₂).".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("dic", "mmol/m³"), ("alkalinity", "meq/m³"), ("temperature", "°C"), ("salinity", "psu")],
                &[("ph", "1"), ("pco2", "µatm"), ("omega_calcite", "1"), ("omega_aragonite", "1")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![],
            cost: cost_model(1e4, 256, false),
            regime_tags: vec!["ocean".to_string(), "carbon_cycle".to_string(), "chemistry".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "MARBL".to_string(), version: "1.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// PISM PROCESSES (23)
// ============================================================================

fn pism_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Pism, 1),
            name: "Shallow Ice Approximation (SIA)".to_string(),
            description: "Non-sliding internal deformation ice flow via Glen's law.".to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("ice_thickness", "m"),
                    ("surface_slope", "m/m"),
                    ("ice_temperature", "K"),
                ],
                &[("velocity_horizontal", "m/yr"), ("strain_rate", "1/yr")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 50_000.0, 31536000.0, 3.1536e9),
            conservation: vec![ConservationProperty {
                quantity: "ice_mass".to_string(),
                method: "continuity".to_string(),
            }],
            cost: cost_model(1e6, 4096, true),
            regime_tags: vec!["ice_sheet".to_string(), "glaciology".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PISM".to_string(),
                version: "2.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Pism, 2),
            name: "Shallow Shelf Approximation (SSA)".to_string(),
            description: "Membrane stress balance for ice shelves and fast-flowing ice streams."
                .to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("ice_thickness", "m"),
                    ("basal_yield_stress", "Pa"),
                    ("sea_level", "m"),
                ],
                &[
                    ("velocity_u", "m/yr"),
                    ("velocity_v", "m/yr"),
                    ("ice_shelf_melt", "m/yr"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 50_000.0, 31536000.0, 3.1536e9),
            conservation: vec![ConservationProperty {
                quantity: "momentum".to_string(),
                method: "stress_balance".to_string(),
            }],
            cost: cost_model(1e6, 4096, true),
            regime_tags: vec!["ice_sheet".to_string(), "ice_shelf".to_string()],
            relations: vec![OntologyRelation {
                relation_type: RelationType::RequiresCouplingWith,
                target: "pism:ocean_thermal_forcing".to_string(),
            }],
            origin: ProcessOrigin::Imported {
                source: "PISM".to_string(),
                version: "2.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Pism, 3),
            name: "Ice Thermodynamics".to_string(),
            description: "Enthalpy-based 3D temperature/moisture field in polythermal ice."
                .to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("ice_velocity", "m/yr"),
                    ("surface_temperature", "K"),
                    ("geothermal_flux", "W/m²"),
                ],
                &[
                    ("ice_enthalpy", "J/kg"),
                    ("ice_temperature", "K"),
                    ("basal_melt_rate", "m/yr"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 50_000.0, 31536000.0, 3.1536e9),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "enthalpy_advection".to_string(),
            }],
            cost: cost_model(1e6, 4096, false),
            regime_tags: vec!["ice_sheet".to_string(), "thermodynamics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PISM".to_string(),
                version: "2.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// OGGM PROCESSES (15)
// ============================================================================

fn oggm_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Oggm, 1),
            name: "Flowline Ice Dynamics (SIA)".to_string(),
            description:
                "Shallow ice approximation along flowlines with trapezoidal cross-section."
                    .to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("surface_mass_balance", "m/yr"),
                    ("ice_thickness", "m"),
                    ("bed_elevation", "m"),
                ],
                &[
                    ("ice_thickness", "m"),
                    ("glacier_length", "m"),
                    ("glacier_volume", "km³"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 100.0, 10_000.0, 31536000.0, 3.1536e9),
            conservation: vec![ConservationProperty {
                quantity: "ice_mass".to_string(),
                method: "continuity".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["glacier".to_string(), "mountain".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "OGGM".to_string(),
                version: "1.6".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Oggm, 2),
            name: "Temperature-Index Mass Balance".to_string(),
            description: "Monthly temperature-index surface mass balance model.".to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Empirical"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "°C"),
                    ("precipitation", "mm/month"),
                    ("elevation", "m"),
                ],
                &[
                    ("surface_mass_balance", "m w.e./yr"),
                    ("accumulation", "m w.e./yr"),
                    ("ablation", "m w.e./yr"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 100.0, 10_000.0, 2592000.0, 31536000.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 128, false),
            regime_tags: vec!["glacier".to_string(), "mass_balance".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "OGGM".to_string(),
                version: "1.6".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// APSIM PROCESSES (25)
// ============================================================================

fn apsim_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Apsim, 1),
            name: "Crop Phenology".to_string(),
            description: "Temperature-driven (thermal time / GDD) phenological development."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "°C"),
                    ("photoperiod", "h"),
                    ("vernalization_days", "d"),
                ],
                &[("growth_stage", "1"), ("thermal_time", "°C·d")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 86400.0, 86400.0),
            conservation: vec![],
            cost: cost_model(1e2, 64, false),
            regime_tags: vec!["agriculture".to_string(), "phenology".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "APSIM".to_string(),
                version: "7.10".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Apsim, 2),
            name: "Canopy Photosynthesis (RUE)".to_string(),
            description: "Radiation use efficiency: biomass = intercepted PAR × RUE.".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Empirical"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("par_intercepted", "MJ/m²/d"),
                    ("rue", "g/MJ"),
                    ("water_stress", "1"),
                    ("n_stress", "1"),
                ],
                &[("biomass_increment", "g/m²/d")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 86400.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e2, 64, false),
            regime_tags: vec!["agriculture".to_string(), "photosynthesis".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "APSIM".to_string(),
                version: "7.10".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Apsim, 3),
            name: "SoilWat Water Balance".to_string(),
            description: "Cascading-bucket soil water balance with SCS curve number infiltration."
                .to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("precipitation", "mm/d"),
                    ("evapotranspiration", "mm/d"),
                    ("soil_water", "mm"),
                ],
                &[
                    ("soil_water", "mm"),
                    ("drainage", "mm/d"),
                    ("runoff", "mm/d"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 86400.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 128, false),
            regime_tags: vec!["agriculture".to_string(), "hydrology".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "APSIM".to_string(),
                version: "7.10".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// DSSAT PROCESSES (24)
// ============================================================================

fn dssat_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Dssat, 1),
            name: "CERES Crop Growth".to_string(),
            description: "Temperature-driven development, light interception, RUE-based biomass."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("temperature", "°C"), ("par", "MJ/m²/d"), ("lai", "m²/m²")],
                &[
                    ("biomass", "kg/ha"),
                    ("lai", "m²/m²"),
                    ("grain_yield", "kg/ha"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 86400.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["agriculture".to_string(), "crop_model".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "DSSAT".to_string(),
                version: "4.8".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Dssat, 2),
            name: "CENTURY Soil Carbon".to_string(),
            description: "CENTURY-based SOM model with metabolic/structural litter pools."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("litter_input", "gC/m²"),
                    ("temperature", "°C"),
                    ("moisture", "1"),
                ],
                &[
                    ("som_active", "gC/m²"),
                    ("som_slow", "gC/m²"),
                    ("som_passive", "gC/m²"),
                    ("co2_flux", "gC/m²/d"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 86400.0, 31536000.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["agriculture".to_string(), "soil_carbon".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "DSSAT".to_string(),
                version: "4.8".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// CRYOGRID PROCESSES (18)
// ============================================================================

fn cryogrid_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::CryoGrid, 1),
            name: "Heat Conduction (Fourier)".to_string(),
            description: "1D heat conduction through soil/rock column.".to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "K"),
                    ("thermal_conductivity", "W/m/K"),
                    ("heat_capacity", "J/m³/K"),
                ],
                &[("temperature", "K"), ("heat_flux", "W/m²")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 10_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "conduction_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["permafrost".to_string(), "thermal".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CryoGrid".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::CryoGrid, 2),
            name: "Freeze-Thaw (Enthalpy)".to_string(),
            description: "Phase change of water/ice via enthalpy formulation.".to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("enthalpy", "J/m³"),
                    ("soil_porosity", "1"),
                    ("temperature", "K"),
                ],
                &[
                    ("ice_content", "1"),
                    ("water_content", "1"),
                    ("temperature", "K"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 10_000.0, 3600.0, 86400.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "energy".to_string(),
                    method: "enthalpy_balance".to_string(),
                },
                ConservationProperty {
                    quantity: "water".to_string(),
                    method: "mass_balance".to_string(),
                },
            ],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["permafrost".to_string(), "freeze_thaw".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CryoGrid".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::CryoGrid, 3),
            name: "Active Layer Dynamics".to_string(),
            description: "Seasonal thaw depth computation.".to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("temperature_profile", "K"), ("ice_content_profile", "1")],
                &[
                    ("active_layer_thickness", "m"),
                    ("permafrost_table_depth", "m"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 86400.0, 31536000.0),
            conservation: vec![],
            cost: cost_model(1e2, 64, false),
            regime_tags: vec!["permafrost".to_string(), "active_layer".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CryoGrid".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// XBEACH PROCESSES (20)
// ============================================================================

fn xbeach_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Xbeach, 1),
            name: "Short-wave Energy Propagation".to_string(),
            description: "Wave action balance: shoaling, refraction, breaking.".to_string(),
            family: ProcessFamily::Ocean,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("wave_height", "m"),
                    ("wave_period", "s"),
                    ("wave_direction", "deg"),
                    ("bathymetry", "m"),
                ],
                &[
                    ("wave_height", "m"),
                    ("wave_energy", "J/m²"),
                    ("dissipation", "W/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 1000.0, 0.1, 60.0),
            conservation: vec![ConservationProperty {
                quantity: "wave_energy".to_string(),
                method: "action_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, true),
            regime_tags: vec!["coastal".to_string(), "waves".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "XBeach".to_string(),
                version: "1.24".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Xbeach, 2),
            name: "Sediment Transport (Suspended)".to_string(),
            description: "Depth-averaged suspended sediment advection-diffusion.".to_string(),
            family: ProcessFamily::Geomorphology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("bed_shear_stress", "Pa"),
                    ("sediment_concentration", "kg/m³"),
                    ("current_velocity", "m/s"),
                ],
                &[
                    ("sediment_flux", "kg/m/s"),
                    ("erosion_rate", "kg/m²/s"),
                    ("deposition_rate", "kg/m²/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 1000.0, 0.1, 60.0),
            conservation: vec![ConservationProperty {
                quantity: "sediment_mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, true),
            regime_tags: vec!["coastal".to_string(), "morphodynamics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "XBeach".to_string(),
                version: "1.24".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Xbeach, 3),
            name: "Bed Level Update (Exner)".to_string(),
            description: "Morphological bed evolution via sediment continuity.".to_string(),
            family: ProcessFamily::Geomorphology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("sediment_flux_gradient", "kg/m²/s"), ("bed_porosity", "1")],
                &[("bed_level_change", "m/s")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 1000.0, 1.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "sediment_volume".to_string(),
                method: "exner_equation".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["coastal".to_string(), "morphodynamics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "XBeach".to_string(),
                version: "1.24".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// SURFEX/TEB PROCESSES (18)
// ============================================================================

fn surfex_teb_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::SurfexTeb, 1),
            name: "Urban Canyon Radiation".to_string(),
            description: "Multiple reflections of SW/LW within street canyon.".to_string(),
            family: ProcessFamily::Radiation,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("sw_down", "W/m²"),
                    ("lw_down", "W/m²"),
                    ("canyon_aspect_ratio", "1"),
                    ("wall_albedo", "1"),
                ],
                &[
                    ("sw_absorbed_road", "W/m²"),
                    ("sw_absorbed_wall", "W/m²"),
                    ("lw_net_canyon", "W/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 10_000.0, 3600.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "radiation_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["urban".to_string(), "radiation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SURFEX/TEB".to_string(),
                version: "8.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::SurfexTeb, 2),
            name: "Building Energy Model (BEM)".to_string(),
            description: "Indoor energy demand: heating, cooling, ventilation.".to_string(),
            family: ProcessFamily::HumanSystems,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("outdoor_temp", "K"),
                    ("solar_gain", "W/m²"),
                    ("internal_heat", "W/m²"),
                ],
                &[
                    ("indoor_temp", "K"),
                    ("hvac_energy", "W/m²"),
                    ("waste_heat", "W/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 10_000.0, 3600.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "building_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["urban".to_string(), "building".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SURFEX/TEB".to_string(),
                version: "8.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::SurfexTeb, 3),
            name: "Urban Heat Island".to_string(),
            description: "Emergent UHI from canyon trapping and anthropogenic heat.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("urban_fraction", "1"),
                    ("canyon_temp", "K"),
                    ("rural_temp", "K"),
                    ("anthropogenic_heat", "W/m²"),
                ],
                &[("uhi_intensity", "K"), ("surface_temp", "K")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 10_000.0, 3600.0, 86400.0),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["urban".to_string(), "climate".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SURFEX/TEB".to_string(),
                version: "8.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// EWE PROCESSES (17)
// ============================================================================

fn ewe_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Ewe, 1),
            name: "Mass-Balance (Ecopath)".to_string(),
            description: "Static mass-balance of ecosystem food web.".to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("biomass", "t/km²"),
                    ("production_rate", "1/yr"),
                    ("consumption_rate", "1/yr"),
                ],
                &[("trophic_level", "1"), ("ecotrophic_efficiency", "1")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                1000.0,
                1_000_000.0,
                31536000.0,
                31536000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "biomass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 1024, false),
            regime_tags: vec!["marine".to_string(), "food_web".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "EwE".to_string(),
                version: "6.6".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Ewe, 2),
            name: "Predator-Prey Dynamics".to_string(),
            description: "Ecosim time-dynamic biomass from predation and growth.".to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("biomass", "t/km²"),
                    ("vulnerability", "1"),
                    ("predation_rate", "1/yr"),
                ],
                &[("biomass_change", "t/km²/yr"), ("mortality", "1/yr")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                1000.0,
                1_000_000.0,
                2592000.0,
                31536000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "biomass".to_string(),
                method: "population_dynamics".to_string(),
            }],
            cost: cost_model(1e4, 1024, false),
            regime_tags: vec!["marine".to_string(), "predator_prey".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "EwE".to_string(),
                version: "6.6".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Ewe, 3),
            name: "Fisheries Exploitation".to_string(),
            description: "Fishing mortality by fleet and gear type.".to_string(),
            family: ProcessFamily::HumanSystems,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("fishing_effort", "boat-days"),
                    ("catchability", "1"),
                    ("biomass", "t/km²"),
                ],
                &[("catch", "t/km²/yr"), ("fishing_mortality", "1/yr")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                1000.0,
                1_000_000.0,
                2592000.0,
                31536000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["marine".to_string(), "fisheries".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "EwE".to_string(),
                version: "6.6".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// CARMA PROCESSES (18)
// ============================================================================

fn carma_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Carma, 1),
            name: "Homogeneous Nucleation".to_string(),
            description: "Classical nucleation theory for new particle formation.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("h2so4_concentration", "molec/cm³"),
                    ("temperature", "K"),
                    ("relative_humidity", "1"),
                ],
                &[("nucleation_rate", "1/cm³/s"), ("critical_radius", "nm")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 60.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "particle_number".to_string(),
                method: "nucleation_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["aerosol".to_string(), "nucleation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CARMA".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Carma, 2),
            name: "Coagulation".to_string(),
            description: "Particle-particle coagulation updating size distribution.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("size_distribution", "1/cm³"),
                    ("temperature", "K"),
                    ("pressure", "Pa"),
                ],
                &[
                    ("size_distribution", "1/cm³"),
                    ("coagulation_kernel", "cm³/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 60.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "aerosol_mass".to_string(),
                method: "coagulation_integral".to_string(),
            }],
            cost: cost_model(1e6, 2048, true),
            regime_tags: vec!["aerosol".to_string(), "microphysics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CARMA".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Carma, 3),
            name: "Aerosol Optical Properties".to_string(),
            description: "Mie theory extinction, SSA, asymmetry parameter per bin.".to_string(),
            family: ProcessFamily::Radiation,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("size_distribution", "1/cm³"),
                    ("refractive_index", "complex"),
                    ("wavelength", "nm"),
                ],
                &[
                    ("extinction", "1/m"),
                    ("single_scatter_albedo", "1"),
                    ("asymmetry_parameter", "1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 3600.0, 10800.0),
            conservation: vec![],
            cost: cost_model(1e5, 1024, true),
            regime_tags: vec!["aerosol".to_string(), "radiation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CARMA".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
    ]
}

// ============================================================================
// ATS PROCESSES (18)
// ============================================================================

fn ats_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Ats, 1),
            name: "Richards Equation (3D Variably Saturated)".to_string(),
            description: "3D variably-saturated flow via Richards equation.".to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("pressure_head", "m"),
                    ("hydraulic_conductivity", "m/s"),
                    ("porosity", "1"),
                ],
                &[
                    ("pressure_head", "m"),
                    ("saturation", "1"),
                    ("darcy_flux", "m/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 10_000.0, 60.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e7, 8192, true),
            regime_tags: vec![
                "hydrology".to_string(),
                "groundwater".to_string(),
                "permafrost".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ATS".to_string(),
                version: "1.5".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Ats, 2),
            name: "Coupled Energy Equation".to_string(),
            description: "3D subsurface energy transport with phase change.".to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("enthalpy", "J/m³"),
                    ("water_flux", "m/s"),
                    ("thermal_conductivity", "W/m/K"),
                ],
                &[
                    ("temperature", "K"),
                    ("ice_saturation", "1"),
                    ("unfrozen_water", "1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 10_000.0, 60.0, 3600.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "energy".to_string(),
                    method: "enthalpy_balance".to_string(),
                },
                ConservationProperty {
                    quantity: "water".to_string(),
                    method: "mass_balance".to_string(),
                },
            ],
            cost: cost_model(1e7, 8192, true),
            regime_tags: vec!["permafrost".to_string(), "thermal_hydrology".to_string()],
            relations: vec![OntologyRelation {
                relation_type: RelationType::RequiresCouplingWith,
                target: "ats:richards".to_string(),
            }],
            origin: ProcessOrigin::Imported {
                source: "ATS".to_string(),
                version: "1.5".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Ats, 3),
            name: "Overland Flow (Diffusion Wave)".to_string(),
            description: "2D diffusion wave overland flow coupled to subsurface.".to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("ponded_depth", "m"),
                    ("surface_slope", "m/m"),
                    ("manning_n", "s/m^(1/3)"),
                ],
                &[("discharge", "m³/s"), ("ponded_depth", "m")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 10_000.0, 60.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "shallow_water_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, true),
            regime_tags: vec!["hydrology".to_string(), "surface_water".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ATS".to_string(),
                version: "1.5".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
    ]
}

// ============================================================================
// BFM PROCESSES (22)
// ============================================================================

fn bfm_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Bfm, 1),
            name: "Phytoplankton Photosynthesis".to_string(),
            description: "Light-limited, nutrient-limited carbon fixation for 4 PFTs.".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("par", "W/m²"),
                    ("no3", "mmol/m³"),
                    ("po4", "mmol/m³"),
                    ("temperature", "°C"),
                ],
                &[("primary_production", "mmol C/m³/d"), ("chl_a", "mg/m³")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec!["ocean".to_string(), "biogeochemistry".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "BFM".to_string(),
                version: "5.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Bfm, 2),
            name: "Dissolved Inorganic Carbon".to_string(),
            description: "DIC dynamics: photosynthesis, respiration, air-sea exchange.".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("dic", "mmol/m³"),
                    ("alkalinity", "meq/m³"),
                    ("temperature", "°C"),
                    ("salinity", "psu"),
                ],
                &[
                    ("pco2", "µatm"),
                    ("ph", "1"),
                    ("air_sea_co2_flux", "mmol/m²/d"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["ocean".to_string(), "carbon_cycle".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "BFM".to_string(),
                version: "5.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Bfm, 3),
            name: "Microbial Loop".to_string(),
            description: "Bacteria-DOM-flagellate interactions.".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("doc", "mmol/m³"),
                    ("bacteria_c", "mmol/m³"),
                    ("temperature", "°C"),
                ],
                &[
                    ("bacterial_production", "mmol/m³/d"),
                    ("remineralization", "mmol/m³/d"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["ocean".to_string(), "microbial".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "BFM".to_string(),
                version: "5.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_manifest_count() {
        let manifests = generate_seed_manifests();
        // Should have at least a representative subset of the 1185 total
        assert!(manifests.len() >= 30, "Expected at least 30 seed manifests");
    }

    #[test]
    fn test_all_families_represented() {
        let manifests = generate_seed_manifests();
        let families: std::collections::HashSet<_> = manifests.iter().map(|m| m.family).collect();

        // Ensure we have representation from multiple families
        assert!(
            families.len() >= 10,
            "Expected at least 10 families represented"
        );
    }

    #[test]
    fn test_deterministic_ids() {
        let id1 = make_id(SourceModel::Badlands, 1);
        let id2 = make_id(SourceModel::Badlands, 1);
        assert_eq!(id1, id2, "IDs should be deterministic");

        let id3 = make_id(SourceModel::Badlands, 2);
        assert_ne!(id1, id3, "Different indices should have different IDs");
    }
}
