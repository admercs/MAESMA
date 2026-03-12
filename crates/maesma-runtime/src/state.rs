//! Simulation state — manages the gridded state arrays.

use chrono::Utc;
use ndarray::{Array2, ArrayD, IxDyn};
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

/// Bridge implementation: SimulationState (2D arrays) implements ProcessState (dynamic arrays).
impl maesma_core::process::ProcessState for SimulationState {
    fn get_field(&self, name: &str) -> Option<&ArrayD<f64>> {
        // We cannot return a reference to a dynamically-shaped view of a 2D array
        // because ProcessState expects ArrayD. Instead, we store a conversion cache.
        // For now, return None — process runners should use the ProcessState2D adapter.
        let _ = name;
        None
    }

    fn get_field_mut(&mut self, name: &str) -> Option<&mut ArrayD<f64>> {
        let _ = name;
        None
    }

    fn get_param(&self, name: &str) -> Option<f64> {
        self.param(name)
    }

    fn time(&self) -> chrono::DateTime<Utc> {
        // Convert seconds-since-epoch to DateTime
        chrono::DateTime::from_timestamp(self.time as i64, 0).unwrap_or_default()
    }
}

/// A ProcessState adapter that wraps SimulationState, bridging Array2 to ArrayD.
pub struct ProcessStateAdapter<'a> {
    state: &'a mut SimulationState,
    /// Temporary dynamic-dimension views (owned, synced back on drop).
    cache: HashMap<String, ArrayD<f64>>,
}

impl<'a> ProcessStateAdapter<'a> {
    pub fn new(state: &'a mut SimulationState) -> Self {
        // Pre-convert all fields to ArrayD
        let cache = state
            .fields
            .iter()
            .map(|(k, v)| {
                let shape = IxDyn(&[v.nrows(), v.ncols()]);
                let arr_d = ArrayD::from_shape_vec(shape, v.iter().copied().collect()).unwrap();
                (k.clone(), arr_d)
            })
            .collect();
        Self { state, cache }
    }

    /// Write modified fields back to the SimulationState.
    pub fn sync_back(&mut self) {
        for (name, arr_d) in &self.cache {
            let nx = self.state.nx;
            let ny = self.state.ny;
            if let Some(field) = self.state.fields.get_mut(name) {
                let data: Vec<f64> = arr_d.iter().copied().collect();
                if let Ok(arr2) = Array2::from_shape_vec((nx, ny), data) {
                    *field = arr2;
                }
            }
        }
    }
}

impl maesma_core::process::ProcessState for ProcessStateAdapter<'_> {
    fn get_field(&self, name: &str) -> Option<&ArrayD<f64>> {
        self.cache.get(name)
    }

    fn get_field_mut(&mut self, name: &str) -> Option<&mut ArrayD<f64>> {
        self.cache.get_mut(name)
    }

    fn get_param(&self, name: &str) -> Option<f64> {
        self.state.param(name)
    }

    fn time(&self) -> chrono::DateTime<Utc> {
        chrono::DateTime::from_timestamp(self.state.time as i64, 0).unwrap_or_default()
    }
}
