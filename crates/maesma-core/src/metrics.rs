//! Skill metrics — quantitative evaluation of process representations.

use serde::{Deserialize, Serialize};

/// A single skill measurement for a process representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetrics {
    /// Root Mean Square Error.
    pub rmse: Option<f64>,
    /// Kling-Gupta Efficiency (KGE 2012 form).
    pub kge: Option<f64>,
    /// Continuous Ranked Probability Score.
    pub crps: Option<f64>,
    /// Nash-Sutcliffe Efficiency.
    pub nse: Option<f64>,
    /// Bias ratio.
    pub bias: Option<f64>,
    /// Correlation coefficient.
    pub correlation: Option<f64>,
    /// Conservation residual (should be ≈ 0 for mass/energy).
    pub conservation_residual: Option<f64>,
    /// Wall-clock time per cell per step (seconds).
    pub wall_time_per_cell: Option<f64>,
    /// Custom metrics keyed by name.
    pub custom: std::collections::HashMap<String, f64>,
}

impl SkillMetrics {
    pub fn empty() -> Self {
        Self {
            rmse: None,
            kge: None,
            crps: None,
            nse: None,
            bias: None,
            correlation: None,
            conservation_residual: None,
            wall_time_per_cell: None,
            custom: std::collections::HashMap::new(),
        }
    }
}

/// Multi-objective Pareto dominance check.
///
/// Returns `true` if `a` dominates `b` (at least as good on all metrics,
/// strictly better on at least one).
pub fn pareto_dominates(a: &SkillMetrics, b: &SkillMetrics) -> bool {
    let pairs: Vec<(Option<f64>, Option<f64>, bool)> = vec![
        // For RMSE, lower is better
        (a.rmse, b.rmse, true),
        // For KGE, higher is better
        (a.kge, b.kge, false),
        // For CRPS, lower is better
        (a.crps, b.crps, true),
        // For NSE, higher is better
        (a.nse, b.nse, false),
        // For conservation residual, lower is better
        (a.conservation_residual, b.conservation_residual, true),
        // For wall time, lower is better
        (a.wall_time_per_cell, b.wall_time_per_cell, true),
    ];

    let mut at_least_one_better = false;

    for (va, vb, lower_is_better) in pairs {
        match (va, vb) {
            (Some(x), Some(y)) => {
                if lower_is_better {
                    if x > y {
                        return false;
                    }
                    if x < y {
                        at_least_one_better = true;
                    }
                } else {
                    if x < y {
                        return false;
                    }
                    if x > y {
                        at_least_one_better = true;
                    }
                }
            }
            _ => {} // skip if either is None
        }
    }

    at_least_one_better
}
