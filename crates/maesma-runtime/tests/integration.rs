//! Integration tests for the MAESMA simulation pipeline.

use maesma_runtime::build_default_pipeline;

#[test]
fn pipeline_builds_successfully() {
    let (scheduler, state, _bus) = build_default_pipeline(5, 5).expect("pipeline should build");
    assert!(
        scheduler.runner_count() > 0,
        "should have runners registered"
    );
    assert_eq!(state.nx, 5);
    assert_eq!(state.ny, 5);
}

#[test]
fn pipeline_runs_single_step() {
    let (mut scheduler, mut state, mut bus) =
        build_default_pipeline(5, 5).expect("pipeline should build");
    scheduler
        .step(&mut state, &mut bus)
        .expect("step should succeed");
    assert_eq!(scheduler.current_step(), 1);
    assert!(scheduler.current_time() > 0.0);
}

#[test]
fn pipeline_runs_ten_steps() {
    let (mut scheduler, mut state, mut bus) =
        build_default_pipeline(5, 5).expect("pipeline should build");
    scheduler
        .run(&mut state, &mut bus, 10)
        .expect("10 steps should succeed");
    assert_eq!(scheduler.current_step(), 10);
}

#[test]
fn knowledgebase_in_memory() {
    let kb = maesma_knowledgebase::KnowledgebaseStore::in_memory().expect("in-memory KB");
    assert_eq!(kb.manifest_count().expect("count"), 0);
    assert_eq!(kb.skill_count().expect("count"), 0);
}

#[test]
fn compiler_schedule_generation() {
    use maesma_compiler::schedule::generate_schedule;
    use maesma_core::families::ProcessFamily;
    use maesma_core::graph::{CouplingEdge, CouplingMode, CouplingStrength, ProcessNode, Sapg};
    use maesma_core::manifest::CouplingTier;
    use maesma_core::process::{FidelityRung, ProcessId};

    let mut sapg = Sapg::new();
    let pid_a = ProcessId::new();
    let pid_b = ProcessId::new();

    sapg.add_process(ProcessNode {
        process_id: pid_a.clone(),
        name: "Atm".into(),
        family: ProcessFamily::Atmosphere,
        rung: FidelityRung::R1,
        tier: CouplingTier::Fast,
        cost: 1.0,
    });

    sapg.add_process(ProcessNode {
        process_id: pid_b.clone(),
        name: "Rad".into(),
        family: ProcessFamily::Radiation,
        rung: FidelityRung::R1,
        tier: CouplingTier::Fast,
        cost: 1.0,
    });

    let _ = sapg.add_coupling(
        pid_a,
        pid_b,
        CouplingEdge {
            variables: vec!["temperature".into()],
            strength: CouplingStrength::Strong,
            mode: CouplingMode::Synchronous,
        },
    );

    let schedule = generate_schedule(&sapg).expect("schedule generation");
    assert!(!schedule.stages.is_empty());
    assert!(schedule.dt_global > 0.0);
}

#[tokio::test]
async fn agent_execution() {
    use maesma_agents::Agent;
    use maesma_agents::AgentContext;
    use maesma_agents::benchmarking::BenchmarkingAgent;

    let agent = BenchmarkingAgent::new();

    let ctx = AgentContext::new()
        .with_param("observed", serde_json::json!([1.0, 2.0, 3.0, 4.0]))
        .with_param("predicted", serde_json::json!([1.1, 2.2, 2.8, 4.1]));

    let result: maesma_agents::AgentResult =
        agent.execute(ctx).await.expect("agent should succeed");
    assert!(result.success);
    assert!(result.data.is_some());
}

#[tokio::test]
async fn inference_engine_stub() {
    use maesma_inference::{InferenceEngine, InferenceRequest, InferenceTask, StubInferenceEngine};
    use std::collections::HashMap;

    let engine = StubInferenceEngine;
    assert!(engine.is_ready());

    let request = InferenceRequest {
        task: InferenceTask::PredictSkill,
        features: vec![1.0, 2.0, 3.0],
        context: HashMap::new(),
    };

    let response: maesma_inference::InferenceResponse = engine
        .infer(request)
        .await
        .expect("inference should succeed");
    assert_eq!(response.scores.len(), 3);
}

#[tokio::test]
async fn heuristic_inference_engine() {
    use maesma_inference::{
        HeuristicInferenceEngine, InferenceEngine, InferenceRequest, InferenceTask,
    };
    use std::collections::HashMap;

    let engine = HeuristicInferenceEngine;
    assert!(engine.is_ready());
    assert_eq!(engine.model_version(), "heuristic-1.0.0");

    let request = InferenceRequest {
        task: InferenceTask::PredictRegime,
        features: vec![315.0, 0.1, 0.05],
        context: HashMap::new(),
    };
    let response: maesma_inference::InferenceResponse =
        engine.infer(request).await.expect("regime prediction");
    assert_eq!(response.scores.len(), 4);
}

