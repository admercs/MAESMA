//! Human systems process family — land use, management, and socioeconomic drivers.
//!
//! Fidelity ladder:
//! - R0: prescribed land-use trajectories
//! - R1: rule-based transition model (Markov chain)
//! - R2: agent-based land-use model
//! - R3: coupled socio-ecological system
//!
//! The R1 model implements a **Markov chain land-use transition model** where
//! the area fraction of each land-use class evolves according to a transition
//! probability matrix modulated by socio-economic drivers:
//!
//!   f(t+1) = T(pop, econ, policy) · f(t)
//!
//! where:
//!   f ∈ ℝⁿ is the vector of land-use class fractions (sum = 1),
//!   T ∈ ℝⁿˣⁿ is a column-stochastic transition matrix.
//!
//! The base transition matrix is scaled by:
//!   - Population density → amplifies conversion to urban/cropland
//!   - Economic driver → amplifies intensification
//!   - Policy → can restrict or promote certain transitions
//!
//! Default classes (n=4): Forest, Cropland, Grassland, Urban.
//!
//! Harvest rate and fertiliser are diagnosed from cropland fraction and
//! economic driver.
//!
//! References:
//!   Verburg, P. H. et al. (2002). "Modeling the spatial dynamics of regional
//!   land use." Env. Management 30(3), 391–405.
//!   Hurtt, G. C. et al. (2011). "Harmonization of land-use scenarios."
//!   Climatic Change 109, 117–161.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// Default 4-class system indices
const FOREST: usize = 0;
const CROPLAND: usize = 1;
#[allow(dead_code)]
const GRASSLAND: usize = 2;
const URBAN: usize = 3;
const N_CLASSES: usize = 4;

/// Rule-based land-use change model (R1, Markov chain).
///
/// **State fields** (per grid cell):
///   - `population_density`      — people per km² [km⁻²]
///   - `economic_driver`         — GDP proxy [0–1 normalised]
///   - `policy`                  — conservation policy strength [0–1]
///   - `land_use_fractions`      — area fractions for each class (len = num_classes)
///
/// **Outputs**:
///   - `land_use_fractions`      — updated area fractions (sum ≈ 1)
///   - `harvest_rate`            — timber/crop harvest [kg m⁻² yr⁻¹]
///   - `fertilizer_application`  — N application rate [kg N m⁻² yr⁻¹]
pub struct LandUseChange {
    /// Number of land-use classes.
    pub num_classes: usize,
    /// Base transition probability matrix (row-major, n×n).
    /// T[i*n + j] = P(transition from class j to class i).
    pub transition_matrix: Vec<f64>,
    /// Population sensitivity: strength of pop-driven conversion.
    pub pop_sensitivity: f64,
    /// Base harvest rate when cropland fraction = 1 [kg m⁻² yr⁻¹].
    pub base_harvest: f64,
    /// Base fertiliser rate [kg N m⁻² yr⁻¹].
    pub base_fertilizer: f64,
}

impl Default for LandUseChange {
    fn default() -> Self {
        // Default 4×4 base transition matrix (close to identity)
        // Rows: destination. Columns: source.
        #[rustfmt::skip]
        let t = vec![
            // From: Forest  Crop    Grass   Urban
            /* To Forest   */ 0.990,  0.002,  0.005,  0.000,
            /* To Crop     */ 0.005,  0.985,  0.010,  0.000,
            /* To Grass    */ 0.004,  0.010,  0.984,  0.000,
            /* To Urban    */ 0.001,  0.003,  0.001,  1.000,
        ];
        Self {
            num_classes: N_CLASSES,
            transition_matrix: t,
            pop_sensitivity: 0.001,
            base_harvest: 0.5,
            base_fertilizer: 0.01,
        }
    }
}

impl LandUseChange {
    /// Apply transition matrix to fraction vector, ensuring fractions sum to 1.
    fn apply_transition(&self, fractions: &[f64], pop: f64, econ: f64, policy: f64) -> Vec<f64> {
        let n = self.num_classes;
        let mut new_frac = vec![0.0f64; n];

        for i in 0..n {
            for j in 0..n {
                let mut tij = self.transition_matrix[i * n + j];

                // Modulate: high population pushes towards urban/cropland
                if i == URBAN && j != URBAN {
                    tij *= 1.0 + self.pop_sensitivity * pop;
                }
                if i == CROPLAND && j == FOREST {
                    tij *= 1.0 + econ * 0.5;
                }

                // Policy protects forest (reduce deforestation)
                if j == FOREST && i != FOREST {
                    tij *= (1.0 - policy * 0.9).max(0.0);
                }

                new_frac[i] += tij * fractions[j];
            }
        }

        // Normalise to ensure sum = 1
        let sum: f64 = new_frac.iter().sum();
        if sum > 0.0 {
            for v in &mut new_frac {
                *v /= sum;
            }
        }
        new_frac
    }
}

