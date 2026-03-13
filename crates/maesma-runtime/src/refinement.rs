//! Runtime refinement — rung upgrade/downgrade engine and disturbance pipeline.
//!
//! Implements adaptive fidelity control: when skill metrics or residuals
//! indicate a model is insufficient, the refinement engine can upgrade
//! a process to a higher-fidelity rung (or downgrade when unnecessary).
//!
//! The disturbance pipeline routes disturbance events (fire, flood, storm)
//! to the appropriate process runners and state modifications.

use maesma_core::families::ProcessFamily;
use maesma_core::process::FidelityRung;

// ---------------------------------------------------------------------------
// Refinement Triggers
// ---------------------------------------------------------------------------

/// Trigger condition for rung upgrade.
#[derive(Debug, Clone)]
pub enum RefinementTrigger {
    /// Skill metric dropped below threshold.
    SkillBelow {
        family: ProcessFamily,
        metric: String,
        threshold: f64,
        current: f64,
    },
    /// Residual analysis detected systematic bias.
    ResidualBias {
        family: ProcessFamily,
        bias: f64,
        threshold: f64,
    },
    /// Event-driven (e.g., fire detected, flood onset).
    EventDriven {
        family: ProcessFamily,
        event_type: String,
    },
    /// Computational budget allows higher fidelity.
    BudgetAvailable {
        family: ProcessFamily,
        available_flops: f64,
    },
}

impl RefinementTrigger {
    /// The family this trigger applies to.
    pub fn family(&self) -> ProcessFamily {
        match self {
            Self::SkillBelow { family, .. } => *family,
            Self::ResidualBias { family, .. } => *family,
            Self::EventDriven { family, .. } => *family,
            Self::BudgetAvailable { family, .. } => *family,
        }
    }
}

/// Decision from the refinement engine.
#[derive(Debug, Clone)]
pub enum RefinementAction {
    /// Upgrade to a higher rung.
    Upgrade {
        family: ProcessFamily,
        from: FidelityRung,
        to: FidelityRung,
        reason: String,
    },
    /// Downgrade to a lower rung (save compute).
    Downgrade {
        family: ProcessFamily,
        from: FidelityRung,
        to: FidelityRung,
        reason: String,
    },
    /// Stay at current rung.
    Hold {
        family: ProcessFamily,
        current: FidelityRung,
    },
}

// ---------------------------------------------------------------------------
// Refinement Engine
// ---------------------------------------------------------------------------

/// Engine that evaluates triggers and decides rung transitions.
pub struct RefinementEngine {
    /// Minimum KGE below which an upgrade is warranted.
    pub skill_threshold: f64,
    /// Maximum acceptable bias magnitude.
    pub bias_threshold: f64,
    /// Whether to allow automatic downgrades.
    pub allow_downgrade: bool,
    /// History of actions taken.
    pub history: Vec<RefinementAction>,
}

impl Default for RefinementEngine {
    fn default() -> Self {
        Self {
            skill_threshold: 0.5,
            bias_threshold: 0.1,
            allow_downgrade: true,
            history: Vec::new(),
        }
    }
}

