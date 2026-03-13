//! Concrete process implementations for all 11 MAESMA process families.
//!
//! Each family module provides implementations at multiple fidelity rungs
//! (R0–R3), from simple empirical parameterizations to full physics-based models.

pub mod atmosphere;
pub mod biogeochemistry;
pub mod cryosphere;
pub mod ecology;
pub mod evolution;
pub mod fire;
pub mod human_systems;
pub mod hydrology;
pub mod ocean;
pub mod radiation;
pub mod trophic_dynamics;

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner};

/// Create a default `ProcessRunner` for the given family and fidelity rung.
///
/// Returns `None` if no implementation exists for the requested (family, rung)
/// combination.
pub fn create_runner(family: ProcessFamily, rung: FidelityRung) -> Option<Box<dyn ProcessRunner>> {
    match (family, rung) {
        // Fire
        (ProcessFamily::Fire, FidelityRung::R0) => {
            Some(Box::new(fire::StochasticFireRegime::default()))
        }
        (ProcessFamily::Fire, FidelityRung::R1) => {
            Some(Box::new(fire::rothermel::RothermelSurface::default()))
        }
        // Hydrology
        (ProcessFamily::Hydrology, FidelityRung::R0) => {
            Some(Box::new(hydrology::BucketModel::default()))
        }
        (ProcessFamily::Hydrology, FidelityRung::R1) => {
            Some(Box::new(hydrology::RichardsInfiltration::default()))
        }
        // Radiation
        (ProcessFamily::Radiation, FidelityRung::R0) => {
            Some(Box::new(radiation::BeerLambertRadiation::default()))
        }
        (ProcessFamily::Radiation, FidelityRung::R1) => {
            Some(Box::new(radiation::TwoStreamRadiation::default()))
        }
        // Atmosphere
        (ProcessFamily::Atmosphere, FidelityRung::R0) => {
            Some(Box::new(atmosphere::BulkTransferAtmosphere::default()))
        }
        (ProcessFamily::Atmosphere, FidelityRung::R1) => {
            Some(Box::new(atmosphere::MoninObukhovSurfaceLayer::default()))
        }
        // Ocean
        (ProcessFamily::Ocean, FidelityRung::R0) => Some(Box::new(ocean::SlabOcean::default())),
        (ProcessFamily::Ocean, FidelityRung::R1) => {
            Some(Box::new(ocean::MixedLayerOcean::default()))
        }
        // Cryosphere
        (ProcessFamily::Cryosphere, FidelityRung::R0) => {
            Some(Box::new(cryosphere::DegreeDayMelt::default()))
        }
        (ProcessFamily::Cryosphere, FidelityRung::R1) => {
            Some(Box::new(cryosphere::SnowpackModel::default()))
        }
        // Biogeochemistry
        (ProcessFamily::Biogeochemistry, FidelityRung::R0) => {
            Some(Box::new(biogeochemistry::SinglePoolCarbon::default()))
        }
        (ProcessFamily::Biogeochemistry, FidelityRung::R1) => {
            Some(Box::new(biogeochemistry::CenturySoilCarbon::default()))
        }
        // Ecology
        (ProcessFamily::Ecology, FidelityRung::R0) => {
            Some(Box::new(ecology::StaticVegetation::default()))
        }
        (ProcessFamily::Ecology, FidelityRung::R1) => {
            Some(Box::new(ecology::CohortVegetation::default()))
        }
        // Trophic Dynamics
        (ProcessFamily::TrophicDynamics, FidelityRung::R0) => {
            Some(Box::new(trophic_dynamics::StaticFoodWeb))
        }
        (ProcessFamily::TrophicDynamics, FidelityRung::R1) => {
            Some(Box::new(trophic_dynamics::LotkaVolterraTrophic::default()))
        }
        // Evolution
        (ProcessFamily::Evolution, FidelityRung::R0) => Some(Box::new(evolution::FixedTraits)),
        (ProcessFamily::Evolution, FidelityRung::R1) => {
            Some(Box::new(evolution::QuantitativeGeneticsEvolution::default()))
        }
        // Human Systems
        (ProcessFamily::HumanSystems, FidelityRung::R0) => {
            Some(Box::new(human_systems::PrescribedLandUse::default()))
        }
        (ProcessFamily::HumanSystems, FidelityRung::R1) => {
            Some(Box::new(human_systems::LandUseChange::default()))
        }
        _ => None,
    }
}

/// Create default runners for all 13 families at their lowest available rung.
pub fn create_default_runners() -> Vec<(ProcessFamily, FidelityRung, Box<dyn ProcessRunner>)> {
    let configs: &[(ProcessFamily, FidelityRung)] = &[
        (ProcessFamily::Fire, FidelityRung::R0),
        (ProcessFamily::Hydrology, FidelityRung::R0),
        (ProcessFamily::Radiation, FidelityRung::R0),
        (ProcessFamily::Atmosphere, FidelityRung::R0),
        (ProcessFamily::Ocean, FidelityRung::R0),
        (ProcessFamily::Cryosphere, FidelityRung::R0),
        (ProcessFamily::Biogeochemistry, FidelityRung::R0),
        (ProcessFamily::Ecology, FidelityRung::R0),
        (ProcessFamily::TrophicDynamics, FidelityRung::R0),
        (ProcessFamily::Evolution, FidelityRung::R0),
        (ProcessFamily::HumanSystems, FidelityRung::R0),
    ];

    configs
        .iter()
        .filter_map(|(fam, rung)| create_runner(*fam, *rung).map(|runner| (*fam, *rung, runner)))
        .collect()
}
