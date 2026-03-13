//! Evolution process family — evolutionary trait dynamics and adaptation.
//!
//! Fidelity ladder:
//! - R0: static traits
//! - R1: quantitative genetics (breeder's equation)
//! - R2: explicit genotype tracking
//! - R3: eco-evolutionary dynamics
//!
//! The R1 model implements the **breeder's equation** and the **Lande equation**
//! for continuous trait evolution under selection:
//!
//!   Δz̄ = h² · S                            (discrete-generation breeder's equation)
//!
//! In continuous time with overlapping generations (Lande 1976):
//!   dz̄/dt = G · β                           (Lande equation)
//!
//! where:
//!   z̄ = population mean trait value,
//!   G = additive genetic variance = h² · Vₚ,
//!   β = selection gradient ∂ ln W̄ / ∂z̄,
//!   S = selection differential (phenotypic covariance of trait and fitness).
//!
//! Genetic variance erodes under directional selection but is maintained by
//! mutation-selection balance:
//!   dG/dt = V_m − G² · β² / Vₚ
//!
//! where V_m is the mutational input of genetic variance per generation.
//!
//! References:
//!   Lande, R. (1976). "Natural selection and random genetic drift in
//!   phenotypic evolution." Evolution 30(2), 314–334.
//!   Falconer, D. S. & Mackay, T. F. C. (1996). Introduction to Quantitative
//!   Genetics. 4th ed. Longman.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

/// Quantitative-genetics evolution model (R1, Lande equation).
///
/// **State fields** (per grid cell / population):
///   - `trait_mean`         — population mean trait value [trait units]
///   - `trait_variance`     — phenotypic variance Vₚ [trait units²]
///   - `selection_gradient` — directional selection gradient β [fitness per trait unit]
///
/// **Outputs**:
///   - `trait_mean`          — updated mean trait [trait units]
///   - `trait_variance`      — updated Vₚ [trait units²]
///   - `genetic_variance`    — additive genetic variance G [trait units²]
///   - `response_to_selection` — Δz̄ per step [trait units]
pub struct QuantitativeGeneticsEvolution {
    /// Heritability of the focal trait: h² = G / Vₚ [-]
    pub heritability: f64,
    /// Initial phenotypic variance [trait units²]
    pub phenotypic_variance: f64,
    /// Mutational variance input per generation [trait units²]
    pub mutational_variance: f64,
    /// Generation time [s]
    pub generation_time: f64,
}

impl Default for QuantitativeGeneticsEvolution {
    fn default() -> Self {
        Self {
            heritability: 0.3,
            phenotypic_variance: 1.0,
            mutational_variance: 0.001,
            generation_time: 365.25 * 86400.0, // 1 year
        }
    }
}

impl QuantitativeGeneticsEvolution {
    /// Additive genetic variance G = h² · Vₚ.
    pub fn genetic_variance(&self, h2: f64, vp: f64) -> f64 {
        h2 * vp
    }

    /// Lande response: Δz̄ = G · β · dt.
    pub fn trait_response(&self, g: f64, beta: f64, dt: f64) -> f64 {
        g * beta * dt
    }

    /// Genetic variance dynamics (mutation-selection balance).
    /// dG/dt = V_m / T_gen − G² · β² / Vₚ
    pub fn dg_dt(&self, g: f64, beta: f64, vp: f64) -> f64 {
        let mutation_input = self.mutational_variance / self.generation_time;
        let erosion = if vp > 0.0 {
            g * g * beta * beta / vp
        } else {
            0.0
        };
        mutation_input - erosion
    }
}

impl ProcessRunner for QuantitativeGeneticsEvolution {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Evolution
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "trait_mean".into(),
            "trait_variance".into(),
            "selection_gradient".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "trait_mean".into(),
            "trait_variance".into(),
            "genetic_variance".into(),
            "response_to_selection".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec![]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let mean_field = state
            .get_field("trait_mean")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: trait_mean".into()))?
            .clone();
        let var_field = state
            .get_field("trait_variance")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: trait_variance".into()))?
            .clone();
        let sel_field = state
            .get_field("selection_gradient")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: selection_gradient".into()))?
            .clone();

        let n = mean_field.len();
        let ms = mean_field.as_slice().unwrap_or(&[]);
        let vs = var_field.as_slice().unwrap_or(&[]);
        let ss = sel_field.as_slice().unwrap_or(&[]);
        let len = n.min(ms.len()).min(vs.len()).min(ss.len());

        let mut mean_out = vec![0.0f64; len];
        let mut var_out = vec![0.0f64; len];
        let mut gvar_out = vec![0.0f64; len];
        let mut resp_out = vec![0.0f64; len];

        for i in 0..len {
            let z_bar = ms[i];
            let vp = vs[i].max(1e-12);
            let beta = ss[i];

            // Current genetic variance
            let g = self.genetic_variance(self.heritability, vp);

            // Trait response (Lande equation)
            let delta_z = self.trait_response(g, beta, dt);

            // Genetic variance dynamics
            let dg = self.dg_dt(g, beta, vp) * dt;
            let new_g = (g + dg).max(0.0);

            // Update heritability implicitly (G changes, Vp adjusts)
            let new_vp = if self.heritability > 0.0 {
                new_g / self.heritability
            } else {
                vp
            };

            mean_out[i] = z_bar + delta_z;
            var_out[i] = new_vp.max(1e-12);
            gvar_out[i] = new_g;
            resp_out[i] = delta_z;
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
        write_field!("trait_mean", mean_out);
        write_field!("trait_variance", var_out);
        write_field!("genetic_variance", gvar_out);
        write_field!("response_to_selection", resp_out);

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
    fn test_genetic_variance_from_heritability() {
        let m = QuantitativeGeneticsEvolution::default();
        let g = m.genetic_variance(0.3, 1.0);
        assert!((g - 0.3).abs() < 1e-12, "G = h²·Vₚ = 0.3: {g}");
    }

    #[test]
    fn test_no_selection_no_change() {
        let m = QuantitativeGeneticsEvolution::default();
        let g = m.genetic_variance(0.3, 1.0);
        let delta_z = m.trait_response(g, 0.0, 86400.0);
        assert_eq!(delta_z, 0.0, "No selection → no trait change");
    }

    #[test]
    fn test_positive_selection_increases_mean() {
        let m = QuantitativeGeneticsEvolution::default();
        let g = m.genetic_variance(0.3, 1.0);
        let delta_z = m.trait_response(g, 1.0, 86400.0);
        assert!(delta_z > 0.0, "Positive β → positive Δz̄: {delta_z}");
    }

    #[test]
    fn test_mutation_maintains_variance() {
        let m = QuantitativeGeneticsEvolution::default();
        // Under no selection, dG/dt = V_m / T_gen > 0
        let dg = m.dg_dt(0.0, 0.0, 1.0);
        assert!(dg > 0.0, "Mutation input sustains G: {dg}");
    }

    #[test]
    fn test_strong_selection_erodes_variance() {
        let m = QuantitativeGeneticsEvolution::default();
        let g = 0.5;
        let beta = 10.0; // very strong selection
        let dg = m.dg_dt(g, beta, 1.0);
        assert!(dg < 0.0, "Strong selection should erode G: {dg}");
    }
}
