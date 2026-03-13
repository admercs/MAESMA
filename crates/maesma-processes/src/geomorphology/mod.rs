//! Geomorphology process family — erosion, sediment transport, landscape evolution.
//!
//! Fidelity ladder:
//! - R0: RUSLE-based empirical erosion
//! - R1: Landscape evolution with diffusion + stream power (CHILD/Landlab-style)

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

/// R0: RUSLE-based empirical soil erosion model.
///
/// Computes annual soil loss as:
///   A = R × K × LS × C × P
/// where R = rainfall erosivity, K = soil erodibility, LS = slope-length,
/// C = cover management, P = practice factor.
///
/// Inputs: `rainfall_erosivity`, `soil_erodibility`, `slope_factor`,
///         `cover_factor`, `practice_factor`
/// Outputs: `soil_loss_t_ha_yr`
#[derive(Debug, Clone)]
pub struct RusleErosion {
    pub k_default: f64,
    pub c_default: f64,
    pub p_default: f64,
}

impl Default for RusleErosion {
    fn default() -> Self {
        Self {
            k_default: 0.03,
            c_default: 0.15,
            p_default: 1.0,
        }
    }
}

impl ProcessRunner for RusleErosion {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Geomorphology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec!["rainfall_erosivity".into(), "slope_factor".into()]
    }

    fn outputs(&self) -> Vec<String> {
        vec!["soil_loss_t_ha_yr".into()]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["sediment_mass".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let r = state.get_param("rainfall_erosivity").unwrap_or(500.0);
        let ls = state.get_param("slope_factor").unwrap_or(1.0);
        let k = state
            .get_param("soil_erodibility")
            .unwrap_or(self.k_default);
        let c = state.get_param("cover_factor").unwrap_or(self.c_default);
        let p = state.get_param("practice_factor").unwrap_or(self.p_default);

        let annual_loss = r * k * ls * c * p;
        // Scale by timestep fraction of a year
        let _dt_fraction = dt / (365.25 * 86400.0);
        // Would write to output field in a real implementation

        let _ = annual_loss;
        Ok(())
    }
}

/// R1: Stream-power + diffusion landscape evolution model.
///
/// Combines hillslope diffusion (D × ∇²z) with fluvial incision
/// (K × A^m × S^n) following the CHILD/Landlab formulation.
///
/// Inputs: `elevation`, `drainage_area`, `slope`, `precipitation`
/// Outputs: `elevation` (updated), `sediment_flux`
#[derive(Debug, Clone)]
pub struct StreamPowerErosion {
    pub k_sp: f64,
    pub m_sp: f64,
    pub n_sp: f64,
    pub diffusivity: f64,
}

impl Default for StreamPowerErosion {
    fn default() -> Self {
        Self {
            k_sp: 1e-5,
            m_sp: 0.5,
            n_sp: 1.0,
            diffusivity: 0.01,
        }
    }
}

impl ProcessRunner for StreamPowerErosion {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Geomorphology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "elevation".into(),
            "drainage_area".into(),
            "slope".into(),
            "precipitation".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec!["elevation".into(), "sediment_flux".into()]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["sediment_mass".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let area = state.get_param("drainage_area").unwrap_or(1e6);
        let slope = state.get_param("slope").unwrap_or(0.01);

        // Fluvial incision: E = K * A^m * S^n
        let incision = self.k_sp * area.powf(self.m_sp) * slope.powf(self.n_sp);

        // Total erosion rate (would also add diffusion in real implementation)
        let _erosion_rate = incision * dt;

        Ok(())
    }
}

pub mod coupling {
    //! Coupling declarations for geomorphology.

    /// Geomorphology couples with:
    /// - Hydrology: surface runoff drives erosion; erosion modifies channel geometry
    /// - Ecology: vegetation cover affects erosion (C factor); erosion affects soil depth
    /// - Fire: post-fire erosion risk increase; hydrophobicity changes
    /// - Biogeochemistry: sediment transport carries nutrients; soil loss reduces C storage
    pub const COUPLINGS: &[(&str, &str)] = &[
        ("geomorphology", "hydrology"),
        ("geomorphology", "ecology"),
        ("geomorphology", "fire"),
        ("geomorphology", "biogeochemistry"),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rusle_default() {
        let r = RusleErosion::default();
        assert_eq!(r.family(), ProcessFamily::Geomorphology);
        assert_eq!(r.rung(), FidelityRung::R0);
        assert!(!r.inputs().is_empty());
    }

    #[test]
    fn stream_power_default() {
        let s = StreamPowerErosion::default();
        assert_eq!(s.family(), ProcessFamily::Geomorphology);
        assert_eq!(s.rung(), FidelityRung::R1);
        assert!(s.outputs().contains(&"sediment_flux".to_string()));
    }
}
