//! Query builder for knowledgebase lookups.

use maesma_core::families::ProcessFamily;
use maesma_core::process::FidelityRung;

/// Fluent query builder for searching the knowledgebase.
#[derive(Debug, Clone, Default)]
pub struct QueryBuilder {
    family: Option<ProcessFamily>,
    rung: Option<FidelityRung>,
    regime_tags: Vec<String>,
    region: Option<String>,
    name_contains: Option<String>,
    limit: Option<usize>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn family(mut self, family: ProcessFamily) -> Self {
        self.family = Some(family);
        self
    }

    pub fn rung(mut self, rung: FidelityRung) -> Self {
        self.rung = Some(rung);
        self
    }

    pub fn regime_tag(mut self, tag: impl Into<String>) -> Self {
        self.regime_tags.push(tag.into());
        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn name_contains(mut self, pattern: impl Into<String>) -> Self {
        self.name_contains = Some(pattern.into());
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Build a SQL WHERE clause and params for manifest queries.
    pub fn to_sql(&self) -> (String, Vec<String>) {
        let mut clauses = Vec::new();
        let mut params = Vec::new();

        if let Some(ref fam) = self.family {
            let fam_json = serde_json::to_string(fam).unwrap_or_default();
            clauses.push(format!("family = ?{}", params.len() + 1));
            params.push(fam_json);
        }

        if let Some(ref rung) = self.rung {
            let rung_json = serde_json::to_string(rung).unwrap_or_default();
            clauses.push(format!("rung = ?{}", params.len() + 1));
            params.push(rung_json);
        }

        if let Some(ref name) = self.name_contains {
            clauses.push(format!("name LIKE ?{}", params.len() + 1));
            params.push(format!("%{}%", name));
        }

        let where_clause = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let limit_clause = self
            .limit
            .map(|n| format!(" LIMIT {}", n))
            .unwrap_or_default();

        let sql = format!(
            "SELECT data FROM manifests {} ORDER BY family, name{}",
            where_clause, limit_clause
        );

        (sql, params)
    }
}
