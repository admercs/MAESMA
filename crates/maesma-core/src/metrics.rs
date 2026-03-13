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
        if let (Some(x), Some(y)) = (va, vb) {
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
    }

    at_least_one_better
}

// ---------------------------------------------------------------------------
// Standard Metric Registry
// ---------------------------------------------------------------------------

/// A registered metric in the ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    /// Metric name (e.g., "rmse", "kge", "crps").
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Whether lower values are better.
    pub lower_is_better: bool,
    /// Typical acceptable range (min, max).
    pub typical_range: (f64, f64),
    /// Default weight in multi-objective aggregation.
    pub default_weight: f64,
}

/// Default fitness function: weighted combination of skill metrics minus cost.
///
/// $F(r,g,\ell) = \sum w_m S_m(r,g,\ell) - \lambda C(r)$
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessFunction {
    /// Name of this fitness function.
    pub name: String,
    /// Metric weights: metric_name → weight.
    pub weights: std::collections::HashMap<String, f64>,
    /// Cost penalty coefficient λ.
    pub cost_penalty: f64,
}

impl FitnessFunction {
    /// Evaluate fitness given metric values and a cost.
    pub fn evaluate(&self, metrics: &SkillMetrics, cost: f64) -> f64 {
        let mut score = 0.0;
        if let Some(kge) = metrics.kge {
            score += self.weights.get("kge").copied().unwrap_or(0.0) * kge;
        }
        if let Some(nse) = metrics.nse {
            score += self.weights.get("nse").copied().unwrap_or(0.0) * nse;
        }
        if let Some(rmse) = metrics.rmse {
            // RMSE: lower is better, so negate
            score -= self.weights.get("rmse").copied().unwrap_or(0.0) * rmse;
        }
        if let Some(corr) = metrics.correlation {
            score += self.weights.get("correlation").copied().unwrap_or(0.0) * corr;
        }
        if let Some(cons) = metrics.conservation_residual {
            score -= self.weights.get("conservation").copied().unwrap_or(0.0) * cons.abs();
        }
        score - self.cost_penalty * cost
    }

    /// Default fitness function with standard weights.
    pub fn default_esm() -> Self {
        let mut weights = std::collections::HashMap::new();
        weights.insert("kge".into(), 0.3);
        weights.insert("nse".into(), 0.2);
        weights.insert("rmse".into(), 0.2);
        weights.insert("correlation".into(), 0.15);
        weights.insert("conservation".into(), 0.15);
        Self {
            name: "default_esm".into(),
            weights,
            cost_penalty: 0.01,
        }
    }
}

/// Expert-prior skill model for a process representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertSkillPrior {
    /// Process family.
    pub family: String,
    /// Fidelity rung.
    pub rung: String,
    /// Accuracy class per regime.
    pub regime_accuracy: std::collections::HashMap<String, AccuracyClass>,
    /// Known failure modes.
    pub failure_modes: Vec<String>,
    /// Whether this is an expert prior (true) or empirical posterior (false).
    pub is_prior: bool,
}

/// Accuracy class for a regime.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccuracyClass {
    High,
    Moderate,
    Low,
    Unreliable,
}

/// Build standard metric definitions.
pub fn standard_metrics() -> Vec<MetricDefinition> {
    vec![
        MetricDefinition {
            name: "rmse".into(),
            description: "Root Mean Square Error".into(),
            lower_is_better: true,
            typical_range: (0.0, 10.0),
            default_weight: 0.2,
        },
        MetricDefinition {
            name: "mae".into(),
            description: "Mean Absolute Error".into(),
            lower_is_better: true,
            typical_range: (0.0, 10.0),
            default_weight: 0.1,
        },
        MetricDefinition {
            name: "bias".into(),
            description: "Bias ratio (predicted/observed)".into(),
            lower_is_better: false, // 1.0 is optimal
            typical_range: (0.0, 3.0),
            default_weight: 0.1,
        },
        MetricDefinition {
            name: "correlation".into(),
            description: "Pearson correlation coefficient".into(),
            lower_is_better: false,
            typical_range: (-1.0, 1.0),
            default_weight: 0.15,
        },
        MetricDefinition {
            name: "kge".into(),
            description: "Kling-Gupta Efficiency (2012 formulation)".into(),
            lower_is_better: false,
            typical_range: (-0.41, 1.0),
            default_weight: 0.3,
        },
        MetricDefinition {
            name: "nse".into(),
            description: "Nash-Sutcliffe Efficiency".into(),
            lower_is_better: false,
            typical_range: (-5.0, 1.0),
            default_weight: 0.2,
        },
        MetricDefinition {
            name: "crps".into(),
            description: "Continuous Ranked Probability Score".into(),
            lower_is_better: true,
            typical_range: (0.0, 5.0),
            default_weight: 0.15,
        },
        MetricDefinition {
            name: "conservation_residual".into(),
            description: "Mass/energy conservation residual".into(),
            lower_is_better: true,
            typical_range: (0.0, 1.0),
            default_weight: 0.15,
        },
        MetricDefinition {
            name: "timing_error".into(),
            description: "Temporal phase error (days)".into(),
            lower_is_better: true,
            typical_range: (0.0, 30.0),
            default_weight: 0.1,
        },
        MetricDefinition {
            name: "spearman".into(),
            description: "Spearman rank correlation".into(),
            lower_is_better: false,
            typical_range: (-1.0, 1.0),
            default_weight: 0.1,
        },
    ]
}

