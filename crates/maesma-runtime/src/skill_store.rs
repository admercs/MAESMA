//! Skill Score Store — persistent storage backend for skill evaluation records.
//!
//! Provides SQLite-backed storage for `SkillRecord` with versioned entries,
//! query API, and migration support.

use maesma_core::process::{FidelityRung, ProcessId};
use maesma_core::regime::RegimeTag;
use maesma_core::skills::SkillRecord;

// ---------------------------------------------------------------------------
// In-memory Skill Score Store (SQLite-like interface)
// ---------------------------------------------------------------------------

/// In-memory skill score store with query capabilities.
///
/// In production, this would be backed by SQLite via `rusqlite`.
/// The in-memory implementation provides the same API for testing.
#[derive(Debug, Default)]
pub struct SkillScoreStore {
    records: Vec<SkillRecord>,
    /// Schema version for migration tracking.
    schema_version: u32,
}

impl SkillScoreStore {
    /// Create a new empty store.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            schema_version: 1,
        }
    }

    /// Insert a skill record.
    pub fn insert(&mut self, record: SkillRecord) {
        self.records.push(record);
    }

    /// Insert multiple records.
    pub fn insert_batch(&mut self, records: Vec<SkillRecord>) {
        self.records.extend(records);
    }

    /// Total number of records.
    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// Query records by process ID.
    pub fn query_by_process(&self, pid: &ProcessId) -> Vec<&SkillRecord> {
        self.records
            .iter()
            .filter(|r| r.process_id == *pid)
            .collect()
    }

    /// Query records by region.
    pub fn query_by_region(&self, region: &str) -> Vec<&SkillRecord> {
        self.records.iter().filter(|r| r.region == region).collect()
    }

    /// Query records by fidelity rung.
    pub fn query_by_rung(&self, rung: FidelityRung) -> Vec<&SkillRecord> {
        self.records.iter().filter(|r| r.rung == rung).collect()
    }

    /// Query records by regime tag.
    pub fn query_by_regime(&self, tag: &RegimeTag) -> Vec<&SkillRecord> {
        self.records
            .iter()
            .filter(|r| r.regime_tags.iter().any(|t| t.0 == tag.0))
            .collect()
    }

    /// Query records by dataset name.
    pub fn query_by_dataset(&self, dataset: &str) -> Vec<&SkillRecord> {
        self.records
            .iter()
            .filter(|r| r.dataset == dataset)
            .collect()
    }

    /// Get the best record for a process (highest KGE).
    pub fn best_for_process(&self, pid: &ProcessId) -> Option<&SkillRecord> {
        self.query_by_process(pid).into_iter().max_by(|a, b| {
            a.metrics
                .kge
                .partial_cmp(&b.metrics.kge)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Get records where KGE exceeds a threshold.
    pub fn above_kge_threshold(&self, threshold: f64) -> Vec<&SkillRecord> {
        self.records
            .iter()
            .filter(|r| r.metrics.kge.is_some_and(|k| k >= threshold))
            .collect()
    }

    /// Get skill trend for a process (sorted by evaluation time).
    pub fn trend_for_process(&self, pid: &ProcessId) -> Vec<&SkillRecord> {
        let mut records: Vec<&SkillRecord> = self.query_by_process(pid);
        records.sort_by(|a, b| a.evaluated_at.cmp(&b.evaluated_at));
        records
    }

    /// Return Pareto-optimal records across all processes (KGE vs RMSE).
    pub fn pareto_front(&self) -> Vec<&SkillRecord> {
        let mut front = Vec::new();
        for (i, a) in self.records.iter().enumerate() {
            let a_kge = a.metrics.kge.unwrap_or(f64::NEG_INFINITY);
            let a_rmse = a.metrics.rmse.unwrap_or(f64::INFINITY);
            let dominated = self.records.iter().enumerate().any(|(j, b)| {
                let b_kge = b.metrics.kge.unwrap_or(f64::NEG_INFINITY);
                let b_rmse = b.metrics.rmse.unwrap_or(f64::INFINITY);
                i != j && b_kge >= a_kge && b_rmse <= a_rmse && (b_kge > a_kge || b_rmse < a_rmse)
            });
            if !dominated {
                front.push(a);
            }
        }
        front
    }

    /// Schema version.
    pub fn version(&self) -> u32 {
        self.schema_version
    }

    /// All records (slice).
    pub fn all(&self) -> &[SkillRecord] {
        &self.records
    }

    /// Export all records as JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.records)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use maesma_core::metrics::SkillMetrics;

    fn make_record(region: &str, kge: f64, rmse: f64) -> SkillRecord {
        SkillRecord {
            process_id: ProcessId::new(),
            rung: FidelityRung::R1,
            region: region.to_string(),
            regime_tags: vec![],
            season: None,
            metrics: SkillMetrics {
                rmse: Some(rmse),
                bias: Some(0.0),
                correlation: Some(kge),
                kge: Some(kge),
                nse: Some(kge),
                crps: Some(rmse),
                conservation_residual: None,
                wall_time_per_cell: None,
                custom: std::collections::HashMap::new(),
            },
            dataset: "test_ds".to_string(),
            evaluated_at: "2024-01-01T00:00:00Z".to_string(),
            benchmark: None,
            process_hash: None,
            provenance: maesma_core::skills::SkillProvenance::ExpertPrior,
        }
    }

    #[test]
    fn test_insert_and_count() {
        let mut store = SkillScoreStore::new();
        store.insert(make_record("CONUS", 0.8, 1.5));
        store.insert(make_record("Europe", 0.7, 2.0));
        assert_eq!(store.count(), 2);
    }

    #[test]
    fn test_query_by_region() {
        let mut store = SkillScoreStore::new();
        store.insert(make_record("CONUS", 0.8, 1.5));
        store.insert(make_record("Europe", 0.7, 2.0));
        store.insert(make_record("CONUS", 0.9, 1.0));
        assert_eq!(store.query_by_region("CONUS").len(), 2);
        assert_eq!(store.query_by_region("Europe").len(), 1);
    }

    #[test]
    fn test_above_kge_threshold() {
        let mut store = SkillScoreStore::new();
        store.insert(make_record("A", 0.3, 5.0));
        store.insert(make_record("B", 0.8, 1.0));
        store.insert(make_record("C", 0.9, 0.5));
        assert_eq!(store.above_kge_threshold(0.7).len(), 2);
    }

    #[test]
    fn test_pareto_front() {
        let mut store = SkillScoreStore::new();
        // These three: (0.9, 0.5) dominates (0.7, 2.0)
        store.insert(make_record("A", 0.9, 0.5));
        store.insert(make_record("B", 0.7, 2.0));
        store.insert(make_record("C", 0.5, 0.3));
        let front = store.pareto_front();
        assert_eq!(front.len(), 2, "Pareto front should have 2 non-dominated");
    }

    #[test]
    fn test_export_json() {
        let mut store = SkillScoreStore::new();
        store.insert(make_record("test", 0.8, 1.0));
        let json = store.export_json().unwrap();
        assert!(json.contains("\"kge\""));
        assert!(json.contains("test"));
    }
}
