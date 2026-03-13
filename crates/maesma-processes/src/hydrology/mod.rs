//! Hydrology process family — water movement through and over land surfaces.
//!
//! Fidelity ladder:
//! - R0: bucket model (SCS curve-number runoff + linear-reservoir drainage)
//! - R1: Richards equation
//! - R2: variable-saturated flow
//! - R3: integrated hydrology (ParFlow)

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

// ───────────────────────────────────────────────────────────────────
// R0: Bucket Model — SCS Curve-Number + Linear Reservoir
// ───────────────────────────────────────────────────────────────────

/// A simple single-layer bucket model (R0).
///
/// **Runoff** is computed via the SCS Curve-Number method:
///
///   S  = 25400 / CN − 254                   [mm]
///   Ia = λ · S  (initial abstraction, λ default 0.2)
///   If P > Ia:  Q = (P − Ia)² / (P − Ia + S)
///   Else:       Q = 0
///
/// **Soil moisture** is tracked as a single bucket of depth `z_root` [m]:
///   θ += (P − Q − E) · dt / z_root
///   θ  = clamp(θ, θ_r, θ_s)
///
/// **Drainage** follows a linear-reservoir approach:
///   D = k_drain · (θ − θ_fc)  if θ > θ_fc, else 0
///
/// **Actual ET** is scaled from potential ET by a moisture stress function:
///   β = (θ − θ_wp) / (θ_fc − θ_wp),  clipped to [0, 1]
///   E = β · PET
///
/// **State fields** (ndarray 2-D):
///   - `precipitation`      [kg m⁻² s⁻¹]  (≈ mm s⁻¹)
///   - `potential_et`        [kg m⁻² s⁻¹]
///   - `soil_moisture`       [m³ m⁻³]       (volumetric)
///
/// **Output fields**:
///   - `runoff`              [kg m⁻² s⁻¹]
///   - `actual_et`           [kg m⁻² s⁻¹]
///   - `drainage`            [kg m⁻² s⁻¹]
///   - `soil_moisture`       [m³ m⁻³]       (updated in-place)
pub struct BucketModel {
    // ── Soil hydraulic parameters ──
    /// SCS curve number (25–100). Higher = more runoff.
    pub cn: f64,
    /// Saturated water content (porosity) [-].
    pub theta_s: f64,
    /// Residual water content [-].
    pub theta_r: f64,
    /// Field capacity [-].
    pub theta_fc: f64,
    /// Wilting point [-].
    pub theta_wp: f64,
    /// Root-zone depth [m].
    pub z_root: f64,
    /// Drainage coefficient [s⁻¹].
    pub k_drain: f64,
    /// SCS initial-abstraction ratio (typically 0.05–0.2).
    pub lambda_ia: f64,
}

impl Default for BucketModel {
    /// Default parameters: loam soil, moderate CN.
    fn default() -> Self {
        Self {
            cn: 70.0,
            theta_s: 0.45,
            theta_r: 0.05,
            theta_fc: 0.30,
            theta_wp: 0.12,
            z_root: 1.0,
            k_drain: 1.0e-6,
            lambda_ia: 0.2,
        }
    }
}

impl BucketModel {
    /// SCS potential maximum retention [mm].
    fn s_mm(&self) -> f64 {
        25_400.0 / self.cn - 254.0
    }

    /// SCS initial abstraction [mm].
    fn ia_mm(&self) -> f64 {
        self.lambda_ia * self.s_mm()
    }

    /// SCS curve-number runoff for a given precipitation depth [mm].
    /// Returns runoff depth [mm].
    fn scs_runoff_mm(&self, p_mm: f64) -> f64 {
        let ia = self.ia_mm();
        if p_mm <= ia {
            0.0
        } else {
            let num = (p_mm - ia).powi(2);
            let den = p_mm - ia + self.s_mm();
            num / den
        }
    }

    /// Moisture stress factor β ∈ [0, 1].
    fn beta(&self, theta: f64) -> f64 {
        let range = self.theta_fc - self.theta_wp;
        if range <= 0.0 {
            return 1.0;
        }
        ((theta - self.theta_wp) / range).clamp(0.0, 1.0)
    }
}

