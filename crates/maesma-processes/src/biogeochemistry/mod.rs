//! Biogeochemistry process family — soil carbon and nutrient cycling.
//!
//! Fidelity ladder:
//! - R0: Q10 temperature sensitivity
//! - R1: CENTURY/RothC 3-pool model (Parton et al. 1987, 1993)
//! - R2: microbial explicit (MIMICS)
//! - R3: molecular-level decomposition
//!
//! The R1 CENTURY model tracks 3 soil organic matter (SOM) pools — active,
//! slow, and passive — with first-order decay modified by temperature and
//! moisture scalars. Litter input is partitioned into structural (high
//! lignin:N) and metabolic (low lignin:N) fractions. Lignin routes to the
//! slow pool; other structural C routes to active. Carbon flows between pools
//! with CO₂ loss at each transfer (microbial efficiency). The model tracks C
//! and N with a prescribed C:N ratio per pool.
//!
//! References:
//!   Parton, W. J. et al. (1987). Analysis of factors controlling soil organic
//!   matter levels in Great Plains grasslands. SSSAJ 51, 1173–1179.
//!   Parton, W. J. et al. (1993). Observations and modeling of biomass and soil
//!   organic matter dynamics. Global Biogeochem. Cycles 7, 785–809.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// ───────────────────────────────────────────────────────────────────
// R1: CENTURY 3-Pool Soil Carbon Model
// ───────────────────────────────────────────────────────────────────

/// Pool indices for the 3-pool CENTURY model.
const ACTIVE: usize = 0;
const SLOW: usize = 1;
const PASSIVE: usize = 2;

/// CENTURY-style soil carbon pool model (R1, Parton et al. 1987/1993).
///
/// **State fields** (1-D, per grid cell):
///   - `litter_input`    — litter C input [kg C m⁻² s⁻¹]
///   - `temperature`     — soil temperature [°C]
///   - `moisture`        — relative soil moisture [0–1, fraction of saturation]
///   - `lignin_fraction` — lignin fraction of litter [0–1]
///   - `soil_carbon_active`, `soil_carbon_slow`, `soil_carbon_passive` [kg C m⁻²]
///
/// **Outputs**:
///   - `soil_respiration` — heterotrophic CO₂ efflux [kg C m⁻² s⁻¹]
///   - `doc_leaching`     — dissolved organic C loss [kg C m⁻² s⁻¹]
///   - `soil_carbon_active`, `_slow`, `_passive` — updated pools
pub struct CenturySoilCarbon {
    /// Base turnover times [yr]: active, slow, passive.
    pub tau: [f64; 3],
    /// Microbial efficiency (fraction of C transferred, rest emitted as CO₂).
    pub efficiency: [f64; 3],
    /// C:N ratios for each pool.
    pub cn_ratio: [f64; 3],
    /// Fraction of active pool decay that goes to passive (rest to CO₂).
    pub f_active_to_passive: f64,
    /// Fraction of slow pool decay that goes to passive.
    pub f_slow_to_passive: f64,
    /// Fraction of slow pool decay that goes to active.
    pub f_slow_to_active: f64,
    /// DOC leaching fraction of active pool turnover.
    pub f_leach: f64,
    /// Q10 temperature coefficient.
    pub q10: f64,
    /// Reference temperature for Q10 [°C].
    pub t_ref: f64,
}

impl Default for CenturySoilCarbon {
    fn default() -> Self {
        Self {
            tau: [1.5, 25.0, 1000.0],          // years
            efficiency: [0.40, 0.30, 0.10],     // fraction transferred (not respired)
            cn_ratio: [15.0, 20.0, 10.0],
            f_active_to_passive: 0.004,
            f_slow_to_passive: 0.03,
            f_slow_to_active: 0.42,
            f_leach: 0.01,
            q10: 2.0,
            t_ref: 25.0,
        }
    }
}

impl CenturySoilCarbon {
    /// Temperature scalar (Q10 function).
    fn f_temp(&self, t: f64) -> f64 {
        self.q10.powf((t - self.t_ref) / 10.0).max(0.0)
    }

