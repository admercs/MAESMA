//! Earth-2 / earth2studio foundation weather model integration.
//!
//! Provides trait-based abstractions for running GPU-accelerated foundation
//! weather models (FourCastNet/SFNO, Pangu-Weather, GraphCast, GenCast,
//! DLWP-CS, CorrDiff) as first-class MAESMA inference backends.
//!
//! Architecture:
//! - `FoundationModelRunner` trait for model-agnostic forecast dispatch
//! - `EnsembleOrchestrator` for multi-model ensemble generation
//! - `DownscalingPipeline` for CorrDiff conditional diffusion downscaling
//! - ONNX Runtime backend via ort crate (planned)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Weather Variable Grid ────────────────────────────────────────────

/// A single weather variable on a lat-lon or cubed-sphere grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherField {
    /// Variable name (e.g., "geopotential_500", "temperature_850").
    pub name: String,
    /// Pressure level in hPa (None for surface variables).
    pub level_hpa: Option<u32>,
    /// Grid data as a flat array (row-major).
    pub data: Vec<f64>,
    /// Grid dimensions (nlat, nlon) or (nface, nx, ny).
    pub shape: Vec<usize>,
    /// Units string.
    pub units: String,
}

/// A snapshot of the atmosphere at a single time step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereState {
    /// Valid time (ISO 8601).
    pub valid_time: String,
    /// Lead time from initialization (hours).
    pub lead_hours: u32,
    /// Fields keyed by variable name.
    pub fields: HashMap<String, WeatherField>,
}

impl AtmosphereState {
    pub fn new(valid_time: impl Into<String>, lead_hours: u32) -> Self {
        Self {
            valid_time: valid_time.into(),
            lead_hours,
            fields: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, field: WeatherField) {
        self.fields.insert(field.name.clone(), field);
    }

    /// Number of variables.
    pub fn num_variables(&self) -> usize {
        self.fields.len()
    }
}

// ── Foundation Model Runner ──────────────────────────────────────────

/// Metadata about a foundation weather model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name (e.g., "FourCastNet-SFNO").
    pub name: String,
    /// Model version.
    pub version: String,
    /// Spatial resolution in degrees.
    pub resolution_deg: f64,
    /// Number of atmospheric variables.
    pub num_variables: usize,
    /// Number of pressure levels.
    pub num_levels: usize,
    /// Forecast time step in hours.
    pub dt_hours: u32,
    /// Maximum lead time in hours.
    pub max_lead_hours: u32,
    /// ONNX model file size in MiB.
    pub model_size_mib: u64,
    /// Estimated GPU memory requirement in MiB.
    pub gpu_memory_mib: u64,
    /// Whether the model produces probabilistic outputs.
    pub probabilistic: bool,
}

/// Trait that all foundation weather models implement.
#[async_trait]
pub trait FoundationModelRunner: Send + Sync {
    /// Get model metadata.
    fn metadata(&self) -> &ModelMetadata;

    /// Initialize the model on a specific GPU device.
    async fn init(&mut self, device_id: u32) -> Result<()>;

    /// Run a single forecast step from the given initial condition.
    async fn step(&self, initial_condition: &AtmosphereState) -> Result<AtmosphereState>;

    /// Run a full rollout from initialization to max_lead_hours.
    async fn rollout(
        &self,
        initial_condition: &AtmosphereState,
        max_lead_hours: u32,
    ) -> Result<Vec<AtmosphereState>> {
        let mut states = Vec::new();
        let mut current = initial_condition.clone();
        let dt = self.metadata().dt_hours;
        let mut elapsed = 0u32;

        while elapsed < max_lead_hours {
            current = self.step(&current).await?;
            elapsed += dt;
            states.push(current.clone());
        }

        Ok(states)
    }

    /// Whether the model is loaded and ready.
    fn is_ready(&self) -> bool;
}

// ── Ensemble Orchestrator ────────────────────────────────────────────

/// Perturbation method for generating ensemble initial conditions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerturbationMethod {
    /// Bred vectors from NWP data assimilation.
    BredVectors,
    /// Lagged-average forecasting.
    LaggedAverage,
    /// Random Gaussian perturbations.
    GaussianNoise,
    /// Singular vectors.
    SingularVectors,
}

/// Ensemble member result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleMember {
    /// Member index.
    pub index: usize,
    /// Which model produced this member.
    pub model_name: String,
    /// Perturbation method used.
    pub perturbation: PerturbationMethod,
    /// Forecast trajectory.
    pub trajectory: Vec<AtmosphereState>,
}

/// Ensemble statistics at a single lead time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleStatistics {
    /// Lead time (hours).
    pub lead_hours: u32,
    /// Ensemble mean for each variable.
    pub mean: HashMap<String, Vec<f64>>,
    /// Ensemble spread (std dev) for each variable.
    pub spread: HashMap<String, Vec<f64>>,
    /// Number of members.
    pub num_members: usize,
}

