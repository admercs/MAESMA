//! Simulation pipeline — wires KB manifests to real process runners.

use maesma_core::graph::{CouplingEdge, CouplingMode, CouplingStrength, ProcessNode, Sapg};
use maesma_core::manifest::CouplingTier;
use maesma_core::process::ProcessId;
use maesma_processes::create_default_runners;
use tracing::info;

use crate::events::EventBus;
use crate::scheduler::Scheduler;
use crate::state::SimulationState;

/// Build a default simulation pipeline with all available process families.
///
/// Creates a SAPG with one node per family, compiles it into a schedule,
/// instantiates default process runners, and wires everything together.
///
/// Returns (scheduler, state, event_bus) ready to run.
pub fn build_default_pipeline(
    nx: usize,
    ny: usize,
) -> maesma_core::Result<(Scheduler, SimulationState, EventBus)> {
    let runners = create_default_runners();

    // Build SAPG from default runners
    let mut sapg = Sapg::new();
    let mut node_ids = Vec::new();

    for (family, rung, _) in &runners {
        let pid = ProcessId::new();
        let node = ProcessNode {
            process_id: pid.clone(),
            name: format!("{:?}_{:?}", family, rung),
            family: *family,
            rung: *rung,
            tier: match family {
                maesma_core::families::ProcessFamily::Atmosphere
                | maesma_core::families::ProcessFamily::Fire
                | maesma_core::families::ProcessFamily::Radiation => CouplingTier::Fast,
                _ => CouplingTier::Slow,
            },
            cost: 1.0,
        };
        sapg.add_process(node);
        node_ids.push((pid, *family));
    }

    // Add coupling edges between known interacting families
    let coupling_pairs = [
        (
            maesma_core::families::ProcessFamily::Atmosphere,
            maesma_core::families::ProcessFamily::Radiation,
        ),
        (
            maesma_core::families::ProcessFamily::Atmosphere,
            maesma_core::families::ProcessFamily::Hydrology,
        ),
        (
            maesma_core::families::ProcessFamily::Hydrology,
            maesma_core::families::ProcessFamily::Ecology,
        ),
        (
            maesma_core::families::ProcessFamily::Ecology,
            maesma_core::families::ProcessFamily::Biogeochemistry,
        ),
        (
            maesma_core::families::ProcessFamily::Fire,
            maesma_core::families::ProcessFamily::Ecology,
        ),
    ];

    for (fa, fb) in &coupling_pairs {
        for (ida, fam_a) in &node_ids {
            if fam_a != fa {
                continue;
            }
            for (idb, fam_b) in &node_ids {
                if fam_b != fb {
                    continue;
                }
                let _ = sapg.add_coupling(
                    ida.clone(),
                    idb.clone(),
                    CouplingEdge {
                        variables: vec![format!("{:?}->{:?}", fa, fb)],
                        strength: CouplingStrength::Moderate,
                        mode: CouplingMode::Synchronous,
                    },
                );
            }
        }
    }

    // Compile the SAPG into a schedule
    let schedule = maesma_compiler::schedule::generate_schedule(&sapg)?;

    info!(
        nodes = sapg.node_count(),
        edges = sapg.edge_count(),
        stages = schedule.stages.len(),
        "Default pipeline built"
    );

    // Create scheduler and register runners
    let mut scheduler = Scheduler::new(schedule);

    // Map ProcessIds from the SAPG stages to runners
    // Since we created nodes in the same order as runners, pair by index
    let _stage_pids: Vec<ProcessId> = scheduler_process_ids(&scheduler);

    // Register runners for the IDs that appear in the schedule
    let mut runner_iter = runners.into_iter().map(|(_, _, r)| r);
    for pid in &node_ids {
        if let Some(runner) = runner_iter.next() {
            scheduler.register_runner(pid.0.clone(), runner);
        }
    }

    info!(runners = scheduler.runner_count(), "Runners registered");

    // Create simulation state with all required fields
    let mut state = SimulationState::new(nx, ny);
    init_default_fields(&mut state);

    let event_bus = EventBus::new();

    Ok((scheduler, state, event_bus))
}