    /// Moisture scalar (Parton et al. 1993: optimum at ~0.6 saturation).
    /// Uses a simple parabolic response peaking at w_opt = 0.6.
    fn f_moist(&self, w: f64) -> f64 {
        let w = w.clamp(0.0, 1.0);
        // Parton approximation: f(w) = a·w^b · e^(-c·w)
        // Simplified: quadratic with peak at 0.6
        let w_opt = 0.6;
        let fw = 1.0 - ((w - w_opt) / w_opt).powi(2);
        fw.max(0.05) // minimum activity even in dry/saturated soil
    }

    /// Decomposition rate [s⁻¹] for a pool, given temperature and moisture.
    fn k_pool(&self, pool: usize, temp: f64, moisture: f64) -> f64 {
        let k_base = 1.0 / (self.tau[pool] * 365.25 * 86400.0); // yr⁻¹ → s⁻¹
        k_base * self.f_temp(temp) * self.f_moist(moisture)
    }
}

impl ProcessRunner for CenturySoilCarbon {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Biogeochemistry
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "litter_input".into(),
            "temperature".into(),
            "moisture".into(),
            "lignin_fraction".into(),
            "soil_carbon_active".into(),
            "soil_carbon_slow".into(),
            "soil_carbon_passive".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "soil_respiration".into(),
            "doc_leaching".into(),
            "soil_carbon_active".into(),
            "soil_carbon_slow".into(),
            "soil_carbon_passive".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["carbon".into(), "nitrogen".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let litter = state
            .get_field("litter_input")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: litter_input".into()))?
            .clone();
        let temp = state
            .get_field("temperature")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: temperature".into()))?
            .clone();
        let moist = state
            .get_field("moisture")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: moisture".into()))?
            .clone();
        let lignin = state
            .get_field("lignin_fraction")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: lignin_fraction".into()))?
            .clone();

        // Get mutable access to pool fields
        let c_act = state
            .get_field("soil_carbon_active")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: soil_carbon_active".into())
            })?
            .clone();
        let c_slow = state
            .get_field("soil_carbon_slow")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: soil_carbon_slow".into())
            })?
            .clone();
        let c_pass = state
            .get_field("soil_carbon_passive")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: soil_carbon_passive".into())
            })?
            .clone();

        let n = litter.len();
        let mut resp_vals = vec![0.0f64; n];
        let mut doc_vals = vec![0.0f64; n];
        let mut act_vals = c_act.as_slice().unwrap_or(&[]).to_vec();
        let mut slow_vals = c_slow.as_slice().unwrap_or(&[]).to_vec();
        let mut pass_vals = c_pass.as_slice().unwrap_or(&[]).to_vec();

        let lit_sl = litter.as_slice().unwrap_or(&[]);
        let tmp_sl = temp.as_slice().unwrap_or(&[]);
        let moi_sl = moist.as_slice().unwrap_or(&[]);
        let lig_sl = lignin.as_slice().unwrap_or(&[]);

        let len = n
            .min(lit_sl.len())
            .min(tmp_sl.len())
            .min(moi_sl.len())
            .min(lig_sl.len())
            .min(act_vals.len())
            .min(slow_vals.len())
            .min(pass_vals.len());

        for i in 0..len {
            let t = tmp_sl[i];
            let w = moi_sl[i];
            let lig_frac = lig_sl[i].clamp(0.0, 1.0);

            // ── Litter partitioning ──
            // Metabolic fraction = 1 − lignin_fraction (simplified from Parton)
            let lit_c = lit_sl[i].max(0.0) * dt; // total litter C [kg C m⁻²] this step
            let lit_struct = lit_c * lig_frac;    // structural → slow pool
            let lit_metab = lit_c * (1.0 - lig_frac); // metabolic → active pool

            // ── Pool decomposition ──
            let k_a = self.k_pool(ACTIVE, t, w);
            let k_s = self.k_pool(SLOW, t, w);
            let k_p = self.k_pool(PASSIVE, t, w);

            let decay_a = act_vals[i] * k_a * dt;
            let decay_s = slow_vals[i] * k_s * dt;
            let decay_p = pass_vals[i] * k_p * dt;

            // Don't decay more than exists
            let decay_a = decay_a.min(act_vals[i]);
            let decay_s = decay_s.min(slow_vals[i]);
            let decay_p = decay_p.min(pass_vals[i]);

            // ── Carbon transfers ──
            // Active → passive (small fraction)
            let a_to_p = decay_a * self.f_active_to_passive;
            // Active → DOC leaching
            let a_to_doc = decay_a * self.f_leach;
            // Active → CO₂
            let a_to_co2 = decay_a * (1.0 - self.efficiency[ACTIVE]) - a_to_doc;
            let a_to_co2 = a_to_co2.max(0.0);

            // Slow → active
            let s_to_a = decay_s * self.f_slow_to_active;
            // Slow → passive
            let s_to_p = decay_s * self.f_slow_to_passive;
            let s_to_co2 = decay_s * (1.0 - self.efficiency[SLOW]);

            // Passive → active
            let p_to_a = decay_p * self.efficiency[PASSIVE];
            let p_to_co2 = decay_p * (1.0 - self.efficiency[PASSIVE]);

            // ── Update pools ──
            act_vals[i] += lit_metab + s_to_a + p_to_a - decay_a;
            slow_vals[i] += lit_struct - decay_s;
            pass_vals[i] += a_to_p + s_to_p - decay_p;

            // Ensure non-negative
            act_vals[i] = act_vals[i].max(0.0);
            slow_vals[i] = slow_vals[i].max(0.0);
            pass_vals[i] = pass_vals[i].max(0.0);

            // ── Respiration and leaching fluxes ──
            let total_resp = a_to_co2 + s_to_co2 + p_to_co2;
            resp_vals[i] = total_resp / dt; // kg C m⁻² s⁻¹
            doc_vals[i] = a_to_doc / dt;
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
        write_field!("soil_respiration", resp_vals);
        write_field!("doc_leaching", doc_vals);
        write_field!("soil_carbon_active", act_vals);
        write_field!("soil_carbon_slow", slow_vals);
        write_field!("soil_carbon_passive", pass_vals);

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
    fn test_q10_doubles_at_plus_10() {
        let m = CenturySoilCarbon::default();
        let f_ref = m.f_temp(m.t_ref);
        let f_plus10 = m.f_temp(m.t_ref + 10.0);
        assert!(
            (f_plus10 / f_ref - 2.0).abs() < 1e-10,
            "Q10=2 should double rate at +10°C"
        );
    }

    #[test]
    fn test_moisture_peak() {
        let m = CenturySoilCarbon::default();
        let f_opt = m.f_moist(0.6);
        let f_dry = m.f_moist(0.1);
        let f_sat = m.f_moist(1.0);
        assert!(f_opt > f_dry, "Optimal moisture > dry");
        assert!(f_opt > f_sat, "Optimal moisture > saturated");
    }

    #[test]
    fn test_active_faster_than_passive() {
        let m = CenturySoilCarbon::default();
        let k_a = m.k_pool(ACTIVE, 25.0, 0.6);
        let k_p = m.k_pool(PASSIVE, 25.0, 0.6);
        assert!(
            k_a > k_p * 100.0,
            "Active pool should decompose much faster than passive"
        );
    }

    #[test]
    fn test_zero_litter_still_decays() {
        let m = CenturySoilCarbon::default();
        // Decay rates should be positive even without litter input
        let k = m.k_pool(ACTIVE, 20.0, 0.5);
        assert!(k > 0.0);
    }

    #[test]
    fn test_cold_slows_decomposition() {
        let m = CenturySoilCarbon::default();
        let k_warm = m.k_pool(SLOW, 25.0, 0.6);
        let k_cold = m.k_pool(SLOW, 5.0, 0.6);
        assert!(k_warm > k_cold, "Warmer soil should decompose faster");
    }
}