/// Orchestrates multi-model ensemble forecasts.
pub struct EnsembleOrchestrator {
    /// Registered models.
    models: Vec<Box<dyn FoundationModelRunner>>,
    /// Perturbation method to use.
    pub perturbation_method: PerturbationMethod,
    /// Members per model.
    pub members_per_model: usize,
}

impl EnsembleOrchestrator {
    pub fn new(perturbation_method: PerturbationMethod, members_per_model: usize) -> Self {
        Self {
            models: Vec::new(),
            perturbation_method,
            members_per_model,
        }
    }

    /// Register a foundation model.
    pub fn add_model(&mut self, model: Box<dyn FoundationModelRunner>) {
        self.models.push(model);
    }

    /// Number of registered models.
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Total ensemble size across all models.
    pub fn total_members(&self) -> usize {
        self.models.len() * self.members_per_model
    }

    /// Get metadata for all registered models.
    pub fn model_metadata(&self) -> Vec<&ModelMetadata> {
        self.models.iter().map(|m| m.metadata()).collect()
    }
}

// ── Downscaling Pipeline ─────────────────────────────────────────────

/// CorrDiff-style conditional diffusion downscaling configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownscalingConfig {
    /// Source resolution in degrees.
    pub source_resolution_deg: f64,
    /// Target resolution in degrees.
    pub target_resolution_deg: f64,
    /// Number of diffusion steps.
    pub diffusion_steps: u32,
    /// Number of stochastic samples.
    pub num_samples: usize,
    /// Conditioning variables.
    pub conditioning_variables: Vec<String>,
}

impl Default for DownscalingConfig {
    fn default() -> Self {
        Self {
            source_resolution_deg: 0.25,
            target_resolution_deg: 0.03,
            diffusion_steps: 50,
            num_samples: 8,
            conditioning_variables: vec![
                "geopotential_500".into(),
                "temperature_850".into(),
                "specific_humidity_700".into(),
                "u_wind_250".into(),
                "v_wind_250".into(),
            ],
        }
    }
}

// ── Stub Implementation ──────────────────────────────────────────────

/// Stub foundation model for testing (produces constant fields).
pub struct StubFoundationModel {
    meta: ModelMetadata,
    ready: bool,
}

impl StubFoundationModel {
    pub fn fourcastnet_stub() -> Self {
        Self {
            meta: ModelMetadata {
                name: "FourCastNet-SFNO-stub".into(),
                version: "0.1.0".into(),
                resolution_deg: 0.25,
                num_variables: 73,
                num_levels: 13,
                dt_hours: 6,
                max_lead_hours: 240,
                model_size_mib: 850,
                gpu_memory_mib: 4096,
                probabilistic: false,
            },
            ready: true,
        }
    }

    pub fn graphcast_stub() -> Self {
        Self {
            meta: ModelMetadata {
                name: "GraphCast-stub".into(),
                version: "0.1.0".into(),
                resolution_deg: 0.25,
                num_variables: 227,
                num_levels: 37,
                dt_hours: 6,
                max_lead_hours: 240,
                model_size_mib: 1200,
                gpu_memory_mib: 8192,
                probabilistic: false,
            },
            ready: true,
        }
    }

    pub fn gencast_stub() -> Self {
        Self {
            meta: ModelMetadata {
                name: "GenCast-stub".into(),
                version: "0.1.0".into(),
                resolution_deg: 0.25,
                num_variables: 227,
                num_levels: 37,
                dt_hours: 12,
                max_lead_hours: 360,
                model_size_mib: 1500,
                gpu_memory_mib: 12288,
                probabilistic: true,
            },
            ready: true,
        }
    }
}

#[async_trait]
impl FoundationModelRunner for StubFoundationModel {
    fn metadata(&self) -> &ModelMetadata {
        &self.meta
    }

    async fn init(&mut self, _device_id: u32) -> Result<()> {
        self.ready = true;
        Ok(())
    }

    async fn step(&self, initial_condition: &AtmosphereState) -> Result<AtmosphereState> {
        let mut next = AtmosphereState::new(
            &initial_condition.valid_time,
            initial_condition.lead_hours + self.meta.dt_hours,
        );
        // Copy fields with stub values
        for (name, field) in &initial_condition.fields {
            next.add_field(WeatherField {
                name: name.clone(),
                level_hpa: field.level_hpa,
                data: field.data.clone(),
                shape: field.shape.clone(),
                units: field.units.clone(),
            });
        }
        Ok(next)
    }

    fn is_ready(&self) -> bool {
        self.ready
    }
}

/// Result type alias for this module.
pub type Result<T> = std::result::Result<T, maesma_core::Error>;
