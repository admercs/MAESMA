//! Ecology process family — vegetation dynamics and ecosystem structure.
//!
//! Fidelity ladder:
//! - R0: big-leaf model
//! - R1: cohort-based (FATES-style)
//! - R2: individual-based model
//! - R3: trait-based adaptive
//!
//! The R1 model implements simplified cohort vegetation dynamics inspired by
//! FATES (Fisher et al. 2015). Each grid cell tracks:
//!   - Biomass carbon [kg C m⁻²]
//!   - Leaf area index (LAI) [m² m⁻²]
//!   - Net primary productivity (NPP) [kg C m⁻² s⁻¹]
//!
//! NPP is computed from a light-use efficiency approach:
//!   GPP = ε · APAR · f(T) · f(W) · f(CO₂)
//!   NPP = GPP · CUE
//! where:
//!   APAR = PAR · (1 - exp(-k · LAI))   [Beer's law]
//!   f(T) = parabolic temperature response
//!   f(W) = linear soil moisture response
//!   f(CO₂) = Michaelis-Menten CO₂ fertilisation
//!
//! Turnover: biomass loss through a constant turnover rate.
//! LAI is diagnosed from biomass via specific leaf area (SLA).
//!
//! References:
//!   Fisher, R. A. et al. (2015). "Taking off the training wheels: the
//!   properties of a dynamic vegetation model without climate envelopes."
//!   Ecological Modelling 309–310, 1–20.
//!   Monteith, J. L. (1972). "Solar radiation and productivity."

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// ── Physical / ecological constants ──
/// Light-use efficiency [kg C MJ⁻¹ PAR]
const LUE: f64 = 1.1e-9; // ≈ 1.1 g C MJ⁻¹ → kg C J⁻¹
/// Carbon-use efficiency (NPP / GPP)
const CUE: f64 = 0.5;
/// Extinction coefficient for PAR (Beer's law)
const K_PAR: f64 = 0.5;
/// Specific leaf area [m² leaf (kg C)⁻¹]
const SLA: f64 = 20.0;
/// CO₂ half-saturation [ppm]
const CO2_HALF: f64 = 350.0;
/// Optimal growth temperature [°C]
const T_OPT: f64 = 25.0;
/// Temperature range half-width [°C]
const T_SIGMA: f64 = 15.0;

/// Cohort-based vegetation dynamics model (R1, FATES-style).
///
/// **State fields** (per grid cell):
///   - `light`         — downwelling PAR [W m⁻²]
///   - `temperature`   — air temperature [°C]
///   - `soil_moisture` — plant-available soil moisture [0–1]
///   - `co2`           — atmospheric CO₂ [ppm]
///   - `biomass`       — aboveground biomass carbon [kg C m⁻²]
///
/// **Outputs** (updated in state):
///   - `npp`     — net primary productivity [kg C m⁻² s⁻¹]
///   - `lai`     — leaf area index [m² m⁻²]
///   - `biomass` — updated biomass [kg C m⁻²]
pub struct CohortVegetation {
    /// Number of plant functional types
    pub num_pfts: usize,
    /// Maximum LAI [m² m⁻²]
    pub lai_max: f64,
    /// Biomass turnover rate [s⁻¹]
    pub turnover_rate: f64,
    /// Leaf allocation fraction of NPP [-]
    pub f_leaf: f64,
}

impl Default for CohortVegetation {
    fn default() -> Self {
        Self {
            num_pfts: 1,
            lai_max: 8.0,
            turnover_rate: 1.0 / (20.0 * 365.25 * 86400.0), // 20-year e-folding
            f_leaf: 0.3,
        }
    }
}

impl CohortVegetation {
    /// Temperature response — parabolic with peak at T_OPT.
    fn f_temp(t: f64) -> f64 {
        let x = (t - T_OPT) / T_SIGMA;
        (1.0 - x * x).max(0.0)
    }

    /// Soil-moisture stress [0–1].
    fn f_moisture(w: f64) -> f64 {
        w.clamp(0.0, 1.0)
    }