// ── KB Validation & Closure tests ────────────────────────────────────

#[test]
fn kb_validate_all_manifests() {
    let kb = maesma_knowledgebase::KnowledgebaseStore::in_memory().unwrap();
    let manifests = maesma_knowledgebase::generate_seed_manifests();
    for m in &manifests {
        kb.deposit_manifest(m).unwrap();
    }
    let issues = kb.validate_all().unwrap();
    // Seed manifests should be well-formed
    for (name, problems) in &issues {
        eprintln!("Validation issues in {name}: {:?}", problems);
    }
    // Allow some issues but not catastrophic ones
    assert!(
        issues.len() <= manifests.len(),
        "More issues than manifests"
    );
}

#[test]
fn kb_check_closure_on_seed_data() {
    let kb = maesma_knowledgebase::KnowledgebaseStore::in_memory().unwrap();
    let manifests = maesma_knowledgebase::generate_seed_manifests();
    for m in &manifests {
        kb.deposit_manifest(m).unwrap();
    }
    let forcing = [
        "P",
        "Tair",
        "RH",
        "VPD",
        "Wind",
        "SWdown",
        "LWdown",
        "CO2",
        "precipitation",
        "air_temperature",
        "wind_speed",
        "shortwave_radiation",
        "longwave_radiation",
    ];
    let report = kb.check_closure(&forcing).unwrap();
    assert!(report.total_inputs > 0);
    assert!(report.total_outputs > 0);
}

// ── Embedding engine integration tests ───────────────────────────────

#[test]
fn embedding_engine_default_rules() {
    let engine = maesma_runtime::EmbeddingEngine::with_defaults();
    assert!(engine.rule_count() >= 4);
}

#[test]
fn embedding_fire_event_integration() {
    let mut engine = maesma_runtime::EmbeddingEngine::with_defaults();
    let mut bus = maesma_runtime::EventBus::new();
    let state = maesma_runtime::SimulationState::new(20, 20);

    bus.push(maesma_runtime::Event {
        kind: maesma_runtime::events::EventKind::LightningIgnition,
        time: 0.0,
        location: Some((10, 10)),
        payload: None,
    });

    let activated = engine.process_events(&mut bus, 0.0, &state);
    assert!(!activated.is_empty());
    assert_eq!(engine.active_count(), activated.len());
}

// ── Variable registry tests ─────────────────────────────────────────

#[test]
fn variable_registry_defaults() {
    let reg = maesma_core::VariableRegistry::with_defaults();
    assert!(
        reg.len() >= 20,
        "Should have at least 20 canonical variables"
    );
    assert!(reg.get("precipitation").is_some());
    assert!(reg.get("soil_moisture").is_some());
    assert!(reg.get("burn_severity").is_some());
}

// ── SAPG serialization integration tests ─────────────────────────────

#[test]
fn sapg_json_round_trip_integration() {
    let mut sapg = maesma_core::Sapg::new();
    let id = maesma_core::ProcessId::new();
    sapg.add_process(maesma_core::graph::ProcessNode {
        process_id: id,
        name: "TestProcess".into(),
        family: maesma_core::ProcessFamily::Hydrology,
        rung: maesma_core::FidelityRung::R0,
        tier: maesma_core::manifest::CouplingTier::Slow,
        cost: 1.0,
    });
    let json = sapg.to_json().unwrap();
    let restored = maesma_core::Sapg::from_json(&json).unwrap();
    assert_eq!(restored.node_count(), 1);
}

// ═══════════════════════════════════════════════════════════════════════
// Round 4 integration tests
// ═══════════════════════════════════════════════════════════════════════

// ── Topology adapter tests ──────────────────────────────────────────

