//! Error types for the MAESMA system.

use thiserror::Error;

/// Core error type for MAESMA operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Process not found: {0}")]
    ProcessNotFound(String),

    #[error("Manifest validation failed: {0}")]
    ManifestValidation(String),

    #[error("Conservation violation: {0}")]
    ConservationViolation(String),

    #[error("Unit mismatch: expected {expected}, got {got}")]
    UnitMismatch { expected: String, got: String },

    #[error("Scale envelope violation: {0}")]
    ScaleViolation(String),

    #[error("Closure check failed: {0}")]
    ClosureFailure(String),

    #[error("Coupling incompatibility: {from_process} <-> {to_process}: {reason}")]
    CouplingIncompatibility {
        from_process: String,
        to_process: String,
        reason: String,
    },

    #[error("Regime mismatch: {0}")]
    RegimeMismatch(String),

    #[error("Compilation failed: {0}")]
    CompilationFailed(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Knowledgebase error: {0}")]
    Knowledgebase(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Inference error: {0}")]
    Inference(String),

    #[error("Federation error: {0}")]
    Federation(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Convenience Result alias.
pub type Result<T> = std::result::Result<T, Error>;
