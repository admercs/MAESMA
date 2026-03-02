//! Rothermel surface fire spread model.
//!
//! Reference: Rothermel, R. C. (1972). "A mathematical model for predicting
//! fire spread in wildland fuels." USDA Forest Service Research Paper INT-115.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

/// Rothermel surface fire spread rate calculator.
pub struct RothermelSurface {
    /// Fuel model parameters.
    pub fuel_bed_depth: f64, // m
    pub fuel_load: f64,              // kg/m²
    pub surface_area_to_volume: f64, // 1/m
    pub fuel_moisture_content: f64,  // fraction
    pub mineral_content: f64,        // fraction
    pub fuel_particle_density: f64,  // kg/m³
    pub heat_content: f64,           // kJ/kg
    pub moisture_of_extinction: f64, // fraction
}

impl RothermelSurface {
    /// Calculate the forward rate of spread (m/s).
    pub fn rate_of_spread(&self, wind_speed: f64, slope: f64) -> f64 {
        // Packing ratio
        let beta = self.fuel_load / (self.fuel_bed_depth * self.fuel_particle_density);
        let beta_opt = 3.348 * self.surface_area_to_volume.powf(-0.8189);
        let beta_ratio = beta / beta_opt;

        // Reaction intensity (kW/m²)
        let gamma_max = self.surface_area_to_volume.powf(1.5)
            / (495.0 + 0.0594 * self.surface_area_to_volume.powf(1.5));
        let gamma = gamma_max
            * beta_ratio.powf(0.8189 - 1.0)
            * (-138.0 / self.surface_area_to_volume).exp();

        let moisture_ratio = self.fuel_moisture_content / self.moisture_of_extinction;
        let eta_m = 1.0 - 2.59 * moisture_ratio + 5.11 * moisture_ratio.powi(2)
            - 3.52 * moisture_ratio.powi(3);
        let eta_m = eta_m.max(0.0);

        let eta_s = 0.174 * self.mineral_content.powf(-0.19);

        let ir = gamma * self.fuel_load * self.heat_content * eta_m * eta_s;

        // Propagating flux ratio
        let xi = (0.792 + 0.681 * self.surface_area_to_volume.powf(0.5)).recip()
            * (192.0 + 0.2595 * self.surface_area_to_volume).recip()
            * (0.792 + 0.681 * self.surface_area_to_volume.powf(0.5));

        // Wind factor
        let c = 7.47 * (-0.133 * self.surface_area_to_volume.powf(0.55)).exp();
        let b = 0.02526 * self.surface_area_to_volume.powf(0.54);
        let e = 0.715 * (-3.59e-4 * self.surface_area_to_volume).exp();
        let phi_w = c * wind_speed.powf(b) * beta_ratio.powf(-e);

        // Slope factor
        let phi_s = 5.275 * beta.powf(-0.3) * slope.powi(2);

        // Effective heating number
        let epsilon = (-138.0 / self.surface_area_to_volume).exp();

        // Heat of pre-ignition
        let q_ig = 250.0 + 1116.0 * self.fuel_moisture_content;

        // Rate of spread (m/min → m/s)
        let ros = (ir * xi * (1.0 + phi_w + phi_s)) / (self.fuel_load * epsilon * q_ig);
        ros / 60.0 // convert m/min to m/s
    }
}

impl ProcessRunner for RothermelSurface {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Fire
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "fuel_load".into(),
            "fuel_moisture".into(),
            "wind_speed".into(),
            "terrain_slope".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "rate_of_spread".into(),
            "fire_intensity".into(),
            "flame_length".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["energy".into(), "carbon".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        // Read inputs and use existing physics to compute fire behaviour
        let fuel = state
            .get_field("fuel_load")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: fuel_load".into()))?
            .clone();
        let moisture = state
            .get_field("fuel_moisture")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: fuel_moisture".into()))?
            .clone();
        let wind = state
            .get_field("wind_speed")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: wind_speed".into()))?
            .clone();
        let slope = state
            .get_field("terrain_slope")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: terrain_slope".into()))?
            .clone();

        let n = fuel.len();
        let fl = fuel.as_slice().unwrap_or(&[]);
        let mo = moisture.as_slice().unwrap_or(&[]);
        let wi = wind.as_slice().unwrap_or(&[]);
        let sl = slope.as_slice().unwrap_or(&[]);
        let len = n.min(fl.len()).min(mo.len()).min(wi.len()).min(sl.len());

        let mut ros_vals = vec![0.0f64; len];
        let mut intensity_vals = vec![0.0f64; len];
        let mut flame_vals = vec![0.0f64; len];

        for i in 0..len {
            // Update model params from state fields
            self.fuel_load = fl[i].max(0.0);
            self.fuel_moisture_content = mo[i].clamp(0.0, 1.0);

            // Compute Rothermel rate of spread
            let ros = self.rate_of_spread(wi[i].max(0.0), sl[i].max(0.0));
            ros_vals[i] = ros;

            // Byram's fireline intensity: I = H · w · R  [kW m⁻¹]
            let intensity = self.heat_content * self.fuel_load * ros;
            intensity_vals[i] = intensity;

            // Flame length (Byram 1959): L = 0.0775 · I^0.46  [m]
            flame_vals[i] = 0.0775 * intensity.max(0.0).powf(0.46);
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
        write_field!("rate_of_spread", ros_vals);
        write_field!("fire_intensity", intensity_vals);
        write_field!("flame_length", flame_vals);

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// Tests
// ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn default_model() -> RothermelSurface {
        RothermelSurface {
            fuel_bed_depth: 0.3,
            fuel_load: 0.5,
            surface_area_to_volume: 5000.0,
            fuel_moisture_content: 0.06,
            mineral_content: 0.055,
            fuel_particle_density: 513.0,
            heat_content: 18600.0, // kJ/kg
            moisture_of_extinction: 0.20,
        }
    }

    #[test]
    fn test_ros_positive() {
        let m = default_model();
        let ros = m.rate_of_spread(2.0, 0.0);
        assert!(ros > 0.0, "ROS should be positive: {ros}");
    }

    #[test]
    fn test_wind_increases_ros() {
        let m = default_model();
        let ros_calm = m.rate_of_spread(0.5, 0.0);
        let ros_windy = m.rate_of_spread(5.0, 0.0);
        assert!(
            ros_windy > ros_calm,
            "Higher wind → higher ROS: {ros_windy} vs {ros_calm}"
        );
    }

    #[test]
    fn test_slope_increases_ros() {
        let m = default_model();
        let ros_flat = m.rate_of_spread(1.0, 0.0);
        let ros_steep = m.rate_of_spread(1.0, 0.5);
        assert!(
            ros_steep > ros_flat,
            "Steeper slope → higher ROS: {ros_steep} vs {ros_flat}"
        );
    }

    #[test]
    fn test_wet_fuel_reduces_ros() {
        let dry = RothermelSurface {
            fuel_moisture_content: 0.05,
            ..default_model()
        };
        let wet = RothermelSurface {
            fuel_moisture_content: 0.15,
            ..default_model()
        };
        let ros_dry = dry.rate_of_spread(2.0, 0.0);
        let ros_wet = wet.rate_of_spread(2.0, 0.0);
        assert!(
            ros_dry > ros_wet,
            "Drier fuel → faster spread: {ros_dry} vs {ros_wet}"
        );
    }
}