impl RefinementEngine {
    /// Evaluate a trigger and decide what to do.
    pub fn evaluate(
        &mut self,
        trigger: &RefinementTrigger,
        current_rung: FidelityRung,
    ) -> RefinementAction {
        let action = match trigger {
            RefinementTrigger::SkillBelow {
                family,
                metric,
                threshold,
                current,
            } => {
                if *current < *threshold && current_rung < FidelityRung::R3 {
                    RefinementAction::Upgrade {
                        family: *family,
                        from: current_rung,
                        to: next_rung(current_rung),
                        reason: format!("{metric} = {current:.3} < {threshold:.3}: upgrade needed"),
                    }
                } else {
                    RefinementAction::Hold {
                        family: *family,
                        current: current_rung,
                    }
                }
            }
            RefinementTrigger::ResidualBias {
                family,
                bias,
                threshold,
            } => {
                if bias.abs() > *threshold && current_rung < FidelityRung::R3 {
                    RefinementAction::Upgrade {
                        family: *family,
                        from: current_rung,
                        to: next_rung(current_rung),
                        reason: format!(
                            "Systematic bias {bias:.4} > {threshold:.4}: upgrade for physics"
                        ),
                    }
                } else {
                    RefinementAction::Hold {
                        family: *family,
                        current: current_rung,
                    }
                }
            }
            RefinementTrigger::EventDriven { family, event_type } => {
                // Events always trigger upgrade to at least R1
                if current_rung < FidelityRung::R1 {
                    RefinementAction::Upgrade {
                        family: *family,
                        from: current_rung,
                        to: FidelityRung::R1,
                        reason: format!("Event '{event_type}' requires higher fidelity"),
                    }
                } else {
                    RefinementAction::Hold {
                        family: *family,
                        current: current_rung,
                    }
                }
            }
            RefinementTrigger::BudgetAvailable { family, .. } => {
                if self.allow_downgrade && current_rung > FidelityRung::R0 {
                    // Could downgrade to save budget, but only if skill is satisfactory
                    RefinementAction::Hold {
                        family: *family,
                        current: current_rung,
                    }
                } else {
                    RefinementAction::Hold {
                        family: *family,
                        current: current_rung,
                    }
                }
            }
        };

        self.history.push(action.clone());
        action
    }

    /// Request a downgrade for a family (e.g., after event subsides).
    pub fn request_downgrade(
        &mut self,
        family: ProcessFamily,
        current: FidelityRung,
        reason: &str,
    ) -> RefinementAction {
        if self.allow_downgrade && current > FidelityRung::R0 {
            let action = RefinementAction::Downgrade {
                family,
                from: current,
                to: prev_rung(current),
                reason: reason.to_string(),
            };
            self.history.push(action.clone());
            action
        } else {
            let action = RefinementAction::Hold { family, current };
            self.history.push(action.clone());
            action
        }
    }
}

/// Get the next higher rung, capped at R3.
fn next_rung(rung: FidelityRung) -> FidelityRung {
    match rung {
        FidelityRung::R0 => FidelityRung::R1,
        FidelityRung::R1 => FidelityRung::R2,
        FidelityRung::R2 => FidelityRung::R3,
        FidelityRung::R3 => FidelityRung::R3,
    }
}

/// Get the next lower rung, capped at R0.
fn prev_rung(rung: FidelityRung) -> FidelityRung {
    match rung {
        FidelityRung::R0 => FidelityRung::R0,
        FidelityRung::R1 => FidelityRung::R0,
        FidelityRung::R2 => FidelityRung::R1,
        FidelityRung::R3 => FidelityRung::R2,
    }
}

// ---------------------------------------------------------------------------
// Disturbance Pipeline
// ---------------------------------------------------------------------------

/// A disturbance event that modifies simulation state.
#[derive(Debug, Clone)]
pub struct DisturbanceEvent {
    /// Which family generated the disturbance.
    pub source_family: ProcessFamily,
    /// Type of disturbance.
    pub disturbance_type: DisturbanceType,
    /// Affected grid cells (indices).
    pub affected_cells: Vec<usize>,
    /// Severity [0-1] per affected cell.
    pub severity: Vec<f64>,
    /// Timestamp (simulation step).
    pub step: u64,
}

/// Type of disturbance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisturbanceType {
    /// Wild/prescribed fire.
    Fire,
    /// Flood / inundation.
    Flood,
    /// Wind damage (storm, tornado, hurricane).
    WindDamage,
    /// Drought.
    Drought,
    /// Insect/pathogen outbreak.
    BiologicalOutbreak,
    /// Landslide / mass wasting.
    MassWasting,
    /// Volcanic eruption.
    Volcanic,
    /// Harvest / logging.
    Harvest,
    /// Land-use conversion.
    LandUseConversion,
}

/// Pipeline that processes disturbance events and modifies state.
#[derive(Debug, Default)]
pub struct DisturbancePipeline {
    /// Pending events to process.
    pending: Vec<DisturbanceEvent>,
    /// Processed events (history).
    processed: Vec<DisturbanceEvent>,
}

