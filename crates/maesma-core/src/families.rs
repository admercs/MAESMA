//! Process families — the 13 top-level organizational categories for
//! process representations in the MAESMA knowledgebase.

use serde::{Deserialize, Serialize};

/// The 13 MAESMA process families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessFamily {
    /// Fire behavior and effects (F0–F3).
    Fire,
    /// Hydrology: infiltration, runoff, routing, groundwater.
    Hydrology,
    /// Ecology: succession, competition, demography, phenology.
    Ecology,
    /// Biogeochemistry: C/N/P cycling, decomposition, emissions.
    Biogeochemistry,
    /// Radiation: shortwave/longwave, canopy transfer, albedo.
    Radiation,
    /// Atmosphere: dynamics, convection, microphysics, chemistry.
    Atmosphere,
    /// Ocean: circulation, mixing, marine BGC, waves.
    Ocean,
    /// Cryosphere: snow, sea ice, glaciers, permafrost.
    Cryosphere,
    /// Geomorphology: erosion, sediment transport, landscape evolution.
    Geomorphology,
    /// Geology: reactive transport, subsurface flow, tectonics.
    Geology,
    /// Human systems: land use, agriculture, infrastructure, IAMs.
    HumanSystems,
    /// Trophic dynamics: food webs, predator-prey, metabolic ecology.
    TrophicDynamics,
    /// Evolution & phylogeography: trait evolution, speciation, gene flow.
    Evolution,
}

impl ProcessFamily {
    /// Returns all process families as a slice.
    pub fn all() -> &'static [ProcessFamily] {
        &[
            Self::Fire,
            Self::Hydrology,
            Self::Ecology,
            Self::Biogeochemistry,
            Self::Radiation,
            Self::Atmosphere,
            Self::Ocean,
            Self::Cryosphere,
            Self::Geomorphology,
            Self::Geology,
            Self::HumanSystems,
            Self::TrophicDynamics,
            Self::Evolution,
        ]
    }

    /// Short identifier string.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Fire => "F",
            Self::Hydrology => "H",
            Self::Ecology => "E",
            Self::Biogeochemistry => "B",
            Self::Radiation => "R",
            Self::Atmosphere => "A",
            Self::Ocean => "O",
            Self::Cryosphere => "C",
            Self::Geomorphology => "GM",
            Self::Geology => "G",
            Self::HumanSystems => "HS",
            Self::TrophicDynamics => "TD",
            Self::Evolution => "EV",
        }
    }

    /// Human-readable display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Fire => "Fire",
            Self::Hydrology => "Hydrology",
            Self::Ecology => "Ecology",
            Self::Biogeochemistry => "Biogeochemistry",
            Self::Radiation => "Radiation",
            Self::Atmosphere => "Atmosphere",
            Self::Ocean => "Ocean",
            Self::Cryosphere => "Cryosphere",
            Self::Geomorphology => "Geomorphology",
            Self::Geology => "Geology",
            Self::HumanSystems => "Human Systems",
            Self::TrophicDynamics => "Trophic Dynamics",
            Self::Evolution => "Evolution & Phylogeography",
        }
    }
}

impl std::fmt::Display for ProcessFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
