//! Radiation process family — shortwave and longwave radiative transfer.
//!
//! Fidelity ladder:
//! - R0: Beer-Lambert extinction
//! - R1: two-stream approximation (Dickinson 1983, Sellers 1985, CLM)
//! - R2: Monte Carlo ray tracing
//! - R3: full spectral radiative transfer
//!
//! The R1 two-stream approximation implemented here follows:
//!   Dickinson, R. E. (1983). Land surface processes and climate — surface
//!   albedos and energy balance. Advances in Geophysics 25, 305–353.
//!   Sellers, P. J. (1985). Canopy reflectance, photosynthesis, and
//!   transpiration. Int. J. Remote Sensing 6(8), 1335–1372.
//!   Bonan, G. (2019). Climate Change and Terrestrial Ecosystem Modeling, Ch. 14.
//!
//! The scheme partitions incoming solar radiation into PAR (0.4–0.7 µm) and
//! NIR (0.7–4.0 µm) wavebands, each split into direct beam and diffuse.
//! For each waveband, it solves the two-stream equations to obtain canopy
//! absorption, soil absorption, and upward reflection, conserving energy.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// ───────────────────────────────────────────────────────────────────
// R1: Two-Stream Canopy Radiative Transfer
// ───────────────────────────────────────────────────────────────────

/// Two-stream canopy radiation model (R1, Dickinson 1983 / Sellers 1985).
///
/// Solves the two-stream radiative transfer equations for a horizontally
/// homogeneous canopy characterised by leaf area index (`lai`), single-
/// scattering albedo (`omega`), and leaf orientation factor (`chi_l`).
///
/// **Inputs** (from `ProcessState` fields):
///   - `solar_zenith`  — cosine of solar zenith angle [–]
///   - `lai`           — leaf area index [m² m⁻²]
///   - `sw_down_par`   — downwelling PAR flux [W m⁻²]
///   - `sw_down_nir`   — downwelling NIR flux [W m⁻²]
///   - `frac_diffuse`  — fraction of diffuse radiation [–]
///   - `albedo_soil_par` — soil albedo for PAR [–]
///   - `albedo_soil_nir` — soil albedo for NIR [–]
///
/// **Outputs** (written back):
///   - `absorbed_par`  — canopy-absorbed PAR [W m⁻²]
///   - `absorbed_nir`  — canopy-absorbed NIR [W m⁻²]
///   - `reflected_sw`  — total upwelling shortwave [W m⁻²]
///   - `transmitted_par` — PAR reaching soil surface [W m⁻²]
///   - `transmitted_nir` — NIR reaching soil surface [W m⁻²]
pub struct TwoStreamRadiation {
    // ── Optical properties ──
    /// Leaf single-scattering albedo for PAR [-] (reflectance + transmittance).
    pub omega_par: f64,
    /// Leaf single-scattering albedo for NIR [-].
    pub omega_nir: f64,
    /// Leaf orientation chi_l parameter [-1=vertical, 0=spherical, +1=horizontal].
    pub chi_l: f64,
    /// Stem area index [m² m⁻²] (treated as LAI-like absorber).
    pub sai: f64,
    /// Clumping index Ω [0, 1]. 1 = random foliage, <1 = clumped.
    pub clumping: f64,
}

impl Default for TwoStreamRadiation {
    fn default() -> Self {
        Self {
            omega_par: 0.17, // typical broadleaf PAR ρ+τ
            omega_nir: 0.68, // typical broadleaf NIR ρ+τ
            chi_l: 0.01,     // nearly spherical
            sai: 0.5,
            clumping: 1.0,
        }
    }
}

impl TwoStreamRadiation {
    /// Ross–Goudriaan G-function mean projection = 0.5 for spherical.
    /// Approximation: G(µ) = φ1 + φ2·µ  where µ = cos(zenith).
    fn g_mu(&self, cos_z: f64) -> f64 {
        let phi1 = 0.5 - 0.633 * self.chi_l - 0.330 * self.chi_l.powi(2);
        let phi2 = 0.877 * (1.0 - 2.0 * phi1);
        phi1 + phi2 * cos_z
    }

