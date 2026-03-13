//! Canadian Forest Service Fire Behaviour Prediction (CFS FBP) system.
//!
//! Reference: Forestry Canada Fire Danger Group (1992). "Development and
//! structure of the Canadian Forest Fire Behavior Prediction System."
//! Information Report ST-X-3.

use maesma_core::families::ProcessFamily;
use maesma_core::process::{FidelityRung, ProcessRunner, ProcessState};

/// CFS FBP fuel type codes.
#[derive(Debug, Clone, Copy)]
pub enum FuelType {
    C1,  // Spruce–lichen woodland
    C2,  // Boreal spruce
    C3,  // Mature jack or lodgepole pine
    C4,  // Immature jack or lodgepole pine
    C5,  // Red and white pine
    C6,  // Conifer plantation
    C7,  // Ponderosa pine / Douglas fir
    D1,  // Leafless aspen
    M1,  // Boreal mixedwood (leafless)
    M2,  // Boreal mixedwood (green)
    M3,  // Dead balsam fir mixedwood (leafless)
    M4,  // Dead balsam fir mixedwood (green)
    O1a, // Matted grass
    O1b, // Standing grass
    S1,  // Jack or lodgepole pine slash
    S2,  // White spruce / balsam slash
    S3,  // Coastal cedar / hemlock / Douglas fir slash
}

/// CFS FBP crown fire spread model.
pub struct CfsFbpCrown {
    pub fuel_type: FuelType,
    /// Fine Fuel Moisture Code.
    pub ffmc: f64,
    /// Build-Up Index.
    pub bui: f64,
    /// Wind speed at 10m (km/h).
    pub wind_speed: f64,
    /// Slope (%).
    pub slope_percent: f64,
    /// Foliar Moisture Content (%).
    pub fmc: f64,
}

impl CfsFbpCrown {
    /// Calculate Initial Spread Index (ISI) from FFMC and wind.
    pub fn initial_spread_index(&self) -> f64 {
        let m = 147.2 * (101.0 - self.ffmc) / (59.5 + self.ffmc);
        let f_w = (-0.05039 * self.wind_speed).exp();
        let f_f = 91.9 * (-0.1386 * m).exp() * (1.0 + m.powf(5.31) / 4.93e7);
        0.208 * f_w * f_f
    }

    /// Calculate Rate of Spread (m/min) for the given fuel type.
    pub fn rate_of_spread(&self) -> f64 {
        let isi = self.initial_spread_index();

        // Simplified: use C2 boreal spruce as default
        let (a, b, c0) = match self.fuel_type {
            FuelType::C1 => (90.0, 0.0649, 4.5),
            FuelType::C2 => (110.0, 0.0282, 1.5),
            FuelType::C3 => (110.0, 0.0444, 3.0),
            FuelType::C4 => (110.0, 0.0293, 1.5),
            FuelType::C5 => (30.0, 0.0697, 4.0),
            FuelType::C6 => (30.0, 0.0800, 3.0),
            FuelType::C7 => (45.0, 0.0305, 2.0),
            FuelType::D1 => (30.0, 0.0232, 1.6),
            _ => (110.0, 0.0282, 1.5), // default to C2
        };

        a * (1.0 - (-b * isi).exp()).powf(c0) // m/min
    }

    /// Determine if crown fire initiation occurs.
    pub fn crown_fraction_burned(&self) -> f64 {
        let ros = self.rate_of_spread();
        // Critical surface intensity for crown fire initiation
        let csi = 0.001 * (460.0 + 25.9 * self.fmc).powi(3).cbrt();
        let rso = csi; // simplified threshold

        if ros > rso {
            1.0_f64.min((ros - rso) / (2.0 * rso)).max(0.0)
        } else {
            0.0
        }
    }
}

impl ProcessRunner for CfsFbpCrown {
    fn family(&self) -> ProcessFamily {
        ProcessFamily::Fire
    }

    fn rung(&self) -> FidelityRung {
        FidelityRung::R1
    }

