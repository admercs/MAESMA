//! Application state shared across all route handlers.


/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// Database path.
    pub db_path: String,
}

impl AppState {
    pub fn new(db_path: impl Into<String>) -> Self {
        Self {
            db_path: db_path.into(),
        }
    }
}