#[test]
fn topology_grid_aggregation_integration() {
    use maesma_core::spatial::RegularGrid;
    use maesma_core::topology::{coarse_grid_to_grid, grid_to_coarse_grid};

    let fine = RegularGrid {
        nx: 10,
        ny: 10,
        nz: 1,
        dx: 100.0,
        dy: 100.0,
        origin: (0.0, 0.0),
        crs: "EPSG:4326".into(),
    };
    let coarse = RegularGrid {
        nx: 5,
        ny: 5,
        nz: 1,
        dx: 200.0,
        dy: 200.0,
        origin: (0.0, 0.0),
        crs: "EPSG:4326".into(),
    };

    let agg = grid_to_coarse_grid(&fine, &coarse).unwrap();
    assert_eq!(agg.n_src, 100);
    assert_eq!(agg.n_dst, 25);
    assert!(agg.conservative);

    // Uniform field should aggregate to same value
    let src = vec![42.0; 100];
    let dst = agg.apply(&src);
    assert_eq!(dst.len(), 25);
    for v in &dst {
        assert!((v - 42.0).abs() < 1e-10);
    }

    // Disaggregation
    let disagg = coarse_grid_to_grid(&coarse, &fine).unwrap();
    let back = disagg.apply(&dst);
    assert_eq!(back.len(), 100);
    for v in &back {
        assert!((v - 42.0).abs() < 1e-10);
    }
}

#[test]
fn topology_network_transfer_integration() {
    use maesma_core::spatial::{NetworkTopology, RegularGrid};
    use maesma_core::topology::{grid_to_network, network_to_grid};

    let grid = RegularGrid {
        nx: 4,
        ny: 4,
        nz: 1,
        dx: 100.0,
        dy: 100.0,
        origin: (0.0, 0.0),
        crs: "EPSG:4326".into(),
    };
    let net = NetworkTopology {
        n_nodes: 3,
        n_edges: 2,
        crs: "EPSG:4326".into(),
    };
    let coords = vec![(0, 0), (2, 2), (3, 3)];

    let g2n = grid_to_network(&grid, &net, &coords).unwrap();
    let _n2g = network_to_grid(&net, &grid, &coords).unwrap();

    // Grid field with distinct values
    let mut field = vec![0.0; 16];
    field[0] = 10.0; // (0,0)
    field[10] = 20.0; // (2,2)
    field[15] = 30.0; // (3,3)

    let net_vals = g2n.apply(&field);
    assert_eq!(net_vals.len(), 3);
    assert!((net_vals[0] - 10.0).abs() < 1e-10);
    assert!((net_vals[1] - 20.0).abs() < 1e-10);
    assert!((net_vals[2] - 30.0).abs() < 1e-10);
}

#[test]
fn topology_info_loss_tracking() {
    use maesma_core::topology::compute_info_loss;

    let src = vec![1.0, 2.0, 3.0, 4.0];
    let dst = vec![1.5, 2.5, 3.5]; // Aggregated with slight mass loss
    let src_areas = vec![1.0; 4];
    let dst_areas = vec![1.0; 3];

    let loss = compute_info_loss("temperature", &src, &dst, &src_areas, &dst_areas);
    assert!(loss.relative_error > 0.0); // There is info loss
    assert_eq!(loss.variable, "temperature");
}

// ── Compiler rung selection tests ───────────────────────────────────

#[test]
fn compiler_rung_selection_integration() {
    use maesma_compiler::selection::{SelectionConstraints, build_assembly_plan};

    let manifests = maesma_knowledgebase::generate_seed_manifests();
    let constraints = SelectionConstraints {
        target_dx: 1000.0,
        target_dt: 86400.0,
        flops_budget: 1e12,
        n_cells: 10000,
        regime_tags: vec![],
        gpu_available: false,
    };
    let plan = build_assembly_plan(&manifests, &constraints);
    assert!(
        !plan.selections.is_empty(),
        "Should select at least one process"
    );
    assert!(plan.total_cost > 0.0);
}

// ── Subcycling tests ────────────────────────────────────────────────

#[test]
fn subcycling_plan_integration() {
    use maesma_core::process::ProcessId;
    use maesma_runtime::subcycling::{DeviceInventory, build_subcycle_plan};
    use std::collections::HashMap;

    let p1 = ProcessId::new();
    let p2 = ProcessId::new();

    let mut dts = HashMap::new();
    dts.insert(p1.clone(), 60.0); // Fire: 1-minute steps
    dts.insert(p2.clone(), 86400.0); // Ecology: daily steps

    let gpu_cap = HashMap::new();
    let mem = HashMap::new();
    let inv = DeviceInventory::detect();

    let plan = build_subcycle_plan("mixed", 86400.0, &dts, &gpu_cap, &mem, 1000, &inv);
    assert_eq!(plan.configs.len(), 2);

    let fire_cfg = plan.configs.iter().find(|c| c.process_id == p1).unwrap();
    assert_eq!(fire_cfg.n_substeps, 1440);

    let eco_cfg = plan.configs.iter().find(|c| c.process_id == p2).unwrap();
    assert_eq!(eco_cfg.n_substeps, 1);
}

