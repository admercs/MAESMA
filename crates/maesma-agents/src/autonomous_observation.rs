//! Autonomous Observation Agent — applies PhiSat-2 on-board AI principles to
//! autonomously filter, prioritize, and task observational data streams.
//!
//! Inspired by ESA's PhiSat-2 mission, this agent implements:
//! - On-board cloud/fire/flood detection to filter low-value imagery
//! - Observation value scoring based on model uncertainty and information gain
//! - Active sensor tasking to maximize process discovery signal
//! - Compressed downlink optimization for bandwidth-constrained environments
//! - Model-in-the-loop deployment: feeds observation needs back to edge sensors

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Observation product types that the agent can classify and filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationProduct {
    /// Multispectral optical imagery.
    OpticalMultispectral,
    /// Synthetic Aperture Radar.
    Sar,
    /// Thermal infrared.
    ThermalIr,
    /// LiDAR point cloud.
    Lidar,
    /// Hyperspectral.
    Hyperspectral,
    /// In-situ sensor network (e.g., flux towers, weather stations).
    InSitu,
    /// Reanalysis product (e.g., ERA5).
    Reanalysis,
}

/// Cloud / scene classification result from on-board AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneClassification {
    /// Cloud cover fraction [0, 1].
    pub cloud_fraction: f64,
    /// Fire detection probability.
    pub fire_probability: f64,
    /// Flood detection probability.
    pub flood_probability: f64,
    /// Vegetation stress index.
    pub vegetation_stress: f64,
    /// Snow/ice fraction.
    pub snow_ice_fraction: f64,
    /// Overall scene usability score [0, 1].
    pub usability_score: f64,
}

/// Observation value map entry — quantifies the information value of an
/// observation at a given location for a given process family.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationValueEntry {
    /// Grid cell (x, y) or spatial region identifier.
    pub location: (usize, usize),
    /// Target process family this observation would benefit.
    pub target_family: String,
    /// Information gain score — higher means more valuable.
    pub information_gain: f64,
    /// Current model uncertainty at this location.
    pub model_uncertainty: f64,
    /// Time since last usable observation (hours).
    pub observation_age_hours: f64,
    /// Recommended product type for maximum value.
    pub recommended_product: ObservationProduct,
}

/// Active tasking request — sent to sensor platforms or data providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskingRequest {
    /// Target spatial bounds (min_lat, min_lon, max_lat, max_lon).
    pub bounds: (f64, f64, f64, f64),
    /// Requested product type.
    pub product: ObservationProduct,
    /// Priority (0 = highest).
    pub priority: u8,
    /// Justification text for provenance.
    pub justification: String,
    /// Estimated information gain if fulfilled.
    pub expected_gain: f64,
}

/// Configuration for the autonomous observation agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationConfig {
    /// Cloud cover threshold above which scenes are filtered.
    pub cloud_threshold: f64,
    /// Minimum information gain to justify a tasking request.
    pub min_information_gain: f64,
    /// Maximum number of active tasking requests.
    pub max_active_taskings: usize,
    /// Whether to enable compressed downlink mode (edge AI filtering).
    pub compressed_downlink: bool,
    /// INT8 quantization for on-board models.
    pub int8_quantization: bool,
}

impl Default for ObservationConfig {
    fn default() -> Self {
        Self {
            cloud_threshold: 0.7,
            min_information_gain: 0.3,
            max_active_taskings: 16,
            compressed_downlink: true,
            int8_quantization: true,
        }
    }
}

/// The Autonomous Observation Agent.
pub struct AutonomousObservationAgent {
    id: AgentId,
    config: ObservationConfig,
    /// Current observation value map.
    value_map: Vec<ObservationValueEntry>,
    /// Active tasking requests.
    active_taskings: Vec<TaskingRequest>,
}

impl AutonomousObservationAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("autonomous_observation".into()),
            config: ObservationConfig::default(),
            value_map: Vec::new(),
            active_taskings: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: ObservationConfig) -> Self {
        self.config = config;
        self
    }

    /// Get the current observation value map.
    pub fn value_map(&self) -> &[ObservationValueEntry] {
        &self.value_map
    }

    /// Get active tasking requests.
    pub fn active_taskings(&self) -> &[TaskingRequest] {
        &self.active_taskings
    }

    /// Score a scene classification and decide whether to keep or discard.
    pub fn should_keep_scene(&self, classification: &SceneClassification) -> bool {
        classification.cloud_fraction < self.config.cloud_threshold
            && classification.usability_score > 0.3
    }

    /// Add an entry to the observation value map.
    pub fn update_value_map(&mut self, entry: ObservationValueEntry) {
        self.value_map.push(entry);
    }

    /// Generate a tasking request for the highest-value unobserved region.
    pub fn generate_tasking(&mut self) -> Option<TaskingRequest> {
        if self.active_taskings.len() >= self.config.max_active_taskings {
            return None;
        }

        // Find the value map entry with highest information gain
        let best = self
            .value_map
            .iter()
            .filter(|e| e.information_gain >= self.config.min_information_gain)
            .max_by(|a, b| {
                a.information_gain
                    .partial_cmp(&b.information_gain)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        best.map(|entry| {
            let request = TaskingRequest {
                bounds: (
                    entry.location.0 as f64,
                    entry.location.1 as f64,
                    entry.location.0 as f64 + 1.0,
                    entry.location.1 as f64 + 1.0,
                ),
                product: entry.recommended_product,
                priority: 0,
                justification: format!(
                    "High model uncertainty ({:.2}) for {} at {:?}, observation age {:.0}h",
                    entry.model_uncertainty,
                    entry.target_family,
                    entry.location,
                    entry.observation_age_hours,
                ),
                expected_gain: entry.information_gain,
            };
            self.active_taskings.push(request.clone());
            request
        })
    }
}

#[async_trait]
impl Agent for AutonomousObservationAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    fn role(&self) -> AgentRole {
        AgentRole::AutonomousObservation
    }

    fn description(&self) -> &str {
        "Applies PhiSat-2 on-board AI principles to autonomously filter, \
         prioritize, and task observational data streams; scores scenes via \
         edge-AI classifiers, maintains an observation value map keyed to \
         model uncertainty, and generates active tasking requests to maximize \
         process discovery signal"
    }

    async fn execute(&self, _ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        // TODO: implement full observation intelligence loop
        // 1. Scan incoming data streams (optical, SAR, thermal, in-situ)
        // 2. Run on-board scene classification (cloud, fire, flood filter)
        // 3. Score scenes against observation value map
        // 4. Keep high-value scenes, discard low-value
        // 5. Update value map from latest model uncertainty fields
        // 6. Generate active tasking requests for data-sparse regions
        Ok(AgentResult::ok(format!(
            "Autonomous observation cycle: {} value map entries, {} active taskings",
            self.value_map.len(),
            self.active_taskings.len(),
        )))
    }
}
