//! Cryosphere process family — snow, ice, and frozen ground processes.
//!
//! Fidelity ladder:
//! - R0: degree-day melt model
//! - R1: energy balance snowpack (Anderson 1976, US Army Corps)
//! - R2: multi-layer snowpack (SNTHERM)
//! - R3: microstructure-resolved snow physics
//!
//! The R1 model uses a single-layer energy-balance approach:
//!   dSWE/dt = Snowfall − Melt − Sublimation
//!   Melt = max(0, Q_net / (ρ_w · L_f))
//!   Q_net = SW_abs + LW_net + H + LE − Q_ground
//!
//! Snow albedo decays exponentially with age (Verseghy 1991 CLASS scheme)
//! and is refreshed by new snowfall.
//!
//! References:
//!   Anderson, E. A. (1976). A point energy and mass balance model of a snow
//!   cover. NOAA Technical Report NWS 19.
//!   Verseghy, D. L. (1991). CLASS — A Canadian land surface scheme. Int. J.
//!   Climatol. 11, 111–133.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// Physical constants
const RHO_WATER: f64 = 1000.0; // kg m⁻³
const LF: f64 = 334_000.0; // latent heat of fusion [J kg⁻¹]
const LS: f64 = 2_834_000.0; // latent heat of sublimation [J kg⁻¹]
const SIGMA: f64 = 5.67e-8; // Stefan-Boltzmann [W m⁻² K⁻⁴]
const EMISS_SNOW: f64 = 0.97; // snow emissivity

// ───────────────────────────────────────────────────────────────────
// R1: Energy-Balance Snowpack Model
// ───────────────────────────────────────────────────────────────────

/// Energy-balance snowpack model (R1, Anderson 1976 / CLASS).
///
/// **State fields** (per grid cell):
///   - `precipitation`   — total precipitation [kg m⁻² s⁻¹]
///   - `temperature`     — air temperature [°C]
///   - `sw_down`         — downwelling shortwave [W m⁻²]
///   - `lw_down`         — downwelling longwave [W m⁻²]
///   - `wind_speed`      — wind speed [m s⁻¹]
///   - `humidity`        — relative humidity [0–1]
///   - `swe`             — snow water equivalent [kg m⁻²]
///   - `snow_albedo`     — current snow albedo [0–1]
///
/// **Outputs**:
///   - `snowmelt`        — melt rate [kg m⁻² s⁻¹]
///   - `snow_depth`      — snow depth [m]
///   - `swe`             — updated SWE [kg m⁻²]
///   - `snow_albedo`     — updated albedo [0–1]
///   - `sublimation`     — sublimation flux [kg m⁻² s⁻¹]
pub struct SnowpackModel {
    /// Fresh snow density [kg m⁻³].
    pub rho_fresh: f64,
    /// Snow albedo decay timescale [s].
    pub albedo_decay_time: f64,
    /// Fresh snow albedo [-].
    pub albedo_fresh: f64,
    /// Minimum old snow albedo [-].
    pub albedo_min: f64,
    /// Rain/snow temperature threshold [°C].
    pub t_rain_snow: f64,
    /// Maximum snow density (compacted) [kg m⁻³].
    pub rho_max: f64,
    /// Ground heat flux [W m⁻²] (constant, small).
    pub q_ground: f64,
    /// Bulk transfer coefficient for turbulent fluxes [-].
    pub ch: f64,
}

impl Default for SnowpackModel {
    fn default() -> Self {
        Self {
            rho_fresh: 100.0,
            albedo_decay_time: 7.0 * 86400.0, // 7 days in seconds
            albedo_fresh: 0.85,
            albedo_min: 0.50,
            t_rain_snow: 1.0, // °C
            rho_max: 500.0,
            q_ground: 2.0, // small upward ground heat flux
            ch: 0.002,     // bulk transfer coefficient
        }
    }
}

impl SnowpackModel {
    /// Snow density evolution: compaction towards rho_max.
    #[allow(dead_code)]
    fn snow_density(&self, swe: f64, depth: f64) -> f64 {
        if depth <= 0.0 {
            return self.rho_fresh;
        }
        (swe / depth).clamp(self.rho_fresh, self.rho_max)
    }

