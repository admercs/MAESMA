//! Extended seed process manifests for the remaining 31 source models.
//!
//! This module adds representative `ProcessManifest` entries for all source models
//! that were not covered in the initial `seed.rs` generators. Each model contributes
//! 2–5 representative entries capturing its primary process families and fidelity range.

use maesma_core::families::ProcessFamily;
use maesma_core::manifest::{
    ComputeBackend, ConservationProperty, CostModel, CouplingTier, IoContract, ProcessManifest,
    ScaleEnvelope, Variable,
};
use maesma_core::process::{FidelityRung, LifecycleStatus, ProcessId, ProcessOrigin};
use uuid::Uuid;

use crate::seed::SourceModel;

// ============================================================================
// HELPERS (re-use same patterns as seed.rs)
// ============================================================================

fn make_id(source: SourceModel, index: u32) -> ProcessId {
    let name = format!("maesma:{}:{}", source.name(), index);
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes());
    ProcessId(uuid)
}

fn io_contract(inputs: &[(&str, &str)], outputs: &[(&str, &str)]) -> IoContract {
    IoContract {
        inputs: inputs
            .iter()
            .map(|(n, u)| Variable {
                name: n.to_string(),
                unit: u.to_string(),
                description: None,
                dimensions: vec!["x".to_string(), "y".to_string()],
            })
            .collect(),
        outputs: outputs
            .iter()
            .map(|(n, u)| Variable {
                name: n.to_string(),
                unit: u.to_string(),
                description: None,
                dimensions: vec!["x".to_string(), "y".to_string()],
            })
            .collect(),
        parameters: Vec::new(),
    }
}

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

fn cost_model(flops: f64, mem: u64, gpu: bool) -> CostModel {
    CostModel {
        flops_per_cell: flops,
        memory_per_cell: mem,
        gpu_capable: gpu,
    }
}

fn fidelity(s: &str) -> FidelityRung {
    match s {
        "Empirical" => FidelityRung::R0,
        "Intermediate" => FidelityRung::R1,
        "Physics-based" => FidelityRung::R2,
        _ => FidelityRung::R1,
    }
}

/// Generate all extended seed manifests.
pub fn generate_extended_manifests() -> Vec<ProcessManifest> {
    let mut manifests = Vec::with_capacity(120);

    // E3SM (unique processes beyond CESM heritage)
    manifests.extend(e3sm_processes());
    // iLand
    manifests.extend(iland_processes());
    // LPJ-GUESS
    manifests.extend(lpj_guess_processes());
    // ORCHIDEE
    manifests.extend(orchidee_processes());
    // VIC
    manifests.extend(vic_processes());
    // GFDL ESM4
    manifests.extend(gfdl_esm4_processes());
    // PEcAn
    manifests.extend(pecan_processes());
    // LANDIS-II Core
    manifests.extend(landis_core_processes());
    // JULES
    manifests.extend(jules_processes());
    // CABLE
    manifests.extend(cable_processes());
    // CLASSIC
    manifests.extend(classic_processes());
    // SUMMA
    manifests.extend(summa_processes());
    // ED2
    manifests.extend(ed2_processes());
    // PFLOTRAN
    manifests.extend(pflotran_processes());
    // ROMS
    manifests.extend(roms_processes());
    // SWAT
    manifests.extend(swat_processes());
    // GEOS-Chem
    manifests.extend(geos_chem_processes());
    // Delft3D
    manifests.extend(delft3d_processes());
    // LISFLOOD-FP
    manifests.extend(lisflood_fp_processes());
    // LM3-PPA / BiomeE
    manifests.extend(lm3_ppa_processes());
    // SORTIE-ND
    manifests.extend(sortie_nd_processes());
    // JABOWA/FORET
    manifests.extend(jabowa_processes());
    // FORMIND
    manifests.extend(formind_processes());
    // LANDIS-II Extensions
    manifests.extend(landis_ext_processes());
    // UVAFME
    manifests.extend(uvafme_processes());
    // FORCLIM
    manifests.extend(forclim_processes());
    // SORTIE-NG (GPU)
    manifests.extend(sortie_ng_processes());
    // DeepLand
    manifests.extend(deepland_processes());
    // Earth-2
    manifests.extend(earth2_processes());
    // PhiSat-2
    manifests.extend(phisat2_processes());
    // FireBench
    manifests.extend(firebench_processes());
    // MAESTRA/MAESTRO
    manifests.extend(maestra_processes());

    manifests
}

// ============================================================================
// E3SM — UNIQUE PROCESSES BEYOND CESM HERITAGE
// ============================================================================

fn e3sm_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::E3sm, 1),
            name: "MPAS-Ocean Unstructured Dynamics".to_string(),
            description: "Multi-resolution ocean dynamics on SCVT unstructured mesh.".to_string(),
            family: ProcessFamily::Ocean,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("wind_stress", "N/m²"),
                    ("heat_flux", "W/m²"),
                    ("salinity", "PSU"),
                ],
                &[("ssh", "m"), ("temperature", "°C"), ("velocity", "m/s")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 5000.0, 500_000.0, 300.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "volume_integral".to_string(),
            }],
            cost: cost_model(1e8, 16384, true),
            regime_tags: vec![
                "ocean".to_string(),
                "global".to_string(),
                "unstructured".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "E3SM".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::E3sm, 2),
            name: "ELM Land Model".to_string(),
            description:
                "E3SM Land Model with enhanced BGC, permafrost carbon, and hillslope hydrology."
                    .to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("precipitation", "mm/s"),
                    ("temperature", "K"),
                    ("sw_down", "W/m²"),
                ],
                &[
                    ("runoff", "mm/s"),
                    ("et", "mm/s"),
                    ("soil_moisture", "m³/m³"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e6, 4096, false),
            regime_tags: vec![
                "land".to_string(),
                "permafrost".to_string(),
                "bgc".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "E3SM".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::E3sm, 3),
            name: "EAM Atmospheric Physics".to_string(),
            description:
                "E3SM atmospheric model with CLUBB unified boundary layer and P3 microphysics."
                    .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("sst", "K"),
                    ("sea_ice_fraction", "1"),
                    ("lw_up_sfc", "W/m²"),
                ],
                &[
                    ("precipitation", "mm/s"),
                    ("olr", "W/m²"),
                    ("wind_10m", "m/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 25_000.0, 100_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "column_integral".to_string(),
            }],
            cost: cost_model(1e8, 32768, true),
            regime_tags: vec!["atmosphere".to_string(), "global".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "E3SM".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
    ]
}

// ============================================================================
// iLand — INDIVIDUAL-BASED FOREST LANDSCAPE MODEL
// ============================================================================

