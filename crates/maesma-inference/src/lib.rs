//! Neural Inference Engine — graph-transformer interface for SAPG reasoning,
//! Earth-2 foundation weather model integration, and PhiSat-2 autonomous
//! observation intelligence.
//!
//! The inference engine provides a trait-based abstraction over neural
//! network backends (ONNX, TorchScript, custom graph transformers).
//! Agents query the engine for:
//! - Skill prediction (before running expensive benchmarks)
//! - Process compatibility scoring
//! - Assembly ranking
//! - Regime prediction from climate state
//! - Foundation weather model forecasts (Earth-2 / earth2studio)
//! - Autonomous observation filtering and tasking (PhiSat-2)

pub mod earth2;
pub mod observation;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Input to the inference engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    /// Task type.
    pub task: InferenceTask,
    /// Input features as a flat vector.
    pub features: Vec<f64>,
    /// Additional context key-value pairs.
    pub context: std::collections::HashMap<String, serde_json::Value>,
}

/// Types of inference tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InferenceTask {
    /// Predict skill scores for a candidate process representation.
    PredictSkill,
    /// Score compatibility between two process representations.
    CompatibilityScore,
    /// Rank candidate SAPG assemblies.
    RankAssemblies,
    /// Predict current environmental regime from state variables.
    PredictRegime,
    /// Suggest next assembly action.
    SuggestAction,
}

/// Output from the inference engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    /// Output scores/probabilities.
    pub scores: Vec<f64>,
    /// Confidence estimate.
    pub confidence: f64,
    /// Latency in milliseconds.
    pub latency_ms: f64,
    /// Model version used.
    pub model_version: String,
}

/// Trait for inference engine backends.
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    /// Run inference.
    async fn infer(&self, request: InferenceRequest) -> maesma_core::Result<InferenceResponse>;

    /// Check if the engine is ready.
    fn is_ready(&self) -> bool;

    /// Get the model version.
    fn model_version(&self) -> &str;
}

/// A stub inference engine that returns random scores (for testing).
pub struct StubInferenceEngine;

#[async_trait]
impl InferenceEngine for StubInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> maesma_core::Result<InferenceResponse> {
        Ok(InferenceResponse {
            scores: vec![0.5; request.features.len().max(1)],
            confidence: 0.0,
            latency_ms: 0.0,
            model_version: "stub-0.0.0".into(),
        })
    }

    fn is_ready(&self) -> bool {
        true
    }

    fn model_version(&self) -> &str {
        "stub-0.0.0"
    }
}
