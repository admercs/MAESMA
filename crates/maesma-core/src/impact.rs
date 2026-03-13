//! Planetary defense and impact cascade modeling — Phase 13
//!
//! NEO data pipeline, impact cascade physics, mass extinction calibration,
//! and deflection strategy assessment.

use serde::{Deserialize, Serialize};

/// NEO data source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NeoDataSource {
    CneosSentry,
    CneosScout,
    Horizons,
    MinorPlanetCenter,
    EsaNeocc,
    SurveyTelescope,
}

/// NEO threat entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoThreatEntry {
    pub designation: String,
    pub source: NeoDataSource,
    pub palermo_scale: f64,
    pub diameter_m: f64,
    pub impact_probability: f64,
    pub closest_approach_year: Option<i32>,
}

/// Standard NEO data sources (Phase 13.1).
pub fn neo_data_sources() -> Vec<NeoDataSource> {
    vec![
        NeoDataSource::CneosSentry,
        NeoDataSource::CneosScout,
        NeoDataSource::Horizons,
        NeoDataSource::MinorPlanetCenter,
        NeoDataSource::EsaNeocc,
        NeoDataSource::SurveyTelescope,
    ]
}

/// Impact parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactParameters {
    pub diameter_km: f64,
    pub velocity_km_s: f64,
    pub angle_deg: f64,
    pub density_kg_m3: f64,
}

/// Energy partition from impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyPartition {
    pub total_energy_mt: f64,
    pub atmospheric_fraction: f64,
    pub thermal_fraction: f64,
    pub seismic_fraction: f64,
    pub ejecta_fraction: f64,
}

/// Compute approximate impact energy in megatons TNT.
pub fn impact_energy_mt(params: &ImpactParameters) -> f64 {
    let radius_m = params.diameter_km * 500.0;
    let volume = (4.0 / 3.0) * std::f64::consts::PI * radius_m.powi(3);
    let mass = volume * params.density_kg_m3;
    let ke_j = 0.5 * mass * (params.velocity_km_s * 1000.0).powi(2);
    ke_j / 4.184e15 // convert joules to megatons
}

/// Impact cascade effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactCascadeEffects {
    pub aod_injection: f64,
    pub radiative_forcing_wm2: f64,
    pub firestorm_probability: f64,
    pub tsunami_height_m: Option<f64>,
    pub nuclear_winter_duration_years: f64,
    pub photosynthesis_reduction_pct: f64,
}

/// Mass extinction calibration scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtinctionCalibration {
    pub name: String,
    pub age_mya: f64,
    pub cause: String,
    pub marine_loss_pct: f64,
    pub recovery_myr: f64,
}

/// Standard calibration scenarios (Phase 13.3).
pub fn extinction_calibrations() -> Vec<ExtinctionCalibration> {
    vec![
        ExtinctionCalibration {
            name: "K-Pg".into(),
            age_mya: 66.0,
            cause: "10 km impactor (Chicxulub)".into(),
            marine_loss_pct: 75.0,
            recovery_myr: 10.0,
        },
        ExtinctionCalibration {
            name: "P-T".into(),
            age_mya: 252.0,
            cause: "Siberian Traps volcanism".into(),
            marine_loss_pct: 96.0,
            recovery_myr: 5.0,
        },
        ExtinctionCalibration {
            name: "Late Devonian".into(),
            age_mya: 372.0,
            cause: "Multiple (anoxia, volcanism)".into(),
            marine_loss_pct: 75.0,
            recovery_myr: 15.0,
        },
        ExtinctionCalibration {
            name: "End-Triassic".into(),
            age_mya: 201.0,
            cause: "CAMP volcanism".into(),
            marine_loss_pct: 80.0,
            recovery_myr: 8.0,
        },
    ]
}

/// Deflection strategy type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeflectionStrategy {
    KineticImpactor,
    GravityTractor,
    IonBeam,
    NuclearStandoff,
}

/// Deflection campaign assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeflectionCampaign {
    pub strategy: DeflectionStrategy,
    pub neo_diameter_m: f64,
    pub lead_time_years: f64,
    pub delta_v_cm_s: f64,
    pub success_probability: f64,
}

/// Simple delta-v estimation for kinetic impactor.
pub fn kinetic_impactor_delta_v(
    impactor_mass_kg: f64,
    impactor_velocity_km_s: f64,
    neo_mass_kg: f64,
    beta: f64,
) -> f64 {
    // beta = momentum enhancement factor (typically 1-5 for DART-class)
    (beta * impactor_mass_kg * impactor_velocity_km_s * 1000.0) / neo_mass_kg * 100.0 // cm/s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impact_energy_chicxulub() {
        let params = ImpactParameters {
            diameter_km: 10.0,
            velocity_km_s: 20.0,
            angle_deg: 45.0,
            density_kg_m3: 3000.0,
        };
        let energy = impact_energy_mt(&params);
        // Should be on order 10^8 MT
        assert!(energy > 1e7);
    }

    #[test]
    fn extinction_calibrations_populated() {
        let c = extinction_calibrations();
        assert_eq!(c.len(), 4);
        assert!(c.iter().any(|e| e.name == "K-Pg"));
    }

    #[test]
    fn deflection_delta_v() {
        let dv = kinetic_impactor_delta_v(500.0, 6.0, 5e9, 3.0);
        assert!(dv > 0.0);
    }

    #[test]
    fn neo_sources_count() {
        assert_eq!(neo_data_sources().len(), 6);
    }
}