fn iland_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::ILand, 1),
            name: "Individual Tree Growth (3-PG)".to_string(),
            description: "Light-use efficiency GPP with VPD/temp/soil water modifiers per tree."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("par", "MJ/m²"), ("vpd", "kPa"), ("soil_water", "mm")],
                &[
                    ("gpp", "kgC/tree"),
                    ("npp", "kgC/tree"),
                    ("dbh_increment", "cm"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 2.0, 1000.0, 86400.0, 31_536_000.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec![
                "forest".to_string(),
                "individual".to_string(),
                "temperate".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "iLand".to_string(),
                version: "1.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::ILand, 2),
            name: "Bark Beetle Disturbance".to_string(),
            description: "Beetle population dynamics, host selection, and outbreak triggers."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "°C"),
                    ("host_dbh", "cm"),
                    ("stand_density", "trees/ha"),
                ],
                &[
                    ("beetle_killed_trees", "trees/ha"),
                    ("outbreak_probability", "1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 100.0, 100_000.0, 86400.0, 31_536_000.0),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec![
                "forest".to_string(),
                "disturbance".to_string(),
                "insect".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "iLand".to_string(),
                version: "1.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::ILand, 3),
            name: "Forest Management".to_string(),
            description: "Thinning, clear-cut, shelter-wood prescriptions with scheduling."
                .to_string(),
            family: ProcessFamily::HumanSystems,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("stand_volume", "m³/ha"), ("stand_age", "years")],
                &[("harvested_volume", "m³/ha"), ("residue_c", "kgC/ha")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                100.0,
                100_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e2, 128, false),
            regime_tags: vec!["forest".to_string(), "management".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "iLand".to_string(),
                version: "1.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// LPJ-GUESS — DYNAMIC GLOBAL VEGETATION MODEL
// ============================================================================

fn lpj_guess_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::LpjGuess, 1),
            name: "Photosynthesis (Haxeltine-Prentice)".to_string(),
            description: "Coupled photosynthesis-stomatal conductance for C3/C4 plants."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("par", "W/m²"), ("co2", "ppm"), ("temperature", "°C")],
                &[("gpp", "kgC/m²/day"), ("transpiration", "mm/day")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10_000.0,
                500_000.0,
                86400.0,
                31_536_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["vegetation".to_string(), "global".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LPJ-GUESS".to_string(),
                version: "4.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::LpjGuess, 2),
            name: "Vegetation Dynamics".to_string(),
            description: "Establishment, mortality, competition for light by cohorts.".to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("npp", "kgC/m²/yr"),
                    ("lai", "m²/m²"),
                    ("climate_suitability", "1"),
                ],
                &[
                    ("pft_cover", "1"),
                    ("biomass", "kgC/m²"),
                    ("stand_density", "ind/ha"),
                ],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10_000.0,
                500_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e3, 512, false),
            regime_tags: vec!["vegetation".to_string(), "dynamics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LPJ-GUESS".to_string(),
                version: "4.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::LpjGuess, 3),
            name: "BLAZE Fire".to_string(),
            description: "Process-based fire model with fuel moisture and fire weather."
                .to_string(),
            family: ProcessFamily::Fire,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("fuel_load", "kgC/m²"),
                    ("fuel_moisture", "1"),
                    ("wind_speed", "m/s"),
                ],
                &[
                    ("area_burned", "ha"),
                    ("fire_emissions", "kgC/m²"),
                    ("fire_severity", "1"),
                ],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10_000.0,
                500_000.0,
                86400.0,
                31_536_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["fire".to_string(), "vegetation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LPJ-GUESS".to_string(),
                version: "4.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// ORCHIDEE — LAND SURFACE MODEL
// ============================================================================

fn orchidee_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Orchidee, 1),
            name: "SECHIBA Surface Energy Balance".to_string(),
            description: "Coupled radiation/turbulent flux solution for canopy and soil."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("sw_down", "W/m²"),
                    ("lw_down", "W/m²"),
                    ("wind", "m/s"),
                    ("temperature", "K"),
                ],
                &[
                    ("sensible_heat", "W/m²"),
                    ("latent_heat", "W/m²"),
                    ("ground_heat", "W/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 500_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "surface_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["land".to_string(), "energy_balance".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ORCHIDEE".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Orchidee, 2),
            name: "11-Layer Richards Hydrology (CWRR)".to_string(),
            description: "Van Genuchten parameterised Richards equation solver.".to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("precipitation", "mm/s"),
                    ("et_demand", "mm/s"),
                    ("soil_properties", "1"),
                ],
                &[
                    ("soil_moisture", "m³/m³"),
                    ("runoff", "mm/s"),
                    ("drainage", "mm/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 500_000.0, 1800.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["hydrology".to_string(), "soil".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ORCHIDEE".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Orchidee, 3),
            name: "STOMATE Vegetation Dynamics".to_string(),
            description: "LPJ-derived dynamic vegetation with PFT competition and phenology."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("npp", "kgC/m²/day"),
                    ("temperature", "°C"),
                    ("soil_water", "mm"),
                ],
                &[
                    ("lai", "m²/m²"),
                    ("pft_cover", "1"),
                    ("biomass_c", "kgC/m²"),
                ],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10_000.0,
                500_000.0,
                86400.0,
                31_536_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["vegetation".to_string(), "dynamics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ORCHIDEE".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// VIC — VARIABLE INFILTRATION CAPACITY MODEL
// ============================================================================

fn vic_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Vic, 1),
            name: "Variable Infiltration Capacity Runoff".to_string(),
            description: "Nonlinear soil moisture-storage-capacity distribution for runoff."
                .to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("precipitation", "mm/s"),
                    ("soil_moisture", "mm"),
                    ("infiltration_shape", "1"),
                ],
                &[
                    ("runoff", "mm/s"),
                    ("baseflow", "mm/s"),
                    ("soil_moisture", "mm"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["hydrology".to_string(), "runoff".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "VIC".to_string(),
                version: "5.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Vic, 2),
            name: "Energy-Balance Snow (2-Layer)".to_string(),
            description: "Mass and energy balance snow model with liquid water retention."
                .to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("sw_down", "W/m²"),
                    ("lw_down", "W/m²"),
                    ("temperature", "K"),
                    ("precipitation", "mm/s"),
                ],
                &[("swe", "mm"), ("snowmelt", "mm/s"), ("snow_albedo", "1")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "water".to_string(),
                    method: "mass_balance".to_string(),
                },
                ConservationProperty {
                    quantity: "energy".to_string(),
                    method: "energy_balance".to_string(),
                },
            ],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["snow".to_string(), "cryosphere".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "VIC".to_string(),
                version: "5.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Vic, 3),
            name: "Frozen Soil Permafrost".to_string(),
            description: "Ice lens formation and thermal conductivity modification.".to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("soil_temperature", "K"), ("soil_moisture", "m³/m³")],
                &[("ice_content", "m³/m³"), ("active_layer_depth", "m")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "energy_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["permafrost".to_string(), "cryosphere".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "VIC".to_string(),
                version: "5.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// NOAA-GFDL ESM4
// ============================================================================

fn gfdl_esm4_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::GfdlEsm4, 1),
            name: "FV3 Dynamical Core".to_string(),
            description: "Finite-volume cubed-sphere non-hydrostatic atmosphere.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("sst", "K"), ("albedo", "1"), ("trace_gas", "kg/kg")],
                &[("wind", "m/s"), ("temperature", "K"), ("pressure", "Pa")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 25_000.0, 200_000.0, 600.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "column_integral".to_string(),
            }],
            cost: cost_model(1e8, 32768, true),
            regime_tags: vec!["atmosphere".to_string(), "global".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "GFDL-ESM4".to_string(),
                version: "ESM4.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::GfdlEsm4, 2),
            name: "COBALT Ocean BGC".to_string(),
            description: "Carbon-Ocean Biogeochemistry and Lower Trophics marine ecosystem."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("no3", "mmol/m³"),
                    ("po4", "mmol/m³"),
                    ("fe", "nmol/m³"),
                    ("par", "W/m²"),
                ],
                &[
                    ("primary_production", "mmol/m³/d"),
                    ("export_production", "mmol/m²/d"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 25_000.0, 200_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e6, 4096, false),
            regime_tags: vec!["ocean".to_string(), "biogeochemistry".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "GFDL-ESM4".to_string(),
                version: "ESM4.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::GfdlEsm4, 3),
            name: "LM4 Cohort Land".to_string(),
            description: "Cohort-based land with soil C-N, plant hydraulics, and river routing."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("precipitation", "mm/s"),
                    ("temperature", "K"),
                    ("co2", "ppm"),
                ],
                &[("npp", "kgC/m²/yr"), ("et", "mm/s"), ("runoff", "mm/s")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 25_000.0, 200_000.0, 1800.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec![
                "land".to_string(),
                "vegetation".to_string(),
                "cohort".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "GFDL-ESM4".to_string(),
                version: "ESM4.1".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// PEcAn — PREDICTIVE ECOSYSTEM ANALYZER
// ============================================================================

fn pecan_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Pecan, 1),
            name: "Bayesian Calibration".to_string(),
            description: "Hierarchical Bayes inversion for Vcmax, Jmax from leaf gas exchange."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("leaf_gas_exchange", "µmol/m²/s"),
                    ("temperature", "°C"),
                    ("par", "µmol/m²/s"),
                ],
                &[
                    ("vcmax", "µmol/m²/s"),
                    ("jmax", "µmol/m²/s"),
                    ("uncertainty", "1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 100_000.0, 86400.0, 31_536_000.0),
            conservation: vec![],
            cost: cost_model(1e6, 2048, false),
            regime_tags: vec!["calibration".to_string(), "bayesian".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PEcAn".to_string(),
                version: "1.7".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Pecan, 2),
            name: "Ensemble Data Assimilation".to_string(),
            description: "Ensemble Kalman filter and particle filter for state updating."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("model_state", "1"),
                    ("observations", "1"),
                    ("uncertainty", "1"),
                ],
                &[("updated_state", "1"), ("posterior_uncertainty", "1")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 100_000.0, 86400.0, 31_536_000.0),
            conservation: vec![],
            cost: cost_model(1e5, 4096, false),
            regime_tags: vec!["data_assimilation".to_string(), "ensemble".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PEcAn".to_string(),
                version: "1.7".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// LANDIS-II CORE
// ============================================================================

fn landis_core_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::LandisII, 1),
            name: "Succession Extension Interface".to_string(),
            description: "Base class for forest succession state variables and dynamics."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("climate", "1"), ("soil_type", "1")],
                &[("cohort_biomass", "g/m²"), ("species_presence", "1")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                100.0,
                100_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["forest".to_string(), "landscape".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LANDIS-II".to_string(),
                version: "7.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::LandisII, 2),
            name: "Seed Dispersal".to_string(),
            description: "Species-specific effective and maximum dispersal distances.".to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("seed_source", "1"), ("distance", "m")],
                &[("seed_rain", "seeds/m²"), ("colonisation_prob", "1")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                100.0,
                100_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e2, 128, false),
            regime_tags: vec!["forest".to_string(), "dispersal".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LANDIS-II".to_string(),
                version: "7.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// JULES — JOINT UK LAND ENVIRONMENT SIMULATOR
// ============================================================================

fn jules_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Jules, 1),
            name: "Surface Energy Balance".to_string(),
            description: "Coupled sensible/latent/ground/radiative flux solution via Penman-Monteith on tiles.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("sw_down", "W/m²"), ("lw_down", "W/m²"), ("wind", "m/s"), ("temperature", "K")],
                &[("sensible_heat", "W/m²"), ("latent_heat", "W/m²"), ("ground_heat", "W/m²")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 200_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty { quantity: "energy".to_string(), method: "surface_balance".to_string() }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["land".to_string(), "energy_balance".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "JULES".to_string(), version: "7.3".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Jules, 2),
            name: "TRIFFID Dynamic Vegetation".to_string(),
            description: "Lotka-Volterra competition among 5 PFTs for fractional cover.".to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("npp", "kgC/m²/yr"), ("temperature", "K"), ("precipitation", "mm/yr")],
                &[("pft_fraction", "1"), ("lai", "m²/m²"), ("vegetation_height", "m")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 200_000.0, 31_536_000.0, 315_360_000.0),
            conservation: vec![ConservationProperty { quantity: "carbon".to_string(), method: "mass_balance".to_string() }],
            cost: cost_model(1e3, 512, false),
            regime_tags: vec!["vegetation".to_string(), "dynamics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "JULES".to_string(), version: "7.3".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Jules, 3),
            name: "INFERNO Fire".to_string(),
            description: "Interactive fire with human/lightning ignition, fuel moisture/load thresholds, burnt area.".to_string(),
            family: ProcessFamily::Fire,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("fuel_load", "kgC/m²"), ("fuel_moisture", "1"), ("population_density", "people/km²")],
                &[("area_burned", "1"), ("fire_emissions", "kgC/m²"), ("fire_count", "1/day")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 200_000.0, 86400.0, 31_536_000.0),
            conservation: vec![ConservationProperty { quantity: "carbon".to_string(), method: "mass_balance".to_string() }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["fire".to_string(), "vegetation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "JULES".to_string(), version: "7.3".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// CABLE — COMMUNITY ATMOSPHERE BIOSPHERE LAND EXCHANGE
// ============================================================================

fn cable_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Cable, 1),
            name: "Two-Leaf Canopy Model".to_string(),
            description: "Separate sunlit/shaded leaf photosynthesis distinguishing direct/diffuse radiation.".to_string(),
            family: ProcessFamily::Radiation,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("sw_direct", "W/m²"), ("sw_diffuse", "W/m²"), ("lai", "m²/m²")],
                &[("apar_sunlit", "W/m²"), ("apar_shaded", "W/m²"), ("canopy_albedo", "1")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 200_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty { quantity: "energy".to_string(), method: "radiation_balance".to_string() }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["canopy".to_string(), "radiation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "CABLE".to_string(), version: "3.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Cable, 2),
            name: "CASA-CNP Biogeochemistry".to_string(),
            description: "Carbon, nitrogen, phosphorus cycling through plant/litter/soil pools.".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("npp", "kgC/m²/yr"), ("temperature", "°C"), ("soil_moisture", "m³/m³")],
                &[("soil_c", "kgC/m²"), ("soil_n", "kgN/m²"), ("soil_p", "kgP/m²")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 200_000.0, 86400.0, 31_536_000.0),
            conservation: vec![
                ConservationProperty { quantity: "carbon".to_string(), method: "mass_balance".to_string() },
                ConservationProperty { quantity: "nitrogen".to_string(), method: "mass_balance".to_string() },
            ],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["biogeochemistry".to_string(), "cnp".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "CABLE".to_string(), version: "3.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Cable, 3),
            name: "POP Demographic Model".to_string(),
            description: "Patch of Patches: age-class patches with self-thinning and disturbance gaps.".to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("npp", "kgC/m²/yr"), ("disturbance_rate", "1/yr")],
                &[("stand_age", "years"), ("stem_density", "trees/ha"), ("wood_c", "kgC/m²")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10_000.0, 200_000.0, 31_536_000.0, 315_360_000.0),
            conservation: vec![ConservationProperty { quantity: "carbon".to_string(), method: "mass_balance".to_string() }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["vegetation".to_string(), "demographic".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "CABLE".to_string(), version: "3.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// CLASSIC — CANADIAN LAND SURFACE SCHEME
// ============================================================================

fn classic_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Classic, 1),
            name: "CLASS Surface Energy Balance".to_string(),
            description: "Implicit coupled solution over 4 sub-areas.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("sw_down", "W/m²"), ("lw_down", "W/m²"), ("wind", "m/s")],
                &[
                    ("sensible_heat", "W/m²"),
                    ("latent_heat", "W/m²"),
                    ("surface_temp", "K"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10_000.0, 500_000.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "surface_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["land".to_string(), "energy_balance".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CLASSIC".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Classic, 2),
            name: "CTEM Dynamic Vegetation".to_string(),
            description: "Competition among 9 PFTs driven by NPP, mortality, bioclimatic limits."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("npp", "kgC/m²/yr"),
                    ("temperature", "K"),
                    ("precipitation", "mm/yr"),
                ],
                &[("pft_fraction", "1"), ("biomass", "kgC/m²")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10_000.0,
                500_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 512, false),
            regime_tags: vec!["vegetation".to_string(), "dynamics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CLASSIC".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Classic, 3),
            name: "CTEM-Fire".to_string(),
            description: "Area burned from fire occurrence probability × spread rate × duration."
                .to_string(),
            family: ProcessFamily::Fire,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("fuel_load", "kgC/m²"),
                    ("soil_moisture", "1"),
                    ("population_density", "people/km²"),
                ],
                &[("area_burned", "fraction"), ("fire_c_emission", "kgC/m²")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10_000.0,
                500_000.0,
                86400.0,
                31_536_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["fire".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "CLASSIC".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// SUMMA — STRUCTURE FOR UNIFYING MULTIPLE MODELING ALTERNATIVES
// ============================================================================

fn summa_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Summa, 1),
            name: "Multi-Physics Richards Hydrology".to_string(),
            description: "Multi-layer Richards equation with switchable hydraulic functions."
                .to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("precipitation", "mm/s"),
                    ("et_demand", "mm/s"),
                    ("soil_params", "1"),
                ],
                &[
                    ("soil_moisture", "m³/m³"),
                    ("runoff", "mm/s"),
                    ("drainage", "mm/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 100_000.0, 900.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["hydrology".to_string(), "multi-physics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SUMMA".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Summa, 2),
            name: "Switchable Snow Scheme".to_string(),
            description:
                "Dynamic layer splitting/merging with variable compaction and albedo models."
                    .to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("snowfall", "mm/s"),
                    ("temperature", "K"),
                    ("sw_down", "W/m²"),
                ],
                &[
                    ("swe", "mm"),
                    ("snow_depth", "m"),
                    ("snow_albedo", "1"),
                    ("melt", "mm/s"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 100_000.0, 900.0, 86400.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "water".to_string(),
                    method: "mass_balance".to_string(),
                },
                ConservationProperty {
                    quantity: "energy".to_string(),
                    method: "energy_balance".to_string(),
                },
            ],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["snow".to_string(), "multi-physics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SUMMA".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Summa, 3),
            name: "Switchable Surface Energy Balance".to_string(),
            description:
                "Implicit coupled surface temperature with multiple turbulent flux formulations."
                    .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("sw_down", "W/m²"),
                    ("lw_down", "W/m²"),
                    ("air_temp", "K"),
                    ("wind", "m/s"),
                ],
                &[
                    ("sensible_heat", "W/m²"),
                    ("latent_heat", "W/m²"),
                    ("ground_heat", "W/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 100_000.0, 900.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "surface_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["energy_balance".to_string(), "multi-physics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SUMMA".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// ED2 — ECOSYSTEM DEMOGRAPHY MODEL 2
// ============================================================================

fn ed2_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Ed2, 1),
            name: "Multi-Layer Canopy Radiation".to_string(),
            description:
                "Exact multi-layer two-stream radiation within vertically resolved canopy strata."
                    .to_string(),
            family: ProcessFamily::Radiation,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("sw_down", "W/m²"),
                    ("lai_profile", "m²/m²"),
                    ("leaf_angle", "deg"),
                ],
                &[("apar_profile", "W/m²"), ("canopy_albedo", "1")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10.0, 10_000.0, 900.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "radiation_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["canopy".to_string(), "radiation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ED2".to_string(),
                version: "2.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Ed2, 2),
            name: "Cohort Dynamics".to_string(),
            description: "Size-structured cohort fusion/fission preserving demographic resolution."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("npp", "kgC/tree"),
                    ("mortality_rate", "1/yr"),
                    ("recruitment", "trees/ha/yr"),
                ],
                &[("dbh", "cm"), ("height", "m"), ("stem_density", "trees/ha")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10.0, 10_000.0, 86400.0, 31_536_000.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["forest".to_string(), "demography".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ED2".to_string(),
                version: "2.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Ed2, 3),
            name: "Patch Dynamics".to_string(),
            description: "Age-structured patch mosaic representing disturbance history."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("disturbance_rate", "1/yr"), ("patch_age", "years")],
                &[("patch_area", "m²"), ("successional_stage", "1")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10.0,
                100_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["forest".to_string(), "disturbance".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "ED2".to_string(),
                version: "2.2".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// PFLOTRAN — SUBSURFACE REACTIVE TRANSPORT
// ============================================================================

fn pflotran_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Pflotran, 1),
            name: "Reactive Transport".to_string(),
            description: "Advection-dispersion-reaction for multicomponent solute transport."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("concentration", "mol/L"),
                    ("velocity", "m/s"),
                    ("porosity", "1"),
                ],
                &[("concentration", "mol/L"), ("reaction_rate", "mol/L/s")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 10_000.0, 60.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e7, 8192, true),
            regime_tags: vec!["subsurface".to_string(), "reactive_transport".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PFLOTRAN".to_string(),
                version: "5.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Pflotran, 2),
            name: "Mineral Precipitation/Dissolution".to_string(),
            description: "Transition State Theory rate laws with surface-area-dependent kinetics."
                .to_string(),
            family: ProcessFamily::Geology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("mineral_surface_area", "m²/m³"),
                    ("saturation_index", "1"),
                    ("temperature", "K"),
                ],
                &[("dissolution_rate", "mol/m³/s"), ("porosity_change", "1")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 3600.0, 31_536_000.0),
            conservation: vec![ConservationProperty {
                quantity: "mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e6, 4096, false),
            regime_tags: vec!["geology".to_string(), "geochemistry".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PFLOTRAN".to_string(),
                version: "5.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Pflotran, 3),
            name: "Geomechanics (Poroelasticity)".to_string(),
            description: "Linear poroelasticity coupling pore pressure to solid displacement."
                .to_string(),
            family: ProcessFamily::Geology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("pressure", "Pa"),
                    ("displacement", "m"),
                    ("youngs_modulus", "Pa"),
                ],
                &[("stress", "Pa"), ("strain", "1"), ("updated_porosity", "1")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 10_000.0, 3600.0, 31_536_000.0),
            conservation: vec![ConservationProperty {
                quantity: "momentum".to_string(),
                method: "force_balance".to_string(),
            }],
            cost: cost_model(1e7, 8192, true),
            regime_tags: vec!["geology".to_string(), "geomechanics".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PFLOTRAN".to_string(),
                version: "5.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// ROMS — REGIONAL OCEAN MODELING SYSTEM
// ============================================================================

fn roms_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Roms, 1),
            name: "3D Primitive Equations".to_string(),
            description: "Hydrostatic Boussinesq momentum with free surface and terrain-following coordinates.".to_string(),
            family: ProcessFamily::Ocean,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("wind_stress", "N/m²"), ("heat_flux", "W/m²"), ("tidal_bc", "m")],
                &[("ssh", "m"), ("temperature", "°C"), ("salinity", "PSU"), ("velocity", "m/s")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 100_000.0, 60.0, 3600.0),
            conservation: vec![
                ConservationProperty { quantity: "mass".to_string(), method: "volume_integral".to_string() },
                ConservationProperty { quantity: "energy".to_string(), method: "energy_balance".to_string() },
            ],
            cost: cost_model(1e7, 8192, true),
            regime_tags: vec!["ocean".to_string(), "coastal".to_string(), "regional".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "ROMS".to_string(), version: "4.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Roms, 2),
            name: "Sediment Transport".to_string(),
            description: "Non-cohesive bed load and suspended load with multiple grain classes.".to_string(),
            family: ProcessFamily::Geomorphology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("bed_shear_stress", "Pa"), ("grain_size", "m"), ("concentration", "kg/m³")],
                &[("bedload_flux", "kg/m/s"), ("suspended_concentration", "kg/m³"), ("bed_change", "m")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 100.0, 100_000.0, 60.0, 86400.0),
            conservation: vec![ConservationProperty { quantity: "sediment_mass".to_string(), method: "mass_balance".to_string() }],
            cost: cost_model(1e6, 4096, false),
            regime_tags: vec!["sediment".to_string(), "coastal".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "ROMS".to_string(), version: "4.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Roms, 3),
            name: "NPZD Biogeochemistry".to_string(),
            description: "Nutrient-Phytoplankton-Zooplankton-Detritus pelagic ecosystem.".to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("no3", "mmol/m³"), ("par", "W/m²"), ("temperature", "°C")],
                &[("phyto", "mmol/m³"), ("zoo", "mmol/m³"), ("export", "mmol/m²/d")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 100_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty { quantity: "nitrogen".to_string(), method: "mass_balance".to_string() }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec!["ocean".to_string(), "biogeochemistry".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported { source: "ROMS".to_string(), version: "4.0".to_string() },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// SWAT — SOIL AND WATER ASSESSMENT TOOL
// ============================================================================

fn swat_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Swat, 1),
            name: "SCS Curve Number Runoff".to_string(),
            description: "Modified SCS Curve Number method with antecedent moisture.".to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Empirical"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("precipitation", "mm"),
                    ("curve_number", "1"),
                    ("soil_moisture", "mm"),
                ],
                &[("surface_runoff", "mm"), ("infiltration", "mm")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 500_000.0, 86400.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["hydrology".to_string(), "watershed".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SWAT".to_string(),
                version: "2012".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Swat, 2),
            name: "MUSLE Sediment Yield".to_string(),
            description: "Modified Universal Soil Loss Equation driven by runoff energy."
                .to_string(),
            family: ProcessFamily::Geomorphology,
            rung: fidelity("Empirical"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("runoff_volume", "m³"),
                    ("peak_rate", "m³/s"),
                    ("soil_erodibility", "1"),
                ],
                &[("sediment_yield", "tonnes"), ("enrichment_ratio", "1")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 500_000.0, 86400.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "sediment_mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e2, 128, false),
            regime_tags: vec!["erosion".to_string(), "watershed".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SWAT".to_string(),
                version: "2012".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Swat, 3),
            name: "Crop Growth (simplified EPIC)".to_string(),
            description:
                "Heat-unit phenology, radiation-use-efficiency biomass, nutrient/water stress."
                    .to_string(),
            family: ProcessFamily::HumanSystems,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "°C"),
                    ("solar_radiation", "MJ/m²"),
                    ("water_stress", "1"),
                ],
                &[
                    ("biomass", "kg/ha"),
                    ("yield", "kg/ha"),
                    ("harvest_index", "1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1000.0, 500_000.0, 86400.0, 31_536_000.0),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["crop".to_string(), "agriculture".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SWAT".to_string(),
                version: "2012".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// GEOS-Chem — ATMOSPHERIC CHEMISTRY TRANSPORT
// ============================================================================

fn geos_chem_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::GeosChem, 1),
            name: "Full Tropospheric Chemistry".to_string(),
            description: "~350 reaction mechanism with HOx, NOx, Ox, VOC, halogen chemistry."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("species_concentration", "mol/mol"),
                    ("temperature", "K"),
                    ("pressure", "Pa"),
                    ("j_values", "1/s"),
                ],
                &[
                    ("species_tendency", "mol/mol/s"),
                    ("ozone", "mol/mol"),
                    ("oh", "mol/mol"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 25_000.0, 500_000.0, 600.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e7, 16384, true),
            regime_tags: vec!["chemistry".to_string(), "troposphere".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "GEOS-Chem".to_string(),
                version: "14.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::GeosChem, 2),
            name: "Aerosol Thermodynamics (ISORROPIA-II)".to_string(),
            description: "Thermodynamic equilibrium for HNO3-NH3-H2SO4 system.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("hno3", "µg/m³"),
                    ("nh3", "µg/m³"),
                    ("h2so4", "µg/m³"),
                    ("rh", "1"),
                ],
                &[
                    ("no3_aerosol", "µg/m³"),
                    ("nh4_aerosol", "µg/m³"),
                    ("water", "µg/m³"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 25_000.0, 500_000.0, 600.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec!["aerosol".to_string(), "chemistry".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "GEOS-Chem".to_string(),
                version: "14.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::GeosChem, 3),
            name: "Mercury Cycle".to_string(),
            description:
                "Elemental/reactive/particulate Hg emissions, oxidation, deposition, re-emission."
                    .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("hg0", "ng/m³"), ("br", "ppt"), ("oh", "mol/cm³")],
                &[("hg2", "ng/m³"), ("hg_deposition", "µg/m²/yr")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 25_000.0, 500_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec!["mercury".to_string(), "biogeochemistry".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "GEOS-Chem".to_string(),
                version: "14.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// Delft3D — COASTAL AND ESTUARINE HYDRODYNAMICS
// ============================================================================

fn delft3d_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Delft3d, 1),
            name: "Shallow Water Equations".to_string(),
            description: "2D/3D sigma-layer hydrostatic Navier-Stokes with wetting/drying."
                .to_string(),
            family: ProcessFamily::Ocean,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("wind_stress", "N/m²"),
                    ("tidal_bc", "m"),
                    ("river_discharge", "m³/s"),
                ],
                &[
                    ("water_level", "m"),
                    ("velocity", "m/s"),
                    ("salinity", "PSU"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10.0, 50_000.0, 10.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "mass".to_string(),
                method: "volume_integral".to_string(),
            }],
            cost: cost_model(1e7, 8192, true),
            regime_tags: vec!["coastal".to_string(), "estuarine".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Delft3D".to_string(),
                version: "FM".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::Delft3d, 2),
            name: "Cohesive Sediment Transport".to_string(),
            description: "Krone/Partheniades erosion/deposition for mud with flocculation."
                .to_string(),
            family: ProcessFamily::Geomorphology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("bed_shear_stress", "Pa"),
                    ("critical_shear", "Pa"),
                    ("settling_velocity", "m/s"),
                ],
                &[
                    ("suspended_mud", "kg/m³"),
                    ("deposition_flux", "kg/m²/s"),
                    ("bed_change", "m"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10.0, 50_000.0, 10.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "sediment_mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e6, 4096, false),
            regime_tags: vec!["sediment".to_string(), "cohesive".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Delft3D".to_string(),
                version: "FM".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Delft3d, 3),
            name: "DELWAQ Water Quality".to_string(),
            description:
                "100+ water quality processes: BOD/DO, nutrients, algae, toxics, heavy metals."
                    .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("do", "mg/L"),
                    ("bod", "mg/L"),
                    ("no3", "mg/L"),
                    ("po4", "mg/L"),
                ],
                &[("do", "mg/L"), ("chlorophyll_a", "µg/L"), ("nh4", "mg/L")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10.0, 50_000.0, 3600.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "mass".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec!["water_quality".to_string(), "estuarine".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Delft3D".to_string(),
                version: "FM".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// LISFLOOD-FP — LARGE-SCALE FLOOD INUNDATION
// ============================================================================

fn lisflood_fp_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::LisfloodFp, 1),
            name: "Sub-Grid Channel Flow".to_string(),
            description:
                "1D kinematic/diffusion wave in sub-grid channels with floodplain coupling."
                    .to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("discharge_bc", "m³/s"),
                    ("channel_geometry", "m"),
                    ("manning_n", "1"),
                ],
                &[
                    ("water_depth", "m"),
                    ("discharge", "m³/s"),
                    ("flood_extent", "m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10.0, 100_000.0, 1.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e5, 1024, true),
            regime_tags: vec!["flood".to_string(), "inundation".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LISFLOOD-FP".to_string(),
                version: "8.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::LisfloodFp, 2),
            name: "2D Floodplain Inundation".to_string(),
            description: "Inertial shallow water equations on raster grid for floodplain flow."
                .to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("dem", "m"), ("manning_n", "1"), ("rainfall", "mm/hr")],
                &[
                    ("water_depth", "m"),
                    ("velocity", "m/s"),
                    ("inundated_area", "m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 10.0, 100_000.0, 0.1, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "water".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e6, 2048, true),
            regime_tags: vec!["flood".to_string(), "2d".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LISFLOOD-FP".to_string(),
                version: "8.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
        },
    ]
}

// ============================================================================
// LM3-PPA / BiomeE
// ============================================================================

fn lm3_ppa_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Lm3Ppa, 1),
            name: "Perfect Plasticity Approximation".to_string(),
            description: "Crown packing / canopy closure via PPA — O(N log N) instead of O(N²)."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("crown_area", "m²"), ("tree_height", "m"), ("dbh", "cm")],
                &[
                    ("canopy_layer", "1"),
                    ("light_availability", "1"),
                    ("lai_profile", "m²/m²"),
                ],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10.0,
                100_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["forest".to_string(), "canopy".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LM3-PPA/BiomeE".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Lm3Ppa, 2),
            name: "BiomeE C-N-P".to_string(),
            description: "Coupled C-N-P biogeochemistry with soil microbial decomposition."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("litter_c", "kgC/m²"),
                    ("temperature", "°C"),
                    ("soil_moisture", "1"),
                ],
                &[
                    ("soil_resp", "kgC/m²/yr"),
                    ("n_mineral", "kgN/m²"),
                    ("p_avail", "kgP/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10.0, 100_000.0, 86400.0, 31_536_000.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "carbon".to_string(),
                    method: "mass_balance".to_string(),
                },
                ConservationProperty {
                    quantity: "nitrogen".to_string(),
                    method: "mass_balance".to_string(),
                },
            ],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["biogeochemistry".to_string(), "cnp".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LM3-PPA/BiomeE".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// SORTIE-ND — SPATIALLY EXPLICIT INDIVIDUAL-BASED FOREST MODEL
// ============================================================================

fn sortie_nd_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::SortieNd, 1),
            name: "Hemispheric Light Competition".to_string(),
            description: "Individual tree light interception via hemispheric photo projections."
                .to_string(),
            family: ProcessFamily::Radiation,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("tree_positions", "m"),
                    ("crown_radius", "m"),
                    ("crown_depth", "m"),
                ],
                &[("gli", "1"), ("par_absorbed", "µmol/m²/s")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 2.0, 1000.0, 31_536_000.0, 315_360_000.0),
            conservation: vec![],
            cost: cost_model(1e5, 1024, false),
            regime_tags: vec!["forest".to_string(), "individual".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SORTIE-ND".to_string(),
                version: "7.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::SortieNd, 2),
            name: "Diameter Growth".to_string(),
            description: "Species-specific NCI-based diameter increment with competition indices."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("gli", "1"), ("dbh", "cm"), ("nci", "1")],
                &[("dbh_increment", "cm/yr"), ("height_increment", "m/yr")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 2.0, 1000.0, 31_536_000.0, 315_360_000.0),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["forest".to_string(), "growth".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SORTIE-ND".to_string(),
                version: "7.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// JABOWA / FORET — CLASSIC GAP MODELS
// ============================================================================

fn jabowa_processes() -> Vec<ProcessManifest> {
    vec![ProcessManifest {
        id: make_id(SourceModel::JabowaForet, 1),
        name: "Gap-Phase Succession".to_string(),
        description: "Diameter-increment growth in canopy gaps with species competition."
            .to_string(),
        family: ProcessFamily::Ecology,
        rung: fidelity("Empirical"),
        version: "1.0.0".to_string(),
        io: io_contract(
            &[
                ("light_index", "1"),
                ("growing_degree_days", "°C·d"),
                ("soil_moisture", "1"),
            ],
            &[
                ("dbh_increment", "cm/yr"),
                ("mortality_flag", "1"),
                ("recruitment", "trees"),
            ],
        ),
        scale: scale_envelope(CouplingTier::Slow, 10.0, 500.0, 31_536_000.0, 315_360_000.0),
        conservation: vec![],
        cost: cost_model(1e2, 128, false),
        regime_tags: vec!["forest".to_string(), "gap_model".to_string()],
        relations: Vec::new(),
        origin: ProcessOrigin::Imported {
            source: "JABOWA/FORET".to_string(),
            version: "3.0".to_string(),
        },
        lifecycle: LifecycleStatus::Production,
        backends: vec![ComputeBackend::Cpu],
    }]
}

// ============================================================================
// FORMIND — INDIVIDUAL-BASED TROPICAL FOREST MODEL
// ============================================================================

fn formind_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Formind, 1),
            name: "Individual Tree Competition".to_string(),
            description: "Height-structured light competition in 20m×20m patches.".to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("par_above", "W/m²"),
                    ("tree_height", "m"),
                    ("crown_area", "m²"),
                ],
                &[("absorbed_par", "W/m²"), ("growth_rate", "cm/yr")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                20.0,
                10_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["forest".to_string(), "tropical".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "FORMIND".to_string(),
                version: "4.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Formind, 2),
            name: "Logging & Selective Harvest".to_string(),
            description: "Reduced-impact and conventional logging with collateral damage."
                .to_string(),
            family: ProcessFamily::HumanSystems,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("dbh_threshold", "cm"), ("harvest_intensity", "m³/ha")],
                &[
                    ("harvested_volume", "m³/ha"),
                    ("collateral_damage", "trees/ha"),
                ],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                20.0,
                100_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e2, 128, false),
            regime_tags: vec![
                "forest".to_string(),
                "management".to_string(),
                "tropical".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "FORMIND".to_string(),
                version: "4.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// LANDIS-II EXTENSIONS
// ============================================================================

fn landis_ext_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::LandisNecn, 1),
            name: "NECN Succession".to_string(),
            description: "Net Ecosystem Carbon Nitrogen succession with soil decomposition."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "°C"),
                    ("precipitation", "mm"),
                    ("soil_organic_c", "gC/m²"),
                ],
                &[
                    ("npp", "gC/m²/yr"),
                    ("nee", "gC/m²/yr"),
                    ("soil_resp", "gC/m²/yr"),
                ],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                100.0,
                100_000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![
                ConservationProperty {
                    quantity: "carbon".to_string(),
                    method: "mass_balance".to_string(),
                },
                ConservationProperty {
                    quantity: "nitrogen".to_string(),
                    method: "mass_balance".to_string(),
                },
            ],
            cost: cost_model(1e4, 512, false),
            regime_tags: vec!["forest".to_string(), "succession".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LANDIS-II NECN".to_string(),
                version: "7.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::LandisNecn, 2),
            name: "SCRPPLE Fire".to_string(),
            description: "Social-Climate Related Pyrogenic Processes and their Landscape Effects."
                .to_string(),
            family: ProcessFamily::Fire,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("fire_weather_index", "1"),
                    ("fuel_load", "g/m²"),
                    ("suppression_effort", "1"),
                ],
                &[
                    ("area_burned", "ha"),
                    ("fire_severity", "1"),
                    ("fire_emissions_c", "gC/m²"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 100.0, 100_000.0, 86400.0, 31_536_000.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["fire".to_string(), "landscape".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "LANDIS-II SCRPPLE".to_string(),
                version: "3.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// UVAFME — UNIVERSITY OF VIRGINIA FOREST MODEL ENHANCED
// ============================================================================

fn uvafme_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Uvafme, 1),
            name: "Arctic-Boreal Individual Tree Dynamics".to_string(),
            description:
                "Individual tree growth, mortality, regeneration tuned for boreal/arctic treeline."
                    .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("growing_degree_days", "°C·d"),
                    ("permafrost_depth", "m"),
                    ("soil_moisture", "1"),
                ],
                &[
                    ("dbh", "cm"),
                    ("height", "m"),
                    ("stem_density", "trees/ha"),
                    ("biomass", "kgC/ha"),
                ],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10.0,
                1000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance".to_string(),
            }],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec![
                "forest".to_string(),
                "boreal".to_string(),
                "arctic".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "UVAFME".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        ProcessManifest {
            id: make_id(SourceModel::Uvafme, 2),
            name: "Permafrost-Vegetation Feedback".to_string(),
            description:
                "Active layer depth effects on tree growth and soil respiration at treeline."
                    .to_string(),
            family: ProcessFamily::Cryosphere,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("soil_temperature_profile", "°C"),
                    ("organic_layer_depth", "m"),
                ],
                &[("active_layer_depth", "m"), ("root_available_depth", "m")],
            ),
            scale: scale_envelope(
                CouplingTier::Slow,
                10.0,
                1000.0,
                31_536_000.0,
                315_360_000.0,
            ),
            conservation: vec![],
            cost: cost_model(1e3, 256, false),
            regime_tags: vec!["permafrost".to_string(), "treeline".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "UVAFME".to_string(),
                version: "2.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
    ]
}

// ============================================================================
// FORCLIM — FOREST DYNAMICS UNDER CLIMATE CHANGE
// ============================================================================

fn forclim_processes() -> Vec<ProcessManifest> {
    vec![ProcessManifest {
        id: make_id(SourceModel::Forclim, 1),
        name: "Patch-Based Forest Dynamics".to_string(),
        description: "Climate-sensitive gap model with bioclimatic species envelopes.".to_string(),
        family: ProcessFamily::Ecology,
        rung: fidelity("Intermediate"),
        version: "1.0.0".to_string(),
        io: io_contract(
            &[
                ("temperature", "°C"),
                ("precipitation", "mm/yr"),
                ("growing_degree_days", "°C·d"),
            ],
            &[
                ("basal_area", "m²/ha"),
                ("species_composition", "1"),
                ("stand_volume", "m³/ha"),
            ],
        ),
        scale: scale_envelope(
            CouplingTier::Slow,
            10.0,
            1000.0,
            31_536_000.0,
            315_360_000.0,
        ),
        conservation: vec![],
        cost: cost_model(1e3, 256, false),
        regime_tags: vec![
            "forest".to_string(),
            "climate_change".to_string(),
            "european".to_string(),
        ],
        relations: Vec::new(),
        origin: ProcessOrigin::Imported {
            source: "ForClim".to_string(),
            version: "4.0".to_string(),
        },
        lifecycle: LifecycleStatus::Production,
        backends: vec![ComputeBackend::Cpu],
    }]
}

// ============================================================================
// SORTIE-NG (ERICKSON) — GPU-NATIVE INDIVIDUAL-BASED FOREST MODEL
// ============================================================================

fn sortie_ng_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::SortieNg, 1),
            name: "GPU Hemispheric Light Integration".to_string(),
            description:
                "CUDA-parallelised hemispheric photo computation replacing O(N²) CPU loop."
                    .to_string(),
            family: ProcessFamily::Radiation,
            rung: fidelity("Physics-based"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("tree_positions", "m"),
                    ("crown_geometry", "m"),
                    ("solar_angles", "rad"),
                ],
                &[("gli", "1"), ("par_absorbed", "W/m²")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 2.0, 1000.0, 31_536_000.0, 315_360_000.0),
            conservation: vec![ConservationProperty {
                quantity: "energy".to_string(),
                method: "radiation_balance".to_string(),
            }],
            cost: cost_model(1e4, 1024, true),
            regime_tags: vec![
                "forest".to_string(),
                "gpu".to_string(),
                "individual".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SORTIE-NG".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Provisional,
            backends: vec![ComputeBackend::Cuda],
        },
        ProcessManifest {
            id: make_id(SourceModel::SortieNg, 2),
            name: "GPU Seed Dispersal (FFT)".to_string(),
            description: "FFT-accelerated kernel convolution for seed dispersal on device."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("seed_production", "seeds/tree"), ("dispersal_kernel", "1")],
                &[("seed_rain", "seeds/m²"), ("colonisation_prob", "1")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 2.0, 1000.0, 31_536_000.0, 315_360_000.0),
            conservation: vec![],
            cost: cost_model(1e3, 512, true),
            regime_tags: vec![
                "forest".to_string(),
                "gpu".to_string(),
                "dispersal".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "SORTIE-NG".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Provisional,
            backends: vec![ComputeBackend::Cuda],
        },
    ]
}

