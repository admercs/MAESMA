//! A2A Federation Protocol — agent-to-agent cross-instance collaboration.
//!
//! Implements the Agent-to-Agent (A2A) protocol for federating
//! knowledgebase entries across MAESMA instances. Peer instances
//! can share process representations, skill records, and ontology
//! relations while maintaining provenance and trust.

use serde::{Deserialize, Serialize};

/// A federated peer instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// Peer identifier.
    pub id: String,
    /// Base URL for the peer's API.
    pub url: String,
    /// Human-readable label.
    pub label: String,
    /// Trust level.
    pub trust: TrustLevel,
    /// Last successful sync timestamp (ISO 8601).
    pub last_sync: Option<String>,
}

/// Trust level for federated peers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    /// Fully trusted (e.g., same organization).
    Full,
    /// Verified (e.g., known institution).
    Verified,
    /// Community (e.g., public contribution).
    Community,
    /// Untrusted (quarantine entries for review).
    Untrusted,
}

/// A federation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationRequest {
    /// Request type.
    pub kind: FederationRequestKind,
    /// Source instance ID.
    pub source: String,
    /// Payload.
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FederationRequestKind {
    /// Share a process manifest.
    ShareManifest,
    /// Share skill records.
    ShareSkills,
    /// Query for process representations matching criteria.
    QueryProcesses,
    /// Ping/health check.
    Ping,
}

/// A federation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// Federation client for communicating with peers.
pub struct FederationClient {
    peers: Vec<Peer>,
    client: reqwest::Client,
}

impl FederationClient {
    pub fn new() -> Self {
        Self {
            peers: Vec::new(),
            client: reqwest::Client::new(),
        }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.peers.push(peer);
    }

    pub fn peers(&self) -> &[Peer] {
        &self.peers
    }

    /// Send a federation request to a specific peer.
    pub async fn send(
        &self,
        peer_id: &str,
        request: FederationRequest,
    ) -> maesma_core::Result<FederationResponse> {
        let peer =
            self.peers.iter().find(|p| p.id == peer_id).ok_or_else(|| {
                maesma_core::Error::Federation(format!("Unknown peer: {}", peer_id))
            })?;

        let url = format!("{}/api/v1/federation", peer.url);

        let resp = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| maesma_core::Error::Federation(e.to_string()))?;

        let fed_resp: FederationResponse = resp
            .json()
            .await
            .map_err(|e| maesma_core::Error::Federation(e.to_string()))?;

        Ok(fed_resp)
    }

    /// Broadcast a request to all peers with sufficient trust level.
    pub async fn broadcast(
        &self,
        request: FederationRequest,
        min_trust: TrustLevel,
    ) -> Vec<(String, maesma_core::Result<FederationResponse>)> {
        let mut results = Vec::new();
        for peer in &self.peers {
            if peer.trust as u8 >= min_trust as u8 {
                // TODO: parallelize with tokio::join
                let result = self.send(&peer.id, request.clone()).await;
                results.push((peer.id.clone(), result));
            }
        }
        results
    }
}

impl Default for FederationClient {
    fn default() -> Self {
        Self::new()
    }
}

// ── Phase 8: A2A Federation Infrastructure ──

/// Authentication configuration for A2A protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2aAuthConfig {
    pub method: AuthMethod,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub ca_path: Option<String>,
    pub bearer_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthMethod {
    MutualTls,
    BearerToken,
    None,
}

/// IR fragment for cross-instance model assembly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrFragment {
    pub source_instance: String,
    pub family: String,
    pub rung: String,
    pub boundary_vars: Vec<String>,
    pub checksum: String,
}

/// Cross-boundary conservation validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossBoundaryValidation {
    pub boundary: String,
    pub max_flux_error: f64,
    pub passed: bool,
}

/// Execution mode for federated run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FederatedExecutionMode {
    /// Remote execution with boundary conditions via streaming.
    Remote,
    /// Colocated execution (download fragment and run locally).
    Colocated,
}

/// Anonymized skill record for cross-instance sharing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizedSkillRecord {
    pub config_hash: String,
    pub family: String,
    pub rung: String,
    pub metrics: Vec<(String, f64)>,
    pub provenance: SkillShareProvenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillShareProvenance {
    Local,
    Imported { source_instance: String },
}

/// Trust-weighted Bayesian incorporation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustWeightedIncorporation {
    pub source_trust: f64,
    pub weight: f64,
    pub differential_privacy_epsilon: Option<f64>,
}

/// Compute trust weight from TrustLevel.
pub fn trust_weight(level: TrustLevel) -> f64 {
    match level {
        TrustLevel::Full => 1.0,
        TrustLevel::Verified => 0.7,
        TrustLevel::Community => 0.3,
        TrustLevel::Untrusted => 0.05,
    }
}

/// Extended A2A task types for data discovery and skill sharing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum A2aTaskType {
    DiscoverData,
    ShareSkill,
    ProposeRung,
    RemoteScore,
    ExchangeIrFragment,
}

/// A federated assembly plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedAssemblyPlan {
    pub local_families: Vec<String>,
    pub remote_fragments: Vec<IrFragment>,
    pub execution_mode: FederatedExecutionMode,
    pub boundary_validations: Vec<CrossBoundaryValidation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_weight_ordering() {
        assert!(trust_weight(TrustLevel::Full) > trust_weight(TrustLevel::Verified));
        assert!(trust_weight(TrustLevel::Verified) > trust_weight(TrustLevel::Community));
        assert!(trust_weight(TrustLevel::Community) > trust_weight(TrustLevel::Untrusted));
    }

    #[test]
    fn federated_assembly_plan() {
        let plan = FederatedAssemblyPlan {
            local_families: vec!["fire".into(), "hydrology".into()],
            remote_fragments: vec![IrFragment {
                source_instance: "peer_1".into(),
                family: "ocean".into(),
                rung: "R1".into(),
                boundary_vars: vec!["sst".into(), "salinity".into()],
                checksum: "abc123".into(),
            }],
            execution_mode: FederatedExecutionMode::Remote,
            boundary_validations: vec![],
        };
        assert_eq!(plan.remote_fragments.len(), 1);
    }

    #[test]
    fn anonymized_skill_record() {
        let record = AnonymizedSkillRecord {
            config_hash: "sha256:abcdef".into(),
            family: "fire".into(),
            rung: "R1".into(),
            metrics: vec![("rmse".into(), 0.05), ("bias".into(), -0.01)],
            provenance: SkillShareProvenance::Local,
        };
        assert_eq!(record.provenance, SkillShareProvenance::Local);
    }

    #[test]
    fn auth_config() {
        let auth = A2aAuthConfig {
            method: AuthMethod::MutualTls,
            cert_path: Some("/certs/client.pem".into()),
            key_path: Some("/certs/client.key".into()),
            ca_path: Some("/certs/ca.pem".into()),
            bearer_token: None,
        };
        assert_eq!(auth.method, AuthMethod::MutualTls);
    }
}
