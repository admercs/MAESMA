//! Shared variable registry — canonical descriptors for forcing, slow-state,
//! and disturbance variables shared across process families.

use serde::{Deserialize, Serialize};

use crate::families::ProcessFamily;
use crate::units::PhysicalUnit;

/// A canonical variable descriptor with metadata for coupling validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDescriptor {
    /// Canonical name (e.g., "precipitation", "soil_moisture").
    pub name: String,
    /// Physical unit.
    pub unit: PhysicalUnit,
    /// Variable category.
    pub category: VariableCategory,
    /// Physical lower bound (if applicable).
    pub lower_bound: Option<f64>,
    /// Physical upper bound (if applicable).
    pub upper_bound: Option<f64>,
    /// Which process family has primary write authority.
    pub update_authority: Option<ProcessFamily>,
    /// Human-readable description.
    pub description: String,
}

/// Broad category of a shared variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableCategory {
    /// Externally prescribed atmospheric or oceanic forcing.
    Forcing,
    /// Slowly evolving state updated by ecology, BGC, cryosphere, etc.
    SlowState,
    /// Disturbance-related state (fire, flood, landslide).
    DisturbanceState,
}

/// The shared variable registry.
pub struct VariableRegistry {
    entries: Vec<VariableDescriptor>,
}

