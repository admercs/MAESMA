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

/// A heuristic inference engine that uses rule-based scoring.
///
/// Provides meaningful inference without requiring a trained neural network.
/// Scoring is based on:
/// - Family coverage (how many process families are represented)
/// - Rung diversity (spread across fidelity levels)
/// - I/O compatibility (input/output overlap between coupled processes)
/// - Cost efficiency (skill per unit cost)
pub struct HeuristicInferenceEngine;

#[async_trait]
impl InferenceEngine for HeuristicInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> maesma_core::Result<InferenceResponse> {
        let start = std::time::Instant::now();

        let scores = match request.task {
            InferenceTask::PredictSkill => self.predict_skill(&request),
            InferenceTask::CompatibilityScore => self.compatibility_score(&request),
            InferenceTask::RankAssemblies => self.rank_assemblies(&request),
            InferenceTask::PredictRegime => self.predict_regime(&request),
            InferenceTask::SuggestAction => self.suggest_action(&request),
        };

        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
        let confidence = if scores.is_empty() {
            0.0
        } else {
            // Confidence based on feature richness and score spread
            let mean = scores.iter().sum::<f64>() / scores.len() as f64;
            let variance =
                scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / scores.len() as f64;
            // Higher confidence when scores are well-separated (high variance)
            (1.0 - (-variance * 5.0).exp()).clamp(0.1, 0.95)
        };

        Ok(InferenceResponse {
            scores,
            confidence,
            latency_ms,
            model_version: "heuristic-1.0.0".into(),
        })
    }

    fn is_ready(&self) -> bool {
        true
    }

    fn model_version(&self) -> &str {
        "heuristic-1.0.0"
    }
}

impl HeuristicInferenceEngine {
    /// Predict skill scores from process features.
    ///
    /// Features expected: [family_code, rung, n_inputs, n_outputs, cost, ...]
    fn predict_skill(&self, request: &InferenceRequest) -> Vec<f64> {
        let features = &request.features;
        if features.len() < 3 {
            return vec![0.5];
        }

        let rung = features.get(1).copied().unwrap_or(1.0);
        let n_inputs = features.get(2).copied().unwrap_or(3.0);
        let n_outputs = features.get(3).copied().unwrap_or(2.0);
        let cost = features.get(4).copied().unwrap_or(1.0);

        // Higher rung → generally higher skill
        let rung_bonus = rung * 0.15;
        // More I/O variables → more complex, potentially higher skill
        let io_bonus = ((n_inputs + n_outputs) / 20.0).min(0.2);
        // Cost penalty (very expensive models may not be worth it)
        let cost_penalty = (cost / 100.0).min(0.3);

        let predicted_rmse = (0.5 - rung_bonus - io_bonus + cost_penalty * 0.1).clamp(0.01, 2.0);
        let predicted_kge = (0.3 + rung_bonus + io_bonus).clamp(0.0, 1.0);
        let predicted_nse = (0.2 + rung_bonus * 0.8 + io_bonus * 0.5).clamp(-1.0, 1.0);

        vec![predicted_rmse, predicted_kge, predicted_nse]
    }

    /// Score compatibility between two processes.
    ///
    /// Features: [family_a, rung_a, n_outputs_a, family_b, rung_b, n_inputs_b, shared_vars]
    fn compatibility_score(&self, request: &InferenceRequest) -> Vec<f64> {
        let f = &request.features;
        if f.len() < 7 {
            return vec![0.5];
        }

        let family_a = f[0] as usize;
        let family_b = f[3] as usize;
        let shared_vars = f[6];

        // Same family → low compatibility (redundant)
        let family_score = if family_a == family_b { 0.1 } else { 0.7 };
        // More shared variables → higher compatibility
        let io_overlap = (shared_vars / 10.0).min(1.0) * 0.3;

        vec![(family_score + io_overlap).clamp(0.0, 1.0)]
    }

    /// Rank candidate SAPG assemblies.
    ///
    /// Features grouped in chunks: each chunk = [n_families, n_processes, total_cost, avg_rung]
    fn rank_assemblies(&self, request: &InferenceRequest) -> Vec<f64> {
        let chunk_size = 4;
        let f = &request.features;
        if f.len() < chunk_size {
            return vec![0.5];
        }

        f.chunks(chunk_size)
            .map(|chunk| {
                let n_families = chunk.first().copied().unwrap_or(1.0);
                let n_processes = chunk.get(1).copied().unwrap_or(1.0);
                let total_cost = chunk.get(2).copied().unwrap_or(10.0);
                let avg_rung = chunk.get(3).copied().unwrap_or(1.0);

                // Reward family coverage (out of 13)
                let coverage = (n_families / 13.0).min(1.0) * 0.4;
                // Reward moderate process count
                let process_score = (1.0 - (n_processes - 13.0).abs() / 20.0).max(0.0) * 0.2;
                // Penalize high cost
                let cost_score = (1.0 - total_cost / 1000.0).max(0.0) * 0.2;
                // Reward higher average rung
                let rung_score = (avg_rung / 3.0).min(1.0) * 0.2;

                (coverage + process_score + cost_score + rung_score).clamp(0.0, 1.0)
            })
            .collect()
    }

    /// Predict environmental regime from state variables.
    ///
    /// Features: [temperature, precipitation, soil_moisture, wind_speed, ...]
    fn predict_regime(&self, request: &InferenceRequest) -> Vec<f64> {
        let f = &request.features;
        let temp = f.first().copied().unwrap_or(288.0);
        let precip = f.get(1).copied().unwrap_or(2.0);
        let soil_moisture = f.get(2).copied().unwrap_or(0.3);

        // Simple regime probabilities: [normal, drought, fire, flood]
        let drought_signal = if soil_moisture < 0.15 && precip < 0.5 {
            0.7
        } else {
            0.1
        };
        let fire_signal = if temp > 310.0 && soil_moisture < 0.1 {
            0.8
        } else {
            0.05
        };
        let flood_signal = if precip > 20.0 && soil_moisture > 0.8 {
            0.7
        } else {
            0.05
        };
        let normal = (1.0_f64 - drought_signal - fire_signal - flood_signal).max(0.0);

        vec![normal, drought_signal, fire_signal, flood_signal]
    }

    /// Suggest next assembly action.
    ///
    /// Features: [current_rmse, current_kge, n_families, n_processes, generation]
    fn suggest_action(&self, request: &InferenceRequest) -> Vec<f64> {
        let f = &request.features;
        let rmse = f.first().copied().unwrap_or(1.0);
        let kge = f.get(1).copied().unwrap_or(0.3);
        let n_families = f.get(2).copied().unwrap_or(5.0);

        // Action probabilities: [add_process, swap_rung, remove_process, discover_new, keep]
        let needs_more = if n_families < 8.0 { 0.4 } else { 0.1 };
        let needs_upgrade = if kge < 0.5 && n_families >= 8.0 {
            0.3
        } else {
            0.1
        };
        let needs_trim = if rmse > 1.5 { 0.1 } else { 0.05 };
        let needs_discovery = if kge < 0.3 && n_families >= 10.0 {
            0.3
        } else {
            0.1
        };
        let keep = (1.0_f64 - needs_more - needs_upgrade - needs_trim - needs_discovery).max(0.05);

        vec![needs_more, needs_upgrade, needs_trim, needs_discovery, keep]
    }
}