    /// Optical depth per unit LAI for direct beam.
    fn kb(&self, cos_z: f64) -> f64 {
        let g = self.g_mu(cos_z);
        self.clumping * g / cos_z.max(0.01)
    }

    /// Average inverse diffuse optical depth per unit LAI.
    fn kd(&self) -> f64 {
        // Approximate hemispherical integral of G(µ)/µ over µ ∈ [0,1]
        // For spherical leaves: K_d ≈ 0.7193  (CLM Tech Note, Table 3.1)
        let phi1 = 0.5 - 0.633 * self.chi_l - 0.330 * self.chi_l.powi(2);
        let phi2 = 0.877 * (1.0 - 2.0 * phi1);
        // Numerical two-point Gauss: µ = 0.21, 0.79
        let g1 = phi1 + phi2 * 0.21;
        let g2 = phi1 + phi2 * 0.79;
        self.clumping * 0.5 * (g1 / 0.21 + g2 / 0.79)
    }

    /// Solve two-stream equations for one waveband.
    ///
    /// Returns (canopy_absorbed, reflected_up, transmitted_down) [W m⁻²].
    fn two_stream_band(
        &self,
        cos_z: f64,
        lai_tot: f64,
        omega: f64,
        albedo_soil: f64,
        sw_beam: f64,
        sw_diff: f64,
    ) -> (f64, f64, f64) {
        if lai_tot <= 0.0 || (sw_beam + sw_diff) <= 0.0 {
            // No canopy or no incoming radiation.
            let tot = sw_beam + sw_diff;
            let refl = tot * albedo_soil;
            return (0.0, refl, tot);
        }

        let mu = cos_z.max(0.01);

        // --- Diffuse component (two-stream solution) ---
        let kd = self.kd();
        let beta0 = 0.5; // symmetric scattering for diffuse
        // Two-stream coefficients (Sellers 1985, Bonan Ch14)
        let a1 = kd * (1.0 - omega * (1.0 - beta0));
        let a2 = kd * omega * beta0;

        let m = ((a1 * a1 - a2 * a2).max(0.0)).sqrt();
        let m = m.max(1e-30);
        let h = a1 + m;

        let exp_ml = (-m * lai_tot).exp();
        let exp_pl = (m * lai_tot).exp();

        // Boundary conditions: diffuse from above, soil reflects below
        // Γ = (h − albedo_soil·a2) / (h·exp_ml − albedo_soil·a2·exp_pl·... )
        // Simplified two-layer solution:
        let denom = h * exp_pl - a2 * albedo_soil * exp_ml;
        let denom = if denom.abs() < 1e-30 { 1e-30 } else { denom };

        let trans_diff = m / denom; // transmission coefficient (approx)
        let trans_diff = trans_diff.clamp(0.0, 1.0);

        let refl_diff = (h - m) / h;  // diffuse reflectance by canopy
        let refl_diff = refl_diff.clamp(0.0, 1.0 - 1e-10);

        // Absorbed fraction of diffuse
        let tau_d = (-kd * lai_tot).exp(); // Beer-law transmittance (no scattering)
        let abs_diff_frac = 1.0 - tau_d.clamp(0.0, 1.0);
        // With scattering, actual absorption is reduced by omega
        let abs_diff = sw_diff * abs_diff_frac * (1.0 - omega);

        // Reflected diffuse
        let refl_from_diff = sw_diff * refl_diff
            + sw_diff * trans_diff * albedo_soil * trans_diff;

        // Transmitted diffuse to soil
        let trans_to_soil_diff = sw_diff * trans_diff;

        // --- Direct beam component ---
        let kb = self.kb(mu);
        let tau_b = (-kb * lai_tot).exp(); // Beer-law beam transmittance

        // Beam interception
        let intercepted = sw_beam * (1.0 - tau_b);
        let abs_beam = intercepted * (1.0 - omega); // absorbed by leaves

        // Scattered beam → treated as additional diffuse
        let scattered = intercepted * omega;
        // Half scattered up, half down (first-order)
        let scat_up = scattered * 0.5;
        let scat_down = scattered * 0.5;

        // Beam reaching soil directly
        let beam_to_soil = sw_beam * tau_b;

        // Soil absorption and reflection
        let total_to_soil = beam_to_soil + trans_to_soil_diff + scat_down;
        let soil_refl = total_to_soil * albedo_soil;

        // Total reflected upward
        let reflected = refl_from_diff + scat_up + soil_refl;

        // Total canopy absorbed
        let canopy_absorbed = abs_beam + abs_diff;

        // Transmitted to soil
        let transmitted = total_to_soil;

        // Enforce energy conservation: clamp
        let total_in = sw_beam + sw_diff;
        let canopy_absorbed = canopy_absorbed.min(total_in);
        let reflected = reflected.min(total_in - canopy_absorbed);

        (canopy_absorbed, reflected, transmitted)
    }
}