impl VariableRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Create a registry pre-populated with MAESMA canonical variables.
    pub fn with_defaults() -> Self {
        let mut reg = Self::new();
        reg.register_forcing_variables();
        reg.register_slow_state_variables();
        reg.register_disturbance_variables();
        reg
    }

    /// Register a single variable descriptor.
    pub fn register(&mut self, desc: VariableDescriptor) {
        self.entries.push(desc);
    }

    /// Look up a variable by name.
    pub fn get(&self, name: &str) -> Option<&VariableDescriptor> {
        self.entries.iter().find(|v| v.name == name)
    }

    /// All registered variables.
    pub fn all(&self) -> &[VariableDescriptor] {
        &self.entries
    }

    /// Variables in a given category.
    pub fn by_category(&self, cat: VariableCategory) -> Vec<&VariableDescriptor> {
        self.entries.iter().filter(|v| v.category == cat).collect()
    }

    /// Check that a value is within the declared bounds of a variable.
    pub fn check_bounds(&self, name: &str, value: f64) -> BoundsResult {
        match self.get(name) {
            None => BoundsResult::UnknownVariable,
            Some(desc) => {
                if let Some(lo) = desc.lower_bound {
                    if value < lo {
                        return BoundsResult::BelowLower { value, bound: lo };
                    }
                }
                if let Some(hi) = desc.upper_bound {
                    if value > hi {
                        return BoundsResult::AboveUpper { value, bound: hi };
                    }
                }
                BoundsResult::Ok
            }
        }
    }

    /// Number of registered variables.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    // ── Default variable sets ────────────────────────────────────

    fn register_forcing_variables(&mut self) {
        use crate::units::common;
        let forcing = VariableCategory::Forcing;

        self.register(VariableDescriptor {
            name: "precipitation".into(),
            unit: common::mass_flux(),
            category: forcing,
            lower_bound: Some(0.0),
            upper_bound: Some(0.5), // 500 mm/hr extreme
            update_authority: Some(ProcessFamily::Atmosphere),
            description: "Precipitation rate (P)".into(),
        });

        self.register(VariableDescriptor {
            name: "air_temperature".into(),
            unit: common::kelvin(),
            category: forcing,
            lower_bound: Some(180.0),
            upper_bound: Some(340.0),
            update_authority: Some(ProcessFamily::Atmosphere),
            description: "Near-surface air temperature (Tair)".into(),
        });

        self.register(VariableDescriptor {
            name: "relative_humidity".into(),
            unit: PhysicalUnit::dimensionless(),
            category: forcing,
            lower_bound: Some(0.0),
            upper_bound: Some(1.0),
            update_authority: Some(ProcessFamily::Atmosphere),
            description: "Relative humidity (RH), 0–1".into(),
        });

        self.register(VariableDescriptor {
            name: "vapor_pressure_deficit".into(),
            unit: common::pascal(),
            category: forcing,
            lower_bound: Some(0.0),
            upper_bound: Some(10000.0),
            update_authority: Some(ProcessFamily::Atmosphere),
            description: "Vapor pressure deficit (VPD)".into(),
        });

        self.register(VariableDescriptor {
            name: "wind_speed".into(),
            unit: common::velocity(),
            category: forcing,
            lower_bound: Some(0.0),
            upper_bound: Some(120.0),
            update_authority: Some(ProcessFamily::Atmosphere),
            description: "Near-surface wind speed (Wind)".into(),
        });

        self.register(VariableDescriptor {
            name: "shortwave_radiation".into(),
            unit: common::energy_flux(),
            category: forcing,
            lower_bound: Some(0.0),
            upper_bound: Some(1400.0),
            update_authority: Some(ProcessFamily::Radiation),
            description: "Downwelling shortwave radiation (SWdown)".into(),
        });

        self.register(VariableDescriptor {
            name: "longwave_radiation".into(),
            unit: common::energy_flux(),
            category: forcing,
            lower_bound: Some(0.0),
            upper_bound: Some(600.0),
            update_authority: Some(ProcessFamily::Radiation),
            description: "Downwelling longwave radiation (LWdown)".into(),
        });

        self.register(VariableDescriptor {
            name: "co2_concentration".into(),
            unit: PhysicalUnit {
                label: "ppmv".into(),
                m: 0,
                kg: 0,
                s: 0,
                k: 0,
                mol: 0,
                scale: 1e-6,
                offset: 0.0,
            },
            category: forcing,
            lower_bound: Some(150.0),
            upper_bound: Some(2000.0),
            update_authority: Some(ProcessFamily::Atmosphere),
            description: "Atmospheric CO₂ concentration".into(),
        });
    }

    fn register_slow_state_variables(&mut self) {
        use crate::units::common;
        let slow = VariableCategory::SlowState;

        self.register(VariableDescriptor {
            name: "leaf_area_index".into(),
            unit: PhysicalUnit::dimensionless(),
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(15.0),
            update_authority: Some(ProcessFamily::Ecology),
            description: "Leaf area index (LAI)".into(),
        });

        self.register(VariableDescriptor {
            name: "canopy_height".into(),
            unit: common::meter(),
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(120.0),
            update_authority: Some(ProcessFamily::Ecology),
            description: "Vegetation canopy height".into(),
        });

        self.register(VariableDescriptor {
            name: "canopy_bulk_density".into(),
            unit: PhysicalUnit {
                label: "kg m⁻³".into(),
                m: -3,
                kg: 1,
                s: 0,
                k: 0,
                mol: 0,
                scale: 1.0,
                offset: 0.0,
            },
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(0.5),
            update_authority: Some(ProcessFamily::Ecology),
            description: "Canopy bulk density (CBD) for crown fire".into(),
        });

        self.register(VariableDescriptor {
            name: "surface_fuel_load".into(),
            unit: common::carbon_stock(),
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(50.0),
            update_authority: Some(ProcessFamily::Ecology),
            description: "Surface fuel load for fire behavior".into(),
        });

        self.register(VariableDescriptor {
            name: "soil_moisture".into(),
            unit: PhysicalUnit::dimensionless(),
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(1.0),
            update_authority: Some(ProcessFamily::Hydrology),
            description: "Volumetric soil moisture (top layer), 0–1".into(),
        });

        self.register(VariableDescriptor {
            name: "soil_temperature".into(),
            unit: common::kelvin(),
            category: slow,
            lower_bound: Some(200.0),
            upper_bound: Some(350.0),
            update_authority: Some(ProcessFamily::Hydrology),
            description: "Soil temperature (top layer)".into(),
        });

        self.register(VariableDescriptor {
            name: "snow_water_equivalent".into(),
            unit: common::mm_water(),
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(5000.0),
            update_authority: Some(ProcessFamily::Cryosphere),
            description: "Snow water equivalent (SWE)".into(),
        });

        self.register(VariableDescriptor {
            name: "soil_carbon".into(),
            unit: common::carbon_stock(),
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(200.0),
            update_authority: Some(ProcessFamily::Biogeochemistry),
            description: "Soil organic carbon pool".into(),
        });

        self.register(VariableDescriptor {
            name: "soil_nitrogen".into(),
            unit: PhysicalUnit {
                label: "kg N m⁻²".into(),
                m: -2,
                kg: 1,
                s: 0,
                k: 0,
                mol: 0,
                scale: 1.0,
                offset: 0.0,
            },
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(20.0),
            update_authority: Some(ProcessFamily::Biogeochemistry),
            description: "Soil nitrogen pool".into(),
        });

        self.register(VariableDescriptor {
            name: "streamflow".into(),
            unit: PhysicalUnit {
                label: "m³ s⁻¹".into(),
                m: 3,
                kg: 0,
                s: -1,
                k: 0,
                mol: 0,
                scale: 1.0,
                offset: 0.0,
            },
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: None,
            update_authority: Some(ProcessFamily::Hydrology),
            description: "River discharge / streamflow".into(),
        });

        self.register(VariableDescriptor {
            name: "water_table_depth".into(),
            unit: common::meter(),
            category: slow,
            lower_bound: Some(0.0),
            upper_bound: Some(100.0),
            update_authority: Some(ProcessFamily::Hydrology),
            description: "Depth to water table".into(),
        });
    }

    fn register_disturbance_variables(&mut self) {
        let dist = VariableCategory::DisturbanceState;

        self.register(VariableDescriptor {
            name: "burn_severity".into(),
            unit: PhysicalUnit::dimensionless(),
            category: dist,
            lower_bound: Some(0.0),
            upper_bound: Some(1.0),
            update_authority: Some(ProcessFamily::Fire),
            description: "Burn severity index, 0 (unburned) to 1 (stand-replacing)".into(),
        });

        self.register(VariableDescriptor {
            name: "mortality_fraction".into(),
            unit: PhysicalUnit::dimensionless(),
            category: dist,
            lower_bound: Some(0.0),
            upper_bound: Some(1.0),
            update_authority: Some(ProcessFamily::Fire),
            description: "Tree mortality fraction from disturbance".into(),
        });

        self.register(VariableDescriptor {
            name: "soil_hydrophobicity".into(),
            unit: PhysicalUnit::dimensionless(),
            category: dist,
            lower_bound: Some(0.0),
            upper_bound: Some(1.0),
            update_authority: Some(ProcessFamily::Fire),
            description: "Post-fire soil hydrophobicity index".into(),
        });

        self.register(VariableDescriptor {
            name: "char_fraction".into(),
            unit: PhysicalUnit::dimensionless(),
            category: dist,
            lower_bound: Some(0.0),
            upper_bound: Some(1.0),
            update_authority: Some(ProcessFamily::Fire),
            description: "Fraction of biomass converted to char (black carbon)".into(),
        });

        self.register(VariableDescriptor {
            name: "ash_nutrient_pulse".into(),
            unit: PhysicalUnit {
                label: "kg N m⁻²".into(),
                m: -2,
                kg: 1,
                s: 0,
                k: 0,
                mol: 0,
                scale: 1.0,
                offset: 0.0,
            },
            category: dist,
            lower_bound: Some(0.0),
            upper_bound: None,
            update_authority: Some(ProcessFamily::Fire),
            description: "Nutrient pulse from ash deposition after fire".into(),
        });
    }
}

