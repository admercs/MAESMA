//! Closure validator agent — checks conservation closure of SAPG assemblies.

use std::collections::HashSet;

use async_trait::async_trait;
use tracing::info;

use crate::traits::{Agent, AgentContext, AgentId, AgentResult, AgentRole};

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
        "Validates mass, energy, and momentum closure across SAPG assemblies"
    }

    async fn execute(&self, ctx: AgentContext) -> maesma_core::Result<AgentResult> {
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

        // State-space closure: every input must be produced or externally forced
        let unmet: Vec<String> = inputs
            .iter()
            .filter(|i| !outputs.contains(*i) && !externals.contains(*i))
            .cloned()
            .collect();

        // Conservation check: conserved quantities should appear symmetrically
        let conserved = ctx
            .params
            .get("conserved_quantities")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mut diagnostics = Vec::new();
        for q in &conserved {
            let has_source = outputs.iter().any(|o| o.contains(q));
            let has_sink = inputs.iter().any(|i| i.contains(q));
            if has_source != has_sink {
                diagnostics.push(format!(
                    "Conservation warning: '{}' has {} but no {}",
                    q,
                    if has_source { "sources" } else { "sinks" },
                    if has_source { "sinks" } else { "sources" },
                ));
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
            Ok(
                AgentResult::ok("State-space closure verified \u{2014} all inputs satisfied")
                    .with_data(serde_json::json!({
                        "closed": true,
                        "inputs_checked": inputs.len(),
                        "outputs_available": outputs.len(),
                    }))
                    .with_next("run benchmarking"),
            )
        } else {
            Ok(AgentResult::ok(format!(
                "Closure issues: {} unmet inputs, {} conservation warnings",
                unmet.len(),
                diagnostics.len()
            ))
            .with_data(serde_json::json!({
                "closed": false,
                "unmet_inputs": unmet,
                "diagnostics": diagnostics,
            }))
            .with_next("resolve closure gaps"))
        }
    }
}
