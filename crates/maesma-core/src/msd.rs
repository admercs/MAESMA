//! MultiSector Dynamics types — Phase 9
//!
//! Human system coupling levels, water-energy-land nexus, digital testbed
//! definitions, and scenario space exploration.

use serde::{Deserialize, Serialize};

/// Human system coupling level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsCouplingLevel {
    /// HS0: one-way (exogenous scenarios).
    Hs0,
    /// HS1: bidirectional via MSD coupling agent.
    Hs1,
    /// HS2: agent-based infrastructure (stretch).
    Hs2,
}

/// A resource accounting entry for water-energy-land nexus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusAccounting {
    pub water_demand_m3: f64,
    pub energy_demand_gwh: f64,
    pub land_use_km2: f64,
    pub competition_resolved: bool,
}

/// Human system exchange: commodities and constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanSystemExchange {
    pub from_sector: String,
    pub to_sector: String,
    pub commodity: String,
    pub direction: CouplingDirection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CouplingDirection {
    NaturalToHuman,
    HumanToNatural,
    Bidirectional,
}

/// A dynamical digital testbed definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTestbed {
    pub name: String,
    pub region: String,
    pub stressors: Vec<String>,
    pub coupled_families: Vec<String>,
    pub template: TestbedTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestbedTemplate {
    Coastal,
    WesternUs,
    Arctic,
    Custom,
}

/// DOE template testbeds (Phase 9.3).
pub fn doe_testbeds() -> Vec<DigitalTestbed> {
    vec![
        DigitalTestbed {
            name: "Coastal Inundation".into(),
            region: "US East Coast".into(),
            stressors: vec![
                "sea_level_rise".into(),
                "storm_surge".into(),
                "subsidence".into(),
            ],
            coupled_families: vec!["ocean".into(), "hydrology".into(), "human_systems".into()],
            template: TestbedTemplate::Coastal,
        },
        DigitalTestbed {
            name: "Western US Fire-Water-Energy".into(),
            region: "Western US".into(),
            stressors: vec!["drought".into(), "wildfire".into(), "heatwave".into()],
            coupled_families: vec![
                "fire".into(),
                "hydrology".into(),
                "atmosphere".into(),
                "human_systems".into(),
            ],
            template: TestbedTemplate::WesternUs,
        },
        DigitalTestbed {
            name: "Arctic Systems".into(),
            region: "Pan-Arctic".into(),
            stressors: vec![
                "permafrost_thaw".into(),
                "sea_ice_loss".into(),
                "arctic_amplification".into(),
            ],
            coupled_families: vec![
                "cryosphere".into(),
                "biogeochemistry".into(),
                "ocean".into(),
                "atmosphere".into(),
            ],
            template: TestbedTemplate::Arctic,
        },
    ]
}

/// Scenario definition for MSD ensemble.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsdScenarioDef {
    pub ssp: String,
    pub rcp: String,
    pub technology: String,
    pub policy: String,
}

/// Standard SSP × RCP combinations.
pub fn standard_scenario_matrix() -> Vec<MsdScenarioDef> {
    vec![
        MsdScenarioDef {
            ssp: "SSP1".into(),
            rcp: "RCP2.6".into(),
            technology: "green".into(),
            policy: "aggressive_mitigation".into(),
        },
        MsdScenarioDef {
            ssp: "SSP2".into(),
            rcp: "RCP4.5".into(),
            technology: "mixed".into(),
            policy: "moderate".into(),
        },
        MsdScenarioDef {
            ssp: "SSP3".into(),
            rcp: "RCP7.0".into(),
            technology: "fragmented".into(),
            policy: "weak".into(),
        },
        MsdScenarioDef {
            ssp: "SSP5".into(),
            rcp: "RCP8.5".into(),
            technology: "fossil_intensive".into(),
            policy: "none".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doe_testbeds_populated() {
        let t = doe_testbeds();
        assert_eq!(t.len(), 3);
        assert!(t.iter().any(|tb| tb.template == TestbedTemplate::Arctic));
    }

    #[test]
    fn scenario_matrix_populated() {
        let m = standard_scenario_matrix();
        assert_eq!(m.len(), 4);
        assert!(m.iter().any(|s| s.ssp == "SSP5"));
    }

    #[test]
    fn nexus_accounting() {
        let n = NexusAccounting {
            water_demand_m3: 1e6,
            energy_demand_gwh: 50.0,
            land_use_km2: 100.0,
            competition_resolved: false,
        };
        assert!(!n.competition_resolved);
    }
}
