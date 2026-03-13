//! Knowledgebase store — SQLite-backed persistence layer.

use maesma_core::manifest::ProcessManifest;
use maesma_core::ontology::Relation;
use maesma_core::process::ProcessId;
use maesma_core::skills::SkillRecord;
use rusqlite::{Connection, params};
use tracing::info;

/// The central knowledgebase store.
pub struct KnowledgebaseStore {
    conn: Connection,
}

impl KnowledgebaseStore {
    /// Open or create a knowledgebase at the given path.
    pub fn open(path: &str) -> maesma_core::Result<Self> {
        let conn =
            Connection::open(path).map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
        let store = Self { conn };
        store.initialize_schema()?;
        info!(path, "Knowledgebase opened");
        Ok(store)
    }

    /// Create an in-memory knowledgebase (for testing).
    pub fn in_memory() -> maesma_core::Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
        let store = Self { conn };
        store.initialize_schema()?;
        Ok(store)
    }

    fn initialize_schema(&self) -> maesma_core::Result<()> {
        self.conn
            .execute_batch(
                "
                CREATE TABLE IF NOT EXISTS manifests (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    family TEXT NOT NULL,
                    rung TEXT NOT NULL,
                    version TEXT NOT NULL,
                    lifecycle TEXT NOT NULL,
                    content_hash TEXT NOT NULL,
                    data TEXT NOT NULL,
                    created_at TEXT NOT NULL DEFAULT (datetime('now')),
                    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                );

                CREATE TABLE IF NOT EXISTS skill_records (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    process_id TEXT NOT NULL,
                    rung TEXT NOT NULL,
                    region TEXT NOT NULL,
                    dataset TEXT NOT NULL,
                    data TEXT NOT NULL,
                    evaluated_at TEXT NOT NULL,
                    FOREIGN KEY (process_id) REFERENCES manifests(id)
                );

                CREATE TABLE IF NOT EXISTS ontology_relations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    source_id TEXT NOT NULL,
                    relation_type TEXT NOT NULL,
                    target_id TEXT NOT NULL,
                    justification TEXT,
                    FOREIGN KEY (source_id) REFERENCES manifests(id),
                    FOREIGN KEY (target_id) REFERENCES manifests(id)
                );

                CREATE INDEX IF NOT EXISTS idx_manifests_family ON manifests(family);
                CREATE INDEX IF NOT EXISTS idx_manifests_rung ON manifests(rung);
                CREATE INDEX IF NOT EXISTS idx_skill_process ON skill_records(process_id);
                CREATE INDEX IF NOT EXISTS idx_ontology_source ON ontology_relations(source_id);
                CREATE INDEX IF NOT EXISTS idx_ontology_target ON ontology_relations(target_id);
                ",
            )
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
        Ok(())
    }

    /// Deposit a process manifest into the knowledgebase.
    pub fn deposit_manifest(&self, manifest: &ProcessManifest) -> maesma_core::Result<String> {
        let data = serde_json::to_string(manifest)
            .map_err(|e| maesma_core::Error::Serialization(e.to_string()))?;
        let content_hash = blake3::hash(data.as_bytes()).to_hex().to_string();
        let id_str = manifest.id.to_string();

        self.conn
            .execute(
                "INSERT OR REPLACE INTO manifests (id, name, family, rung, version, lifecycle, content_hash, data)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    id_str,
                    manifest.name,
                    serde_json::to_string(&manifest.family).unwrap_or_default(),
                    serde_json::to_string(&manifest.rung).unwrap_or_default(),
                    manifest.version,
                    serde_json::to_string(&manifest.lifecycle).unwrap_or_default(),
                    content_hash,
                    data,
                ],
            )
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;

        info!(id = %id_str, name = %manifest.name, "Manifest deposited");
        Ok(content_hash)
    }

    /// Retrieve a manifest by process ID.
    pub fn get_manifest(&self, id: ProcessId) -> maesma_core::Result<Option<ProcessManifest>> {
        let id_str = id.to_string();
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM manifests WHERE id = ?1")
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;

        let result = stmt
            .query_row(params![id_str], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })
            .ok();

        match result {
            Some(data) => {
                let manifest: ProcessManifest = serde_json::from_str(&data)
                    .map_err(|e| maesma_core::Error::Serialization(e.to_string()))?;
                Ok(Some(manifest))
            }
            None => Ok(None),
        }
    }

    /// Deposit a skill record.
    pub fn deposit_skill(&self, record: &SkillRecord) -> maesma_core::Result<()> {
        let data = serde_json::to_string(record)
            .map_err(|e| maesma_core::Error::Serialization(e.to_string()))?;

        self.conn
            .execute(
                "INSERT INTO skill_records (process_id, rung, region, dataset, data, evaluated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    record.process_id.to_string(),
                    serde_json::to_string(&record.rung).unwrap_or_default(),
                    record.region,
                    record.dataset,
                    data,
                    record.evaluated_at,
                ],
            )
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;

        Ok(())
    }

    /// List all manifest IDs and names.
    pub fn list_manifests(&self) -> maesma_core::Result<Vec<(String, String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, family FROM manifests ORDER BY family, name")
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;

        let mut results = Vec::new();
        for r in rows.flatten() {
            results.push(r);
        }
        Ok(results)
    }

    /// Count total manifests.
    pub fn manifest_count(&self) -> maesma_core::Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM manifests", [], |row| row.get(0))
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
        Ok(count as usize)
    }

    /// Count total skill records.
    pub fn skill_count(&self) -> maesma_core::Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM skill_records", [], |row| row.get(0))
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
        Ok(count as usize)
    }

    /// Deposit an ontology relation.
    pub fn deposit_relation(&self, relation: &Relation) -> maesma_core::Result<()> {
        let rel_type = serde_json::to_string(&relation.relation)
            .map_err(|e| maesma_core::Error::Serialization(e.to_string()))?;

        self.conn
            .execute(
                "INSERT INTO ontology_relations (source_id, relation_type, target_id, justification)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    relation.source.to_string(),
                    rel_type,
                    relation.target.to_string(),
                    relation.justification,
                ],
            )
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;

        Ok(())
    }

    // ── Validation & closure ─────────────────────────────────────

    /// Retrieve all stored manifests (full objects).
    pub fn all_manifests(&self) -> maesma_core::Result<Vec<ProcessManifest>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM manifests")
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
        let rows = stmt
            .query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })
            .map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
        let mut out = Vec::new();
        for r in rows {
            let data = r.map_err(|e| maesma_core::Error::Knowledgebase(e.to_string()))?;
            let m: ProcessManifest = serde_json::from_str(&data)
                .map_err(|e| maesma_core::Error::Serialization(e.to_string()))?;
            out.push(m);
        }
        Ok(out)
    }

    /// Validate every manifest in the KB. Returns a list of (name, issues).
    pub fn validate_all(&self) -> maesma_core::Result<Vec<(String, Vec<String>)>> {
        let manifests = self.all_manifests()?;
        let mut results = Vec::new();
        for m in &manifests {
            let mut issues = Vec::new();
            // Name must be non-empty
            if m.name.trim().is_empty() {
                issues.push("empty name".into());
            }
            // I/O contract: must declare at least one output
            if m.io.outputs.is_empty() {
                issues.push("no outputs declared".into());
            }
            // Scale envelope sanity
            if m.io.inputs.iter().any(|v| v.unit.is_empty()) {
                issues.push("input variable with empty unit".into());
            }
            if m.io.outputs.iter().any(|v| v.unit.is_empty()) {
                issues.push("output variable with empty unit".into());
            }
            if m.scale.dx_min > m.scale.dx_max {
                issues.push(format!(
                    "dx_min ({}) > dx_max ({})",
                    m.scale.dx_min, m.scale.dx_max
                ));
            }
            if m.scale.dt_min > m.scale.dt_max {
                issues.push(format!(
                    "dt_min ({}) > dt_max ({})",
                    m.scale.dt_min, m.scale.dt_max
                ));
            }
            // Parameters: bounds consistency
            for p in &m.io.parameters {
                if let Some((lo, hi)) = p.bounds {
                    if lo > hi {
                        issues.push(format!("param '{}': lower bound > upper bound", p.name));
                    }
                    if p.default < lo || p.default > hi {
                        issues.push(format!("param '{}': default outside bounds", p.name));
                    }
                }
            }
            if !issues.is_empty() {
                results.push((m.name.clone(), issues));
            }
        }
        Ok(results)
    }

    /// Check state-space closure: every input consumed by any process
    /// must be produced as an output by at least one other process,
    /// or be a recognized forcing variable.
    pub fn check_closure(&self, forcing_vars: &[&str]) -> maesma_core::Result<ClosureReport> {
        let manifests = self.all_manifests()?;
        let mut all_outputs: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut all_inputs: std::collections::HashSet<String> = std::collections::HashSet::new();

        for m in &manifests {
            for v in &m.io.outputs {
                all_outputs.insert(v.name.clone());
            }
            for v in &m.io.inputs {
                all_inputs.insert(v.name.clone());
            }
        }

        let forcing_set: std::collections::HashSet<String> =
            forcing_vars.iter().map(|s| s.to_string()).collect();

        let unsatisfied: Vec<String> = all_inputs
            .iter()
            .filter(|v| !all_outputs.contains(v.as_str()) && !forcing_set.contains(v.as_str()))
            .cloned()
            .collect();

        let unused_outputs: Vec<String> = all_outputs
            .iter()
            .filter(|v| !all_inputs.contains(v.as_str()))
            .cloned()
            .collect();

        Ok(ClosureReport {
            total_inputs: all_inputs.len(),
            total_outputs: all_outputs.len(),
            unsatisfied_inputs: unsatisfied,
            unused_outputs,
        })
    }
}

/// Result of a state-space closure check.
#[derive(Debug, Clone)]
pub struct ClosureReport {
    pub total_inputs: usize,
    pub total_outputs: usize,
    pub unsatisfied_inputs: Vec<String>,
    pub unused_outputs: Vec<String>,
}
