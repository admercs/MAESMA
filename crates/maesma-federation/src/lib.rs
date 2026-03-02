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
        let peer = self
            .peers
            .iter()
            .find(|p| p.id == peer_id)
            .ok_or_else(|| maesma_core::Error::Federation(format!("Unknown peer: {}", peer_id)))?;

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
