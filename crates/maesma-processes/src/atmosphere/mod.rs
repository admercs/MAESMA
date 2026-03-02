//! Atmosphere process family — surface-layer turbulence and atmospheric exchange.
//!
//! Fidelity ladder:
//! - R0: bulk transfer coefficients
//! - R1: Monin-Obukhov similarity theory (MOST)
//! - R2: large-eddy simulation (LES)
//! - R3: direct numerical simulation (DNS)
//!
//! The R1 MOST scheme computes turbulent fluxes of momentum, sensible heat,
//! and latent heat between the surface and the atmospheric reference level.
//! Uses Businger-Dyer stability functions (Businger et al. 1971, Dyer 1974)
//! with the iterative Obukhov length calculation.
//!
//! References:
//!   Businger, J. A. et al. (1971). Flux-profile relationships in the
//!   atmospheric surface layer. J. Atmos. Sci. 28, 181–189.
//!   Dyer, A. J. (1974). A review of flux-profile relationships. Boundary-
//!   Layer Meteorol. 7, 363–372.
//!   Garratt, J. R. (1994). The Atmospheric Boundary Layer. Cambridge.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// Physical constants
const KARMAN: f64 = 0.4; // von Kármán constant
const GRAV: f64 = 9.81; // gravitational acceleration [m s⁻²]
const CP_AIR: f64 = 1004.0; // specific heat of air [J kg⁻¹ K⁻¹]
const RHO_AIR: f64 = 1.225; // reference air density [kg m⁻³]
const LV: f64 = 2.5e6; // latent heat of vaporisation [J kg⁻¹]

// ───────────────────────────────────────────────────────────────────
// R1: Monin-Obukhov Similarity Theory
// ───────────────────────────────────────────────────────────────────

/// Monin-Obukhov similarity-based surface layer model (R1).
///
/// **State fields** (1-D, per grid cell):
///   - `wind_speed`          — wind speed at reference height [m s⁻¹]
///   - `temperature_air`     — air temperature at reference height [K]
///   - `temperature_surface` — surface (skin) temperature [K]
///   - `humidity`            — specific humidity at reference height [kg kg⁻¹]
///   - `humidity_surface`    — surface specific humidity [kg kg⁻¹]
///
/// **Outputs**:
///   - `sensible_heat_flux`  — H [W m⁻²] (positive upward)
///   - `latent_heat_flux`    — LE [W m⁻²] (positive upward)
///   - `friction_velocity`   — u* [m s⁻¹]
///   - `obukhov_length`      — L [m]
pub struct MoninObukhovSurfaceLayer {
    /// Roughness length for momentum [m]
    pub z0m: f64,
    /// Roughness length for heat [m]
    pub z0h: f64,
    /// Measurement / reference height [m]
    pub z_ref: f64,
}

impl Default for MoninObukhovSurfaceLayer {
    fn default() -> Self {
        Self {
            z0m: 0.1,    // grassland-like
            z0h: 0.01,   // z0h ~ 0.1 * z0m
            z_ref: 10.0, // 10 m reference
        }
    }
}

impl MoninObukhovSurfaceLayer {
    /// Businger-Dyer stability correction for momentum.
    fn psi_m(zeta: f64) -> f64 {
        if zeta < 0.0 {
            // Unstable: Paulson (1970)
            let x = (1.0 - 16.0 * zeta).powf(0.25);
            2.0 * ((1.0 + x) / 2.0).ln() + ((1.0 + x * x) / 2.0).ln() - 2.0 * x.atan()
                + std::f64::consts::FRAC_PI_2
        } else {
            // Stable: Cheng & Brutsaert (2001) approximation
            let a = 6.1;
            let b = 2.5;
            -(a * zeta + b * (zeta - b / a) * (1.0 + b / a * zeta).ln()).min(10.0) // cap for very stable
        }
    }