    fn inputs(&self) -> Vec<String> {
        vec![
            "ffmc".into(),
            "bui".into(),
            "wind_speed_10m".into(),
            "terrain_slope".into(),
            "foliar_moisture_content".into(),
            "fuel_type".into(),
        ]
    }

    fn outputs(&self) -> Vec<String> {
        vec![
            "crown_rate_of_spread".into(),
            "crown_fraction_burned".into(),
            "head_fire_intensity".into(),
        ]
    }

    fn conserved_quantities(&self) -> Vec<String> {
        vec!["energy".into(), "carbon".into()]
    }

    fn step(&mut self, state: &mut dyn ProcessState, _dt: f64) -> maesma_core::Result<()> {
        // Read inputs and use existing CFS FBP physics
        let ffmc_field = state
            .get_field("ffmc")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: ffmc".into()))?
            .clone();
        let bui_field = state
            .get_field("bui")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: bui".into()))?
            .clone();
        let wind_field = state
            .get_field("wind_speed_10m")
            .ok_or_else(|| maesma_core::Error::Runtime("missing field: wind_speed_10m".into()))?
            .clone();
        let fmc_field = state
            .get_field("foliar_moisture_content")
            .ok_or_else(|| {
                maesma_core::Error::Runtime("missing field: foliar_moisture_content".into())
            })?
            .clone();

        let n = ffmc_field.len();
        let ff = ffmc_field.as_slice().unwrap_or(&[]);
        let bu = bui_field.as_slice().unwrap_or(&[]);
        let wi = wind_field.as_slice().unwrap_or(&[]);
        let fm = fmc_field.as_slice().unwrap_or(&[]);
        let len = n.min(ff.len()).min(bu.len()).min(wi.len()).min(fm.len());

        let mut ros_vals = vec![0.0f64; len];
        let mut cfb_vals = vec![0.0f64; len];
        let mut hfi_vals = vec![0.0f64; len];

        for i in 0..len {
            self.ffmc = ff[i].clamp(0.0, 101.0);
            self.bui = bu[i].max(0.0);
            self.wind_speed = wi[i].max(0.0);
            self.fmc = fm[i].max(0.0);

            let ros = self.rate_of_spread(); // m/min
            let cfb = self.crown_fraction_burned();

            // Head fire intensity: I = 300 · ROS · fuel consumed (simplified)
            // Using 300 kJ/m² as typical heat-per-unit-area for boreal fuels
            let hfi = 300.0 * ros; // kW m⁻¹ (simplified)

            ros_vals[i] = ros;
            cfb_vals[i] = cfb;
            hfi_vals[i] = hfi;
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
        write_field!("crown_rate_of_spread", ros_vals);
        write_field!("crown_fraction_burned", cfb_vals);
        write_field!("head_fire_intensity", hfi_vals);

        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────
// Tests
// ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn default_model() -> CfsFbpCrown {
        CfsFbpCrown {
            fuel_type: FuelType::C2,
            ffmc: 90.0,
            bui: 50.0,
            wind_speed: 20.0,
            slope_percent: 0.0,
            fmc: 100.0,
        }
    }

    #[test]
    fn test_isi_positive() {
        let m = default_model();
        let isi = m.initial_spread_index();
        assert!(isi > 0.0, "ISI should be positive: {isi}");
    }

    #[test]
    fn test_ros_positive() {
        let m = default_model();
        let ros = m.rate_of_spread();
        assert!(ros > 0.0, "ROS should be positive: {ros}");
    }

    #[test]
    fn test_higher_ffmc_faster_ros() {
        let low = CfsFbpCrown {
            ffmc: 70.0,
            ..default_model()
        };
        let high = CfsFbpCrown {
            ffmc: 95.0,
            ..default_model()
        };
        assert!(
            high.rate_of_spread() > low.rate_of_spread(),
            "Higher FFMC → faster ROS"
        );
    }

    #[test]
    fn test_cfb_bounded() {
        let m = default_model();
        let cfb = m.crown_fraction_burned();
        assert!((0.0..=1.0).contains(&cfb), "CFB should be in [0,1]: {cfb}");
    }
}
