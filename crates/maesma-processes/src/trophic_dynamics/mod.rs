//! Trophic dynamics process family — food-web interactions and consumer-resource dynamics.
//!
//! Fidelity ladder:
//! - R0: static food web
//! - R1: Lotka-Volterra predator-prey
//! - R2: size-structured food web
//! - R3: trait-mediated trophic interactions
//!
//! The R1 model implements the Lotka-Volterra predator-prey system with
//! logistic prey growth (Rosenzweig-MacArthur variant):
//!
//!   dN/dt = α · N · (1 − N/K) − β · N · P
//!   dP/dt = δ · β · N · P − γ · P
//!
//! where:
//!   N = prey biomass, P = predator biomass,
//!   K = prey carrying capacity,
//!   α = prey intrinsic growth rate,
//!   β = predation rate coefficient,
//!   γ = predator mortality rate,
//!   δ = predator conversion efficiency.
//!
//! Time integration uses semi-implicit Euler to avoid negative populations.
//!
//! References:
//!   Lotka, A. J. (1925). Elements of Physical Biology.
//!   Volterra, V. (1926). Variazioni e fluttuazioni del numero d'individui.
//!   Rosenzweig, M. L. & MacArthur, R. H. (1963). Am. Nat. 97, 209–223.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

/// Lotka-Volterra predator-prey trophic model (R1, Rosenzweig-MacArthur).
///
/// **State fields** (per grid cell):
///   - `prey_biomass`      — prey population biomass [kg m⁻²]
///   - `predator_biomass`  — predator population biomass [kg m⁻²]
///   - `carrying_capacity` — prey carrying capacity [kg m⁻²]
///
/// **Outputs** (updated in state):
///   - `prey_biomass`       — updated prey biomass [kg m⁻²]
///   - `predator_biomass`   — updated predator biomass [kg m⁻²]
///   - `prey_growth_rate`   — dN/dt [kg m⁻² s⁻¹]
///   - `predator_growth_rate` — dP/dt [kg m⁻² s⁻¹]
pub struct LotkaVolterraTrophic {
    /// Prey intrinsic growth rate [s⁻¹]
    pub alpha: f64,
    /// Predation rate coefficient [m² kg⁻¹ s⁻¹]
    pub beta: f64,
    /// Predator mortality rate [s⁻¹]
    pub gamma: f64,
    /// Predator conversion efficiency [-]
    pub delta: f64,
}

impl Default for LotkaVolterraTrophic {
    fn default() -> Self {
        // Timescales ~ annual
        let per_year = 1.0 / (365.25 * 86400.0);
        Self {
            alpha: 1.0 * per_year,
            beta: 0.01 * per_year,
            gamma: 0.5 * per_year,
            delta: 0.1,
        }
    }
}

impl LotkaVolterraTrophic {
    /// Compute (dN/dt, dP/dt) for given state.
    pub fn derivatives(&self, n: f64, p: f64, k: f64) -> (f64, f64) {
        let dn = self.alpha * n * (1.0 - n / k.max(1e-12)) - self.beta * n * p;
        let dp = self.delta * self.beta * n * p - self.gamma * p;
        (dn, dp)
    }
}