// ============================================================================
// DeepLand (ERICKSON) — DEEP LEARNING LAND SURFACE MODEL
// ============================================================================

fn deepland_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::DeepLand, 1),
            name: "Neural ET Emulator".to_string(),
            description:
                "Deep learning evapotranspiration prediction from met forcings and soil state."
                    .to_string(),
            family: ProcessFamily::Hydrology,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("temperature", "K"),
                    ("humidity", "kg/kg"),
                    ("sw_down", "W/m²"),
                    ("soil_moisture", "m³/m³"),
                ],
                &[("et", "mm/s"), ("sensible_heat", "W/m²")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 1800.0, 3600.0),
            conservation: vec![],
            cost: cost_model(1e4, 2048, true),
            regime_tags: vec!["emulator".to_string(), "deep_learning".to_string()],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "DeepLand".to_string(),
                version: "0.1".to_string(),
            },
            lifecycle: LifecycleStatus::Provisional,
            backends: vec![ComputeBackend::Cuda, ComputeBackend::MlEmulator],
        },
        ProcessManifest {
            id: make_id(SourceModel::DeepLand, 2),
            name: "Neural Carbon Flux Emulator".to_string(),
            description: "Deep learning GPP/NEE prediction from remote sensing and met data."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("ndvi", "1"),
                    ("temperature", "K"),
                    ("par", "W/m²"),
                    ("soil_moisture", "m³/m³"),
                ],
                &[("gpp", "kgC/m²/day"), ("nee", "kgC/m²/day")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1000.0, 100_000.0, 1800.0, 86400.0),
            conservation: vec![],
            cost: cost_model(1e4, 2048, true),
            regime_tags: vec![
                "emulator".to_string(),
                "deep_learning".to_string(),
                "carbon".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "DeepLand".to_string(),
                version: "0.1".to_string(),
            },
            lifecycle: LifecycleStatus::Provisional,
            backends: vec![ComputeBackend::Cuda, ComputeBackend::MlEmulator],
        },
    ]
}