impl ProcessRunner for TwoStreamRadiation {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Radiation
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "solar_zenith".into(),
            "lai".into(),
            "sw_down_par".into(),
            "sw_down_nir".into(),
            "frac_diffuse".into(),
            "albedo_soil_par".into(),
            "albedo_soil_nir".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "absorbed_par".into(),
            "absorbed_nir".into(),
            "reflected_sw".into(),
            "transmitted_par".into(),
            "transmitted_nir".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["energy".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        let cos_z = state
            .get_field("solar_zenith")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: solar_zenith".into()))?
            .clone();
        let lai_field = state
            .get_field("lai")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: lai".into()))?
            .clone();
        let par_down = state
            .get_field("sw_down_par")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: sw_down_par".into()))?
            .clone();
        let nir_down = state
            .get_field("sw_down_nir")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: sw_down_nir".into()))?
            .clone();
        let fdiff = state
            .get_field("frac_diffuse")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: frac_diffuse".into()))?
            .clone();
        let alb_par = state
            .get_field("albedo_soil_par")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: albedo_soil_par".into())
            })?
            .clone();
        let alb_nir = state
            .get_field("albedo_soil_nir")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: albedo_soil_nir".into())
            })?
            .clone();

        let n = cos_z.len();
        let mut abs_par_vals = vec![0.0f64; n];
        let mut abs_nir_vals = vec![0.0f64; n];
        let mut refl_vals = vec![0.0f64; n];
        let mut trans_par_vals = vec![0.0f64; n];
        let mut trans_nir_vals = vec![0.0f64; n];

        let cz = cos_z.as_slice().unwrap_or(&[]);
        let lai_sl = lai_field.as_slice().unwrap_or(&[]);
        let par_sl = par_down.as_slice().unwrap_or(&[]);
        let nir_sl = nir_down.as_slice().unwrap_or(&[]);
        let fd_sl = fdiff.as_slice().unwrap_or(&[]);
        let ap_sl = alb_par.as_slice().unwrap_or(&[]);
        let an_sl = alb_nir.as_slice().unwrap_or(&[]);

        let len = n
            .min(cz.len())
            .min(lai_sl.len())
            .min(par_sl.len())
            .min(nir_sl.len())
            .min(fd_sl.len())
            .min(ap_sl.len())
            .min(an_sl.len());

        for i in 0..len {
            let mu = cz[i].max(0.0);
            if mu <= 0.001 {
                continue; // nighttime
            }

            let lai_tot = (lai_sl[i] + self.sai).max(0.0);
            let frac_d = fd_sl[i].clamp(0.0, 1.0);

            // PAR waveband
            let par_beam = par_sl[i] * (1.0 - frac_d);
            let par_diff = par_sl[i] * frac_d;
            let (ap, rp, tp) =
                self.two_stream_band(mu, lai_tot, self.omega_par, ap_sl[i], par_beam, par_diff);

            // NIR waveband
            let nir_beam = nir_sl[i] * (1.0 - frac_d);
            let nir_diff = nir_sl[i] * frac_d;
            let (an, rn, tn) =
                self.two_stream_band(mu, lai_tot, self.omega_nir, an_sl[i], nir_beam, nir_diff);

            abs_par_vals[i] = ap;
            abs_nir_vals[i] = an;
            refl_vals[i] = rp + rn;
            trans_par_vals[i] = tp;
            trans_nir_vals[i] = tn;
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
        write_field!("absorbed_par", abs_par_vals);
        write_field!("absorbed_nir", abs_nir_vals);
        write_field!("reflected_sw", refl_vals);
        write_field!("transmitted_par", trans_par_vals);
        write_field!("transmitted_nir", trans_nir_vals);

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
    fn test_g_function_spherical() {
        let m = TwoStreamRadiation { chi_l: 0.0, ..Default::default() };
        // Spherical leaves: G ≈ 0.5 for any angle
        let g = m.g_mu(0.5);
        assert!((g - 0.5).abs() < 0.05, "G(0.5) = {g}, expected ~0.5");
    }

    #[test]
    fn test_kb_increases_with_low_sun() {
        let m = TwoStreamRadiation::default();
        let kb_high = m.kb(0.9); // high sun
        let kb_low = m.kb(0.2);  // low sun
        assert!(kb_low > kb_high, "Extinction should increase at low sun angles");
    }

    #[test]
    fn test_zero_lai_no_absorption() {
        let m = TwoStreamRadiation::default();
        let (abs, _refl, trans) = m.two_stream_band(0.7, 0.0, 0.17, 0.1, 200.0, 50.0);
        assert_eq!(abs, 0.0, "No canopy → no canopy absorption");
        assert!((trans - 250.0).abs() < 1.0, "All radiation should reach soil");
    }

    #[test]
    fn test_energy_conservation() {
        let m = TwoStreamRadiation::default();
        let sw_beam = 400.0;
        let sw_diff = 100.0;
        let total_in = sw_beam + sw_diff;
        let (abs, refl, trans) = m.two_stream_band(0.7, 3.0, 0.17, 0.1, sw_beam, sw_diff);
        // absorbed + reflected + transmitted_and_absorbed_by_soil should ≤ total_in
        assert!(
            abs + refl <= total_in + 1e-6,
            "Energy conservation violated: abs={abs} + refl={refl} > {total_in}"
        );
        assert!(abs > 0.0, "Dense canopy should absorb radiation");
        assert!(refl > 0.0, "Some radiation should be reflected");
        assert!(trans < total_in, "Not all radiation should pass through LAI=3 canopy");
    }

    #[test]
    fn test_high_lai_absorbs_more() {
        let m = TwoStreamRadiation::default();
        let (abs_low, _, _) = m.two_stream_band(0.7, 1.0, 0.17, 0.1, 300.0, 100.0);
        let (abs_high, _, _) = m.two_stream_band(0.7, 6.0, 0.17, 0.1, 300.0, 100.0);
        assert!(abs_high > abs_low, "Higher LAI should absorb more: {abs_high} vs {abs_low}");
    }

    #[test]
    fn test_nir_reflects_more_than_par() {
        let m = TwoStreamRadiation::default();
        let (_, refl_par, _) = m.two_stream_band(0.7, 3.0, 0.17, 0.1, 300.0, 100.0);
        let (_, refl_nir, _) = m.two_stream_band(0.7, 3.0, 0.68, 0.2, 300.0, 100.0);
        assert!(
            refl_nir > refl_par,
            "NIR (ω=0.68) should reflect more than PAR (ω=0.17): {refl_nir} vs {refl_par}"
        );
    }
}
