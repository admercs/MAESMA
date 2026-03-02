//! Ontology seed — initial relations between process representations.
//!
//! This module provides the foundational ontology graph that captures
//! compatibility, incompatibility, coupling requirements, and supersession
//! relations between processes in the knowledgebase.

use maesma_core::manifest::RelationType;
use maesma_core::ontology::Relation;
use maesma_core::process::ProcessId;
use uuid::Uuid;

use crate::seed::SourceModel;

/// Helper to create a deterministic UUID from source model and process index.
fn make_id(source: SourceModel, index: u32) -> ProcessId {
    let name = format!("maesma:{}:{}", source.name(), index);
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes());
    ProcessId(uuid)
}

/// Generate seed ontology relations between processes.
///
/// The ontology captures:
/// - **CompatibleWith**: Processes that can run together without conflict
/// - **IncompatibleWith**: Mutually exclusive representations (e.g., different fire models)
/// - **RequiresCouplingWith**: Processes that must exchange state (e.g., fire ↔ atmosphere)
/// - **Supersedes**: Higher-fidelity processes that can replace lower-fidelity ones
/// - **VariantOf**: Alternative parameterizations of the same underlying process
pub fn generate_seed_relations() -> Vec<Relation> {
    let mut relations = Vec::with_capacity(200);

    // ========================================================================
    // FIRE PROCESS RELATIONS
    // ========================================================================

    // WRF-SFIRE level set is incompatible with SPITFIRE (different paradigms)
    relations.push(Relation {
        source: make_id(SourceModel::WrfSfire, 1), // Level Set Fire Spread
        relation: RelationType::IncompatibleWith,
        target: make_id(SourceModel::Fates, 2), // SPITFIRE
        justification: Some(
            "Level-set and Rothermel paradigms are mutually exclusive fire spread models"
                .to_string(),
        ),
    });

    // WRF-SFIRE requires atmosphere coupling
    relations.push(Relation {
        source: make_id(SourceModel::WrfSfire, 1), // Level Set Fire Spread
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::WrfSfire, 2), // Fire-Atmosphere Coupling
        justification: Some(
            "Level-set fire spread requires two-way atmosphere coupling for plume dynamics"
                .to_string(),
        ),
    });

    // SPITFIRE is compatible with FATES demography
    relations.push(Relation {
        source: make_id(SourceModel::Fates, 2), // SPITFIRE
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Fates, 1), // Cohort Demography
        justification: Some(
            "SPITFIRE fire effects directly modify cohort mortality and fuel loads".to_string(),
        ),
    });

    // ========================================================================
    // HYDROLOGY PROCESS RELATIONS
    // ========================================================================

    // ParFlow 3D Richards supersedes CLM 1D Richards
    relations.push(Relation {
        source: make_id(SourceModel::ParFlow, 1), // 3D Richards
        relation: RelationType::Supersedes,
        target: make_id(SourceModel::Cesm, 11), // CLM Soil Hydrology
        justification: Some(
            "ParFlow 3D variably-saturated flow is higher fidelity than CLM 1D column".to_string(),
        ),
    });

    // ATS Richards supersedes CLM Richards
    relations.push(Relation {
        source: make_id(SourceModel::Ats, 1), // Richards (3D)
        relation: RelationType::Supersedes,
        target: make_id(SourceModel::Cesm, 11), // CLM Soil Hydrology
        justification: Some(
            "ATS 3D Richards with freeze-thaw is higher fidelity than CLM 1D".to_string(),
        ),
    });

    // ATS requires coupling with its energy equation for permafrost
    relations.push(Relation {
        source: make_id(SourceModel::Ats, 1), // Richards
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Ats, 2), // Coupled Energy
        justification: Some(
            "Thermal-hydrological coupling is essential for freeze-thaw dynamics".to_string(),
        ),
    });

    // ATS overland flow compatible with Richards
    relations.push(Relation {
        source: make_id(SourceModel::Ats, 3), // Overland Flow
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Ats, 1), // Richards
        justification: Some(
            "Surface-subsurface coupling via infiltration/exfiltration".to_string(),
        ),
    });

    // Landlab FlowRouter compatible with Badlands stream power
    relations.push(Relation {
        source: make_id(SourceModel::Landlab, 1), // FlowRouter
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Badlands, 1), // Stream Power
        justification: Some(
            "Flow routing provides drainage area input for stream power erosion".to_string(),
        ),
    });

    // ========================================================================
    // CRYOSPHERE PROCESS RELATIONS
    // ========================================================================

    // PISM SIA compatible with SSA (hybrid model)
    relations.push(Relation {
        source: make_id(SourceModel::Pism, 1), // SIA
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Pism, 2), // SSA
        justification: Some("SIA+SSA hybrid is the standard PISM configuration".to_string()),
    });

    // PISM thermodynamics compatible with both
    relations.push(Relation {
        source: make_id(SourceModel::Pism, 3), // Ice Thermodynamics
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Pism, 1), // SIA
        justification: Some("Temperature-dependent viscosity feeds back to ice flow".to_string()),
    });

    // CryoGrid freeze-thaw requires heat conduction
    relations.push(Relation {
        source: make_id(SourceModel::CryoGrid, 2), // Freeze-Thaw
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::CryoGrid, 1), // Heat Conduction
        justification: Some("Phase change is driven by heat conduction".to_string()),
    });

    // CryoGrid active layer requires freeze-thaw
    relations.push(Relation {
        source: make_id(SourceModel::CryoGrid, 3), // Active Layer
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::CryoGrid, 2), // Freeze-Thaw
        justification: Some("Active layer depth is determined by freeze-thaw front".to_string()),
    });

    // CryoGrid supersedes CLM permafrost for arctic applications
    relations.push(Relation {
        source: make_id(SourceModel::CryoGrid, 1), // Heat Conduction
        relation: RelationType::Supersedes,
        target: make_id(SourceModel::Cesm, 11), // CLM Soil Hydrology (for permafrost)
        justification: Some(
            "CryoGrid has higher vertical resolution and explicit excess ice".to_string(),
        ),
    });

    // OGGM flowline variant of PISM SIA (simplified for mountain glaciers)
    relations.push(Relation {
        source: make_id(SourceModel::Oggm, 1), // Flowline SIA
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Pism, 1), // Full SIA
        justification: Some(
            "OGGM flowline is a dimensionally-reduced variant for valley glaciers".to_string(),
        ),
    });

    // ========================================================================
    // OCEAN BIOGEOCHEMISTRY RELATIONS
    // ========================================================================

    // MARBL air-sea exchange requires carbonate chemistry
    relations.push(Relation {
        source: make_id(SourceModel::Marbl, 2), // Air-Sea Gas Exchange
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Marbl, 3), // Carbonate Chemistry
        justification: Some(
            "pCO2 calculation requires full carbonate chemistry solver".to_string(),
        ),
    });

    // MARBL phytoplankton requires nutrient sources (iron, etc.)
    relations.push(Relation {
        source: make_id(SourceModel::Marbl, 1), // Phytoplankton Growth
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Marbl, 2), // Air-Sea Exchange
        justification: Some("Primary production affects CO2 flux via biological pump".to_string()),
    });

    // BFM is variant of MARBL (European vs US heritage)
    relations.push(Relation {
        source: make_id(SourceModel::Bfm, 1), // BFM Photosynthesis
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Marbl, 1), // MARBL Photosynthesis
        justification: Some(
            "Both implement multi-PFT phytoplankton with different parameterizations".to_string(),
        ),
    });

    // BFM microbial loop compatible with BFM photosynthesis
    relations.push(Relation {
        source: make_id(SourceModel::Bfm, 3), // Microbial Loop
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Bfm, 1), // Photosynthesis
        justification: Some(
            "Microbial remineralization recycles nutrients for primary production".to_string(),
        ),
    });

    // ========================================================================
    // COASTAL/GEOMORPHOLOGY RELATIONS
    // ========================================================================

    // XBeach suspended sediment requires wave energy
    relations.push(Relation {
        source: make_id(SourceModel::Xbeach, 2), // Suspended Sediment
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Xbeach, 1), // Wave Energy
        justification: Some("Wave-induced bed shear stress drives sediment suspension".to_string()),
    });

    // XBeach Exner requires sediment transport
    relations.push(Relation {
        source: make_id(SourceModel::Xbeach, 3), // Exner
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Xbeach, 2), // Suspended Sediment
        justification: Some("Bed level update from sediment flux divergence".to_string()),
    });

    // Badlands stream power compatible with XBeach (different scales)
    relations.push(Relation {
        source: make_id(SourceModel::Badlands, 1), // Stream Power
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Xbeach, 3), // Exner
        justification: Some("Fluvial delivery to coast + nearshore morphodynamics".to_string()),
    });

    // Badlands flexural isostasy compatible with stream power
    relations.push(Relation {
        source: make_id(SourceModel::Badlands, 3), // Flexural Isostasy
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Badlands, 1), // Stream Power
        justification: Some("Sediment redistribution drives isostatic response".to_string()),
    });

    // ========================================================================
    // ATMOSPHERE/RADIATION RELATIONS
    // ========================================================================

    // RRTMGP compatible with deep convection
    relations.push(Relation {
        source: make_id(SourceModel::Cesm, 2), // RRTMGP
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Cesm, 1), // Deep Convection
        justification: Some("Radiation heating affects convective instability".to_string()),
    });

    // CARMA coagulation requires nucleation
    relations.push(Relation {
        source: make_id(SourceModel::Carma, 2), // Coagulation
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Carma, 1), // Nucleation
        justification: Some("Coagulation operates on particles formed by nucleation".to_string()),
    });

    // CARMA optical properties compatible with coagulation
    relations.push(Relation {
        source: make_id(SourceModel::Carma, 3), // Optical Properties
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Carma, 2), // Coagulation
        justification: Some(
            "Optical properties computed from evolved size distribution".to_string(),
        ),
    });

    // CARMA optical properties can feed RRTMGP
    relations.push(Relation {
        source: make_id(SourceModel::Carma, 3), // Optical Properties
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Cesm, 2), // RRTMGP
        justification: Some("Aerosol optical properties modify radiative transfer".to_string()),
    });

    // ========================================================================
    // URBAN/LAND SURFACE RELATIONS
    // ========================================================================

    // SURFEX/TEB canyon radiation compatible with BEM
    relations.push(Relation {
        source: make_id(SourceModel::SurfexTeb, 1), // Canyon Radiation
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::SurfexTeb, 2), // BEM
        justification: Some("Canyon radiation determines building wall temperatures".to_string()),
    });

    // SURFEX/TEB UHI emerges from canyon + anthropogenic heat
    relations.push(Relation {
        source: make_id(SourceModel::SurfexTeb, 3), // UHI
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::SurfexTeb, 1), // Canyon Radiation
        justification: Some("UHI intensity depends on canyon trapping".to_string()),
    });

    // SURFEX/TEB UHI requires BEM for anthropogenic heat
    relations.push(Relation {
        source: make_id(SourceModel::SurfexTeb, 3), // UHI
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::SurfexTeb, 2), // BEM
        justification: Some("Building energy demand produces waste heat".to_string()),
    });

    // ========================================================================
    // CROP/AGRICULTURAL RELATIONS
    // ========================================================================

    // APSIM RUE compatible with phenology
    relations.push(Relation {
        source: make_id(SourceModel::Apsim, 2), // RUE Photosynthesis
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Apsim, 1), // Phenology
        justification: Some("Growth stage affects RUE and partitioning".to_string()),
    });

    // APSIM SoilWat compatible with RUE
    relations.push(Relation {
        source: make_id(SourceModel::Apsim, 3), // SoilWat
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Apsim, 2), // RUE
        justification: Some("Water stress modifies RUE".to_string()),
    });

    // DSSAT CERES compatible with CENTURY
    relations.push(Relation {
        source: make_id(SourceModel::Dssat, 1), // CERES
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Dssat, 2), // CENTURY
        justification: Some("Crop residues feed soil carbon pools".to_string()),
    });

    // APSIM variant of DSSAT (Australian vs US heritage)
    relations.push(Relation {
        source: make_id(SourceModel::Apsim, 1), // APSIM Phenology
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Dssat, 1), // DSSAT CERES
        justification: Some(
            "Both thermal-time phenology with different crop parameterizations".to_string(),
        ),
    });

    // ========================================================================
    // ECOSYSTEM/ECOLOGY RELATIONS
    // ========================================================================

    // FATES demography compatible with CLM photosynthesis
    relations.push(Relation {
        source: make_id(SourceModel::Fates, 1), // Cohort Demography
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Cesm, 10), // CLM Photosynthesis
        justification: Some("FATES cohorts use CLM canopy photosynthesis".to_string()),
    });

    // FATES demography requires allometry
    relations.push(Relation {
        source: make_id(SourceModel::Fates, 1), // Cohort Demography
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Fates, 3), // Tree Allometry
        justification: Some("Allometry converts DBH to biomass and crown area".to_string()),
    });

    // EwE mass balance compatible with predator-prey
    relations.push(Relation {
        source: make_id(SourceModel::Ewe, 1), // Mass Balance
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Ewe, 2), // Predator-Prey
        justification: Some("Ecopath provides initial conditions for Ecosim".to_string()),
    });

    // EwE predator-prey compatible with fisheries
    relations.push(Relation {
        source: make_id(SourceModel::Ewe, 2), // Predator-Prey
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Ewe, 3), // Fisheries
        justification: Some("Fishing mortality affects population dynamics".to_string()),
    });

    // Landlab SpeciesEvolver compatible with FlowRouter
    relations.push(Relation {
        source: make_id(SourceModel::Landlab, 2), // SpeciesEvolver
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Landlab, 1), // FlowRouter
        justification: Some(
            "River networks affect habitat connectivity and speciation".to_string(),
        ),
    });

    // ========================================================================
    // CROSS-DOMAIN COUPLING REQUIREMENTS
    // ========================================================================

    // Ocean circulation requires surface fluxes
    relations.push(Relation {
        source: make_id(SourceModel::Cesm, 50), // Ocean Primitive Equations
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Cesm, 2), // RRTMGP (via surface energy balance)
        justification: Some("Ocean receives heat flux from atmosphere".to_string()),
    });

    // Sea ice dynamics requires ocean coupling
    relations.push(Relation {
        source: make_id(SourceModel::Cesm, 80), // Sea Ice EVP
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Cesm, 50), // Ocean
        justification: Some("Ice-ocean stress balance and basal melt".to_string()),
    });

    // PISM SSA requires ocean thermal forcing
    relations.push(Relation {
        source: make_id(SourceModel::Pism, 2), // SSA
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Cesm, 50), // Ocean (for sub-shelf melt)
        justification: Some("Ice shelf melt from ocean thermal forcing".to_string()),
    });

    // Noah-MP canopy radiation compatible with CLM photosynthesis
    relations.push(Relation {
        source: make_id(SourceModel::NoahMp, 1), // Multi-layer Canopy
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Cesm, 10), // CLM Photosynthesis
        justification: Some("Canopy radiation provides PAR for photosynthesis".to_string()),
    });

    // ========================================================================
    // EXTENDED MODEL RELATIONS (Phase 27)
    // ========================================================================

    // --- Vegetation model variants ---

    // JULES TRIFFID is a variant of LPJ-GUESS vegetation dynamics
    relations.push(Relation {
        source: make_id(SourceModel::Jules, 2), // TRIFFID
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::LpjGuess, 2), // Vegetation Dynamics
        justification: Some(
            "Both Lotka-Volterra PFT competition, different parameterizations".to_string(),
        ),
    });

    // CLASSIC CTEM is a variant of LPJ-GUESS vegetation dynamics
    relations.push(Relation {
        source: make_id(SourceModel::Classic, 2), // CTEM
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::LpjGuess, 2), // Vegetation Dynamics
        justification: Some(
            "Both PFT competition models, CTEM derived from Canadian lineage".to_string(),
        ),
    });

    // ORCHIDEE STOMATE is a variant of LPJ-GUESS (shared LPJ heritage)
    relations.push(Relation {
        source: make_id(SourceModel::Orchidee, 3), // STOMATE
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::LpjGuess, 2), // Vegetation Dynamics
        justification: Some("STOMATE derives from LPJ vegetation dynamics".to_string()),
    });

    // CABLE POP demographic model is a variant of ED2 cohort dynamics
    relations.push(Relation {
        source: make_id(SourceModel::Cable, 3), // POP
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Ed2, 2), // Cohort Dynamics
        justification: Some("Both age/size-structured forest demography".to_string()),
    });

    // ED2 cohort dynamics is a variant of FATES demography
    relations.push(Relation {
        source: make_id(SourceModel::Ed2, 2), // Cohort Dynamics
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Fates, 1), // Cohort Demography
        justification: Some(
            "FATES extends ED2 cohort-patch framework with allometric flexibility".to_string(),
        ),
    });

    // LM3-PPA is a variant of ED2 patch dynamics
    relations.push(Relation {
        source: make_id(SourceModel::Lm3Ppa, 1), // Perfect Plasticity Approximation
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Ed2, 3), // Patch Dynamics
        justification: Some(
            "PPA simplifies ED2 patch representation with crown-packing".to_string(),
        ),
    });

    // --- Fire model variants ---

    // JULES INFERNO is a variant of LPJ-GUESS BLAZE
    relations.push(Relation {
        source: make_id(SourceModel::Jules, 3), // INFERNO
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::LpjGuess, 3), // BLAZE
        justification: Some(
            "Both process-based fire models with ignition/spread/emission".to_string(),
        ),
    });

    // CLASSIC CTEM-Fire is a variant of LPJ-GUESS BLAZE
    relations.push(Relation {
        source: make_id(SourceModel::Classic, 3), // CTEM-Fire
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::LpjGuess, 3), // BLAZE
        justification: Some("Both area-burned models with fire occurrence × spread".to_string()),
    });

    // LANDIS-II SCRPPLE is incompatible with WRF-SFIRE (different resolution)
    relations.push(Relation {
        source: make_id(SourceModel::LandisNecn, 2), // SCRPPLE
        relation: RelationType::IncompatibleWith,
        target: make_id(SourceModel::WrfSfire, 1), // Level Set
        justification: Some(
            "Landscape-scale fire vs. metre-scale fire front — resolution mismatch".to_string(),
        ),
    });

    // FireBench benchmark compatible with WRF-SFIRE for validation
    relations.push(Relation {
        source: make_id(SourceModel::FireBench, 1), // Benchmark
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::WrfSfire, 1), // Level Set
        justification: Some(
            "FireBench provides validation datasets for high-fidelity fire models".to_string(),
        ),
    });

    // PhiSat-2 fire detection compatible with FireBench
    relations.push(Relation {
        source: make_id(SourceModel::PhiSat2, 2), // Fire Detection
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::FireBench, 1), // Benchmark
        justification: Some(
            "Satellite fire detection provides observational input for benchmarking".to_string(),
        ),
    });

    // --- Hydrology/land surface model variants ---

    // SUMMA multi-physics Richards is a variant of ORCHIDEE CWRR
    relations.push(Relation {
        source: make_id(SourceModel::Summa, 1), // Multi-Physics Richards
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Orchidee, 2), // CWRR Richards
        justification: Some(
            "Both multi-layer Richards, SUMMA adds switchable parameterizations".to_string(),
        ),
    });

    // VIC runoff is incompatible with SUMMA Richards (different paradigms)
    relations.push(Relation {
        source: make_id(SourceModel::Vic, 1), // Variable Infiltration Capacity
        relation: RelationType::IncompatibleWith,
        target: make_id(SourceModel::Summa, 1), // Richards
        justification: Some(
            "VIC capacity-curve vs Richards PDE — different runoff philosophies".to_string(),
        ),
    });

    // LISFLOOD-FP 2D inundation supersedes VIC for flood applications
    relations.push(Relation {
        source: make_id(SourceModel::LisfloodFp, 2), // 2D Inundation
        relation: RelationType::Supersedes,
        target: make_id(SourceModel::Vic, 1), // VIC Runoff
        justification: Some(
            "2D inertial equations resolve flood extent, VIC is 1D column".to_string(),
        ),
    });

    // LISFLOOD-FP sub-grid channel requires 2D floodplain
    relations.push(Relation {
        source: make_id(SourceModel::LisfloodFp, 1), // Sub-Grid Channel
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::LisfloodFp, 2), // 2D Floodplain
        justification: Some("Channel overtopping feeds floodplain inundation".to_string()),
    });

    // --- Land surface energy balance variants ---

    // JULES surface energy is a variant of ORCHIDEE SECHIBA
    relations.push(Relation {
        source: make_id(SourceModel::Jules, 1), // Surface Energy Balance
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Orchidee, 1), // SECHIBA
        justification: Some("Both tile-based coupled radiation/turbulent flux solvers".to_string()),
    });

    // CLASSIC CLASS is a variant of JULES surface energy
    relations.push(Relation {
        source: make_id(SourceModel::Classic, 1), // CLASS
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Jules, 1), // Surface Energy Balance
        justification: Some("CLASS and JULES share Met Office heritage".to_string()),
    });

    // SUMMA surface energy is a variant of ORCHIDEE SECHIBA
    relations.push(Relation {
        source: make_id(SourceModel::Summa, 3), // Switchable SEB
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Orchidee, 1), // SECHIBA
        justification: Some(
            "Both implicit coupled SEB, SUMMA adds switchable formulations".to_string(),
        ),
    });

    // --- Biogeochemistry variants ---

    // CABLE CASA-CNP is a variant of LANDIS-II NECN
    relations.push(Relation {
        source: make_id(SourceModel::Cable, 2), // CASA-CNP
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::LandisNecn, 1), // NECN
        justification: Some("Both CENTURY-derived C-N models with different coupling".to_string()),
    });

    // LM3-PPA BiomeE CNP is a variant of CABLE CASA-CNP
    relations.push(Relation {
        source: make_id(SourceModel::Lm3Ppa, 2), // BiomeE C-N-P
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Cable, 2), // CASA-CNP
        justification: Some("Both C-N-P cycling with different vegetation models".to_string()),
    });

    // --- Ocean and coastal model relations ---

    // E3SM MPAS-Ocean is a variant of CESM MOM6
    relations.push(Relation {
        source: make_id(SourceModel::E3sm, 1), // MPAS-Ocean
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Cesm, 50), // MOM6
        justification: Some(
            "Both 3D ocean primitive equations, MPAS uses unstructured mesh".to_string(),
        ),
    });

    // GFDL COBALT is a variant of MARBL
    relations.push(Relation {
        source: make_id(SourceModel::GfdlEsm4, 2), // COBALT
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Marbl, 1), // MARBL Phytoplankton
        justification: Some(
            "Both multi-PFT ocean biogeochemistry with different nutrient cycles".to_string(),
        ),
    });

    // ROMS primitive equations are compatible with Delft3D shallow water
    relations.push(Relation {
        source: make_id(SourceModel::Roms, 1), // 3D Primitive Equations
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Delft3d, 1), // Shallow Water
        justification: Some("ROMS far-field to Delft3D nearshore nesting".to_string()),
    });

    // ROMS sediment transport is a variant of Delft3D sediment
    relations.push(Relation {
        source: make_id(SourceModel::Roms, 2), // Sediment Transport
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Delft3d, 2), // Cohesive Sediment
        justification: Some(
            "Both Exner-based morphodynamics with different sediment classes".to_string(),
        ),
    });

    // Delft3D DELWAQ requires shallow water for transport
    relations.push(Relation {
        source: make_id(SourceModel::Delft3d, 3), // DELWAQ
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Delft3d, 1), // Shallow Water
        justification: Some("Water quality advected by hydrodynamic flow field".to_string()),
    });

    // ROMS NPZD is a variant of GFDL COBALT (simplified)
    relations.push(Relation {
        source: make_id(SourceModel::Roms, 3), // NPZD
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::GfdlEsm4, 2), // COBALT
        justification: Some(
            "NPZD is a reduced-complexity marine ecosystem vs full COBALT".to_string(),
        ),
    });

    // --- Atmosphere and chemistry ---

    // E3SM EAM is a variant of CESM CAM
    relations.push(Relation {
        source: make_id(SourceModel::E3sm, 3), // EAM
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Cesm, 1), // CAM Deep Convection
        justification: Some(
            "EAM shares CAM heritage with CLUBB replacing separate BL/convection".to_string(),
        ),
    });

    // GFDL FV3 is a variant of CESM CAM dynamics
    relations.push(Relation {
        source: make_id(SourceModel::GfdlEsm4, 1), // FV3
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Cesm, 1), // CAM
        justification: Some(
            "FV3 cubed-sphere vs CAM SE: different dynamical cores, same purpose".to_string(),
        ),
    });

    // GEOS-Chem tropospheric chem requires EAM/CAM for met fields
    relations.push(Relation {
        source: make_id(SourceModel::GeosChem, 1), // Tropospheric Chemistry
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::E3sm, 3), // EAM
        justification: Some(
            "Chemistry requires 3D wind, temperature, humidity from atmosphere".to_string(),
        ),
    });

    // GEOS-Chem ISORROPIA compatible with CARMA aerosol
    relations.push(Relation {
        source: make_id(SourceModel::GeosChem, 2), // ISORROPIA-II
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Carma, 1), // Nucleation
        justification: Some(
            "Inorganic aerosol thermodynamics feeds size-resolved microphysics".to_string(),
        ),
    });

    // --- Subsurface and geology ---

    // PFLOTRAN reactive transport supersedes SWAT simple nutrient leaching
    relations.push(Relation {
        source: make_id(SourceModel::Pflotran, 1), // Reactive Transport
        relation: RelationType::Supersedes,
        target: make_id(SourceModel::Swat, 1), // SCS Runoff (nutrient transport)
        justification: Some(
            "3D multicomponent reactive transport vs. empirical curve-number".to_string(),
        ),
    });

    // PFLOTRAN geomechanics requires reactive transport for pressure updates
    relations.push(Relation {
        source: make_id(SourceModel::Pflotran, 3), // Geomechanics
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Pflotran, 1), // Reactive Transport
        justification: Some(
            "Poroelastic stress depends on pore pressure from flow solution".to_string(),
        ),
    });

    // --- Cryosphere ---

    // VIC frozen soil is a variant of CryoGrid freeze-thaw
    relations.push(Relation {
        source: make_id(SourceModel::Vic, 3), // Frozen Soil
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::CryoGrid, 2), // Freeze-Thaw
        justification: Some(
            "Both compute ice lens formation, CryoGrid has higher vertical resolution".to_string(),
        ),
    });

    // SUMMA snow is a variant of VIC snow
    relations.push(Relation {
        source: make_id(SourceModel::Summa, 2), // Switchable Snow
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Vic, 2), // Energy-Balance Snow
        justification: Some(
            "Both energy-balance snow, SUMMA allows switchable compaction/albedo".to_string(),
        ),
    });

    // UVAFME permafrost-vegetation compatible with CryoGrid
    relations.push(Relation {
        source: make_id(SourceModel::Uvafme, 2), // Permafrost-Vegetation Feedback
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::CryoGrid, 3), // Active Layer
        justification: Some(
            "Active layer depth drives root available depth at treeline".to_string(),
        ),
    });

    // --- Individual-based forest model variants ---

    // SORTIE-ND hemispheric light is a variant of ED2 canopy radiation
    relations.push(Relation {
        source: make_id(SourceModel::SortieNd, 1), // Hemispheric Light
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Ed2, 1), // Multi-Layer Canopy Radiation
        justification: Some(
            "Both explicit tree-level light competition — different algorithms".to_string(),
        ),
    });

    // SORTIE-NG GPU hemispheric supersedes SORTIE-ND (same algorithm, GPU)
    relations.push(Relation {
        source: make_id(SourceModel::SortieNg, 1), // GPU Hemispheric Light
        relation: RelationType::Supersedes,
        target: make_id(SourceModel::SortieNd, 1), // CPU Hemispheric Light
        justification: Some(
            "GPU implementation enables larger spatial domains at same fidelity".to_string(),
        ),
    });

    // FORMIND individual tree is a variant of iLand individual tree
    relations.push(Relation {
        source: make_id(SourceModel::Formind, 1), // Individual Tree Competition
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::ILand, 1), // 3-PG Individual
        justification: Some("Both individual-based forest models for different biomes".to_string()),
    });

    // JABOWA gap model is variant of ForClim
    relations.push(Relation {
        source: make_id(SourceModel::JabowaForet, 1), // Gap-Phase Succession
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Forclim, 1), // Patch-Based Dynamics
        justification: Some("ForClim extends JABOWA lineage with climate sensitivity".to_string()),
    });

    // --- ML/Foundation models ---

    // DeepLand neural ET emulator is a variant of ELM evapotranspiration
    relations.push(Relation {
        source: make_id(SourceModel::DeepLand, 1), // Neural ET
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::E3sm, 2), // ELM Land Model
        justification: Some("ML emulator trained on ELM-class land surface physics".to_string()),
    });

    // Earth-2 FourCastNet supersedes empirical weather for speed
    relations.push(Relation {
        source: make_id(SourceModel::Earth2, 1), // FourCastNet
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::GfdlEsm4, 1), // FV3
        justification: Some(
            "ML weather forecast achieves comparable skill orders of magnitude faster".to_string(),
        ),
    });

    // Earth-2 GraphCast is a variant of FourCastNet
    relations.push(Relation {
        source: make_id(SourceModel::Earth2, 2), // GraphCast
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Earth2, 1), // FourCastNet
        justification: Some(
            "Both ML global weather but different architectures — GNN vs AFNO".to_string(),
        ),
    });

    // Earth-2 CorrDiff requires coarse forecast input
    relations.push(Relation {
        source: make_id(SourceModel::Earth2, 3), // CorrDiff
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Earth2, 1), // FourCastNet
        justification: Some(
            "Downscaling requires coarse global forecast as conditioning".to_string(),
        ),
    });

    // PhiSat-2 cloud masking compatible with Earth-2 (observational data)
    relations.push(Relation {
        source: make_id(SourceModel::PhiSat2, 1), // Cloud Masking
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Earth2, 1), // FourCastNet
        justification: Some(
            "Satellite observations provide verification for forecast models".to_string(),
        ),
    });

    // --- Watershed models ---

    // SWAT erosion compatible with SWAT hydrology
    relations.push(Relation {
        source: make_id(SourceModel::Swat, 2), // MUSLE
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Swat, 1), // SCS CN
        justification: Some(
            "MUSLE sediment yield requires runoff volume from SCS Curve Number".to_string(),
        ),
    });

    // SWAT crop growth compatible with SWAT hydrology
    relations.push(Relation {
        source: make_id(SourceModel::Swat, 3), // Crop Growth
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Swat, 1), // SCS CN
        justification: Some("Crop water demand affects soil moisture and runoff".to_string()),
    });

    // SWAT crop is a variant of APSIM phenology (simplified)
    relations.push(Relation {
        source: make_id(SourceModel::Swat, 3), // Crop Growth (EPIC-based)
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Apsim, 1), // APSIM Phenology
        justification: Some(
            "Both heat-unit phenology, SWAT simplified from EPIC for watershed scale".to_string(),
        ),
    });

    // --- Data assimilation ---

    // PEcAn Bayesian calibration compatible with FATES
    relations.push(Relation {
        source: make_id(SourceModel::Pecan, 1), // Bayesian Calibration
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Fates, 1), // Cohort Demography
        justification: Some(
            "PEcAn calibrates trait parameters for FATES cohort models".to_string(),
        ),
    });

    // PEcAn data assimilation compatible with any ecological model
    relations.push(Relation {
        source: make_id(SourceModel::Pecan, 2), // Ensemble DA
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Ed2, 2), // ED2 Cohort Dynamics
        justification: Some(
            "PEcAn EnKF/PF can assimilate data into any ecological model".to_string(),
        ),
    });

    // --- LANDIS and landscape models ---

    // LANDIS-II seed dispersal compatible with iLand bark beetle
    relations.push(Relation {
        source: make_id(SourceModel::LandisII, 2), // Seed Dispersal
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::ILand, 2), // Bark Beetle
        justification: Some("Post-disturbance recolonisation via seed dispersal".to_string()),
    });

    // LANDIS-II NECN compatible with LANDIS-II core succession
    relations.push(Relation {
        source: make_id(SourceModel::LandisNecn, 1), // NECN
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::LandisII, 1), // Succession Interface
        justification: Some("NECN implements the succession extension interface".to_string()),
    });

    // ========================================================================
    // MAESTRA — 3D Individual-Tree Canopy Model
    // ========================================================================

    // MAESTRA Farquhar photosynthesis is a variant of CLM/FATES Farquhar
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 3), // Farquhar Photosynthesis
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Cesm, 5), // CLM Photosynthesis
        justification: Some(
            "Both implement Farquhar-von Caemmerer 1980; MAESTRA uses ECOCRAFT parameterization"
                .to_string(),
        ),
    });

    // MAESTRA photosynthesis requires coupling with stomatal conductance
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 3), // Farquhar Photosynthesis
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Maestra, 4), // Stomatal Conductance
        justification: Some(
            "Photosynthesis-stomatal conductance coupled via iterative Ci solution".to_string(),
        ),
    });

    // MAESTRA stomatal conductance requires coupling with transpiration/energy balance
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 4), // Stomatal Conductance
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Maestra, 5), // Penman-Monteith ET
        justification: Some(
            "Stomatal conductance feeds Penman-Monteith; leaf temperature feedback loop"
                .to_string(),
        ),
    });

    // MAESTRA radiation requires coupling with crown geometry
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 1), // 3D Crown Radiation
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Maestra, 7), // Crown Geometry
        justification: Some(
            "3D radiation transfer needs crown grid points, LAD distribution, and leaf angles"
                .to_string(),
        ),
    });

    // MAESTRA radiation requires coupling with solar geometry
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 1), // 3D Crown Radiation
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Maestra, 2), // Solar Geometry
        justification: Some(
            "Beam radiation needs solar zenith/azimuth; diffuse needs daylength".to_string(),
        ),
    });

    // MAESTRA 3D radiation is compatible with iLand light resource index
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 1), // 3D Crown Radiation
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::ILand, 1), // iLand Light Index
        justification: Some(
            "Both compute individual-tree light interception; MAESTRA at higher fidelity"
                .to_string(),
        ),
    });

    // MAESTRA photosynthesis compatible with FATES/ED2 leaf-level carbon
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 3), // Farquhar Photosynthesis
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Fates, 1), // FATES Photosynthesis
        justification: Some(
            "Both Farquhar-based leaf-level photosynthesis; MAESTRA adds 3D crown sampling"
                .to_string(),
        ),
    });

    // MAESTRA canopy integration compatible with CABLE canopy scheme
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 9), // Canopy Integration
        relation: RelationType::CompatibleWith,
        target: make_id(SourceModel::Cable, 1), // CABLE Canopy
        justification: Some(
            "MAESTRA 3D integration can replace CABLE 1D two-leaf scheme at plot scale".to_string(),
        ),
    });

    // MAESTRA multi-scattering is a variant of CLM/CESM canopy radiation
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 10), // Multi-Scattering
        relation: RelationType::VariantOf,
        target: make_id(SourceModel::Cesm, 3), // CLM Radiation
        justification: Some(
            "Norman 1979 scheme used in both; MAESTRA operates on 3D crown, CLM on horizontally \
             homogeneous layers"
                .to_string(),
        ),
    });

    // MAESTRA met processing requires coupling with 3D radiation
    relations.push(Relation {
        source: make_id(SourceModel::Maestra, 8), // Met Processing
        relation: RelationType::RequiresCouplingWith,
        target: make_id(SourceModel::Maestra, 1), // 3D Crown Radiation
        justification: Some(
            "Met module provides disaggregated beam/diffuse radiation to canopy model".to_string(),
        ),
    });

    relations
}