impl DisturbancePipeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queue a disturbance event.
    pub fn queue(&mut self, event: DisturbanceEvent) {
        self.pending.push(event);
    }

    /// Number of pending events.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Process all pending events, returning state modifications.
    ///
    /// Each returned `StateModification` describes what field to modify
    /// and how (multiply by severity factor, set to zero, etc.).
    pub fn process_pending(&mut self) -> Vec<StateModification> {
        let mut mods = Vec::new();

        for event in self.pending.drain(..) {
            let modifications = match event.disturbance_type {
                DisturbanceType::Fire => fire_disturbance_mods(&event),
                DisturbanceType::Flood => flood_disturbance_mods(&event),
                DisturbanceType::WindDamage => wind_disturbance_mods(&event),
                DisturbanceType::Harvest => harvest_disturbance_mods(&event),
                _ => {
                    // Generic: reduce biomass by severity
                    generic_disturbance_mods(&event)
                }
            };
            mods.extend(modifications);
            self.processed.push(event);
        }

        mods
    }

    /// Total processed events.
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }
}

/// A modification to apply to simulation state.
#[derive(Debug, Clone)]
pub struct StateModification {
    /// Field to modify.
    pub field: String,
    /// Cell indices.
    pub cells: Vec<usize>,
    /// Modification type.
    pub modification: ModificationType,
}

/// How to modify a field.
#[derive(Debug, Clone)]
pub enum ModificationType {
    /// Multiply by a factor per cell.
    Scale(Vec<f64>),
    /// Set to a value per cell.
    Set(Vec<f64>),
    /// Add a delta per cell.
    Add(Vec<f64>),
}

fn fire_disturbance_mods(event: &DisturbanceEvent) -> Vec<StateModification> {
    // Fire kills vegetation proportional to severity
    let biomass_factors: Vec<f64> = event.severity.iter().map(|s| 1.0 - s).collect();
    // Fire removes litter
    let litter_factors: Vec<f64> = event.severity.iter().map(|s| 1.0 - 0.8 * s).collect();
    // Fire resets LAI
    let lai_factors: Vec<f64> = event.severity.iter().map(|s| 1.0 - 0.9 * s).collect();
    // Fire adds charcoal to soil carbon
    let char_additions: Vec<f64> = event.severity.iter().map(|s| s * 0.05).collect();

    vec![
        StateModification {
            field: "biomass".into(),
            cells: event.affected_cells.clone(),
            modification: ModificationType::Scale(biomass_factors),
        },
        StateModification {
            field: "litter".into(),
            cells: event.affected_cells.clone(),
            modification: ModificationType::Scale(litter_factors),
        },
        StateModification {
            field: "lai".into(),
            cells: event.affected_cells.clone(),
            modification: ModificationType::Scale(lai_factors),
        },
        StateModification {
            field: "soil_carbon".into(),
            cells: event.affected_cells.clone(),
            modification: ModificationType::Add(char_additions),
        },
    ]
}

fn flood_disturbance_mods(event: &DisturbanceEvent) -> Vec<StateModification> {
    // Floods saturate soil
    let sat_vals: Vec<f64> = event.severity.to_vec();
    vec![StateModification {
        field: "soil_moisture".into(),
        cells: event.affected_cells.clone(),
        modification: ModificationType::Set(sat_vals),
    }]
}

fn wind_disturbance_mods(event: &DisturbanceEvent) -> Vec<StateModification> {
    // Wind kills trees proportional to severity
    let factors: Vec<f64> = event.severity.iter().map(|s| 1.0 - 0.7 * s).collect();
    // Downed trees become litter
    let litter: Vec<f64> = event.severity.iter().map(|s| s * 0.5).collect();
    vec![
        StateModification {
            field: "biomass".into(),
            cells: event.affected_cells.clone(),
            modification: ModificationType::Scale(factors),
        },
        StateModification {
            field: "litter".into(),
            cells: event.affected_cells.clone(),
            modification: ModificationType::Add(litter),
        },
    ]
}

fn harvest_disturbance_mods(event: &DisturbanceEvent) -> Vec<StateModification> {
    let factors: Vec<f64> = event.severity.iter().map(|s| 1.0 - 0.8 * s).collect();
    vec![StateModification {
        field: "biomass".into(),
        cells: event.affected_cells.clone(),
        modification: ModificationType::Scale(factors),
    }]
}

