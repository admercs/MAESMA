//! Skill records — rung × region × regime × season × metric vectors.

use serde::{Deserialize, Serialize};

use crate::metrics::SkillMetrics;
use crate::process::{FidelityRung, ProcessId};
use crate::regime::RegimeTag;

/// A single skill record binding a process representation's measured skill
/// to a specific evaluation context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRecord {
    /// Which process representation was evaluated.
    pub process_id: ProcessId,
    /// Fidelity rung at time of evaluation.
    pub rung: FidelityRung,
    /// Geographic region (e.g., FLUXNET site, watershed, globe).
    pub region: String,
    /// Regime tags active during evaluation.
    pub regime_tags: Vec<RegimeTag>,
    /// Season (if seasonal split).
    pub season: Option<String>,
    /// The measured skill metrics.
    pub metrics: SkillMetrics,
    /// Dataset used for evaluation.
    pub dataset: String,
    /// ISO 8601 timestamp.
    pub evaluated_at: String,
    /// Evaluation method / benchmark name.
    pub benchmark: Option<String>,
    /// Content hash of the process version evaluated.
    pub process_hash: Option<String>,
}

/// A collection of skill records, typically for one process.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillHistory {
    pub records: Vec<SkillRecord>,
}

impl SkillHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, record: SkillRecord) {
        self.records.push(record);
    }

    /// Get the most recent skill record.
    pub fn latest(&self) -> Option<&SkillRecord> {
        self.records.last()
    }

    /// Filter records by regime tag.
    pub fn for_regime(&self, tag: &RegimeTag) -> Vec<&SkillRecord> {
        self.records
            .iter()
            .filter(|r| r.regime_tags.iter().any(|t| t.0 == tag.0))
            .collect()
    }

    /// Filter records by region.
    pub fn for_region(&self, region: &str) -> Vec<&SkillRecord> {
        self.records.iter().filter(|r| r.region == region).collect()
    }

    /// Get Pareto-optimal records (non-dominated on skill–cost tradeoff).
    pub fn pareto_front(&self) -> Vec<&SkillRecord> {
        let mut front = Vec::new();
        for (i, a) in self.records.iter().enumerate() {
            let dominated =
                self.records.iter().enumerate().any(|(j, b)| {
                    i != j && crate::metrics::pareto_dominates(&b.metrics, &a.metrics)
                });
            if !dominated {
                front.push(a);
            }
        }
        front
    }
}