// ============================================================================
// NVIDIA Earth-2 / earth2studio
// ============================================================================

fn earth2_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::Earth2, 1),
            name: "FourCastNet Global Forecast".to_string(),
            description: "AFNO-based global weather forecast at 0.25° with 6h time steps."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("geopotential_500", "m²/s²"),
                    ("temperature_850", "K"),
                    ("u10", "m/s"),
                    ("v10", "m/s"),
                ],
                &[("forecast_fields", "1"), ("ensemble_spread", "1")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 25_000.0, 25_000.0, 21600.0, 864_000.0),
            conservation: vec![],
            cost: cost_model(1e6, 32768, true),
            regime_tags: vec![
                "foundation_model".to_string(),
                "weather".to_string(),
                "gpu".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Earth-2".to_string(),
                version: "0.7".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cuda, ComputeBackend::MlEmulator],
        },
        ProcessManifest {
            id: make_id(SourceModel::Earth2, 2),
            name: "GraphCast Global Forecast".to_string(),
            description: "GNN-based global weather forecast with mesh-based message passing."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[
                    ("geopotential_500", "m²/s²"),
                    ("temperature_850", "K"),
                    ("specific_humidity", "kg/kg"),
                ],
                &[
                    ("forecast_fields", "1"),
                    ("temperature_2m", "K"),
                    ("precipitation", "mm"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 25_000.0, 25_000.0, 21600.0, 864_000.0),
            conservation: vec![],
            cost: cost_model(1e6, 32768, true),
            regime_tags: vec![
                "foundation_model".to_string(),
                "weather".to_string(),
                "gpu".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Earth-2".to_string(),
                version: "0.7".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cuda, ComputeBackend::MlEmulator],
        },
        ProcessManifest {
            id: make_id(SourceModel::Earth2, 3),
            name: "CorrDiff Super-Resolution".to_string(),
            description: "Diffusion-based downscaling for 25km → 2km regional refinement."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Intermediate"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("coarse_fields", "1"), ("topography", "m")],
                &[("high_res_fields", "1"), ("uncertainty", "1")],
            ),
            scale: scale_envelope(CouplingTier::Fast, 2000.0, 25_000.0, 3600.0, 21600.0),
            conservation: vec![],
            cost: cost_model(1e5, 16384, true),
            regime_tags: vec![
                "downscaling".to_string(),
                "diffusion".to_string(),
                "gpu".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "Earth-2".to_string(),
                version: "0.7".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cuda, ComputeBackend::MlEmulator],
        },
    ]
}

