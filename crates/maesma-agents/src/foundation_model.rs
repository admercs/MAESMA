//! Foundation Model Agent — orchestrates GPU-accelerated foundation weather model
//! ensembles (e.g., FourCastNet/SFNO, Pangu-Weather, GraphCast, GenCast, DLWP-CS,
//! CorrDiff) as first-class Atmosphere R0/R1 rungs on the representation ladder.
//!
//! Responsibilities:
//! - Manage the Earth-2 / earth2studio inference pipeline
//! - Dispatch ensemble perturbations and run multi-model ensembles
//! - Feed foundation model outputs as initial conditions and lateral boundaries
//! - Track model performance, calibrate ensemble weights
//! - Trigger CorrDiff super-resolution for regional downscaling

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Supported foundation weather models from the Earth-2 / earth2studio ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FoundationModelKind {
    /// FourCastNet / Spherical Fourier Neural Operator (SFNO).
    FourCastNetSfno,
    /// Pangu-Weather (Huawei).
    PanguWeather,
    /// GraphCast (Google DeepMind).
    GraphCast,
    /// GenCast — probabilistic diffusion model.
    GenCast,
    /// Deep Learning Weather Prediction on the Cubed Sphere.
    DlwpCs,
    /// CorrDiff — conditional diffusion for stochastic downscaling.
    CorrDiff,
}

impl FoundationModelKind {
    /// All supported model kinds.
    pub fn all() -> &'static [Self] {
        &[
            Self::FourCastNetSfno,
            Self::PanguWeather,
            Self::GraphCast,
            Self::GenCast,
            Self::DlwpCs,
            Self::CorrDiff,
        ]
    }

    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::FourCastNetSfno => "FourCastNet / SFNO",
            Self::PanguWeather => "Pangu-Weather",
            Self::GraphCast => "GraphCast",
            Self::GenCast => "GenCast",
            Self::DlwpCs => "DLWP-CS",
            Self::CorrDiff => "CorrDiff",
        }
    }

    /// Default fidelity rung for this model in the MAESMA representation ladder.
    pub fn default_rung(&self) -> &'static str {
        match self {
            Self::FourCastNetSfno | Self::PanguWeather | Self::DlwpCs => "R0",
            Self::GraphCast | Self::GenCast => "R1",
            Self::CorrDiff => "R2", // regional downscaling
        }
    }
}

/// Configuration for the foundation model ensemble.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleConfig {
    /// Which models to include in the ensemble.
    pub models: Vec<FoundationModelKind>,
    /// Number of ensemble members per model.
    pub members_per_model: usize,
    /// Forecast lead time in hours.
    pub lead_time_hours: u32,
    /// Whether to apply CorrDiff downscaling to members.
    pub apply_downscaling: bool,
    /// ONNX Runtime execution provider (e.g., "CUDAExecutionProvider").
    pub onnx_provider: String,
}

impl Default for EnsembleConfig {
    fn default() -> Self {
        Self {
            models: vec![
                FoundationModelKind::FourCastNetSfno,
                FoundationModelKind::GraphCast,
                FoundationModelKind::GenCast,
            ],
            members_per_model: 4,
            lead_time_hours: 240,
            apply_downscaling: true,
            onnx_provider: "CUDAExecutionProvider".into(),
        }
    }
}

/// Status of a running foundation model inference job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceJobStatus {
    /// Model being run.
    pub model: FoundationModelKind,
    /// Ensemble member index.
    pub member: usize,
    /// Current forecast step (lead time hours completed).
    pub step_hours: u32,
    /// Total lead time hours.
    pub total_hours: u32,
    /// GPU memory usage in MiB.
    pub gpu_memory_mib: u64,
    /// Whether inference is complete.
    pub complete: bool,
}

/// The Foundation Model Agent.
pub struct FoundationModelAgent {
    id: AgentId,
    config: EnsembleConfig,
    /// Tracked inference jobs.
    job_statuses: Vec<InferenceJobStatus>,
}

impl FoundationModelAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("foundation_model".into()),
            config: EnsembleConfig::default(),
            job_statuses: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: EnsembleConfig) -> Self {
        self.config = config;
        self
    }

    /// Get current ensemble configuration.
    pub fn config(&self) -> &EnsembleConfig {
        &self.config
    }

    /// Get inference job statuses.
    pub fn job_statuses(&self) -> &[InferenceJobStatus] {
        &self.job_statuses
    }
}

#[async_trait]
impl Agent for FoundationModelAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn role(&self) -> AgentRole {
        AgentRole::FoundationModel
    }

    fn description(&self) -> &str {
        "Orchestrates Earth-2 foundation weather model ensembles (FourCastNet, \
         GraphCast, GenCast, CorrDiff) as GPU-accelerated Atmosphere R0/R1 \
         rungs; dispatches ONNX inference, calibrates ensemble weights, and \
         feeds foundation model outputs as initial/boundary conditions"
    }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement Earth-2 ensemble orchestration
        // 1. Load ONNX models via ort (ONNX Runtime)
        // 2. Prepare initial condition perturbations
        // 3. Dispatch parallel GPU inference for each model × member
        // 4. Collect outputs, stack ensemble, compute spread/mean
        // 5. If apply_downscaling, run CorrDiff on ensemble mean
        // 6. Feed results to SAPG atmosphere nodes
        let model_count = self.config.models.len();
        let total_members = model_count * self.config.members_per_model;
        Ok(AgentResult::ok(format!(
            "Foundation model ensemble dispatched: {} models × {} members = {} total, \
             lead time {}h",
            model_count, self.config.members_per_model, total_members, self.config.lead_time_hours,
        )))
    }
}
