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

        if let Some(f) = state.get_field_mut("burned_area_fraction")
            && let Some(sl) = f.as_slice_mut()
        {
            for (o, v) in sl.iter_mut().zip(burned.iter()) {
                *o = *v;
            }
        }

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// Fuel Moisture Model
// ───────────────────────────────────────────────────────────────────

/// Fuel-moisture drying model (Nelson 2000 / Van Wagner 1987).
///
/// Implements the exponential drying/wetting response of dead fine fuels:
///   dM/dt = (M_eq − M) / τ_resp
///
/// where M_eq is the equilibrium moisture content (from temperature and
/// relative humidity via the Anderson 1990 tables) and τ_resp is the
/// response time constant (1-hr, 10-hr, etc.).
pub struct FuelMoistureModel {
    /// Response time constant [s]
    pub response_time: f64,
}

impl Default for FuelMoistureModel {
    fn default() -> Self {
        Self {
            response_time: 3600.0, // 1-hour fuels
        }
    }
}

impl FuelMoistureModel {
    /// Equilibrium moisture content from temperature [°C] and relative humidity [0-1].
    /// Anderson (1990) approximation.
    pub fn equilibrium_moisture(temp_c: f64, rh: f64) -> f64 {
        let rh_pct = (rh * 100.0).clamp(0.0, 100.0);
        let emc_pct = if rh_pct < 10.0 {
            0.03229 + 0.281073 * rh_pct - temp_c * 0.000578
        } else if rh_pct < 50.0 {
            2.22749 + 0.160107 * rh_pct - 0.01478 * temp_c
        } else {
            21.0606 + 0.005565 * rh_pct.powi(2) - 0.00035 * rh_pct * temp_c - 0.483199 * rh_pct
        };
        emc_pct / 100.0
    }

    /// Advance fuel moisture towards equilibrium by dt seconds.
    pub fn step_moisture(&self, current_m: f64, m_eq: f64, dt: f64) -> f64 {
        let factor = (-dt / self.response_time).exp();
        m_eq + (current_m - m_eq) * factor
    }
}

// ───────────────────────────────────────────────────────────────────
// Fire Disturbance Output
// ───────────────────────────────────────────────────────────────────

/// Diagnosed fire disturbance effects on vegetation and soil.
#[derive(Debug, Clone, Default)]
pub struct FireDisturbanceOutput {
    /// Burn severity index [0-1] per cell
    pub severity: Vec<f64>,
    /// Carbon emitted [kg C m⁻² s⁻¹]
    pub carbon_emissions: Vec<f64>,
    /// Fuel consumed [kg m⁻²]
    pub fuel_consumed: Vec<f64>,
}

impl FireDisturbanceOutput {
    /// Diagnose disturbance effects from burned area fraction and fuel load.
    pub fn diagnose(burned_frac: &[f64], fuel_load: &[f64]) -> Self {
        let n = burned_frac.len().min(fuel_load.len());
        let mut severity = vec![0.0f64; n];
        let mut emissions = vec![0.0f64; n];
        let mut consumed = vec![0.0f64; n];

        for i in 0..n {
            let bf = burned_frac[i].clamp(0.0, 1.0);
            let fl = fuel_load[i].max(0.0);

            // Severity proportional to burned fraction
            severity[i] = bf;
            // Combustion completeness ~ 0.5 for surface fires
            let completeness = 0.5;
            consumed[i] = bf * fl * completeness;
            // Carbon ≈ 45% of dry biomass
            emissions[i] = consumed[i] * 0.45;
        }

        Self {
            severity,
            carbon_emissions: emissions,
            fuel_consumed: consumed,
        }
    }
}

// ───────────────────────────────────────────────────────────────────
// Tests
// ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_moisture_drying() {
        let fm = FuelMoistureModel::default();
        let m_eq = 0.05;
        let current = 0.30;
        let new_m = fm.step_moisture(current, m_eq, 7200.0); // 2 hours
        assert!(new_m < current, "Fuel should dry: {new_m} < {current}");
        assert!(new_m > m_eq, "Should not reach equilibrium in 2h: {new_m}");
    }

    #[test]
    fn test_fuel_moisture_wetting() {
        let fm = FuelMoistureModel::default();
        let m_eq = 0.25;
        let current = 0.05;
        let new_m = fm.step_moisture(current, m_eq, 7200.0);
        assert!(new_m > current, "Fuel should wet: {new_m} > {current}");
    }

    #[test]
    fn test_equilibrium_moisture_ranges() {
        // Hot dry: low EMC
        let emc_dry = FuelMoistureModel::equilibrium_moisture(35.0, 0.1);
        assert!(emc_dry < 0.10, "Hot+dry: EMC should be low: {emc_dry}");
        // Cool humid: high EMC
        let emc_wet = FuelMoistureModel::equilibrium_moisture(10.0, 0.9);
        assert!(
            emc_wet > emc_dry,
            "Cool+humid > hot+dry: {emc_wet} vs {emc_dry}"
        );
    }

    #[test]
    fn test_disturbance_output_scales() {
        let bf = vec![0.0, 0.5, 1.0];
        let fl = vec![2.0, 2.0, 2.0];
        let out = FireDisturbanceOutput::diagnose(&bf, &fl);
        assert_eq!(out.severity[0], 0.0);
        assert!((out.severity[1] - 0.5).abs() < 1e-10);
        assert!((out.severity[2] - 1.0).abs() < 1e-10);
        assert!(out.carbon_emissions[2] > out.carbon_emissions[1]);
    }

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
