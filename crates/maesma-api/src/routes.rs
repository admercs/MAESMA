//! Route handlers for the MAESMA API.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::state::AppState;

/// Health check.
pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "maesma-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// List all process manifests.
pub async fn list_manifests(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let kb = maesma_knowledgebase::KnowledgebaseStore::open(&state.db_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let manifests = kb
        .list_manifests()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items: Vec<Value> = manifests
        .iter()
        .map(|(id, name, family)| {
            json!({
                "id": id,
                "name": name,
                "family": family,
            })
        })
        .collect();

    Ok(Json(json!({ "manifests": items, "count": items.len() })))
}

/// Get a specific manifest by ID.
pub async fn get_manifest(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: parse UUID and look up manifest
    Ok(Json(json!({ "id": id, "status": "not_implemented" })))
}

/// Knowledgebase statistics.
pub async fn kb_stats(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let kb = maesma_knowledgebase::KnowledgebaseStore::open(&state.db_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "manifests": kb.manifest_count().unwrap_or(0),
        "skill_records": kb.skill_count().unwrap_or(0),
    })))
}

/// Get the current SAPG as a graph structure.
pub async fn get_sapg() -> Json<Value> {
    // TODO: serialize current SAPG
    Json(json!({ "nodes": [], "edges": [], "status": "no_active_sapg" }))
}

/// Validate a SAPG configuration.
pub async fn validate_sapg(Json(_payload): Json<Value>) -> Json<Value> {
    // TODO: compile and validate
    Json(json!({ "valid": true, "diagnostics": [] }))
}

/// List registered agents.
pub async fn list_agents() -> Json<Value> {
    let agents = vec![
        "kb_retrieval", "assembly", "closure_validator", "benchmarking",
        "selection", "optimizer", "discovery", "data_scout", "a2a_gateway",
        "regime_detector", "scale_negotiator", "provenance", "salient_dynamics",
        "ensemble", "diagnostics", "sensitivity", "hypothesis",
        "geoengineering", "planetary_defense", "trophic", "evolution",
        "meta_learner", "runtime_sentinel",
    ];
    Json(json!({ "agents": agents, "count": agents.len() }))
}

/// Get skill records for a process.
pub async fn get_skills(Path(process_id): Path<String>) -> Json<Value> {
    // TODO: query from knowledgebase
    Json(json!({ "process_id": process_id, "records": [] }))
}

/// Get the Pareto front across all processes.
pub async fn pareto_front() -> Json<Value> {
    // TODO: compute Pareto front
    Json(json!({ "front": [] }))
}

/// Simulation status.
pub async fn simulation_status() -> Json<Value> {
    Json(json!({
        "running": false,
        "step": 0,
        "time": 0.0,
    }))
}

/// Federation endpoint (receives requests from peers).
pub async fn federation_endpoint(Json(_payload): Json<Value>) -> Json<Value> {
    // TODO: handle federation requests
    Json(json!({ "success": true, "message": "received" }))
}

/// List federation peers.
pub async fn list_peers() -> Json<Value> {
    Json(json!({ "peers": [] }))
}
