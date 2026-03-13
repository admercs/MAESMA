//! Data contracts — schema for inter-process data exchange guarantees.
//!
//! A data contract specifies what data a process produces or consumes,
//! including variable names, shapes, units, update frequency, and
//! quality constraints.  The runtime validates contracts at schedule
//! compilation time to catch coupling mismatches early.

use serde::{Deserialize, Serialize};

use crate::families::ProcessFamily;
use crate::process::FidelityRung;
use crate::units::PhysicalUnit;
use crate::variables::VariableCategory;

/// A field specification within a data contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSpec {
    /// Variable name (must match `VariableRegistry` canonical names).
    pub name: String,
    /// Physical unit.
    pub unit: PhysicalUnit,
    /// Variable category.
    pub category: VariableCategory,
    /// Expected dimensionality (e.g., 1 for scalar, 2 for grid, 3 for 3-D).
    pub ndim: u8,
    /// Whether this field must be present (vs. optional).
    pub required: bool,
    /// Physical lower bound, if any.
    pub lower_bound: Option<f64>,
    /// Physical upper bound, if any.
    pub upper_bound: Option<f64>,
    /// Expected fill / missing-data value.
    pub fill_value: Option<f64>,
}

/// A data contract for a process representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataContract {
    /// Process family.
    pub family: ProcessFamily,
    /// Fidelity rung this contract applies to.
    pub rung: FidelityRung,
    /// Human-readable name (e.g., "RichardsInfiltration-R1").
    pub process_name: String,
    /// Version of this contract.
    pub version: String,
    /// Fields this process reads (inputs).
    pub inputs: Vec<FieldSpec>,
    /// Fields this process writes (outputs).
    pub outputs: Vec<FieldSpec>,
    /// Conserved quantities this process tracks.
    pub conserved: Vec<ConservedQuantity>,
    /// Temporal coupling constraints.
    pub temporal: TemporalConstraint,
}

/// A quantity that must be conserved across a time step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservedQuantity {
    /// Name of the conserved integral (e.g., "total_water", "total_energy").
    pub name: String,
    /// Unit of the conserved quantity.
    pub unit: PhysicalUnit,
    /// Maximum acceptable residual (absolute).
    pub tolerance: f64,
}

/// Temporal coupling constraints for a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConstraint {
    /// Maximum stable time step (seconds).
    pub max_dt: f64,
    /// Minimum meaningful time step (seconds).
    pub min_dt: f64,
    /// Whether the process supports subcycling.
    pub supports_subcycling: bool,
    /// Preferred coupling interval (seconds).
    pub preferred_dt: f64,
}

/// Result of validating two contracts for compatibility.
#[derive(Debug, Clone)]
pub enum ContractValidation {
    /// Contracts are fully compatible.
    Compatible,
    /// Warning: non-fatal mismatch.
    Warning(Vec<String>),
    /// Error: incompatible contracts.
    Incompatible(Vec<String>),
}

/// Validate that a producer contract provides all fields a consumer needs.
pub fn validate_coupling(producer: &DataContract, consumer: &DataContract) -> ContractValidation {
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    for input in &consumer.inputs {
        match producer.outputs.iter().find(|o| o.name == input.name) {
            None => {
                errors.push(format!(
                    "Consumer '{}' requires '{}' but producer '{}' does not output it",
                    consumer.process_name, input.name, producer.process_name
                ));
            }
            Some(output) => {
                // Check unit compatibility
                if output.unit != input.unit {
                    errors.push(format!(
                        "Unit mismatch for '{}': producer={:?} consumer={:?}",
                        input.name, output.unit, input.unit
                    ));
                }
                // Check dimensionality
                if output.ndim != input.ndim {
                    warnings.push(format!(
                        "Dimension mismatch for '{}': producer={}D consumer={}D",
                        input.name, output.ndim, input.ndim
                    ));
                }
            }
        }
    }

    if !errors.is_empty() {
        ContractValidation::Incompatible(errors)
    } else if !warnings.is_empty() {
        ContractValidation::Warning(warnings)
    } else {
        ContractValidation::Compatible
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn field(name: &str, ndim: u8) -> FieldSpec {
        FieldSpec {
            name: name.to_string(),
            unit: PhysicalUnit::dimensionless(),
            category: VariableCategory::Forcing,
            ndim,
            required: true,
            lower_bound: None,
            upper_bound: None,
            fill_value: None,
        }
    }

    fn temporal() -> TemporalConstraint {
        TemporalConstraint {
            max_dt: 3600.0,
            min_dt: 1.0,
            supports_subcycling: true,
            preferred_dt: 1800.0,
        }
    }

    #[test]
    fn test_compatible_coupling() {
        let producer = DataContract {
            family: ProcessFamily::Atmosphere,
            rung: FidelityRung::R0,
            process_name: "BulkAtmo".into(),
            version: "1.0".into(),
            inputs: vec![],
            outputs: vec![field("air_temperature", 2), field("precipitation", 2)],
            conserved: vec![],
            temporal: temporal(),
        };

        let consumer = DataContract {
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R1,
            process_name: "Richards".into(),
            version: "1.0".into(),
            inputs: vec![field("air_temperature", 2), field("precipitation", 2)],
            outputs: vec![field("soil_moisture", 2)],
            conserved: vec![],
            temporal: temporal(),
        };

        assert!(matches!(
            validate_coupling(&producer, &consumer),
            ContractValidation::Compatible
        ));
    }

    #[test]
    fn test_missing_field_incompatible() {
        let producer = DataContract {
            family: ProcessFamily::Atmosphere,
            rung: FidelityRung::R0,
            process_name: "BulkAtmo".into(),
            version: "1.0".into(),
            inputs: vec![],
            outputs: vec![field("air_temperature", 2)],
            conserved: vec![],
            temporal: temporal(),
        };

        let consumer = DataContract {
            family: ProcessFamily::Hydrology,
            rung: FidelityRung::R1,
            process_name: "Richards".into(),
            version: "1.0".into(),
            inputs: vec![field("air_temperature", 2), field("precipitation", 2)],
            outputs: vec![],
            conserved: vec![],
            temporal: temporal(),
        };

        match validate_coupling(&producer, &consumer) {
            ContractValidation::Incompatible(errors) => {
                assert_eq!(errors.len(), 1);
                assert!(errors[0].contains("precipitation"));
            }
            _ => panic!("Expected incompatible"),
        }
    }

    #[test]
    fn test_dimension_mismatch_warning() {
        let producer = DataContract {
            family: ProcessFamily::Atmosphere,
            rung: FidelityRung::R0,
            process_name: "Atmo".into(),
            version: "1.0".into(),
            inputs: vec![],
            outputs: vec![field("temperature", 3)],
            conserved: vec![],
            temporal: temporal(),
        };

        let consumer = DataContract {
            family: ProcessFamily::Ecology,
            rung: FidelityRung::R0,
            process_name: "Eco".into(),
            version: "1.0".into(),
            inputs: vec![field("temperature", 2)],
            outputs: vec![],
            conserved: vec![],
            temporal: temporal(),
        };

        assert!(matches!(
            validate_coupling(&producer, &consumer),
            ContractValidation::Warning(_)
        ));
    }
}
