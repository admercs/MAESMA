//! MAESMA API Server — HTTP/WebSocket endpoints for the dashboard.
//!
//! Provides RESTful endpoints for knowledgebase queries, agent status,
//! SAPG visualization, skill metrics, and real-time simulation monitoring
//! via WebSocket.

pub mod routes;
pub mod state;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::state::AppState;

/// Build the application router.
pub fn app(state: AppState) -> Router {
    Router::new()
        // Health
        .route("/api/v1/health", get(routes::health))
        // Knowledgebase
        .route("/api/v1/kb/manifests", get(routes::list_manifests))
        .route("/api/v1/kb/manifests/{id}", get(routes::get_manifest))
        .route("/api/v1/kb/stats", get(routes::kb_stats))
        // SAPG
        .route("/api/v1/sapg", get(routes::get_sapg))
        .route("/api/v1/sapg/validate", post(routes::validate_sapg))
        // Agents
        .route("/api/v1/agents", get(routes::list_agents))
        // Skills
        .route("/api/v1/skills/{process_id}", get(routes::get_skills))
        .route("/api/v1/skills/pareto", get(routes::pareto_front))
        // Simulation
        .route("/api/v1/simulation/status", get(routes::simulation_status))
        // Federation
        .route("/api/v1/federation", post(routes::federation_endpoint))
        .route("/api/v1/federation/peers", get(routes::list_peers))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