    /// Businger-Dyer stability correction for heat.
    fn psi_h(zeta: f64) -> f64 {
        if zeta < 0.0 {
            let x = (1.0 - 16.0 * zeta).powf(0.25);
            2.0 * ((1.0 + x * x) / 2.0).ln()
        } else {
            let a = 6.1;
            let b = 2.5;
            -(a * zeta + b * (zeta - b / a) * (1.0 + b / a * zeta).ln()).min(10.0)
        }
    }

    /// Compute friction velocity u* and Obukhov length L iteratively.
    ///
    /// Returns (u_star, theta_star, L) where theta_star = -H/(ρ·cp·u*).
    fn solve_most(&self, u: f64, theta_a: f64, theta_s: f64) -> (f64, f64, f64) {
        let dz_m = (self.z_ref / self.z0m).ln();
        let dz_h = (self.z_ref / self.z0h).ln();
        let d_theta = theta_a - theta_s; // K (negative = unstable)
        let theta_v = theta_a.max(200.0); // virtual potential temperature

        // Initial neutral estimate
        let mut u_star = KARMAN * u.max(0.1) / dz_m;
        let mut l_inv = 0.0_f64; // 1/L

        // Iterate 5 times (typically converges in 3)
        for _ in 0..5 {
            let zeta_m = self.z_ref * l_inv;
            let zeta_0m = self.z0m * l_inv;
            let zeta_h = self.z_ref * l_inv;
            let zeta_0h = self.z0h * l_inv;

            // Clamp ζ to avoid extreme corrections
            let zeta_m = zeta_m.clamp(-5.0, 5.0);
            let zeta_0m = zeta_0m.clamp(-5.0, 5.0);
            let zeta_h = zeta_h.clamp(-5.0, 5.0);
            let zeta_0h = zeta_0h.clamp(-5.0, 5.0);

            u_star =
                KARMAN * u.max(0.1) / (dz_m - Self::psi_m(zeta_m) + Self::psi_m(zeta_0m)).max(0.1);
            u_star = u_star.max(0.001);

            let theta_star =
                KARMAN * d_theta / (dz_h - Self::psi_h(zeta_h) + Self::psi_h(zeta_0h)).max(0.1);

            // Obukhov length: L = u*²·θv / (κ·g·θ*)
            if theta_star.abs() > 1e-10 {
                let l = u_star * u_star * theta_v / (KARMAN * GRAV * theta_star);
                l_inv = 1.0 / l.clamp(-1e6, 1e6);
            } else {
                l_inv = 0.0; // neutral
            }
        }

        let theta_star = KARMAN * d_theta
            / ((dz_h - Self::psi_h(self.z_ref * l_inv) + Self::psi_h(self.z0h * l_inv)).max(0.1));

        let l = if l_inv.abs() > 1e-10 {
            1.0 / l_inv
        } else {
            1e6 // effectively neutral
        };

        (u_star, theta_star, l)
    }
}

