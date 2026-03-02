//! SAPG Compiler — validates and compiles Scale-Aware Process Graphs into runnable form.
//!
//! The compiler performs:
//! - Conservation closure checks (mass, energy, momentum)
//! - Scale compatibility validation
//! - Coupling consistency verification
//! - Ontology constraint enforcement
//! - Operator-splitting schedule generation

pub mod schedule;
pub mod validators;

use std::collections::HashSet;

use maesma_core::graph::Sapg;
use tracing::info;

pub use validators::ManifestIndex;

/// Result of compiling a SAPG into a runnable schedule.
#[derive(Debug)]
pub struct CompilationResult {
    /// Whether compilation succeeded.
    pub success: bool,
    /// Validation diagnostics.
    pub diagnostics: Vec<Diagnostic>,
    /// The generated execution schedule (if successful).
    pub schedule: Option<schedule::ExecutionSchedule>,
}

/// A diagnostic message from compilation.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

/// Compile a SAPG into an execution schedule.
///
/// `manifests` supplies I/O contracts, scale envelopes, and conservation
/// properties for each process node. `external_forcings` names variables
/// that are supplied as boundary conditions (not produced by the graph).
pub fn compile(
    sapg: &Sapg,
    manifests: &ManifestIndex,
    external_forcings: &HashSet<String>,
) -> maesma_core::Result<CompilationResult> {
    let mut diagnostics = Vec::new();

    // Step 1: validate conservation closure
    diagnostics.extend(validators::check_conservation_closure(sapg, manifests));

    // Step 2: validate scale compatibility
    diagnostics.extend(validators::check_scale_compatibility(sapg, manifests));

    // Step 3: validate coupling consistency
    diagnostics.extend(validators::check_coupling_consistency(sapg, manifests));

    // Step 4: detect double-counting
    diagnostics.extend(validators::check_double_counting(sapg, manifests));

    // Step 5: verify state-space closure
    diagnostics.extend(validators::check_state_space_closure(
        sapg,
        manifests,
        external_forcings,
    ));

    // Step 6: CFL / numerical stability heuristics
    diagnostics.extend(validators::check_numerical_stability(sapg, manifests));

    // Check for errors
    let has_errors = diagnostics
        .iter()
        .any(|d| matches!(d.level, DiagnosticLevel::Error));

    if has_errors {
        info!(
            errors = diagnostics
                .iter()
                .filter(|d| matches!(d.level, DiagnosticLevel::Error))
                .count(),
            "SAPG compilation failed"
        );
        return Ok(CompilationResult {
            success: false,
            diagnostics,
            schedule: None,
        });
    }

    // Step 7: generate execution schedule
    let exec_schedule = schedule::generate_schedule(sapg)?;

    info!(
        nodes = sapg.node_count(),
        edges = sapg.edge_count(),
        diagnostics = diagnostics.len(),
        "SAPG compiled successfully"
    );

    Ok(CompilationResult {
        success: true,
        diagnostics,
        schedule: Some(exec_schedule),
    })
}