fn generic_disturbance_mods(event: &DisturbanceEvent) -> Vec<StateModification> {
    let factors: Vec<f64> = event.severity.iter().map(|s| 1.0 - 0.5 * s).collect();
    vec![StateModification {
        field: "biomass".into(),
        cells: event.affected_cells.clone(),
        modification: ModificationType::Scale(factors),
    }]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_below_triggers_upgrade() {
        let mut engine = RefinementEngine::default();
        let trigger = RefinementTrigger::SkillBelow {
            family: ProcessFamily::Hydrology,
            metric: "kge".into(),
            threshold: 0.5,
            current: 0.3,
        };
        let action = engine.evaluate(&trigger, FidelityRung::R0);
        match action {
            RefinementAction::Upgrade { from, to, .. } => {
                assert_eq!(from, FidelityRung::R0);
                assert_eq!(to, FidelityRung::R1);
            }
            _ => panic!("Expected upgrade"),
        }
    }

    #[test]
    fn test_good_skill_holds() {
        let mut engine = RefinementEngine::default();
        let trigger = RefinementTrigger::SkillBelow {
            family: ProcessFamily::Hydrology,
            metric: "kge".into(),
            threshold: 0.5,
            current: 0.8,
        };
        let action = engine.evaluate(&trigger, FidelityRung::R1);
        assert!(matches!(action, RefinementAction::Hold { .. }));
    }

    #[test]
    fn test_r3_cannot_upgrade() {
        let mut engine = RefinementEngine::default();
        let trigger = RefinementTrigger::SkillBelow {
            family: ProcessFamily::Fire,
            metric: "rmse".into(),
            threshold: 0.5,
            current: 0.1,
        };
        let action = engine.evaluate(&trigger, FidelityRung::R3);
        assert!(matches!(action, RefinementAction::Hold { .. }));
    }

    #[test]
    fn test_event_triggers_upgrade_from_r0() {
        let mut engine = RefinementEngine::default();
        let trigger = RefinementTrigger::EventDriven {
            family: ProcessFamily::Fire,
            event_type: "wildfire".into(),
        };
        let action = engine.evaluate(&trigger, FidelityRung::R0);
        match action {
            RefinementAction::Upgrade { to, .. } => {
                assert_eq!(to, FidelityRung::R1);
            }
            _ => panic!("Expected upgrade"),
        }
    }

    #[test]
    fn test_downgrade() {
        let mut engine = RefinementEngine::default();
        let action = engine.request_downgrade(ProcessFamily::Hydrology, FidelityRung::R2, "budget");
        match action {
            RefinementAction::Downgrade { from, to, .. } => {
                assert_eq!(from, FidelityRung::R2);
                assert_eq!(to, FidelityRung::R1);
            }
            _ => panic!("Expected downgrade"),
        }
    }

    #[test]
    fn test_disturbance_pipeline_fire() {
        let mut pipeline = DisturbancePipeline::new();
        pipeline.queue(DisturbanceEvent {
            source_family: ProcessFamily::Fire,
            disturbance_type: DisturbanceType::Fire,
            affected_cells: vec![0, 1, 2],
            severity: vec![0.8, 0.5, 0.3],
            step: 100,
        });

        assert_eq!(pipeline.pending_count(), 1);
        let mods = pipeline.process_pending();
        assert_eq!(pipeline.pending_count(), 0);
        assert_eq!(pipeline.processed_count(), 1);
        // Fire produces 4 modifications: biomass, litter, lai, soil_carbon
        assert_eq!(mods.len(), 4);
    }

    #[test]
    fn test_disturbance_pipeline_flood() {
        let mut pipeline = DisturbancePipeline::new();
        pipeline.queue(DisturbanceEvent {
            source_family: ProcessFamily::Hydrology,
            disturbance_type: DisturbanceType::Flood,
            affected_cells: vec![5],
            severity: vec![1.0],
            step: 50,
        });
        let mods = pipeline.process_pending();
        assert_eq!(mods.len(), 1);
        assert_eq!(mods[0].field, "soil_moisture");
    }
}
