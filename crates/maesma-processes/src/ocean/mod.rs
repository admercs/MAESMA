//! Ocean process family — ocean dynamics and thermodynamics.
//!
//! Fidelity ladder:
//! - R0: slab ocean
//! - R1: mixed-layer model
//! - R2: isopycnal coordinate model
//! - R3: eddy-resolving general circulation
//!
//! The R1 model implements a slab mixed-layer ocean (Manabe & Stouffer 1988,
//! Petoukhov et al. 2000). Thermodynamic equation:
//!   ρ·cₚ·h · dSST/dt = Q_net − Q_deep
//!
//! where:
//!   Q_net = SW_net + LW_net + SH + LH          (surface heat fluxes)
//!   Q_deep = λ · (SST − T_deep)                  (entrainment / diffusion)
//!   h is the mixed-layer depth (seasonally variable via Kraus-Turner)
//!
//! Mixed-layer deepening by wind stirring (TKE input):
//!   w_e = u*³ / (α·g·Δρ·h)                       (entrainment velocity)
//!
//! References:
//!   Manabe, S. & Stouffer, R. J. (1988). J. Climate 1, 841–866.
//!   Niiler, P. P. & Kraus, E. B. (1977). One-dimensional models of the
//!   upper ocean. Modelling and Prediction of the Upper Layers of the Ocean.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// ── Physical constants ──
const RHO_SW: f64 = 1025.0; // seawater density [kg m⁻³]
const CP_SW: f64 = 3985.0; // seawater specific heat [J kg⁻¹ K⁻¹]
const ALPHA_T: f64 = 2.0e-4; // thermal expansion coefficient [K⁻¹]
const GRAV: f64 = 9.81; // gravity [m s⁻²]

/// Mixed-layer ocean model (R1, Kraus-Turner slab).
///
/// **State fields** (per grid cell):
///   - `solar_radiation`  — net surface SW absorbed by ocean [W m⁻²]
///   - `longwave_net`     — net longwave at surface [W m⁻²]
///   - `sensible_heat`    — sensible heat flux (into ocean positive) [W m⁻²]
///   - `latent_heat`      — latent heat flux (into ocean positive) [W m⁻²]
///   - `wind_stress`      — surface wind stress magnitude [N m⁻²]
///   - `sst`              — sea surface temperature [°C]
///   - `mixed_layer_depth`— current MLD [m]
///
/// **Outputs**:
///   - `sst`              — updated SST [°C]
///   - `sst_tendency`     — dSST/dt [K s⁻¹]
///   - `mixed_layer_depth`— updated MLD [m]
///   - `ocean_heat_uptake`— Q_net [W m⁻²]
pub struct MixedLayerOcean {
    /// Minimum mixed-layer depth [m].
    pub mld_min: f64,
    /// Maximum mixed-layer depth [m].
    pub mld_max: f64,
    /// Deep-ocean relaxation temperature [°C].
    pub t_deep: f64,
    /// Relaxation timescale to deep ocean [s].
    pub tau_deep: f64,
    /// Entrainment efficiency parameter [-].
    pub entrainment_eff: f64,
}

impl Default for MixedLayerOcean {
    fn default() -> Self {
        Self {
            mld_min: 10.0,
            mld_max: 500.0,
            t_deep: 4.0,                // deep ocean ~4°C
            tau_deep: 365.25 * 86400.0, // 1-year relaxation
            entrainment_eff: 0.5,
        }
    }
}

impl MixedLayerOcean {
    /// Friction velocity from wind stress: u* = sqrt(τ / ρ).
    fn u_star(tau: f64) -> f64 {
        (tau.abs() / RHO_SW).sqrt()
    }

    /// Entrainment velocity from TKE balance (simplified Niiler-Kraus).
    fn w_entrainment(&self, u_star: f64, delta_t: f64, h: f64) -> f64 {
        if delta_t <= 0.0 || h <= 0.0 {
            return 0.0;
        }
        let buoyancy_jump = ALPHA_T * GRAV * delta_t; // [m s⁻²]
        self.entrainment_eff * u_star.powi(3) / (buoyancy_jump * h)
    }
}