/// Initialize all fields that the default process runners require.
fn init_default_fields(state: &mut SimulationState) {
    // Radiation (TwoStreamRadiation inputs/outputs)
    state.init_field_const("solar_zenith", 0.5);
    state.init_field_const("lai", 3.0);
    state.init_field_const("sw_down_par", 200.0);
    state.init_field_const("sw_down_nir", 150.0);
    state.init_field_const("frac_diffuse", 0.3);
    state.init_field_const("albedo_soil_par", 0.1);
    state.init_field_const("albedo_soil_nir", 0.2);
    state.init_field("absorbed_par");
    state.init_field("absorbed_nir");
    state.init_field("reflected_sw");
    state.init_field("transmitted_par");
    state.init_field("transmitted_nir");

    // Atmosphere (MOST inputs/outputs)
    state.init_field_const("wind_speed", 3.0);
    state.init_field_const("temperature_air", 288.0);
    state.init_field_const("temperature_surface", 290.0);
    state.init_field_const("temperature", 288.0);
    state.init_field_const("humidity", 0.01);
    state.init_field_const("humidity_surface", 0.012);
    state.init_field("sensible_heat_flux");
    state.init_field("latent_heat_flux");
    state.init_field("friction_velocity");

    // Fire (simple + Rothermel inputs)
    state.init_field_const("fuel_load", 0.5);
    state.init_field_const("weather_fire_danger_index", 0.3);
    state.init_field_const("fuel_moisture", 0.08);
    state.init_field_const("terrain_slope", 0.0);
    state.init_field("burned_area_fraction");
    state.init_field("rate_of_spread");

    // Fire (CFSFBP inputs)
    state.init_field_const("ffmc", 85.0);
    state.init_field_const("bui", 60.0);
    state.init_field_const("wind_speed_10m", 5.0);
    state.init_field_const("foliar_moisture_content", 100.0);

    // Hydrology (SCS + Richards inputs/outputs)
    state.init_field_const("precipitation", 5e-5);
    state.init_field_const("potential_et", 3e-5);
    state.init_field_const("soil_moisture", 0.3);
    state.init_field("runoff");
    state.init_field("infiltration");
    state.init_field("evaporation");

    // Ocean (slab ocean inputs/outputs)
    state.init_field_const("sst", 290.0);
    state.init_field_const("mixed_layer_depth", 50.0);
    state.init_field_const("solar_radiation", 200.0);
    state.init_field_const("longwave_net", -40.0);
    state.init_field_const("sensible_heat", -20.0);
    state.init_field_const("latent_heat", -80.0);
    state.init_field_const("wind_stress", 0.1);

    // Cryosphere (snowpack inputs/outputs)
    state.init_field_const("sw_down", 200.0);
    state.init_field_const("lw_down", 300.0);
    state.init_field_const("swe", 50.0);
    state.init_field_const("snow_albedo", 0.8);
    state.init_field("snowmelt");

    // Biogeochemistry (CENTURY inputs/outputs)
    state.init_field_const("litter_input", 0.01);
    state.init_field_const("moisture", 0.3);
    state.init_field_const("lignin_fraction", 0.2);
    state.init_field_const("soil_carbon_active", 2.0);
    state.init_field_const("soil_carbon_slow", 10.0);
    state.init_field_const("soil_carbon_passive", 20.0);
    state.init_field("soil_respiration");

    // Ecology (LPJ-style inputs/outputs)
    state.init_field_const("light", 200.0);
    state.init_field_const("co2", 400.0);
    state.init_field_const("biomass", 10.0);
    state.init_field("npp");
    state.init_field("mortality");

    // Trophic dynamics
    state.init_field_const("prey_biomass", 100.0);
    state.init_field_const("predator_biomass", 10.0);
    state.init_field_const("carrying_capacity", 500.0);

    // Evolution
    state.init_field_const("trait_mean", 0.5);
    state.init_field_const("trait_variance", 0.1);
    state.init_field_const("selection_gradient", 0.02);

    // Human systems (land use transition inputs)
    state.init_field_const("population_density", 50.0);
    state.init_field_const("economic_driver", 0.5);
    state.init_field_const("policy", 0.3);
    state.init_field_const("land_use_fractions", 0.3);

    // Parameters
    state.set_param("latitude", 45.0);
    state.set_param("dt", 86400.0);
}
/// Extract all ProcessIds from the scheduler's schedule stages.
fn scheduler_process_ids(_scheduler: &Scheduler) -> Vec<ProcessId> {
    // The scheduler owns the schedule internally; we rely on node_ids ordering
    Vec::new()
}
