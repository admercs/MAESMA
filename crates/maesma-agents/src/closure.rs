//! Closure validator agent — Phase 5.4
//!
//! Automated validation: missing variables, double-counted physics,
//! unit mismatches, conservation violations, CFL stability checking.

use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

/// Unit mismatch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitMismatch {
    pub variable: String,
    pub producer_unit: String,
    pub consumer_unit: String,
}

/// CFL stability check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CflCheck {
    pub process: String,
    pub dx_m: f64,
    pub dt_s: f64,
    pub velocity_ms: f64,
    pub cfl_number: f64,
    pub stable: bool,
}

/// Detect unit mismatches from declared variable→unit maps.
pub fn detect_unit_mismatches(
    producers: &HashMap<String, String>,
    consumers: &HashMap<String, String>,
) -> Vec<UnitMismatch> {
    let mut mismatches = Vec::new();
    for (var, consumer_unit) in consumers {
        if let Some(producer_unit) = producers.get(var)
            && producer_unit != consumer_unit
        {
            mismatches.push(UnitMismatch {
                variable: var.clone(),
                producer_unit: producer_unit.clone(),
                consumer_unit: consumer_unit.clone(),
            });
        }
    }
    mismatches
}

/// CFL stability check: CFL = v * dt / dx < 1.
pub fn check_cfl(process: &str, dx_m: f64, dt_s: f64, velocity_ms: f64) -> CflCheck {
    let cfl = velocity_ms * dt_s / dx_m;
    CflCheck {
        process: process.into(),
        dx_m,
        dt_s,
        velocity_ms,
        cfl_number: cfl,
        stable: cfl < 1.0,
    }
}

/// Detect double-counted physics (same variable produced by multiple processes).
pub fn detect_double_counting(
    process_outputs: &HashMap<String, Vec<String>>,
) -> Vec<(String, Vec<String>)> {
    let mut var_producers: HashMap<String, Vec<String>> = HashMap::new();
    for (proc_name, outputs) in process_outputs {
        for var in outputs {
            var_producers
                .entry(var.clone())
                .or_default()
                .push(proc_name.clone());
        }
    }
    var_producers
        .into_iter()
        .filter(|(_, procs)| procs.len() > 1)
        .collect()
}

pub struct ClosureValidatorAgent {
    id: AgentId,
}

impl Default for ClosureValidatorAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl ClosureValidatorAgent {
    pub fn new() -> Self {
        Self {
            id: AgentId("closure_validator".into()),
        }
    }
}

#[async_trait]
impl Agent for ClosureValidatorAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }
    fn role(&self) -> AgentRole {
        AgentRole::ClosureValidator
    }
    fn description(&self) -> &str {
        "Validates closure: missing variables, unit mismatches, double counting, conservation, CFL"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
        let action = ctx
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("check");

        match action {
            "check" => {
                let inputs: HashSet<String> = ctx
                    .params
                    .get("inputs")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let outputs: HashSet<String> = ctx
                    .params
                    .get("outputs")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let externals: HashSet<String> = ctx
                    .params
                    .get("external_forcings")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let unmet: Vec<String> = inputs
                    .iter()
                    .filter(|i| !outputs.contains(*i) && !externals.contains(*i))
                    .cloned()
                    .collect();
                let conserved: Vec<String> = ctx
                    .params
                    .get("conserved_quantities")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let mut diagnostics = Vec::new();
                for q in &conserved {
                    let has_source = outputs.iter().any(|o| o.contains(q));
                    let has_sink = inputs.iter().any(|i| i.contains(q));
                    if has_source != has_sink {
                        diagnostics.push(format!("Conservation: '{}' asymmetric", q));
                    }
                }
                let closed = unmet.is_empty();
                info!(
                    closed,
                    unmet = unmet.len(),
                    diagnostics = diagnostics.len(),
                    "Closure check"
                );
                if closed && diagnostics.is_empty() {
                    Ok(AgentResult::ok("Closure verified").with_data(serde_json::json!({
                        "closed": true, "inputs_checked": inputs.len(), "outputs_available": outputs.len(),
                    })).with_next("run benchmarking"))
                } else {
                    Ok(AgentResult::ok(format!(
                        "{} unmet, {} warnings",
                        unmet.len(),
                        diagnostics.len()
                    ))
                    .with_data(serde_json::json!({
                        "closed": false, "unmet_inputs": unmet, "diagnostics": diagnostics,
                    }))
                    .with_next("resolve closure gaps"))
                }
            }
            "cfl" => {
                let dx = ctx
                    .params
                    .get("dx_m")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1000.0);
                let dt = ctx
                    .params
                    .get("dt_s")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(60.0);
                let vel = ctx
                    .params
                    .get("velocity_ms")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(50.0);
                let proc = ctx
                    .params
                    .get("process")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");
                let result = check_cfl(proc, dx, dt, vel);
                let data = serde_json::json!({ "cfl": result });
                Ok(AgentResult::ok(format!(
                    "CFL={:.3}, stable={}",
                    result.cfl_number, result.stable
                ))
                .with_data(data))
            }
            _ => {
                let data = serde_json::json!({ "available_actions": ["check", "cfl"] });
                Ok(AgentResult::ok("Closure validator status").with_data(data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfl_stable() {
        let c = check_cfl("atm", 1000.0, 10.0, 50.0);
        assert!(c.stable);
        assert!((c.cfl_number - 0.5).abs() < 1e-10);
    }

    #[test]
    fn cfl_unstable() {
        let c = check_cfl("atm", 100.0, 10.0, 50.0);
        assert!(!c.stable);
    }

    #[test]
    fn unit_mismatch_detection() {
        let mut prod = HashMap::new();
        prod.insert("temperature".into(), "K".into());
        let mut cons = HashMap::new();
        cons.insert("temperature".into(), "C".into());
        let m = detect_unit_mismatches(&prod, &cons);
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn double_counting_detection() {
        let mut po: HashMap<String, Vec<String>> = HashMap::new();
        po.insert("proc_a".into(), vec!["heat_flux".into()]);
        po.insert("proc_b".into(), vec!["heat_flux".into()]);
        let dc = detect_double_counting(&po);
        assert_eq!(dc.len(), 1);
    }
}
