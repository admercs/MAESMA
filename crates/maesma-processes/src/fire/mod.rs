//! Fire process family — from stochastic regimes to coupled fire–atmosphere.
//!
//! Fidelity ladder:
//! - R0: Stochastic fire regime (Li et al. 2012)
//! - R1: Rothermel surface spread + CFS FBP crown fire
//! - R2: Physics-based (Balbi model, level-set propagation, convective plume)
//! - R3: Fully coupled fire–atmosphere (WRF-SFIRE style)
//!
//! The R0 model computes burned-area fraction using a probabilistic approach:
//!   P(fire) = P_ign · f(fuel) · f(FDI)
//!   burned_area_fraction = P(fire) · A_mean / A_cell
//!
//! where:
//!   P_ign = base ignition probability (human + lightning),
//!   f(fuel) = fuel availability factor (0–1),
//!   f(FDI) = fire danger index scaling (0–1),
//!   A_mean = mean fire size [ha] from fire return interval.
//!
//! References:
//!   Li, F. et al. (2012). "A process-based fire parameterization of
//!   intermediate complexity in a Dynamic Global Vegetation Model."
//!   Biogeosciences 9, 2761–2780.

pub mod cfsfbp;
pub mod rothermel;

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

/// R0: Stochastic fire regime — burned-area empirical model.
///
/// **State fields** (per grid cell):
///   - `fuel_load`                — fuel mass [kg m⁻²]
///   - `weather_fire_danger_index` — fire weather index [0–1]
///
/// **Outputs**:
///   - `burned_area_fraction`     — fraction of cell burned this step [0–1]
pub struct StochasticFireRegime {
    /// Annual probability of ignition per grid cell.
    pub ignition_probability: f64,
    /// Mean fire return interval [years].
    pub fire_return_interval: f64,
    /// Critical fuel threshold [kg m⁻²] — below this, fire cannot sustain.
    pub fuel_threshold: f64,
    /// Mean fire size [fraction of cell area per fire event].
    pub mean_fire_size: f64,
}

impl Default for StochasticFireRegime {
    fn default() -> Self {
        Self {
            ignition_probability: 0.001,
            fire_return_interval: 100.0,
            fuel_threshold: 0.2,
            mean_fire_size: 0.05, // 5% of cell per event
        }
    }
}

impl StochasticFireRegime {
    /// Fuel availability factor: sigmoid ramp from 0 to 1 above threshold.
    fn fuel_factor(&self, fuel_load: f64) -> f64 {
        if fuel_load <= 0.0 {
            return 0.0;
        }
        let x = (fuel_load - self.fuel_threshold) / self.fuel_threshold.max(0.01);
        1.0 / (1.0 + (-5.0 * x).exp()) // sigmoid centred at threshold
    }

    /// Fire probability per timestep, given dt in seconds.
    fn fire_probability(&self, fuel_load: f64, fdi: f64, dt: f64) -> f64 {
        let annual_rate = self.ignition_probability;
        // Convert annual rate to per-step rate
        let rate_per_s = annual_rate / (365.25 * 86400.0);
        let p = rate_per_s * dt;
        let ff = self.fuel_factor(fuel_load);
        let fd = fdi.clamp(0.0, 1.0);
        (p * ff * fd).min(1.0)
    }
}

impl ProcessRunner for StochasticFireRegime {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Fire
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec!["fuel_load".into(), "weather_fire_danger_index".into()]
    }

    fn outputs(&self) -> Vec<String> {
        vec!["burned_area_fraction".into()]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["carbon".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let fuel = state
            .get_field("fuel_load")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: fuel_load".into()))?
            .clone();
        let fdi = state
            .get_field("weather_fire_danger_index")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: weather_fire_danger_index".into())
            })?
            .clone();

        let n = fuel.len();
        let fl = fuel.as_slice().unwrap_or(&[]);
        let fd = fdi.as_slice().unwrap_or(&[]);
        let len = n.min(fl.len()).min(fd.len());

        let mut burned = vec![0.0f64; len];

        for i in 0..len {
            let p_fire = self.fire_probability(fl[i], fd[i], dt);
            // Expected burned fraction = probability × mean fire size
            burned[i] = (p_fire * self.mean_fire_size).min(1.0);
        }

        if let Some(f) = state.get_field_mut("burned_area_fraction") {
            if let Some(sl) = f.as_slice_mut() {
                for (o, v) in sl.iter_mut().zip(burned.iter()) {
                    *o = *v;
                }
            }
        }

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
    fn test_fuel_factor_below_threshold() {
        let m = StochasticFireRegime::default();
        let ff = m.fuel_factor(0.0);
        assert_eq!(ff, 0.0, "No fuel → factor should be 0");
    }

    #[test]
    fn test_fuel_factor_above_threshold() {
        let m = StochasticFireRegime::default();
        let ff = m.fuel_factor(1.0);
        assert!(ff > 0.5, "Ample fuel → factor near 1: {ff}");
    }

    #[test]
    fn test_fire_probability_increases_with_fdi() {
        let m = StochasticFireRegime::default();
        let dt = 86400.0;
        let p_low = m.fire_probability(1.0, 0.1, dt);
        let p_high = m.fire_probability(1.0, 0.9, dt);
        assert!(
            p_high > p_low,
            "Higher FDI → higher fire probability: {p_high} vs {p_low}"
        );
    }

    #[test]
    fn test_no_fuel_no_fire() {
        let m = StochasticFireRegime::default();
        let p = m.fire_probability(0.0, 1.0, 86400.0);
        assert_eq!(p, 0.0, "No fuel → no fire probability");
    }
}