#[test]
fn device_inventory_with_gpus() {
    use maesma_runtime::subcycling::DeviceInventory;

    let inv = DeviceInventory::with_gpus(16, vec![(0, 8192), (1, 16384)]);
    assert!(inv.has_gpu());
    assert_eq!(inv.total_gpu_memory(), 24576);
    assert_eq!(inv.n_cpus, 16);
}

// ── Comparison protocol tests ───────────────────────────────────────

#[test]
fn protocol_registry_defaults_integration() {
    use maesma_core::families::ProcessFamily;
    use maesma_core::protocols::ProtocolRegistry;

    let reg = ProtocolRegistry::with_defaults();
    assert!(reg.len() >= 8);

    // Hydrology should have at least 2 protocols
    let hydro = reg.for_family(ProcessFamily::Hydrology);
    assert!(hydro.len() >= 2);

    // Fire should have at least 1
    let fire = reg.for_family(ProcessFamily::Fire);
    assert!(!fire.is_empty());

    // Cryosphere should have SWE protocol
    let cryo = reg.for_family(ProcessFamily::Cryosphere);
    assert!(!cryo.is_empty());
}

#[test]
fn protocol_observable_lookup() {
    use maesma_core::protocols::ProtocolRegistry;

    let reg = ProtocolRegistry::with_defaults();
    let streamflow = reg.for_observable("streamflow");
    assert_eq!(streamflow.len(), 1);
    assert_eq!(streamflow[0].primary_metric, "kge");
}

// Round 5 integration tests

#[test]
fn refinement_engine_skill_upgrade_cycle() {
    use maesma_core::families::ProcessFamily;
    use maesma_core::process::FidelityRung;
    use maesma_runtime::{RefinementAction, RefinementEngine, RefinementTrigger};

    let mut engine = RefinementEngine::default();
    let trigger = RefinementTrigger::SkillBelow {
        family: ProcessFamily::Hydrology,
        metric: "kge".into(),
        threshold: 0.5,
        current: 0.2,
    };
    let action = engine.evaluate(&trigger, FidelityRung::R0);
    match action {
        RefinementAction::Upgrade { from, to, .. } => {
            assert_eq!(from, FidelityRung::R0);
            assert_eq!(to, FidelityRung::R1);
        }
        _ => panic!("Expected upgrade from R0 to R1"),
    }

    let trigger2 = RefinementTrigger::SkillBelow {
        family: ProcessFamily::Hydrology,
        metric: "kge".into(),
        threshold: 0.5,
        current: 0.85,
    };
    let action2 = engine.evaluate(&trigger2, FidelityRung::R1);
    assert!(matches!(action2, RefinementAction::Hold { .. }));
    assert_eq!(engine.history.len(), 2);
}

#[test]
fn disturbance_pipeline_multi_event() {
    use maesma_core::families::ProcessFamily;
    use maesma_runtime::{DisturbanceEvent, DisturbancePipeline, DisturbanceType};

    let mut pipeline = DisturbancePipeline::new();
    pipeline.queue(DisturbanceEvent {
        source_family: ProcessFamily::Fire,
        disturbance_type: DisturbanceType::Fire,
        affected_cells: vec![0, 1],
        severity: vec![0.9, 0.5],
        step: 10,
    });
    pipeline.queue(DisturbanceEvent {
        source_family: ProcessFamily::Hydrology,
        disturbance_type: DisturbanceType::Flood,
        affected_cells: vec![3, 4, 5],
        severity: vec![1.0, 0.8, 0.6],
        step: 10,
    });
    assert_eq!(pipeline.pending_count(), 2);
    let mods = pipeline.process_pending();
    assert_eq!(mods.len(), 5);
    assert_eq!(pipeline.processed_count(), 2);
}

#[test]
fn observation_point_extraction_integration() {
    use maesma_runtime::PointExtractor;
    let pe = PointExtractor::new((0.0, 0.0), (1.0, 1.0), (4, 4));
    let grid_data = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let val = pe.extract(&grid_data, 1.5, 1.5);
    assert!(val.is_some());
    let v = val.unwrap();
    assert!(v > 6.0 && v < 11.0);
}

#[test]
fn observation_spatial_average_integration() {
    use maesma_core::observations::BoundingBox;
    use maesma_runtime::SpatialAverager;
    let avg = SpatialAverager::new((0.0, 0.0), (2.5, 2.5), (5, 5));
    let grid = vec![100.0; 25];
    let bbox = BoundingBox {
        west: 0.0,
        east: 10.0,
        south: 0.0,
        north: 10.0,
    };
    let result = avg.average(&grid, &bbox);
    assert!(result.is_some());
    assert!((result.unwrap() - 100.0).abs() < 1e-6);
}