impl ProcessRunner for MixedLayerOcean {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Ocean
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "solar_radiation".into(),
            "longwave_net".into(),
            "sensible_heat".into(),
            "latent_heat".into(),
            "wind_stress".into(),
            "sst".into(),
            "mixed_layer_depth".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "sst".into(),
            "sst_tendency".into(),
            "mixed_layer_depth".into(),
            "ocean_heat_uptake".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["energy".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let sw = state
            .get_field("solar_radiation")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: solar_radiation".into()))?
            .clone();
        let lw = state
            .get_field("longwave_net")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: longwave_net".into()))?
            .clone();
        let sh = state
            .get_field("sensible_heat")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: sensible_heat".into()))?
            .clone();
        let lh = state
            .get_field("latent_heat")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: latent_heat".into()))?
            .clone();
        let tau = state
            .get_field("wind_stress")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: wind_stress".into()))?
            .clone();
        let sst_field = state
            .get_field("sst")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: sst".into()))?
            .clone();
        let mld_field = state
            .get_field("mixed_layer_depth")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: mixed_layer_depth".into()))?
            .clone();

        let n = sw.len();
        let sw_s = sw.as_slice().unwrap_or(&[]);
        let lw_s = lw.as_slice().unwrap_or(&[]);
        let sh_s = sh.as_slice().unwrap_or(&[]);
        let lh_s = lh.as_slice().unwrap_or(&[]);
        let tau_s = tau.as_slice().unwrap_or(&[]);
        let sst_s = sst_field.as_slice().unwrap_or(&[]);
        let mld_s = mld_field.as_slice().unwrap_or(&[]);
        let len = n
            .min(sw_s.len())
            .min(lw_s.len())
            .min(sh_s.len())
            .min(lh_s.len())
            .min(tau_s.len())
            .min(sst_s.len())
            .min(mld_s.len());

        let mut sst_out = vec![0.0f64; len];
        let mut tend_out = vec![0.0f64; len];
        let mut mld_out = vec![0.0f64; len];
        let mut qnet_out = vec![0.0f64; len];

        for i in 0..len {
            let h = mld_s[i].clamp(self.mld_min, self.mld_max);
            let sst = sst_s[i];

            // Net surface heat flux (positive into ocean)
            let q_net = sw_s[i] + lw_s[i] + sh_s[i] + lh_s[i];

            // Deep-ocean relaxation
            let q_deep = RHO_SW * CP_SW * h * (sst - self.t_deep) / self.tau_deep;

            // Heat capacity of mixed layer
            let heat_cap = RHO_SW * CP_SW * h;

            // SST tendency
            let dsst_dt = (q_net - q_deep) / heat_cap;
            let new_sst = sst + dsst_dt * dt;

            // MLD adjustment via entrainment
            let us = Self::u_star(tau_s[i]);
            let delta_t = (sst - self.t_deep).max(0.0);
            let w_e = self.w_entrainment(us, delta_t, h);
            let new_h = (h + w_e * dt).clamp(self.mld_min, self.mld_max);

            sst_out[i] = new_sst;
            tend_out[i] = dsst_dt;
            mld_out[i] = new_h;
            qnet_out[i] = q_net;
        }

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
        write_field!("sst", sst_out);
        write_field!("sst_tendency", tend_out);
        write_field!("mixed_layer_depth", mld_out);
        write_field!("ocean_heat_uptake", qnet_out);

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// R0: Slab Ocean (prescribed Q-flux)
// ───────────────────────────────────────────────────────────────────

/// Slab (mixed-layer) ocean model with prescribed Q-flux (R0).
///
/// Evolves SST as:
///   dT/dt = (Q_net + Q_flux) / (ρ_w · c_w · h)
///
/// where Q_net is the net surface heat flux, Q_flux is a prescribed ocean
/// heat transport convergence (Q-flux), and h is the mixed-layer depth.
///
/// **Inputs**: `sst`, `net_heat_flux`, `q_flux` (prescribed)
/// **Outputs**: `sst` (updated), `ocean_heat_content`
pub struct SlabOcean {
    /// Mixed-layer depth [m]
    pub depth: f64,
}

impl Default for SlabOcean {
    fn default() -> Self {
        Self { depth: 50.0 }
    }
}

impl ProcessRunner for SlabOcean {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Ocean
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec!["sst".into(), "net_heat_flux".into()]
    }

    fn outputs(&self) -> Vec<String> {
        vec!["sst".into(), "ocean_heat_content".into()]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["energy".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let sst = state
            .get_field("sst")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: sst".into()))?
            .clone();
        let flux = state
            .get_field("net_heat_flux")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: net_heat_flux".into()))?
            .clone();

        let n = sst.len();
        let sst_sl = sst.as_slice().unwrap_or(&[]);
        let flux_sl = flux.as_slice().unwrap_or(&[]);
        let len = n.min(sst_sl.len()).min(flux_sl.len());

        // Optional prescribed Q-flux
        let qflux_field = state.get_field("q_flux");

        let rho_cw_h = RHO_SW * CP_SW * self.depth; // J m⁻² K⁻¹

        let mut sst_out = vec![0.0f64; len];
        let mut ohc_out = vec![0.0f64; len];

        for i in 0..len {
            let t = sst_sl[i];
            let q_net = flux_sl[i];
            let q_flux = qflux_field
                .and_then(|f| f.as_slice())
                .and_then(|s| s.get(i).copied())
                .unwrap_or(0.0);

            let dt_sst = (q_net + q_flux) / rho_cw_h * dt;
            sst_out[i] = t + dt_sst;
            ohc_out[i] = rho_cw_h * sst_out[i]; // absolute OHC proxy [J m⁻²]
        }

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
        write_field!("sst", sst_out);
        write_field!("ocean_heat_content", ohc_out);

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
    fn test_u_star_from_stress() {
        let us = MixedLayerOcean::u_star(0.1); // moderate wind stress
        assert!(us > 0.0, "u* should be positive");
        assert!(us < 0.1, "u* should be order ~cm/s: {us}");
    }

    #[test]
    fn test_no_entrainment_when_cold_on_top() {
        let m = MixedLayerOcean::default();
        // SST ≤ T_deep → delta_t ≤ 0 → no buoyancy barrier → no entrainment calc
        let w = m.w_entrainment(0.01, 0.0, 50.0);
        assert_eq!(w, 0.0, "No entrainment when no buoyancy barrier");
    }

    #[test]
    fn test_warming_increases_sst() {
        // +200 W/m² net heat flux into a 50m mixed layer
        let h = 50.0;
        let q_net = 200.0; // W m⁻²
        let heat_cap = RHO_SW * CP_SW * h;
        let dsst_dt = q_net / heat_cap;
        assert!(
            dsst_dt > 0.0,
            "Positive heat flux should warm SST: {dsst_dt}"
        );
        // After 1 day
        let delta_sst = dsst_dt * 86400.0;
        assert!(
            delta_sst < 1.0,
            "SST change over a day should be modest: {delta_sst}"
        );
    }

    #[test]
    fn test_deep_relaxation_cools_warm_sst() {
        let m = MixedLayerOcean::default();
        let sst = 20.0;
        let h = 50.0;
        let q_deep = RHO_SW * CP_SW * h * (sst - m.t_deep) / m.tau_deep;
        assert!(q_deep > 0.0, "Warm SST should lose heat to deep: {q_deep}");
    }
}