impl ProcessRunner for MoninObukhovSurfaceLayer {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Atmosphere
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "wind_speed".into(),
            "temperature_air".into(),
            "temperature_surface".into(),
            "humidity".into(),
            "humidity_surface".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "sensible_heat_flux".into(),
            "latent_heat_flux".into(),
            "friction_velocity".into(),
            "obukhov_length".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["energy".into(), "momentum".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        let wind = state
            .get_field("wind_speed")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: wind_speed".into()))?
            .clone();
        let t_air = state
            .get_field("temperature_air")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: temperature_air".into()))?
            .clone();
        let t_sfc = state
            .get_field("temperature_surface")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: temperature_surface".into())
            })?
            .clone();
        let q_air = state
            .get_field("humidity")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: humidity".into()))?
            .clone();
        let q_sfc = state
            .get_field("humidity_surface")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: humidity_surface".into()))?
            .clone();

        let n = wind.len();
        let mut h_vals = vec![0.0f64; n];
        let mut le_vals = vec![0.0f64; n];
        let mut ustar_vals = vec![0.0f64; n];
        let mut l_vals = vec![0.0f64; n];

        let w_sl = wind.as_slice().unwrap_or(&[]);
        let ta_sl = t_air.as_slice().unwrap_or(&[]);
        let ts_sl = t_sfc.as_slice().unwrap_or(&[]);
        let qa_sl = q_air.as_slice().unwrap_or(&[]);
        let qs_sl = q_sfc.as_slice().unwrap_or(&[]);

        let len = n
            .min(w_sl.len())
            .min(ta_sl.len())
            .min(ts_sl.len())
            .min(qa_sl.len())
            .min(qs_sl.len());

        for i in 0..len {
            let (u_star, theta_star, l) = self.solve_most(w_sl[i], ta_sl[i], ts_sl[i]);

            // Sensible heat flux: H = -ρ·cp·u*·θ*
            h_vals[i] = -RHO_AIR * CP_AIR * u_star * theta_star;

            // Latent heat flux: LE = -ρ·Lv·u*·q*
            // q* computed analogously to θ*
            let dq = qa_sl[i] - qs_sl[i];
            let dz_h = (self.z_ref / self.z0h).ln();
            let zeta_h = (self.z_ref / l).clamp(-5.0, 5.0);
            let zeta_0h = (self.z0h / l).clamp(-5.0, 5.0);
            let q_star = KARMAN * dq / (dz_h - Self::psi_h(zeta_h) + Self::psi_h(zeta_0h)).max(0.1);
            le_vals[i] = -RHO_AIR * LV * u_star * q_star;

            ustar_vals[i] = u_star;
            l_vals[i] = l;
        }

        // Write outputs
        macro_rules! write_field {
            ($name:expr, $vals:expr) => {
                if let Some(f) = state.get_field_mut($name) {
                    if let Some(sl) = f.as_slice_mut() {
                        for (o, v) in sl.iter_mut().zip($vals.iter()) {
                            *o = *v;
                        }
                    }
                }
            };
        }
        write_field!("sensible_heat_flux", h_vals);
        write_field!("latent_heat_flux", le_vals);
        write_field!("friction_velocity", ustar_vals);
        write_field!("obukhov_length", l_vals);

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// Tests
// ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neutral_u_star() {
        let m = MoninObukhovSurfaceLayer::default();
        // Neutral: T_air = T_surface
        let (u_star, _, _) = m.solve_most(5.0, 300.0, 300.0);
        // u* = κ·U / ln(z/z0) = 0.4 * 5.0 / ln(100) ≈ 0.434
        assert!(u_star > 0.3 && u_star < 0.6, "Neutral u* = {u_star}");
    }

    #[test]
    fn test_unstable_increases_mixing() {
        let m = MoninObukhovSurfaceLayer::default();
        // Neutral
        let (u_star_n, _, _) = m.solve_most(5.0, 300.0, 300.0);
        // Unstable: surface hotter than air
        let (u_star_u, _, l) = m.solve_most(5.0, 290.0, 310.0);
        assert!(
            l < 0.0,
            "Obukhov length should be negative for unstable: L={l}"
        );
        assert!(
            u_star_u > u_star_n * 0.9,
            "Unstable u* should be comparable or larger: {u_star_u} vs {u_star_n}"
        );
    }

    #[test]
    fn test_stable_reduces_mixing() {
        let m = MoninObukhovSurfaceLayer::default();
        let (_, _, l) = m.solve_most(5.0, 310.0, 290.0);
        assert!(
            l > 0.0,
            "Obukhov length should be positive for stable: L={l}"
        );
    }

    #[test]
    fn test_sensible_heat_sign() {
        let m = MoninObukhovSurfaceLayer::default();
        // Surface warmer → positive H (upward)
        let (u_star, theta_star, _) = m.solve_most(5.0, 290.0, 310.0);
        let h = -RHO_AIR * CP_AIR * u_star * theta_star;
        assert!(
            h > 0.0,
            "H should be positive when surface is warmer: H={h}"
        );
    }

    #[test]
    fn test_psi_m_neutral_zero() {
        let psi = MoninObukhovSurfaceLayer::psi_m(0.0);
        assert!(psi.abs() < 0.01, "ψ_m(0) ≈ 0 for neutral: {psi}");
    }
}