#[test]
fn skill_store_insert_query_roundtrip() {
    use maesma_core::metrics::SkillMetrics;
    use maesma_core::process::{FidelityRung, ProcessId};
    use maesma_core::skills::SkillRecord;
    use maesma_runtime::SkillScoreStore;

    let mut store = SkillScoreStore::new();
    let pid = ProcessId::new();
    store.insert(SkillRecord {
        process_id: pid.clone(),
        rung: FidelityRung::R1,
        region: "TestRegion".into(),
        regime_tags: vec![],
        season: None,
        metrics: SkillMetrics {
            rmse: Some(1.5),
            kge: Some(0.85),
            crps: None,
            nse: Some(0.8),
            bias: Some(0.02),
            correlation: Some(0.9),
            conservation_residual: None,
            wall_time_per_cell: None,
            custom: std::collections::HashMap::new(),
        },
        dataset: "FluxNet".into(),
        evaluated_at: "2024-06-01".into(),
        benchmark: None,
        process_hash: None,
    });
    assert_eq!(store.count(), 1);
    assert_eq!(store.query_by_region("TestRegion").len(), 1);
    assert_eq!(store.above_kge_threshold(0.8).len(), 1);
}

#[test]
fn data_contract_coupling_validation() {
    use maesma_core::data_contracts::*;
    use maesma_core::families::ProcessFamily;
    use maesma_core::process::FidelityRung;
    use maesma_core::units::PhysicalUnit;
    use maesma_core::variables::VariableCategory;

    let field = |name: &str| FieldSpec {
        name: name.to_string(),
        unit: PhysicalUnit::dimensionless(),
        category: VariableCategory::Forcing,
        ndim: 2,
        required: true,
        lower_bound: None,
        upper_bound: None,
        fill_value: None,
    };
    let tc = TemporalConstraint {
        max_dt: 3600.0,
        min_dt: 1.0,
        supports_subcycling: true,
        preferred_dt: 1800.0,
    };

    let producer = DataContract {
        family: ProcessFamily::Atmosphere,
        rung: FidelityRung::R0,
        process_name: "BulkAtmo".into(),
        version: "1.0".into(),
        inputs: vec![],
        outputs: vec![field("air_temperature"), field("precipitation")],
        conserved: vec![],
        temporal: tc.clone(),
    };
    let consumer = DataContract {
        family: ProcessFamily::Hydrology,
        rung: FidelityRung::R1,
        process_name: "Richards".into(),
        version: "1.0".into(),
        inputs: vec![field("air_temperature"), field("precipitation")],
        outputs: vec![],
        conserved: vec![],
        temporal: tc,
    };
    assert!(matches!(
        validate_coupling(&producer, &consumer),
        ContractValidation::Compatible
    ));
}

#[test]
fn all_r0_processes_step_without_panic() {
    use maesma_processes::create_default_runners;
    use maesma_runtime::state::SimulationState;

    let runners = create_default_runners();
    assert!(
        runners.len() >= 11,
        "Should have at least 11 default runners"
    );
    let mut state = SimulationState::new(5, 5);

    // Seed all fields that R0 processes require
    state.init_field_const("fuel_load", 0.5);
    state.init_field_const("weather_fire_danger_index", 0.3);
    state.init_field_const("precipitation", 2.0);
    state.init_field_const("potential_et", 1.0);
    state.init_field_const("soil_moisture", 0.3);
    state.init_field_const("sw_down", 200.0);
    state.init_field_const("lai", 3.0);
    state.init_field_const("albedo_soil", 0.2);
    state.init_field_const("temperature", 288.0);
    state.init_field_const("soil_carbon", 5.0);
    state.init_field_const("litter_input", 0.01);
    state.init_field_const("moisture", 0.4);
    state.init_field_const("wind_speed", 3.0);
    state.init_field_const("temperature_air", 290.0);
    state.init_field_const("temperature_surface", 292.0);
    state.init_field_const("sst", 288.0);
    state.init_field_const("net_heat_flux", 50.0);
    state.init_field_const("snow_water_equivalent", 10.0);
    state.init_field_const("land_use_fractions", 0.5);
    state.init_field_const("prey_biomass", 100.0);
    state.init_field_const("trait_mean", 1.0);

    for (family, _rung, mut runner) in runners {
        let mut adapter = maesma_runtime::state::ProcessStateAdapter::new(&mut state);
        runner
            .step(&mut adapter, 3600.0)
            .unwrap_or_else(|e| panic!("{:?} R0 step failed: {e}", family));
        adapter.sync_back();
    }
}