impl ProcessRunner for BucketModel {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Hydrology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R0
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "precipitation".into(),
            "potential_et".into(),
            "soil_moisture".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "runoff".into(),
            "actual_et".into(),
            "drainage".into(),
            "soil_moisture".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["water_mass".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, dt: f64) -> maesma_core::Result<()> {
        // Fetch 2-D fields. If missing, bail with an error.
        let precip = state
            .get_field("precipitation")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: precipitation".into()))?
            .clone();
        let pet = state
            .get_field("potential_et")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: potential_et".into()))?
            .clone();

        let sm = state
            .get_field_mut("soil_moisture")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: soil_moisture".into()))?;

        // Allocate output arrays with the same shape as soil_moisture.
        let _shape = sm.shape().to_vec();

        // We also need mutable access to runoff, actual_et, drainage — but
        // ProcessState only gives us get_field_mut one at a time. We'll
        // compute into local arrays then write back.
        let n = sm.len();
        let mut runoff_vals = vec![0.0f64; n];
        let mut et_vals = vec![0.0f64; n];
        let mut drain_vals = vec![0.0f64; n];

        // Iterate over all cells. We flatten to 1-D iterators.
        let precip_sl = precip.as_slice().unwrap_or(&[]);
        let pet_sl = pet.as_slice().unwrap_or(&[]);
        let sm_sl = sm.as_slice_mut().unwrap_or(&mut []);

        for i in 0..n.min(precip_sl.len()).min(pet_sl.len()).min(sm_sl.len()) {
            let p_flux = precip_sl[i].max(0.0); // kg m⁻² s⁻¹
            let pet_flux = pet_sl[i].max(0.0);
            let theta = sm_sl[i];

            // Convert precipitation flux to depth [mm] over timestep.
            // 1 kg m⁻² = 1 mm water depth.
            let p_mm = p_flux * dt; // mm

            // ── Runoff (SCS) ──
            let q_mm = self.scs_runoff_mm(p_mm);
            let q_flux = q_mm / dt; // back to kg m⁻² s⁻¹
            runoff_vals[i] = q_flux;

            // ── Actual ET ──
            let beta = self.beta(theta);
            let et_flux = beta * pet_flux;
            et_vals[i] = et_flux;

            // ── Drainage ──
            let d_flux = if theta > self.theta_fc {
                self.k_drain * (theta - self.theta_fc) * 1000.0 // convert m³/m³/s → kg m⁻² s⁻¹ approx via ρ·z
            } else {
                0.0
            };
            drain_vals[i] = d_flux;

            // ── Update soil moisture ──
            // Net flux into bucket: infiltration − ET − drainage.
            let infil_flux = p_flux - q_flux; // kg m⁻² s⁻¹
            let net_flux = infil_flux - et_flux - d_flux;

            // Convert flux to Δθ: Δθ = net_flux · dt / (ρ_w · z_root)
            // ρ_w = 1000 kg/m³
            let delta_theta = net_flux * dt / (1000.0 * self.z_root);
            sm_sl[i] = (theta + delta_theta).clamp(self.theta_r, self.theta_s);
        }

        // Write back output fields if they exist.
        if let Some(runoff_field) = state.get_field_mut("runoff")
            && let Some(sl) = runoff_field.as_slice_mut()
        {
            for (o, v) in sl.iter_mut().zip(runoff_vals.iter()) {
                *o = *v;
            }
        }
        if let Some(et_field) = state.get_field_mut("actual_et")
            && let Some(sl) = et_field.as_slice_mut()
        {
            for (o, v) in sl.iter_mut().zip(et_vals.iter()) {
                *o = *v;
            }
        }
        if let Some(drain_field) = state.get_field_mut("drainage")
            && let Some(sl) = drain_field.as_slice_mut()
        {
            for (o, v) in sl.iter_mut().zip(drain_vals.iter()) {
                *o = *v;
            }
        }

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// R1: Richards Equation
// ───────────────────────────────────────────────────────────────────

/// Richards-equation-based infiltration model (R1).
pub struct RichardsInfiltration {
    /// Saturated hydraulic conductivity [m/s]
    pub k_sat: f64,
    /// Soil porosity [-]
    pub porosity: f64,
}

impl Default for RichardsInfiltration {
    fn default() -> Self {
        Self {
            k_sat: 1e-5,
            porosity: 0.45,
        }
    }
}

impl ProcessRunner for RichardsInfiltration {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Hydrology
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "precipitation".into(),
            "soil_moisture".into(),
            "hydraulic_conductivity".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "infiltration_rate".into(),
            "runoff".into(),
            "soil_moisture_updated".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["water".into()]
    }

    fn step(&mut self, _state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        // TODO: implement Richards equation infiltration
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
    fn test_scs_zero_rain() {
        let m = BucketModel::default();
        assert_eq!(m.scs_runoff_mm(0.0), 0.0);
    }

    #[test]
    fn test_scs_light_rain_no_runoff() {
        let m = BucketModel::default();
        // S ≈ 108.86 mm, Ia ≈ 21.77 mm
        assert_eq!(m.scs_runoff_mm(10.0), 0.0, "10mm < Ia → no runoff");
    }

    #[test]
    fn test_scs_heavy_rain_runoff() {
        let m = BucketModel::default();
        let q = m.scs_runoff_mm(100.0);
        // (100 − 21.77)² / (100 − 21.77 + 108.86) ≈ 32.6 mm
        assert!(
            q > 20.0,
            "Heavy rain should produce significant runoff, got {q}"
        );
        assert!(q < 100.0, "Runoff can't exceed precipitation");
    }

    #[test]
    fn test_scs_high_cn_more_runoff() {
        let lo = BucketModel {
            cn: 50.0,
            ..BucketModel::default()
        };
        let hi = BucketModel {
            cn: 95.0,
            ..BucketModel::default()
        };
        let q_lo = lo.scs_runoff_mm(80.0);
        let q_hi = hi.scs_runoff_mm(80.0);
        assert!(
            q_hi > q_lo,
            "Higher CN should produce more runoff: {q_hi} > {q_lo}"
        );
    }

    #[test]
    fn test_beta_moisture_stress() {
        let m = BucketModel::default();
        // Below wilting point → β = 0
        assert_eq!(m.beta(0.05), 0.0);
        // At field capacity → β = 1
        assert!((m.beta(m.theta_fc) - 1.0).abs() < 1e-10);
        // Midpoint → β = 0.5
        let mid = (m.theta_wp + m.theta_fc) / 2.0;
        assert!((m.beta(mid) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_conservation_property() {
        let m = BucketModel::default();
        assert_eq!(m.conserved_quantities(), vec!["water_mass"]);
        assert_eq!(m.family(), ProcessFamily::Hydrology);
        assert_eq!(m.rung(), FidelityRung::R0);
    }
}