/// Build default expert-prior skill models for standard families × rungs.
pub fn default_expert_priors() -> Vec<ExpertSkillPrior> {
    use AccuracyClass::*;
    let mut priors = Vec::new();

    let families_rungs = vec![
        (
            "Hydrology",
            "R0",
            vec![
                ("normal", Moderate),
                ("drought", Low),
                ("flood", Unreliable),
            ],
            vec!["No lateral flow", "No groundwater interaction"],
        ),
        (
            "Hydrology",
            "R1",
            vec![("normal", High), ("drought", Moderate), ("flood", Moderate)],
            vec!["Single-column only"],
        ),
        (
            "Fire",
            "R0",
            vec![("normal", Low), ("fire_weather", Unreliable)],
            vec!["No spatial spread", "Stochastic only"],
        ),
        (
            "Fire",
            "R1",
            vec![("normal", Moderate), ("fire_weather", Moderate)],
            vec!["Wind field coarsening", "No plume feedback"],
        ),
        (
            "Ecology",
            "R0",
            vec![("normal", Moderate), ("disturbance", Low)],
            vec!["Static LAI", "No succession"],
        ),
        (
            "Ecology",
            "R1",
            vec![("normal", High), ("disturbance", Moderate)],
            vec!["Age-class approximation"],
        ),
        (
            "Radiation",
            "R0",
            vec![("clear_sky", Moderate), ("cloudy", Low)],
            vec!["No cloud interaction", "Daily only"],
        ),
        (
            "Radiation",
            "R1",
            vec![("clear_sky", High), ("cloudy", Moderate)],
            vec!["Two-stream approximation"],
        ),
        (
            "Atmosphere",
            "R0",
            vec![("normal", Moderate)],
            vec!["Prescribed forcing only"],
        ),
        (
            "Ocean",
            "R0",
            vec![("normal", Low)],
            vec!["No dynamics", "Fixed depth"],
        ),
        (
            "Cryosphere",
            "R0",
            vec![("normal", Moderate), ("warm_winter", Low)],
            vec!["Degree-day only", "No albedo feedback"],
        ),
        (
            "Biogeochemistry",
            "R0",
            vec![("normal", Moderate)],
            vec!["Single pool", "No nitrogen"],
        ),
        (
            "Biogeochemistry",
            "R1",
            vec![("normal", High), ("disturbance", Moderate)],
            vec!["CENTURY approximation"],
        ),
    ];

    for (family, rung, regimes, failures) in families_rungs {
        priors.push(ExpertSkillPrior {
            family: family.into(),
            rung: rung.into(),
            regime_accuracy: regimes
                .into_iter()
                .map(|(r, a)| (r.to_string(), a))
                .collect(),
            failure_modes: failures.into_iter().map(String::from).collect(),
            is_prior: true,
        });
    }

    priors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_metrics_count() {
        let metrics = standard_metrics();
        assert!(metrics.len() >= 10);
    }

    #[test]
    fn fitness_function_evaluation() {
        let ff = FitnessFunction::default_esm();
        let metrics = SkillMetrics {
            kge: Some(0.8),
            nse: Some(0.7),
            rmse: Some(0.5),
            correlation: Some(0.9),
            conservation_residual: Some(0.001),
            ..SkillMetrics::empty()
        };
        let fitness = ff.evaluate(&metrics, 10.0);
        assert!(fitness > 0.0);
    }

    #[test]
    fn expert_priors_populated() {
        let priors = default_expert_priors();
        assert!(priors.len() >= 10);
        assert!(priors.iter().all(|p| p.is_prior));
    }

    #[test]
    fn pareto_dominance_basic() {
        let a = SkillMetrics {
            rmse: Some(0.5),
            kge: Some(0.8),
            ..SkillMetrics::empty()
        };
        let b = SkillMetrics {
            rmse: Some(1.0),
            kge: Some(0.6),
            ..SkillMetrics::empty()
        };
        assert!(pareto_dominates(&a, &b));
        assert!(!pareto_dominates(&b, &a));
    }
}