impl ProcessRunner for LandUseChange {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::HumanSystems
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "population_density".into(),
            "economic_driver".into(),
            "policy".into(),
            "land_use_fractions".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "land_use_fractions".into(),
            "harvest_rate".into(),
            "fertilizer_application".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["area".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        let pop = state
            .get_field("population_density")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: population_density".into()))?
            .clone();
        let econ = state
            .get_field("economic_driver")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: economic_driver".into()))?
            .clone();
        let pol = state
            .get_field("policy")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: policy".into()))?
            .clone();
        let frac_field = state
            .get_field("land_use_fractions")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: land_use_fractions".into()))?
            .clone();

        // For the simple case: each grid cell has N_CLASSES fractions stored
        // contiguously. We assume the first dimension indexes cells, second is class.
        let n = self.num_classes;
        let frac_data = frac_field.as_slice().unwrap_or(&[]);
        let pop_data = pop.as_slice().unwrap_or(&[]);
        let econ_data = econ.as_slice().unwrap_or(&[]);
        let pol_data = pol.as_slice().unwrap_or(&[]);

        // Number of cells
        let n_cells = if n > 0 { frac_data.len() / n } else { 0 };
        let n_cells = n_cells
            .min(pop_data.len())
            .min(econ_data.len())
            .min(pol_data.len());

        let mut new_fracs = vec![0.0f64; n_cells * n];
        let mut harvest = vec![0.0f64; n_cells];
        let mut fert = vec![0.0f64; n_cells];

        for c in 0..n_cells {
            let start = c * n;
            let end = start + n;
            let cell_frac = &frac_data[start..end];

            let new_f = self.apply_transition(cell_frac, pop_data[c], econ_data[c], pol_data[c]);

            new_fracs[start..end].copy_from_slice(&new_f);

            // Diagnosed harvest and fertiliser
            let crop_frac = if n > CROPLAND { new_f[CROPLAND] } else { 0.0 };
            harvest[c] = self.base_harvest * crop_frac * (1.0 + econ_data[c]);
            fert[c] = self.base_fertilizer * crop_frac * (1.0 + econ_data[c] * 0.5);
        }

        // Write outputs
        if let Some(f) = state.get_field_mut("land_use_fractions") {
            if let Some(sl) = f.as_slice_mut() {
                for (o, v) in sl.iter_mut().zip(new_fracs.iter()) {
                    *o = *v;
                }
            }
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
        write_field!("harvest_rate", harvest);
        write_field!("fertilizer_application", fert);

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
    fn test_fractions_sum_to_one() {
        let m = LandUseChange::default();
        let f = vec![0.5, 0.2, 0.2, 0.1];
        let new_f = m.apply_transition(&f, 100.0, 0.5, 0.0);
        let sum: f64 = new_f.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10, "Fractions must sum to 1: {sum}");
    }

    #[test]
    fn test_high_population_increases_urban() {
        let m = LandUseChange::default();
        let f = vec![0.4, 0.3, 0.2, 0.1];
        let low_pop = m.apply_transition(&f, 10.0, 0.5, 0.0);
        let high_pop = m.apply_transition(&f, 1000.0, 0.5, 0.0);
        assert!(
            high_pop[URBAN] > low_pop[URBAN],
            "High population should increase urban: {} vs {}",
            high_pop[URBAN],
            low_pop[URBAN]
        );
    }

    #[test]
    fn test_policy_protects_forest() {
        let m = LandUseChange::default();
        let f = vec![0.5, 0.2, 0.2, 0.1];
        let no_policy = m.apply_transition(&f, 500.0, 0.5, 0.0);
        let strong_policy = m.apply_transition(&f, 500.0, 0.5, 1.0);
        assert!(
            strong_policy[FOREST] > no_policy[FOREST],
            "Policy should protect forest: {} vs {}",
            strong_policy[FOREST],
            no_policy[FOREST]
        );
    }

    #[test]
    fn test_identity_preserves_fractions() {
        // With no external drivers and base matrix close to identity,
        // fractions should change only slightly
        let m = LandUseChange::default();
        let f = vec![0.25, 0.25, 0.25, 0.25];
        let new_f = m.apply_transition(&f, 0.0, 0.0, 0.0);
        for i in 0..N_CLASSES {
            assert!(
                (new_f[i] - f[i]).abs() < 0.05,
                "Class {i} should stay near 0.25: {}",
                new_f[i]
            );
        }
    }
}
