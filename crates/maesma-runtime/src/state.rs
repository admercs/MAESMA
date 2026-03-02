//! Simulation state — manages the gridded state arrays.

use ndarray::Array2;
use std::collections::HashMap;

/// The simulation state holds all field variables as named 2D arrays.
pub struct SimulationState {
    /// Named field variables (e.g., "soil_moisture", "temperature").
    fields: HashMap<String, Array2<f64>>,
    /// Named parameters (scalar or small arrays).
    params: HashMap<String, f64>,
    /// Current simulation time (seconds).
    time: f64,
    /// Grid dimensions.
    pub nx: usize,
    pub ny: usize,
}

impl SimulationState {
    /// Create a new state with given grid dimensions.
    pub fn new(nx: usize, ny: usize) -> Self {
        Self {
            fields: HashMap::new(),
            params: HashMap::new(),
            time: 0.0,
            nx,
            ny,
        }
    }

    /// Initialize a field with zeros.
    pub fn init_field(&mut self, name: impl Into<String>) {
        self.fields
            .insert(name.into(), Array2::zeros((self.nx, self.ny)));
    }

    /// Initialize a field with a constant value.
    pub fn init_field_const(&mut self, name: impl Into<String>, value: f64) {
        self.fields
            .insert(name.into(), Array2::from_elem((self.nx, self.ny), value));
    }

    /// Get a field by name.
    pub fn field(&self, name: &str) -> Option<&Array2<f64>> {
        self.fields.get(name)
    }

    /// Get a mutable field by name.
    pub fn field_mut(&mut self, name: &str) -> Option<&mut Array2<f64>> {
        self.fields.get_mut(name)
    }

    /// Set a scalar parameter.
    pub fn set_param(&mut self, name: impl Into<String>, value: f64) {
        self.params.insert(name.into(), value);
    }

    /// Get a scalar parameter.
    pub fn param(&self, name: &str) -> Option<f64> {
        self.params.get(name).copied()
    }

    /// Get current simulation time.
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Advance the time.
    pub fn advance_time(&mut self, dt: f64) {
        self.time += dt;
    }

    /// List all field names.
    pub fn field_names(&self) -> Vec<&str> {
        self.fields.keys().map(|s| s.as_str()).collect()
    }

    /// Check if any field contains NaN values.
    pub fn has_nan(&self) -> bool {
        self.fields.values().any(|arr| arr.iter().any(|v| v.is_nan()))
    }

    /// Check if any field contains infinite values.
    pub fn has_inf(&self) -> bool {
        self.fields
            .values()
            .any(|arr| arr.iter().any(|v| v.is_infinite()))
    }
}
