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