impl Default for VariableRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a bounds check.
#[derive(Debug, Clone, PartialEq)]
pub enum BoundsResult {
    Ok,
    UnknownVariable,
    BelowLower { value: f64, bound: f64 },
    AboveUpper { value: f64, bound: f64 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_registry_has_forcing_vars() {
        let reg = VariableRegistry::with_defaults();
        assert!(reg.len() > 0);
        let forcing = reg.by_category(VariableCategory::Forcing);
        assert!(forcing.len() >= 7);
        assert!(reg.get("precipitation").is_some());
        assert!(reg.get("air_temperature").is_some());
        assert!(reg.get("co2_concentration").is_some());
    }

    #[test]
    fn default_registry_has_slow_state_vars() {
        let reg = VariableRegistry::with_defaults();
        let slow = reg.by_category(VariableCategory::SlowState);
        assert!(slow.len() >= 5);
        assert!(reg.get("leaf_area_index").is_some());
        assert!(reg.get("soil_moisture").is_some());
        assert!(reg.get("snow_water_equivalent").is_some());
    }

    #[test]
    fn default_registry_has_disturbance_vars() {
        let reg = VariableRegistry::with_defaults();
        let dist = reg.by_category(VariableCategory::DisturbanceState);
        assert!(dist.len() >= 4);
        assert!(reg.get("burn_severity").is_some());
    }

    #[test]
    fn bounds_check_ok() {
        let reg = VariableRegistry::with_defaults();
        assert_eq!(reg.check_bounds("soil_moisture", 0.5), BoundsResult::Ok);
    }

    #[test]
    fn bounds_check_below() {
        let reg = VariableRegistry::with_defaults();
        match reg.check_bounds("soil_moisture", -0.1) {
            BoundsResult::BelowLower { .. } => {}
            other => panic!("Expected BelowLower, got {:?}", other),
        }
    }

    #[test]
    fn bounds_check_above() {
        let reg = VariableRegistry::with_defaults();
        match reg.check_bounds("soil_moisture", 1.5) {
            BoundsResult::AboveUpper { .. } => {}
            other => panic!("Expected AboveUpper, got {:?}", other),
        }
    }

    #[test]
    fn bounds_check_unknown_variable() {
        let reg = VariableRegistry::with_defaults();
        assert_eq!(
            reg.check_bounds("nonexistent", 0.0),
            BoundsResult::UnknownVariable
        );
    }
}
