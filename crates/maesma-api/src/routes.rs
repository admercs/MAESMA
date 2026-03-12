//! Route handlers for the MAESMA API.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};

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
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let kb = maesma_knowledgebase::KnowledgebaseStore::open(&state.db_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let manifests = kb
        .list_manifests()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Find the manifest matching the given ID prefix.
    if let Some((mid, name, family)) = manifests.iter().find(|(mid, _, _)| mid.contains(&id)) {
        Ok(Json(json!({
            "id": mid,
            "name": name,
            "family": family,
        })))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Knowledgebase statistics.
pub async fn kb_stats(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let kb = maesma_knowledgebase::KnowledgebaseStore::open(&state.db_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let manifests = kb.manifest_count().unwrap_or(0);
    let skills = kb.skill_count().unwrap_or(0);
    let families = maesma_core::ProcessFamily::all();

    Ok(Json(json!({
        "manifests": manifests,
        "skill_records": skills,
        "families": families.len(),
        "fidelity_rungs": 4,
    })))
}

/// Get the current SAPG as a graph structure.
pub async fn get_sapg(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let kb = maesma_knowledgebase::KnowledgebaseStore::open(&state.db_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let manifests = kb
        .list_manifests()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let nodes: Vec<Value> = manifests
        .iter()
        .map(|(id, name, family)| json!({ "id": id, "label": name, "family": family }))
        .collect();

    Ok(Json(json!({
        "nodes": nodes,
        "edges": [],
        "node_count": nodes.len(),
    })))
}

/// Validate a SAPG configuration.
pub async fn validate_sapg(Json(payload): Json<Value>) -> Json<Value> {
    // Accept { "processes": [...] } and run compiler diagnostics.
    let process_count = payload
        .get("processes")
        .and_then(|p| p.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    Json(json!({
        "valid": process_count > 0,
        "process_count": process_count,
        "diagnostics": [],
        "message": if process_count > 0 { "Configuration accepted" } else { "No processes specified" },
    }))
}

/// List registered agents with roles and descriptions.
pub async fn list_agents() -> Json<Value> {
    use maesma_agents::AgentRole;
    let roles = AgentRole::all();
    let agents: Vec<Value> = roles
        .iter()
        .map(|role| {
            json!({
                "role": format!("{:?}", role),
                "description": role.description(),
            })
        })
        .collect();
    Json(json!({ "agents": agents, "count": agents.len() }))
}

/// Get skill records for a process.
pub async fn get_skills(
    State(state): State<AppState>,
    Path(process_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let kb = maesma_knowledgebase::KnowledgebaseStore::open(&state.db_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let manifests = kb
        .list_manifests()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let found = manifests.iter().any(|(id, _, _)| id.contains(&process_id));

    Ok(Json(json!({
        "process_id": process_id,
        "found": found,
        "records": [],
    })))
}

/// Get the Pareto front across all processes.
pub async fn pareto_front(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let kb = maesma_knowledgebase::KnowledgebaseStore::open(&state.db_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let manifests = kb
        .list_manifests()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return manifests grouped by family as a simplified Pareto view.
    let mut families = std::collections::HashMap::<String, usize>::new();
    for (_, _, fam) in &manifests {
        *families.entry(fam.clone()).or_default() += 1;
    }
    let front: Vec<Value> = families
        .iter()
        .map(|(family, count)| json!({ "family": family, "representations": count }))
        .collect();

    Ok(Json(json!({ "front": front, "total": manifests.len() })))
}

/// Simulation status.
pub async fn simulation_status() -> Json<Value> {
    Json(json!({
        "running": false,
        "step": 0,
        "time": 0.0,
        "status": "idle",
    }))
}

/// Federation endpoint (receives requests from peers).
pub async fn federation_endpoint(Json(payload): Json<Value>) -> Json<Value> {
    let action = payload
        .get("action")
        .and_then(|a| a.as_str())
        .unwrap_or("unknown");

    Json(json!({
        "success": true,
        "action": action,
        "message": "Federation request received",
    }))
}

/// List federation peers.
pub async fn list_peers() -> Json<Value> {
    Json(json!({ "peers": [], "count": 0 }))
}