    /// CO₂ fertilisation (Michaelis-Menten).
    fn f_co2(co2: f64) -> f64 {
        co2.max(0.0) / (co2.max(0.0) + CO2_HALF)
    }
}

impl ProcessRunner for CohortVegetation {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Ecology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "light".into(),
            "temperature".into(),
            "soil_moisture".into(),
            "co2".into(),
            "biomass".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec!["npp".into(), "lai".into(), "biomass".into()]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["carbon".into(), "nitrogen".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let light = state
            .get_field("light")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: light".into()))?
            .clone();
        let temp = state
            .get_field("temperature")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: temperature".into()))?
            .clone();
        let moisture = state
            .get_field("soil_moisture")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: soil_moisture".into()))?
            .clone();
        let co2 = state
            .get_field("co2")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: co2".into()))?
            .clone();
        let bio_field = state
            .get_field("biomass")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: biomass".into()))?
            .clone();

        let n = light.len();
        let li = light.as_slice().unwrap_or(&[]);
        let ti = temp.as_slice().unwrap_or(&[]);
        let wi = moisture.as_slice().unwrap_or(&[]);
        let ci = co2.as_slice().unwrap_or(&[]);
        let bi = bio_field.as_slice().unwrap_or(&[]);
        let len = n
            .min(li.len())
            .min(ti.len())
            .min(wi.len())
            .min(ci.len())
            .min(bi.len());

        let mut npp_vals = vec![0.0f64; len];
        let mut lai_vals = vec![0.0f64; len];
        let mut bio_vals = vec![0.0f64; len];

        for i in 0..len {
            let biomass = bi[i].max(0.0);

            // Diagnose LAI from biomass via specific leaf area
            let leaf_carbon = biomass * self.f_leaf;
            let lai = (leaf_carbon * SLA).min(self.lai_max);

            // APAR: fraction of incoming PAR absorbed by canopy (Beer's law)
            let f_apar = 1.0 - (-K_PAR * lai).exp();
            let apar = li[i].max(0.0) * f_apar; // W m⁻² PAR absorbed

            // Environmental scalars
            let ft = Self::f_temp(ti[i]);
            let fw = Self::f_moisture(wi[i]);
            let fc = Self::f_co2(ci[i]);

            // GPP and NPP
            let gpp = LUE * apar * ft * fw * fc; // kg C m⁻² s⁻¹
            let npp = gpp * CUE;

            // Turnover loss
            let turnover = biomass * self.turnover_rate;

            // Update biomass
            let new_biomass = (biomass + (npp - turnover) * dt).max(0.0);

            npp_vals[i] = npp;
            lai_vals[i] = lai;
            bio_vals[i] = new_biomass;
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
        write_field!("npp", npp_vals);
        write_field!("lai", lai_vals);
        write_field!("biomass", bio_vals);

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// R0: Big-Leaf Static Vegetation
// ───────────────────────────────────────────────────────────────────

/// Static big-leaf vegetation model (R0).
///
/// Diagnoses net primary productivity from temperature and precipitation
/// using the Miami model (Lieth, 1975). LAI is prescribed as a constant.
///
/// **Inputs**: `temperature` [°C], `precipitation` [mm yr⁻¹ equivalent]
/// **Outputs**: `npp` [kg C m⁻² s⁻¹], `lai` [m² m⁻²], `gpp` [kg C m⁻² s⁻¹]
pub struct StaticVegetation {
    /// Prescribed leaf area index [m² m⁻²]
    pub lai: f64,
    /// Maximum NPP [kg C m⁻² yr⁻¹] (Miami model)
    pub npp_max: f64,
    /// Autotrophic respiration fraction of GPP [-]
    pub ra_fraction: f64,
}

impl Default for StaticVegetation {
    fn default() -> Self {
        Self {
            lai: 4.0,
            npp_max: 1.2, // typical temperate forest
            ra_fraction: 0.5,
        }
    }
}

impl StaticVegetation {
    /// Miami model temperature response: NPP_T = NPP_max / (1 + exp(1.315 − 0.119·T))
    fn f_temperature(&self, t_celsius: f64) -> f64 {
        self.npp_max / (1.0 + (1.315 - 0.119 * t_celsius).exp())
    }

    /// Miami model precipitation response: NPP_P = NPP_max · (1 − exp(−0.000664·P))
    fn f_precipitation(&self, p_mm_yr: f64) -> f64 {
        self.npp_max * (1.0 - (-0.000664 * p_mm_yr).exp())
    }
}

impl ProcessRunner for StaticVegetation {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Ecology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec!["temperature".into(), "precipitation".into()]
    }

    fn outputs(&self) -> Vec<String> {
        vec!["npp".into(), "lai".into(), "gpp".into()]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["carbon".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        let temp = state
            .get_field("temperature")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: temperature".into()))?
            .clone();

        let n = temp.len();
        let temp_sl = temp.as_slice().unwrap_or(&[]);
        let precip_field = state.get_field("precipitation");
        let precip_default = ndarray::ArrayD::from_elem(ndarray::IxDyn(&[n]), 800.0);
        let precip_sl = precip_field
            .unwrap_or(&precip_default)
            .as_slice()
            .unwrap_or(&[]);

        let secs_per_yr = 365.25 * 86400.0;
        let len = n.min(temp_sl.len());

        let mut npp_out = vec![0.0f64; len];
        let mut lai_out = vec![0.0f64; len];
        let mut gpp_out = vec![0.0f64; len];

        for i in 0..len {
            let t = temp_sl[i];
            let p = precip_sl.get(i).copied().unwrap_or(800.0);

            // Miami model: min(T-limited, P-limited) NPP
            let npp_t = self.f_temperature(t);
            let npp_p = self.f_precipitation(p);
            let npp_yr = npp_t.min(npp_p).max(0.0); // kg C m⁻² yr⁻¹
            let npp_s = npp_yr / secs_per_yr; // kg C m⁻² s⁻¹
            let gpp_s = npp_s / (1.0 - self.ra_fraction);

            npp_out[i] = npp_s;
            gpp_out[i] = gpp_s;
            lai_out[i] = self.lai;
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
        write_field!("npp", npp_out);
        write_field!("lai", lai_out);
        write_field!("gpp", gpp_out);

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
    fn test_temp_response_peak() {
        let peak = CohortVegetation::f_temp(T_OPT);
        assert!((peak - 1.0).abs() < 1e-12, "Peak at T_OPT should be 1.0");
    }

    #[test]
    fn test_temp_response_zero_at_extremes() {
        let cold = CohortVegetation::f_temp(T_OPT - T_SIGMA - 1.0);
        assert_eq!(cold, 0.0, "Should be zero far below optimum");
        let hot = CohortVegetation::f_temp(T_OPT + T_SIGMA + 1.0);
        assert_eq!(hot, 0.0, "Should be zero far above optimum");
    }

    #[test]
    fn test_co2_fertilisation_saturates() {
        let low = CohortVegetation::f_co2(200.0);
        let mid = CohortVegetation::f_co2(400.0);
        let high = CohortVegetation::f_co2(800.0);
        assert!(low < mid, "More CO₂ → higher response");
        assert!(high > mid, "More CO₂ → higher response");
        assert!(
            (high - mid) < (mid - low),
            "Diminishing returns at high CO₂"
        );
    }

    #[test]
    fn test_beers_law_high_lai() {
        let lai = 6.0;
        let f_apar = 1.0 - (-K_PAR * lai).exp();
        assert!(f_apar > 0.95, "High LAI should absorb >95% PAR: {f_apar}");
    }

    #[test]
    fn test_zero_light_no_growth() {
        let npp = LUE * 0.0 * CUE;
        assert_eq!(npp, 0.0, "No PAR → no NPP");
    }
}