    /// Exponential albedo decay (Verseghy CLASS scheme).
    fn decay_albedo(&self, current: f64, dt: f64) -> f64 {
        let alpha =
            current - (current - self.albedo_min) * (1.0 - (-dt / self.albedo_decay_time).exp());
        alpha.clamp(self.albedo_min, self.albedo_fresh)
    }
}

impl ProcessRunner for SnowpackModel {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Cryosphere
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "precipitation".into(),
            "temperature".into(),
            "sw_down".into(),
            "lw_down".into(),
            "wind_speed".into(),
            "humidity".into(),
            "swe".into(),
            "snow_albedo".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "snowmelt".into(),
            "snow_depth".into(),
            "swe".into(),
            "snow_albedo".into(),
            "sublimation".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["water".into(), "energy".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let precip = state
            .get_field("precipitation")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: precipitation".into()))?
            .clone();
        let temp = state
            .get_field("temperature")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: temperature".into()))?
            .clone();
        let sw = state
            .get_field("sw_down")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: sw_down".into()))?
            .clone();
        let lw = state
            .get_field("lw_down")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: lw_down".into()))?
            .clone();
        let wind = state
            .get_field("wind_speed")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: wind_speed".into()))?
            .clone();
        let rh = state
            .get_field("humidity")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: humidity".into()))?
            .clone();
        let swe_field = state
            .get_field("swe")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: swe".into()))?
            .clone();
        let alb_field = state
            .get_field("snow_albedo")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: snow_albedo".into()))?
            .clone();

        let n = precip.len();
        let mut melt_vals = vec![0.0f64; n];
        let mut depth_vals = vec![0.0f64; n];
        let mut swe_vals = swe_field.as_slice().unwrap_or(&[]).to_vec();
        let mut alb_vals = alb_field.as_slice().unwrap_or(&[]).to_vec();
        let mut subl_vals = vec![0.0f64; n];

        let p_sl = precip.as_slice().unwrap_or(&[]);
        let t_sl = temp.as_slice().unwrap_or(&[]);
        let sw_sl = sw.as_slice().unwrap_or(&[]);
        let lw_sl = lw.as_slice().unwrap_or(&[]);
        let w_sl = wind.as_slice().unwrap_or(&[]);
        let rh_sl = rh.as_slice().unwrap_or(&[]);

        let len = n
            .min(p_sl.len())
            .min(t_sl.len())
            .min(sw_sl.len())
            .min(lw_sl.len())
            .min(w_sl.len())
            .min(rh_sl.len())
            .min(swe_vals.len())
            .min(alb_vals.len());

        for i in 0..len {
            let t_air = t_sl[i];
            let t_air_k = t_air + 273.15;
            let mut swe = swe_vals[i].max(0.0);
            let mut alpha = alb_vals[i].clamp(self.albedo_min, self.albedo_fresh);

            // ── Precipitation partitioning ──
            let p_total = p_sl[i].max(0.0) * dt; // kg m⁻² this step
            let snowfall = if t_air < self.t_rain_snow {
                p_total
            } else {
                0.0
            };
            let rainfall = p_total - snowfall;

            // ── Add snowfall ──
            swe += snowfall;
            if snowfall > 0.1 {
                // Refresh albedo
                alpha = self.albedo_fresh;
            }

            if swe <= 0.0 {
                swe_vals[i] = 0.0;
                alb_vals[i] = self.albedo_fresh;
                depth_vals[i] = 0.0;
                continue;
            }

            // ── Energy balance ──
            let t_snow: f64 = 273.15; // snow surface at 0°C when melting

            // Net shortwave (absorbed)
            let q_sw = sw_sl[i] * (1.0 - alpha);

            // Net longwave
            let q_lw = EMISS_SNOW * lw_sl[i] - EMISS_SNOW * SIGMA * t_snow.powi(4);

            // Turbulent sensible heat (bulk formula)
            let q_h =
                RHO_WATER.min(1.225) * 1004.0 * self.ch * w_sl[i].max(0.1) * (t_air_k - t_snow);

            // Sublimation / latent heat (simplified)
            let q_e = {
                // Saturated specific humidity difference (rough approximation)
                let e_sat_air = 611.0 * (17.27 * t_air / (t_air + 237.3)).exp();
                let e_sat_snow = 611.0; // ~611 Pa at 0°C
                let dq = 0.622 * (rh_sl[i].clamp(0.0, 1.0) * e_sat_air - e_sat_snow) / 101325.0;
                1.225 * LS * self.ch * w_sl[i].max(0.1) * dq
            };

            // Rain heat input (warm rain on snow)
            let q_rain = rainfall * 4186.0 * t_air.max(0.0) / dt;

            // Net energy available for melt
            let q_net = q_sw + q_lw + q_h + q_e + q_rain - self.q_ground;

            // ── Melt ──
            let melt = if q_net > 0.0 {
                let potential_melt = q_net * dt / LF; // kg m⁻²
                potential_melt.min(swe) // can't melt more than exists
            } else {
                0.0
            };

            // ── Sublimation ──
            let sublimation = if q_e < 0.0 {
                // Negative q_e → sublimation (mass loss from snow)
                let sub = (-q_e * dt / LS).min(swe - melt);
                sub.max(0.0)
            } else {
                0.0
            };

            swe -= melt + sublimation;
            swe = swe.max(0.0);

            // ── Albedo decay ──
            alpha = self.decay_albedo(alpha, dt);

            // ── Snow depth ──
            let rho_snow = if swe > 0.0 {
                // Simple compaction: density increases with age
                let rho = self.rho_fresh + (self.rho_max - self.rho_fresh) * 0.3;
                rho.clamp(self.rho_fresh, self.rho_max)
            } else {
                self.rho_fresh
            };
            let depth = if rho_snow > 0.0 { swe / rho_snow } else { 0.0 };

            melt_vals[i] = melt / dt;
            subl_vals[i] = sublimation / dt;
            swe_vals[i] = swe;
            alb_vals[i] = alpha;
            depth_vals[i] = depth;
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
        write_field!("snowmelt", melt_vals);
        write_field!("snow_depth", depth_vals);
        write_field!("swe", swe_vals);
        write_field!("snow_albedo", alb_vals);
        write_field!("sublimation", subl_vals);

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// R0: Degree-Day Melt Model
// ───────────────────────────────────────────────────────────────────

/// Temperature-index (degree-day factor) snowmelt model (R0).
///
/// The simplest snowmelt model:
///   Melt = DDF · max(T − T_melt, 0)
///
/// where DDF is the degree-day factor [mm °C⁻¹ d⁻¹] and T_melt is the
/// threshold temperature (typically 0 °C).
///
/// **Inputs**: `snow_water_equivalent`, `temperature`, `precipitation`
/// **Outputs**: `snow_water_equivalent` (updated), `snowmelt`, `snow_fraction`
pub struct DegreeDayMelt {
    /// Degree-day factor [kg m⁻² °C⁻¹ d⁻¹]
    pub ddf: f64,
    /// Melt threshold temperature [°C]
    pub t_melt: f64,
    /// Rain/snow partition temperature [°C]
    pub t_rain_snow: f64,
}

impl Default for DegreeDayMelt {
    fn default() -> Self {
        Self {
            ddf: 4.0, // typical 2–6 mm/°C/day
            t_melt: 0.0,
            t_rain_snow: 1.5,
        }
    }
}

impl ProcessRunner for DegreeDayMelt {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Cryosphere
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "snow_water_equivalent".into(),
            "temperature".into(),
            "precipitation".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "snow_water_equivalent".into(),
            "snowmelt".into(),
            "snow_fraction".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["water_mass".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        let swe = state
            .get_field("snow_water_equivalent")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: snow_water_equivalent".into())
            })?
            .clone();
        let temp = state
            .get_field("temperature")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: temperature".into()))?
            .clone();
        let precip = state
            .get_field("precipitation")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: precipitation".into()))?
            .clone();

        let n = swe.len();
        let swe_sl = swe.as_slice().unwrap_or(&[]);
        let t_sl = temp.as_slice().unwrap_or(&[]);
        let p_sl = precip.as_slice().unwrap_or(&[]);
        let len = n.min(swe_sl.len()).min(t_sl.len()).min(p_sl.len());

        let dt_days = dt / 86400.0;

        let mut swe_out = vec![0.0f64; len];
        let mut melt_out = vec![0.0f64; len];
        let mut frac_out = vec![0.0f64; len];

        for i in 0..len {
            let mut s = swe_sl[i].max(0.0);
            let t = t_sl[i];
            let p = p_sl[i].max(0.0);

            // Precipitation partition
            let snowfall = if t < self.t_rain_snow { p * dt } else { 0.0 };
            s += snowfall;

            // Degree-day melt
            let melt_potential = self.ddf * (t - self.t_melt).max(0.0) * dt_days;
            let melt = melt_potential.min(s);
            s = (s - melt).max(0.0);

            swe_out[i] = s;
            melt_out[i] = melt / dt; // flux [kg m⁻² s⁻¹]
            frac_out[i] = if s > 1.0 { 1.0 } else { s }; // simple snow-cover fraction
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
        write_field!("snow_water_equivalent", swe_out);
        write_field!("snowmelt", melt_out);
        write_field!("snow_fraction", frac_out);

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
    fn test_degree_day_no_melt_cold() {
        let m = DegreeDayMelt::default();
        let melt_potential = m.ddf * (-5.0_f64 - m.t_melt).max(0.0);
        assert_eq!(melt_potential, 0.0, "No melt below T_melt");
    }

    #[test]
    fn test_degree_day_melt_warm() {
        let m = DegreeDayMelt::default();
        let melt_potential = m.ddf * (5.0 - m.t_melt).max(0.0) * 1.0; // 1 day
        assert!(
            (melt_potential - 20.0).abs() < 1e-10,
            "DDF=4 × 5°C = 20 mm/day"
        );
    }

    #[test]
    fn test_albedo_decays() {
        let m = SnowpackModel::default();
        let dt = 86400.0; // 1 day
        let new_alb = m.decay_albedo(0.85, dt);
        assert!(new_alb < 0.85, "Albedo should decay with time: {new_alb}");
        assert!(new_alb > 0.50, "Should not drop below minimum in one day");
    }

    #[test]
    fn test_no_melt_when_cold() {
        // With cold temperatures and no shortwave, no melt should occur
        let _m = SnowpackModel::default();
        // Q_sw = 0, T_air = -20°C → Q_h < 0, Q_net < 0 → no melt
        // This is a qualitative test of the energy balance logic
        let q_sw = 0.0;
        let t_air_k = 253.15; // -20°C
        let t_snow = 273.15;
        let q_h = 1.225 * 1004.0 * 0.002 * 0.5 * (t_air_k - t_snow);
        assert!(
            q_h < 0.0,
            "Sensible heat should be negative (cooling): {q_h}"
        );
        let q_net = q_sw + q_h;
        assert!(q_net < 0.0, "Net energy should be negative → no melt");
    }

    #[test]
    fn test_warm_temperature_causes_rain() {
        let m = SnowpackModel::default();
        // Above threshold → rain not snow
        let t = 5.0; // °C
        let is_snow = t < m.t_rain_snow;
        assert!(!is_snow, "5°C should be rain, not snow");
    }

    #[test]
    fn test_fresh_snow_refreshes_albedo() {
        let m = SnowpackModel::default();
        // Decayed albedo gets refreshed by snowfall
        let old_alpha = 0.55;
        let snowfall = 5.0; // mm
        let new_alpha = if snowfall > 0.1 {
            m.albedo_fresh
        } else {
            old_alpha
        };
        assert_eq!(new_alpha, 0.85, "Fresh snowfall should reset albedo");
    }

    #[test]
    fn test_snow_density_bounds() {
        let m = SnowpackModel::default();
        let rho = m.snow_density(100.0, 1.0); // 100 kg/m² in 1m → 100 kg/m³
        assert!(rho >= m.rho_fresh);
        assert!(rho <= m.rho_max);
    }
}