/// Count relations by type.
pub fn relation_statistics(relations: &[Relation]) -> RelationStats {
    let mut stats = RelationStats::default();
    for r in relations {
        match r.relation {
            RelationType::CompatibleWith => stats.compatible += 1,
            RelationType::IncompatibleWith => stats.incompatible += 1,
            RelationType::RequiresCouplingWith => stats.requires_coupling += 1,
            RelationType::Supersedes => stats.supersedes += 1,
            RelationType::VariantOf => stats.variant_of += 1,
        }
    }
    stats
}

#[derive(Debug, Default)]
pub struct RelationStats {
    pub compatible: usize,
    pub incompatible: usize,
    pub requires_coupling: usize,
    pub supersedes: usize,
    pub variant_of: usize,
}

impl RelationStats {
    pub fn total(&self) -> usize {
        self.compatible
            + self.incompatible
            + self.requires_coupling
            + self.supersedes
            + self.variant_of
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_relations_count() {
        let relations = generate_seed_relations();
        assert!(relations.len() >= 30, "Expected at least 30 seed relations");
    }

    #[test]
    fn test_relation_statistics() {
        let relations = generate_seed_relations();
        let stats = relation_statistics(&relations);

        assert!(stats.compatible > 0, "Expected some compatible relations");
        assert!(
            stats.requires_coupling > 0,
            "Expected some coupling requirements"
        );
        assert_eq!(stats.total(), relations.len());
    }

    #[test]
    fn test_no_self_relations() {
        let relations = generate_seed_relations();
        for r in &relations {
            assert_ne!(r.source, r.target, "No self-relations allowed");
        }
    }
}
