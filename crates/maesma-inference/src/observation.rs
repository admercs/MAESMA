//! PhiSat-2 autonomous observation intelligence.
//!
//! Implements on-board AI principles inspired by ESA's PhiSat-2 mission:
//! - Scene classification (cloud, fire, flood, vegetation stress)
//! - Observation value scoring against model uncertainty
//! - Compressed downlink optimization
//! - Active sensor tasking for maximum information gain
//! - Edge-AI inference with INT8 quantization support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Scene Classification ─────────────────────────────────────────────

/// Scene classification results from an on-board AI classifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneClassification {
    /// Unique scene identifier.
    pub scene_id: String,
    /// Acquisition timestamp (ISO 8601).
    pub acquisition_time: String,
    /// Spatial footprint (min_lat, min_lon, max_lat, max_lon).
    pub footprint: (f64, f64, f64, f64),
    /// Cloud cover fraction [0, 1].
    pub cloud_fraction: f64,
    /// Per-class probabilities.
    pub class_probabilities: HashMap<String, f64>,
    /// Overall usability score [0, 1].
    pub usability_score: f64,
    /// Whether the scene passed the on-board filter.
    pub accepted: bool,
    /// Classifier model version.
    pub classifier_version: String,
    /// Inference latency on edge device (ms).
    pub inference_latency_ms: f64,
}

/// Supported on-board detection products.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionProduct {
    CloudMask,
    FireHotspot,
    FloodExtent,
    VegetationStress,
    LandCoverChange,
    SnowIceMask,
    WaterQuality,
}

impl DetectionProduct {
    pub fn all() -> &'static [Self] {
        &[
            Self::CloudMask,
            Self::FireHotspot,
            Self::FloodExtent,
            Self::VegetationStress,
            Self::LandCoverChange,
            Self::SnowIceMask,
            Self::WaterQuality,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::CloudMask => "Cloud Mask",
            Self::FireHotspot => "Fire Hotspot",
            Self::FloodExtent => "Flood Extent",
            Self::VegetationStress => "Vegetation Stress",
            Self::LandCoverChange => "Land Cover Change",
            Self::SnowIceMask => "Snow/Ice Mask",
            Self::WaterQuality => "Water Quality",
        }
    }
}

// ── Observation Value Map ────────────────────────────────────────────

/// An entry in the observation value map.
///
/// Tracks the information value of potential observations at each spatial
/// location, keyed to model uncertainty and data sparsity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueMapEntry {
    /// Grid cell or region identifier.
    pub cell: (usize, usize),
    /// Target process family.
    pub target_family: String,
    /// Model uncertainty (std dev of ensemble spread).
    pub model_uncertainty: f64,
    /// Information gain score [0, 1].
    pub information_gain: f64,
    /// Hours since last usable observation.
    pub observation_age_hours: f64,
    /// Recommended sensor product.
    pub recommended_product: DetectionProduct,
    /// Priority rank (0 = highest).
    pub priority: u8,
}

/// The observation value map — spatial grid of information values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObservationValueMap {
    /// Entries indexed by cell.
    pub entries: Vec<ValueMapEntry>,
    /// Last update timestamp.
    pub last_updated: Option<String>,
}

impl ObservationValueMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or update an entry.
    pub fn upsert(&mut self, entry: ValueMapEntry) {
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|e| e.cell == entry.cell && e.target_family == entry.target_family)
        {
            *existing = entry;
        } else {
            self.entries.push(entry);
        }
    }

    /// Get entries sorted by information gain (descending).
    pub fn top_priority(&self, n: usize) -> Vec<&ValueMapEntry> {
        let mut sorted: Vec<&ValueMapEntry> = self.entries.iter().collect();
        sorted.sort_by(|a, b| {
            b.information_gain
                .partial_cmp(&a.information_gain)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted.truncate(n);
        sorted
    }

    /// Total number of cells tracked.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ── Active Tasking ───────────────────────────────────────────────────

/// An active sensor tasking request to a satellite or observation platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskingRequest {
    /// Request ID.
    pub request_id: String,
    /// Target spatial bounds.
    pub bounds: (f64, f64, f64, f64),
    /// Requested product.
    pub product: DetectionProduct,
    /// Priority (0 = highest).
    pub priority: u8,
    /// Justification for provenance tracking.
    pub justification: String,
    /// Expected information gain.
    pub expected_gain: f64,
    /// Status of the request.
    pub status: TaskingStatus,
    /// Requested acquisition window (start, end) as ISO 8601.
    pub window: Option<(String, String)>,
}

/// Status of a tasking request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskingStatus {
    Pending,
    Accepted,
    Scheduled,
    Acquired,
    Failed,
    Cancelled,
}

// ── Edge Deployment Config ───────────────────────────────────────────

/// Configuration for on-board / edge AI deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDeployConfig {
    /// Target hardware accelerator.
    pub accelerator: EdgeAccelerator,
    /// INT8 quantization enabled.
    pub int8_quantization: bool,
    /// Maximum inference latency budget (ms).
    pub max_latency_ms: f64,
    /// Maximum model size (MiB).
    pub max_model_size_mib: u64,
    /// Downlink bandwidth budget (Mbps).
    pub downlink_bandwidth_mbps: f64,
    /// Compression ratio for filtered downlink.
    pub compression_ratio: f64,
}

impl Default for EdgeDeployConfig {
    fn default() -> Self {
        Self {
            accelerator: EdgeAccelerator::IntelMovidiusVpu,
            int8_quantization: true,
            max_latency_ms: 100.0,
            max_model_size_mib: 64,
            downlink_bandwidth_mbps: 100.0,
            compression_ratio: 10.0,
        }
    }
}

/// Edge hardware accelerators for on-board inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeAccelerator {
    /// Intel Movidius VPU (as used on PhiSat-2).
    IntelMovidiusVpu,
    /// NVIDIA Jetson (Orin, AGX).
    NvidiaJetson,
    /// Google Coral Edge TPU.
    CoralEdgeTpu,
    /// Qualcomm Hexagon NPU.
    QualcommHexagon,
    /// Generic CPU (fallback).
    Cpu,
}

impl EdgeAccelerator {
    pub fn label(&self) -> &'static str {
        match self {
            Self::IntelMovidiusVpu => "Intel Movidius VPU",
            Self::NvidiaJetson => "NVIDIA Jetson",
            Self::CoralEdgeTpu => "Google Coral Edge TPU",
            Self::QualcommHexagon => "Qualcomm Hexagon NPU",
            Self::Cpu => "CPU",
        }
    }
}

// ── Observation Pipeline ─────────────────────────────────────────────

/// Full observation intelligence pipeline summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationPipelineSummary {
    /// Total scenes processed.
    pub scenes_processed: u64,
    /// Scenes accepted (passed filter).
    pub scenes_accepted: u64,
    /// Scenes rejected (failed filter).
    pub scenes_rejected: u64,
    /// Acceptance rate.
    pub acceptance_rate: f64,
    /// Active tasking requests.
    pub active_taskings: usize,
    /// Total data downlinked (GiB).
    pub data_downlinked_gib: f64,
    /// Data saved by filtering (GiB).
    pub data_saved_gib: f64,
    /// Bandwidth savings ratio.
    pub bandwidth_savings_ratio: f64,
}