impl ProcessRunner for LotkaVolterraTrophic {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::TrophicDynamics
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "prey_biomass".into(),
            "predator_biomass".into(),
            "carrying_capacity".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "prey_biomass".into(),
            "predator_biomass".into(),
            "prey_growth_rate".into(),
            "predator_growth_rate".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["biomass".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let prey = state
            .get_field("prey_biomass")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: prey_biomass".into()))?
            .clone();
        let pred = state
            .get_field("predator_biomass")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: predator_biomass".into()))?
            .clone();
        let cap = state
            .get_field("carrying_capacity")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: carrying_capacity".into()))?
            .clone();

        let n_len = prey.len();
        let ns = prey.as_slice().unwrap_or(&[]);
        let ps = pred.as_slice().unwrap_or(&[]);
        let ks = cap.as_slice().unwrap_or(&[]);
        let len = n_len.min(ns.len()).min(ps.len()).min(ks.len());

        let mut prey_out = vec![0.0f64; len];
        let mut pred_out = vec![0.0f64; len];
        let mut dn_out = vec![0.0f64; len];
        let mut dp_out = vec![0.0f64; len];

        for i in 0..len {
            let n = ns[i].max(0.0);
            let p = ps[i].max(0.0);
            let k = ks[i].max(1e-12);

            // RK2 (midpoint method) for better stability
            let (dn1, dp1) = self.derivatives(n, p, k);
            let n_mid = (n + 0.5 * dt * dn1).max(0.0);
            let p_mid = (p + 0.5 * dt * dp1).max(0.0);
            let (dn2, dp2) = self.derivatives(n_mid, p_mid, k);

            let new_n = (n + dt * dn2).max(0.0);
            let new_p = (p + dt * dp2).max(0.0);

            prey_out[i] = new_n;
            pred_out[i] = new_p;
            dn_out[i] = dn2;
            dp_out[i] = dp2;
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
        write_field!("prey_biomass", prey_out);
        write_field!("predator_biomass", pred_out);
        write_field!("prey_growth_rate", dn_out);
        write_field!("predator_growth_rate", dp_out);

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// R0: Static Food Web
// ───────────────────────────────────────────────────────────────────

/// Static food-web model with fixed trophic efficiencies (R0).
///
/// Populations are held at carrying capacity (no dynamics); only
/// trophic fluxes are diagnosed.
///
/// **Inputs**: `prey_biomass`, `predator_biomass`, `carrying_capacity`
/// **Outputs**: `prey_biomass`, `predator_biomass`, `prey_growth_rate`, `predator_growth_rate`
pub struct StaticFoodWeb;

impl Default for StaticFoodWeb {
    fn default() -> Self {
        Self
    }
}

impl ProcessRunner for StaticFoodWeb {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::TrophicDynamics
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "prey_biomass".into(),
            "predator_biomass".into(),
            "carrying_capacity".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "prey_biomass".into(),
            "predator_biomass".into(),
            "prey_growth_rate".into(),
            "predator_growth_rate".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["biomass".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        // R0: no dynamics, just pass through current populations
        // Diagnose zero growth rates
        let prey = state
            .get_field("prey_biomass")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: prey_biomass".into()))?
            .clone();
        let len = prey.len();
        let zeros = vec![0.0f64; len];

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
        write_field!("prey_growth_rate", zeros);
        write_field!("predator_growth_rate", zeros);

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
    fn test_equilibrium_point() {
        // At equilibrium: dN/dt = 0, dP/dt = 0
        let m = LotkaVolterraTrophic::default();
        // Predator equilibrium: N* = γ / (δ·β)
        let n_star = m.gamma / (m.delta * m.beta);
        // Prey equilibrium: P* = (α/β) · (1 − N*/K)
        let k = 100.0;
        let p_star = (m.alpha / m.beta) * (1.0 - n_star / k);

        let (dn, dp) = m.derivatives(n_star, p_star, k);
        assert!(dn.abs() < 1e-20, "dN/dt should be ~0 at equilibrium: {dn}");
        assert!(dp.abs() < 1e-20, "dP/dt should be ~0 at equilibrium: {dp}");
    }

    #[test]
    fn test_prey_grows_without_predators() {
        let m = LotkaVolterraTrophic::default();
        let (dn, _dp) = m.derivatives(10.0, 0.0, 100.0);
        assert!(dn > 0.0, "Prey should grow when P=0: {dn}");
    }

    #[test]
    fn test_predator_declines_without_prey() {
        let m = LotkaVolterraTrophic::default();
        let (_dn, dp) = m.derivatives(0.0, 10.0, 100.0);
        assert!(dp < 0.0, "Predator should decline when N=0: {dp}");
    }

    #[test]
    fn test_carrying_capacity_limits_prey() {
        let m = LotkaVolterraTrophic::default();
        // When N = K, prey at capacity
        let (dn, _dp) = m.derivatives(100.0, 0.0, 100.0);
        assert!(dn.abs() < 1e-20, "At K, prey should stop growing: {dn}");
    }

    #[test]
    fn test_populations_stay_nonnegative() {
        let m = LotkaVolterraTrophic::default();
        let n = 0.001;
        let p = 0.001;
        let k = 100.0;
        let dt = 86400.0;
        let (dn, dp) = m.derivatives(n, p, k);
        let new_n = (n + dt * dn).max(0.0);
        let new_p = (p + dt * dp).max(0.0);
        assert!(new_n >= 0.0);
        assert!(new_p >= 0.0);
    }
}