// ============================================================================
// ESA PhiSat-2 — ON-BOARD AI EARTH OBSERVATION
// ============================================================================

fn phisat2_processes() -> Vec<ProcessManifest> {
    vec![
        ProcessManifest {
            id: make_id(SourceModel::PhiSat2, 1),
            name: "On-Board Cloud Masking".to_string(),
            description: "Edge-AI CNN cloud detection filtering before downlink.".to_string(),
            family: ProcessFamily::Atmosphere,
            rung: fidelity("Empirical"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("multispectral_image", "DN"), ("viewing_geometry", "deg")],
                &[("cloud_mask", "1"), ("cloud_fraction", "1")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10.0, 100.0, 0.001, 1.0),
            conservation: vec![],
            cost: cost_model(1e3, 512, true),
            regime_tags: vec![
                "edge_ai".to_string(),
                "satellite".to_string(),
                "observation".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PhiSat-2".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cuda, ComputeBackend::MlEmulator],
        },
        ProcessManifest {
            id: make_id(SourceModel::PhiSat2, 2),
            name: "On-Board Fire Detection".to_string(),
            description: "Real-time thermal anomaly detection for active fire mapping.".to_string(),
            family: ProcessFamily::Fire,
            rung: fidelity("Empirical"),
            version: "1.0.0".to_string(),
            io: io_contract(
                &[("swir_band", "W/m²/sr/µm"), ("thermal_band", "K")],
                &[("fire_probability", "1"), ("fire_radiative_power", "MW")],
            ),
            scale: scale_envelope(CouplingTier::Slow, 10.0, 100.0, 0.001, 1.0),
            conservation: vec![],
            cost: cost_model(1e3, 512, true),
            regime_tags: vec![
                "edge_ai".to_string(),
                "fire_detection".to_string(),
                "satellite".to_string(),
            ],
            relations: Vec::new(),
            origin: ProcessOrigin::Imported {
                source: "PhiSat-2".to_string(),
                version: "1.0".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cuda, ComputeBackend::MlEmulator],
        },
    ]
}

// ============================================================================
// FireBench — HIGH-FIDELITY WILDFIRE SIMULATION DATASET
// ============================================================================

fn firebench_processes() -> Vec<ProcessManifest> {
    vec![ProcessManifest {
        id: make_id(SourceModel::FireBench, 1),
        name: "WRF-Fire Benchmark Dataset".to_string(),
        description: "High-fidelity coupled fire-atmosphere simulation data for model validation."
            .to_string(),
        family: ProcessFamily::Fire,
        rung: fidelity("Physics-based"),
        version: "1.0.0".to_string(),
        io: io_contract(
            &[("fuel_type", "1"), ("terrain", "m"), ("wind_field", "m/s")],
            &[
                ("fire_perimeter", "m"),
                ("rate_of_spread", "m/s"),
                ("heat_flux", "kW/m²"),
            ],
        ),
        scale: scale_envelope(CouplingTier::Fast, 5.0, 1000.0, 1.0, 3600.0),
        conservation: vec![],
        cost: cost_model(1e8, 32768, true),
        regime_tags: vec![
            "fire".to_string(),
            "benchmark".to_string(),
            "validation".to_string(),
        ],
        relations: Vec::new(),
        origin: ProcessOrigin::Imported {
            source: "FireBench".to_string(),
            version: "1.0".to_string(),
        },
        lifecycle: LifecycleStatus::Production,
        backends: vec![ComputeBackend::Cpu, ComputeBackend::Cuda],
    }]
}

// ============================================================================
// MAESTRA/MAESTRO — 3D Canopy Radiation & Tree Physiology Model
// ============================================================================

fn maestra_processes() -> Vec<ProcessManifest> {
    vec![
        // 1. 3D Crown Radiation Interception (PAR + NIR + Thermal)
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 1),
            name: "MAESTRA 3D Crown Radiation Transfer".to_string(),
            description: "3D beam/diffuse/thermal radiation transfer through individual tree crowns. \
                Discretizes crowns into grid points (up to 720 per tree), traces ray paths through all \
                neighbouring crowns (TREDST), computes Beer–Lambert extinction with weighted pathlengths \
                (WPATH), multi-scattering via Norman (1979) iterative scheme (SCATTER), and resolves \
                PAR/NIR/thermal wavelengths separately. Supports 6 crown shapes (cone, half-ellipsoid, \
                paraboloid, full-ellipsoid, cylinder, box), beta-function leaf area density distributions, \
                slope corrections (Steven & Unsworth 1979), and sunlit/shaded leaf separation."
                .to_string(),
            family: ProcessFamily::Radiation,
            rung: FidelityRung::R2,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("downwelling_shortwave_radiation", "W m-2"),
                    ("downwelling_longwave_radiation", "W m-2"),
                    ("beam_fraction", "1"),
                    ("solar_zenith_angle", "rad"),
                    ("solar_azimuth_angle", "rad"),
                    ("leaf_area_index", "m2 m-2"),
                    ("leaf_reflectance_par", "1"),
                    ("leaf_transmittance_par", "1"),
                    ("leaf_reflectance_nir", "1"),
                    ("leaf_transmittance_nir", "1"),
                    ("soil_reflectance", "1"),
                ],
                &[
                    ("absorbed_par", "umol m-2 s-1"),
                    ("absorbed_nir", "W m-2"),
                    ("absorbed_thermal", "W m-2"),
                    ("sunlit_leaf_fraction", "1"),
                    ("net_radiation", "W m-2"),
                    ("diffuse_transmittance", "1"),
                    ("beam_transmittance", "1"),
                    ("scattered_radiation_lost", "W m-2"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 100.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "radiant_energy".to_string(),
                method: "iterative_multi_scattering_closure".to_string(),
            }],
            cost: cost_model(5e7, 4096, false),
            regime_tags: vec![
                "forest".into(), "canopy".into(), "individual_tree".into(),
                "3D_radiative_transfer".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (Medlyn/Wang/Jarvis)".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 2. Solar Geometry (astronomical)
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 2),
            name: "MAESTRA Solar Geometry".to_string(),
            description: "Full astronomical solar position calculation (Barkstrom 1981). Computes \
                solar declination, equation of time, daylength, half-hourly zenith and azimuth angles, \
                and slope correction factors for beam, diffuse, and soil-reflected radiation (Steven & \
                Unsworth 1979/1980). Includes orbital eccentricity, nutation, and lunar node corrections."
                .to_string(),
            family: ProcessFamily::Radiation,
            rung: FidelityRung::R2,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("latitude", "rad"),
                    ("longitude", "rad"),
                    ("day_of_year", "1"),
                    ("slope_x", "rad"),
                    ("slope_y", "rad"),
                ],
                &[
                    ("solar_zenith_angle", "rad"),
                    ("solar_azimuth_angle", "rad"),
                    ("daylength", "h"),
                    ("solar_declination", "rad"),
                    ("beam_slope_multiplier", "1"),
                    ("diffuse_slope_multiplier", "1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 1e6, 1800.0, 86400.0),
            conservation: vec![],
            cost: cost_model(1e4, 256, false),
            regime_tags: vec!["astronomy".into(), "solar".into()],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (Barkstrom 1981)".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 3. Farquhar–von Caemmerer C3 Photosynthesis
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 3),
            name: "MAESTRA Farquhar–von Caemmerer Photosynthesis".to_string(),
            description: "Leaf-level C3 photosynthesis (Farquhar, von Caemmerer & Berry 1980) with \
                ECOCRAFT-agreed formulation. Computes Rubisco-limited (Ac) and electron transport-limited \
                (Aj) assimilation rates, temperature-dependent Vcmax and Jmax (peaked Arrhenius, Johnson \
                et al.), Γ* (Brooks & Farquhar or Bernacchi 2001), Km (Kc(1+Oi/Ko)), and quantum yield. \
                Coupled to stomatal conductance model (Ball-Berry, Ball-Berry-Leuning, or Jarvis) via \
                iterative Ci solution. Supports soil moisture stress via Granier & Loustau (1994) modifier."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: FidelityRung::R2,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("absorbed_par", "umol m-2 s-1"),
                    ("leaf_temperature", "degC"),
                    ("co2_concentration", "umol mol-1"),
                    ("relative_humidity", "1"),
                    ("vapour_pressure_deficit", "Pa"),
                    ("atmospheric_pressure", "Pa"),
                    ("soil_moisture_deficit", "mm"),
                ],
                &[
                    ("net_photosynthesis", "umol m-2 s-1"),
                    ("stomatal_conductance", "mol m-2 s-1"),
                    ("leaf_respiration", "umol m-2 s-1"),
                    ("intercellular_co2", "umol mol-1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 100.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance_A_minus_Rd".to_string(),
            }],
            cost: cost_model(1e6, 512, false),
            regime_tags: vec![
                "C3".into(), "forest".into(), "leaf_level".into(),
                "farquhar".into(), "ecocraft".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (FvCB 1980 / ECOCRAFT)".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 4. Stomatal Conductance (3 models)
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 4),
            name: "MAESTRA Stomatal Conductance Suite".to_string(),
            description: "Three stomatal conductance models: (1) Jarvis multiplicative — \
                gs = (gs_ref − gs_min) · f(PAR) · f(VPD) · f(CO₂) · f(T) · f(SWC) + gs_min \
                with 4 VPD response options (Lohammer, linear, MFD, Bray/Bosc); (2) Ball-Berry — \
                gs = g0 + g1·RH·A/(Cs−Γ); (3) Ball-Berry-Leuning — gs = g0 + g1·A/((Cs−Γ)(1+VPD/D0)). \
                All include soil moisture modifier (Granier & Loustau 1994) and boundary layer \
                conductance (forced: Leuning et al. 1995; free convection: Grashof number)."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: FidelityRung::R1,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("absorbed_par", "umol m-2 s-1"),
                    ("vapour_pressure_deficit", "Pa"),
                    ("co2_concentration", "umol mol-1"),
                    ("air_temperature", "degC"),
                    ("soil_water_content", "m3 m-3"),
                    ("net_photosynthesis", "umol m-2 s-1"),
                    ("wind_speed", "m s-1"),
                    ("leaf_width", "m"),
                ],
                &[
                    ("stomatal_conductance", "mol m-2 s-1"),
                    ("boundary_layer_conductance_heat", "mol m-2 s-1"),
                    ("boundary_layer_conductance_water", "mol m-2 s-1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 100.0, 1800.0, 3600.0),
            conservation: vec![],
            cost: cost_model(5e5, 256, false),
            regime_tags: vec![
                "stomata".into(), "ball_berry".into(), "jarvis".into(),
                "leuning".into(), "leaf_level".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (Ball-Berry 1987 / Leuning 1995 / Jarvis 1976)".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 5. Penman-Monteith Transpiration + Leaf Energy Balance
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 5),
            name: "MAESTRA Penman-Monteith Transpiration & Energy Balance".to_string(),
            description: "Leaf-level and canopy-scale Penman-Monteith evapotranspiration with \
                iterative leaf temperature solution (Leuning et al. 1995). Radiation conductance \
                (linearized Stefan-Boltzmann), forced-convection boundary layer conductance \
                (Leuning et al. 1995 eqn E1), free-convection conductance (Grashof number), \
                canopy boundary layer conductance (logarithmic wind profile, Jones 1992). \
                Iterates: photosynthesis → conductances → PM ET → energy balance → leaf T → \
                repeat until |ΔT_leaf| < 0.01 °C."
                .to_string(),
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R2,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("net_radiation", "W m-2"),
                    ("air_temperature", "degC"),
                    ("vapour_pressure_deficit", "Pa"),
                    ("atmospheric_pressure", "Pa"),
                    ("wind_speed", "m s-1"),
                    ("stomatal_conductance", "mol m-2 s-1"),
                    ("leaf_width", "m"),
                ],
                &[
                    ("transpiration", "mol m-2 s-1"),
                    ("canopy_evaporation", "mol m-2 s-1"),
                    ("sensible_heat_flux", "W m-2"),
                    ("leaf_temperature", "degC"),
                    ("canopy_temperature", "degC"),
                    ("latent_heat_flux", "W m-2"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 100.0, 1800.0, 3600.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "energy".to_string(),
                    method: "rnet_equals_sensible_plus_latent".to_string(),
                },
                ConservationProperty {
                    quantity: "water_mass".to_string(),
                    method: "penman_monteith_closure".to_string(),
                },
            ],
            cost: cost_model(2e6, 512, false),
            regime_tags: vec![
                "transpiration".into(), "energy_balance".into(),
                "penman_monteith".into(), "leaf_temperature".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (Leuning et al. 1995)".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 6. Autotrophic Respiration (5 tissue pools)
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 6),
            name: "MAESTRA Autotrophic Respiration".to_string(),
            description: "Q10-based maintenance respiration for 5 tissue pools (foliage, stem, \
                branch, fine root, coarse root) plus growth respiration from biomass increment × \
                construction cost. Maintenance: Rm = R0 · exp(Q10·(T−Tref)). Stem respiration \
                supports diameter-based exponential or surface-area-based formulations. Growth: \
                Rg = ΔW · (1/efficiency − 1) · fC / MC. Day respiration inhibition factor for \
                foliage. Soil temperature drives root respiration."
                .to_string(),
            family: ProcessFamily::Biogeochemistry,
            rung: FidelityRung::R1,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("air_temperature", "degC"),
                    ("soil_temperature", "degC"),
                    ("stem_biomass", "kg"),
                    ("branch_biomass", "kg"),
                    ("root_biomass", "kg"),
                    ("fine_root_fraction", "1"),
                    ("foliar_biomass", "kg"),
                    ("stem_biomass_increment", "g d-1"),
                    ("branch_biomass_increment", "g d-1"),
                    ("root_biomass_increment", "g d-1"),
                ],
                &[
                    ("foliage_maintenance_respiration", "umol tree-1 s-1"),
                    ("stem_maintenance_respiration", "umol tree-1 s-1"),
                    ("branch_maintenance_respiration", "umol tree-1 s-1"),
                    ("fine_root_maintenance_respiration", "umol tree-1 s-1"),
                    ("coarse_root_maintenance_respiration", "umol tree-1 s-1"),
                    ("stem_growth_respiration", "mol tree-1 d-1"),
                    ("branch_growth_respiration", "mol tree-1 d-1"),
                    ("root_growth_respiration", "mol tree-1 d-1"),
                    ("foliage_growth_respiration", "mol tree-1 d-1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 100.0, 1800.0, 86400.0),
            conservation: vec![ConservationProperty {
                quantity: "carbon".to_string(),
                method: "mass_balance_maintenance_plus_growth".to_string(),
            }],
            cost: cost_model(1e5, 256, false),
            regime_tags: vec![
                "respiration".into(), "Q10".into(), "maintenance".into(),
                "growth".into(), "autotrophic".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 7. Crown Structure & Allometry
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 7),
            name: "MAESTRA Crown Geometry & Allometry".to_string(),
            description: "3D crown geometry parameterization supporting 6 crown shapes (cone, \
                half-ellipsoid, paraboloid, full-ellipsoid, cylinder, box). Leaf area density \
                distributed via 4-parameter beta function (vertical and horizontal, up to 3 age \
                classes). Allometric relationships for stem, branch, and root biomass from height \
                and diameter. Ellipsoidal leaf angle distribution (Campbell 1986). Simple 4-parameter \
                phenology (leaf flush/senescence by day-of-year)."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: FidelityRung::R1,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("tree_height", "m"),
                    ("crown_radius_x", "m"),
                    ("crown_radius_y", "m"),
                    ("crown_radius_z", "m"),
                    ("crown_base_height", "m"),
                    ("diameter_breast_height", "m"),
                    ("leaf_area", "m2"),
                    ("day_of_year", "1"),
                ],
                &[
                    ("grid_point_coordinates_x", "m"),
                    ("grid_point_coordinates_y", "m"),
                    ("grid_point_coordinates_z", "m"),
                    ("grid_point_volume", "m3"),
                    ("leaf_area_density", "m2 m-3"),
                    ("stem_biomass", "kg"),
                    ("branch_biomass", "kg"),
                    ("root_biomass", "kg"),
                    ("foliar_biomass", "kg"),
                    ("foliage_area_per_layer", "m2"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Slow, 1.0, 100.0, 86400.0, 86400.0),
            conservation: vec![],
            cost: cost_model(1e5, 2048, false),
            regime_tags: vec![
                "crown".into(), "allometry".into(), "beta_function".into(),
                "individual_tree".into(), "phenology".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (Campbell 1986)".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 8. Meteorological Data Processing & Radiation Partitioning
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 8),
            name: "MAESTRA Met Processing & Radiation Partitioning".to_string(),
            description: "Disaggregation of daily meteorological data to half-hourly values. \
                Diurnal temperature cycle (sinusoidal). Radiation partitioning into beam/diffuse \
                (Spitters et al. 1986, Bristow-Campbell 1984 transmissivity). PAR/NIR split \
                (FPAR=0.5). Incoming thermal radiation estimation (Monteith & Unsworth 1990). \
                Exponential canopy wind profile. Humidity conversions (RH, VPD, MFD, dewpoint). \
                Climate change (CO₂ + T perturbation) and open-top chamber (OTC) scenario modifiers."
                .to_string(),
            family: ProcessFamily::Atmosphere,
            rung: FidelityRung::R1,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("daily_max_temperature", "degC"),
                    ("daily_min_temperature", "degC"),
                    ("daily_total_par", "MJ m-2 d-1"),
                    ("daily_mean_wind_speed", "m s-1"),
                    ("daily_precipitation", "mm"),
                    ("co2_concentration", "umol mol-1"),
                    ("atmospheric_pressure", "Pa"),
                ],
                &[
                    ("air_temperature", "degC"),
                    ("soil_temperature", "degC"),
                    ("downwelling_shortwave_par", "W m-2"),
                    ("downwelling_shortwave_nir", "W m-2"),
                    ("downwelling_longwave_radiation", "W m-2"),
                    ("beam_fraction", "1"),
                    ("relative_humidity", "1"),
                    ("vapour_pressure_deficit", "Pa"),
                    ("wind_speed", "m s-1"),
                    ("precipitation_rate", "mm h-1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 1e6, 1800.0, 86400.0),
            conservation: vec![],
            cost: cost_model(1e4, 256, false),
            regime_tags: vec![
                "meteorology".into(), "disaggregation".into(),
                "spitters".into(), "climate_change".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (Spitters 1986 / Bristow-Campbell 1984)".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 9. Canopy-Scale Integration
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 9),
            name: "MAESTRA Canopy Integration & Stand Aggregation".to_string(),
            description: "Integration of leaf-level fluxes to tree-level and stand-level totals. \
                Outer loop: target trees (up to 720) × days × hours × grid points (up to 720 per tree) \
                × wavelengths (PAR, NIR, thermal). Aggregates absorbed radiation, net CO₂ flux, \
                transpiration, sensible heat, canopy temperature, and all respiration components. \
                Stand-level outputs via SUMTREES post-processing weighted by tree stocking density."
                .to_string(),
            family: ProcessFamily::Ecology,
            rung: FidelityRung::R1,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("gridpoint_photosynthesis", "umol m-2 s-1"),
                    ("gridpoint_transpiration", "mol m-2 s-1"),
                    ("gridpoint_absorbed_par", "umol m-2 s-1"),
                    ("gridpoint_sensible_heat", "W m-2"),
                    ("gridpoint_leaf_area", "m2"),
                    ("tree_stocking_density", "trees ha-1"),
                ],
                &[
                    ("tree_net_co2_flux", "umol tree-1 s-1"),
                    ("tree_transpiration", "mmol tree-1 s-1"),
                    ("tree_sensible_heat_flux", "kW tree-1"),
                    ("canopy_temperature", "degC"),
                    ("canopy_stomatal_conductance", "mol tree-1 s-1"),
                    ("daily_npp", "mol tree-1 d-1"),
                    ("daily_water_use", "mol tree-1 d-1"),
                    ("stand_gpp", "umol m-2 s-1"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 1000.0, 1800.0, 86400.0),
            conservation: vec![
                ConservationProperty {
                    quantity: "carbon".to_string(),
                    method: "sum_gridpoints_equals_tree_total".to_string(),
                },
                ConservationProperty {
                    quantity: "water_mass".to_string(),
                    method: "sum_gridpoints_equals_tree_total".to_string(),
                },
            ],
            cost: cost_model(1e7, 4096, false),
            regime_tags: vec![
                "canopy_integration".into(), "stand_level".into(),
                "scaling".into(), "individual_tree".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09".to_string(),
            },
            lifecycle: LifecycleStatus::Production,
            backends: vec![ComputeBackend::Cpu],
        },
        // 10. Leaf Optical Properties & Canopy Spectral Processing
        ProcessManifest {
            id: make_id(SourceModel::Maestra, 10),
            name: "MAESTRA Leaf Optics & Multi-Scattering".to_string(),
            description: "Leaf-level optical properties (absorptance, reflectance, transmittance) \
                for PAR, NIR, and thermal wavelengths, stratified by canopy layer. Norman (1979) \
                iterative multi-scattering scheme on Equivalent Horizontal Canopy (EHC) layers. \
                Converges when layer-to-layer flux difference < 0.01. Thermal radiation via \
                Stefan-Boltzmann emission (ε_leaf = 0.95). Soil reflectance per wavelength."
                .to_string(),
            family: ProcessFamily::Radiation,
            rung: FidelityRung::R2,
            version: "2001.09".to_string(),
            io: io_contract(
                &[
                    ("leaf_absorptance_par", "1"),
                    ("leaf_reflectance_par", "1"),
                    ("leaf_transmittance_par", "1"),
                    ("leaf_absorptance_nir", "1"),
                    ("soil_reflectance_par", "1"),
                    ("soil_reflectance_nir", "1"),
                    ("air_temperature", "degC"),
                    ("soil_temperature", "degC"),
                ],
                &[
                    ("diffuse_upward_flux_par", "W m-2"),
                    ("diffuse_downward_flux_par", "W m-2"),
                    ("diffuse_upward_flux_nir", "W m-2"),
                    ("diffuse_downward_flux_nir", "W m-2"),
                    ("net_thermal_flux", "W m-2"),
                    ("scattered_radiation_lost", "W m-2"),
                ],
            ),
            scale: scale_envelope(CouplingTier::Fast, 1.0, 100.0, 1800.0, 3600.0),
            conservation: vec![ConservationProperty {
                quantity: "radiant_energy".to_string(),
                method: "ehc_iterative_convergence".to_string(),
            }],
            cost: cost_model(1e7, 2048, false),
            regime_tags: vec![
                "scattering".into(), "norman_1979".into(),
                "leaf_optics".into(), "thermal".into(),
            ],
            relations: vec![],
            origin: ProcessOrigin::Imported {
                source: "MAESTRA".to_string(),
                version: "2001.09 (Norman 1979)".to_string(),
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
    fn test_extended_manifest_count() {
        let manifests = generate_extended_manifests();
        assert!(
            manifests.len() >= 70,
            "Expected at least 70 extended manifests, got {}",
            manifests.len()
        );
    }

    #[test]
    fn test_extended_all_sources_covered() {
        let manifests = generate_extended_manifests();
        let sources: std::collections::HashSet<String> = manifests
            .iter()
            .map(|m| match &m.origin {
                ProcessOrigin::Imported { source, .. } => source.clone(),
                _ => "unknown".to_string(),
            })
            .collect();
        // 31 source models
        assert!(
            sources.len() >= 28,
            "Expected at least 28 unique sources, got {}",
            sources.len()
        );
    }

    #[test]
    fn test_extended_deterministic_ids() {
        let m1 = generate_extended_manifests();
        let m2 = generate_extended_manifests();
        assert_eq!(m1.len(), m2.len());
        for (a, b) in m1.iter().zip(m2.iter()) {
            assert_eq!(a.id, b.id, "IDs should be deterministic");
        }
    }
}
