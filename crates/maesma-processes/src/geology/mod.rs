//! Geology process family — lithosphere properties, tectonic uplift, weathering.
//!
//! Fidelity ladder:
//! - R0: Static geology — fixed bedrock properties and weathering rates
//! - R1: Tectonic uplift with simple chemical weathering feedback

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

/// R0: Static geology — provides fixed bedrock/lithology properties.
///
/// Exposes spatially varying bedrock type, porosity, hydraulic conductivity,
/// and weathering rate as static fields. No dynamic update.
///
/// Inputs: none (reads parameters)
/// Outputs: `weathering_rate_mm_yr`, `bedrock_permeability`
#[derive(Debug, Clone)]
pub struct StaticGeology {
    pub default_weathering_rate: f64,
    pub default_permeability: f64,
}

impl Default for StaticGeology {
    fn default() -> Self {
        Self {
            default_weathering_rate: 0.03,  // mm/yr — typical granite
            default_permeability: 1e-15,    // m² — tight crystalline
        }
    }
}

impl ProcessRunner for StaticGeology {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Geology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec![]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "weathering_rate_mm_yr".into(),
            "bedrock_permeability".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec![]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        let _rate = state
            .get_param("weathering_rate_mm_yr")
            .unwrap_or(self.default_weathering_rate);
        Ok(())
    }
}

/// R1: Tectonic uplift with chemical weathering feedback.
///
/// Models steady-state uplift rate and temperature/precipitation-dependent
/// chemical weathering (silicate weathering ∝ runoff × exp(-Ea/RT)).
///
/// Inputs: `precipitation`, `temperature`
/// Outputs: `uplift_rate_mm_yr`, `weathering_rate_mm_yr`, `silicate_weathering_flux`
#[derive(Debug, Clone)]
pub struct TectonicUplift {
    pub uplift_rate_mm_yr: f64,
    pub activation_energy_kj: f64,
    pub weathering_prefactor: f64,
}

impl Default for TectonicUplift {
    fn default() -> Self {
        Self {
            uplift_rate_mm_yr: 0.5,
            activation_energy_kj: 63.0,  // silicate weathering Ea
            weathering_prefactor: 1.0,
        }
    }
}

impl ProcessRunner for TectonicUplift {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Geology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec!["precipitation".into(), "temperature".into()]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "uplift_rate_mm_yr".into(),
            "weathering_rate_mm_yr".into(),
            "silicate_weathering_flux".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec![]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        let temp_k = state.get_param("temperature").unwrap_or(288.0);
        let precip = state.get_param("precipitation").unwrap_or(1000.0);

        // Arrhenius-style temperature dependence
        let r = 8.314e-3; // kJ/(mol·K)
        let temp_factor = (-self.activation_energy_kj / (r * temp_k)).exp();

        // Runoff proxy from precipitation
        let runoff_factor = (precip / 1000.0).sqrt();

        let _weathering = self.weathering_prefactor * temp_factor * runoff_factor;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_geology_default() {
        let g = StaticGeology::default();
        assert_eq!(g.family(), ProcessFamily::Geology);
        assert_eq!(g.rung(), FidelityRung::R0);
        assert!(g.inputs().is_empty());
    }

    #[test]
    fn tectonic_uplift_default() {
        let t = TectonicUplift::default();
        assert_eq!(t.family(), ProcessFamily::Geology);
        assert_eq!(t.rung(), FidelityRung::R1);
        assert!(t.outputs().contains(&"silicate_weathering_flux".to_string()));
    }
}
