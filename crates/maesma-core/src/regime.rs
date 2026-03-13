//! Regime — environmental regime tagging for context-aware selection.

use serde::{Deserialize, Serialize};

/// A regime tag identifying an environmental context.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegimeTag(pub String);

impl RegimeTag {
    pub fn new(tag: impl Into<String>) -> Self {
        Self(tag.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RegimeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A regime descriptor combining biome, climate, and disturbance context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regime {
    /// Primary biome tag (e.g., "boreal_forest", "savanna", "tundra").
    pub biome: RegimeTag,
    /// Climate zone (e.g., "tropical", "arid", "temperate", "continental", "polar").
    pub climate: RegimeTag,
    /// Active disturbance regimes (e.g., "fire_prone", "flood_prone", "drought_prone").
    pub disturbances: Vec<RegimeTag>,
    /// Management context (e.g., "managed_forest", "cropland", "wildland").
    pub management: Option<RegimeTag>,
    /// Seasonal context if relevant.
    pub season: Option<RegimeTag>,
}

impl Regime {
    /// Collect all tags into a flat vector.
    pub fn all_tags(&self) -> Vec<&RegimeTag> {
        let mut tags = vec![&self.biome, &self.climate];
        tags.extend(self.disturbances.iter());
        if let Some(ref m) = self.management {
            tags.push(m);
        }
        if let Some(ref s) = self.season {
            tags.push(s);
        }
        tags
    }

    /// Check whether this regime matches a set of required tags.
    pub fn matches_all(&self, required: &[RegimeTag]) -> bool {
        let all = self.all_tags();
        required.iter().all(|req| all.iter().any(|t| t.0 == req.0))
    }
}

// ── Well-known regime tags ───────────────────────────────────────────

pub mod tags {
    use super::RegimeTag;

    // Biomes
    pub fn boreal_forest() -> RegimeTag {
        RegimeTag::new("boreal_forest")
    }
    pub fn temperate_forest() -> RegimeTag {
        RegimeTag::new("temperate_forest")
    }
    pub fn tropical_forest() -> RegimeTag {
        RegimeTag::new("tropical_forest")
    }
    pub fn savanna() -> RegimeTag {
        RegimeTag::new("savanna")
    }
    pub fn grassland() -> RegimeTag {
        RegimeTag::new("grassland")
    }
    pub fn tundra() -> RegimeTag {
        RegimeTag::new("tundra")
    }
    pub fn desert() -> RegimeTag {
        RegimeTag::new("desert")
    }
    pub fn wetland() -> RegimeTag {
        RegimeTag::new("wetland")
    }
    pub fn cropland() -> RegimeTag {
        RegimeTag::new("cropland")
    }
    pub fn urban() -> RegimeTag {
        RegimeTag::new("urban")
    }

    // Disturbances
    pub fn fire_prone() -> RegimeTag {
        RegimeTag::new("fire_prone")
    }
    pub fn flood_prone() -> RegimeTag {
        RegimeTag::new("flood_prone")
    }
    pub fn drought_prone() -> RegimeTag {
        RegimeTag::new("drought_prone")
    }
    pub fn insect_outbreak() -> RegimeTag {
        RegimeTag::new("insect_outbreak")
    }
    pub fn permafrost_thaw() -> RegimeTag {
        RegimeTag::new("permafrost_thaw")
    }

    // Seasons
    pub fn dry_season() -> RegimeTag {
        RegimeTag::new("dry_season")
    }
    pub fn wet_season() -> RegimeTag {
        RegimeTag::new("wet_season")
    }
    pub fn fire_season() -> RegimeTag {
        RegimeTag::new("fire_season")
    }
    pub fn growing_season() -> RegimeTag {
        RegimeTag::new("growing_season")
    }
    pub fn dormant_season() -> RegimeTag {
        RegimeTag::new("dormant_season")
    }
}
