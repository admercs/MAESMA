//! Physical units — lightweight dimensional analysis for I/O contracts.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A physical unit expressed as SI base-dimension exponents + a scale factor.
///
/// `value_si = raw_value * scale + offset`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalUnit {
    /// Human-readable label (e.g., "kg m⁻² s⁻¹").
    pub label: String,
    /// Exponent of meter (L).
    pub m: i8,
    /// Exponent of kilogram (M).
    pub kg: i8,
    /// Exponent of second (T).
    pub s: i8,
    /// Exponent of kelvin (Θ).
    pub k: i8,
    /// Exponent of mole (N).
    pub mol: i8,
    /// Scale factor to SI.
    pub scale: f64,
    /// Offset to SI (for °C → K, etc.).
    pub offset: f64,
}

impl PhysicalUnit {
    /// Dimensionless unit.
    pub fn dimensionless() -> Self {
        Self {
            label: "1".into(),
            m: 0,
            kg: 0,
            s: 0,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    /// Check dimensional compatibility (ignoring scale/offset).
    pub fn compatible_with(&self, other: &Self) -> bool {
        self.m == other.m
            && self.kg == other.kg
            && self.s == other.s
            && self.k == other.k
            && self.mol == other.mol
    }

    /// Convert a value from this unit to SI.
    pub fn to_si(&self, value: f64) -> f64 {
        value * self.scale + self.offset
    }

    /// Convert a value from SI to this unit.
    pub fn from_si(&self, si_value: f64) -> f64 {
        (si_value - self.offset) / self.scale
    }
}

impl fmt::Display for PhysicalUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}

// ── Common units ─────────────────────────────────────────────────────

pub mod common {
    use super::PhysicalUnit;

    pub fn kg() -> PhysicalUnit {
        PhysicalUnit {
            label: "kg".into(),
            m: 0,
            kg: 1,
            s: 0,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    pub fn meter() -> PhysicalUnit {
        PhysicalUnit {
            label: "m".into(),
            m: 1,
            kg: 0,
            s: 0,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    pub fn second() -> PhysicalUnit {
        PhysicalUnit {
            label: "s".into(),
            m: 0,
            kg: 0,
            s: 1,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    pub fn kelvin() -> PhysicalUnit {
        PhysicalUnit {
            label: "K".into(),
            m: 0,
            kg: 0,
            s: 0,
            k: 1,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    pub fn celsius() -> PhysicalUnit {
        PhysicalUnit {
            label: "°C".into(),
            m: 0,
            kg: 0,
            s: 0,
            k: 1,
            mol: 0,
            scale: 1.0,
            offset: 273.15,
        }
    }

    /// kg m⁻² s⁻¹  (mass flux)
    pub fn mass_flux() -> PhysicalUnit {
        PhysicalUnit {
            label: "kg m⁻² s⁻¹".into(),
            m: -2,
            kg: 1,
            s: -1,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    /// W m⁻²  (energy flux)
    pub fn energy_flux() -> PhysicalUnit {
        PhysicalUnit {
            label: "W m⁻²".into(),
            m: 0,
            kg: 1,
            s: -3,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    /// m s⁻¹  (velocity)
    pub fn velocity() -> PhysicalUnit {
        PhysicalUnit {
            label: "m s⁻¹".into(),
            m: 1,
            kg: 0,
            s: -1,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    /// mm  (millimeters of water equivalent, common in hydrology)
    pub fn mm_water() -> PhysicalUnit {
        PhysicalUnit {
            label: "mm".into(),
            m: 1,
            kg: 0,
            s: 0,
            k: 0,
            mol: 0,
            scale: 0.001,
            offset: 0.0,
        }
    }

    /// mol m⁻² s⁻¹  (molar flux, for CO₂ etc.)
    pub fn molar_flux() -> PhysicalUnit {
        PhysicalUnit {
            label: "mol m⁻² s⁻¹".into(),
            m: -2,
            kg: 0,
            s: -1,
            k: 0,
            mol: 1,
            scale: 1.0,
            offset: 0.0,
        }
    }

    /// kg C m⁻²  (carbon stock)
    pub fn carbon_stock() -> PhysicalUnit {
        PhysicalUnit {
            label: "kg C m⁻²".into(),
            m: -2,
            kg: 1,
            s: 0,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }

    /// Pa  (pressure)
    pub fn pascal() -> PhysicalUnit {
        PhysicalUnit {
            label: "Pa".into(),
            m: -1,
            kg: 1,
            s: -2,
            k: 0,
            mol: 0,
            scale: 1.0,
            offset: 0.0,
        }
    }
}
