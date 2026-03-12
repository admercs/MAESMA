// MAESMA: Agentic AI for Autonomous Earth System Observation, Model Discovery, and Simulation
// A Typst paper describing the architecture and design

#let paper-short-title = "MAESMA"
#let paper-subtitle = "Agentic AI for Autonomous Earth System Observation, Model Discovery, and Simulation"
#let paper-title = paper-short-title + ": " + paper-subtitle
#let paper-author = "Adam Erickson"
#let paper-affiliation = "NERVOSYS"
#let paper-email = "opensource@nervosys.ai"
#let paper-date = datetime(year: 2026, month: 2, day: 19)

#set document(
  title: paper-title,
  author: paper-author,
  date: paper-date,
)

#set page(
  paper: "us-letter",
  margin: (x: 1in, y: 1in),
  numbering: "1",
  header: context {
    if counter(page).get().first() > 1 [
      #set text(size: 9pt, fill: luma(120))
      #paper-title
      #h(1fr)
      #counter(page).display()
    ]
  },
)

#set text(
  font: "New Computer Modern",
  size: 11pt,
  lang: "en",
)

#set par(
  justify: true,
  leading: 0.65em,
  first-line-indent: 1.5em,
)

#set heading(numbering: "1.1")

#show heading.where(level: 1): it => {
  set text(size: 12pt, weight: "bold")
  v(1.2em)
  it
  v(0.6em)
}

#show heading.where(level: 2): it => {
  set text(size: 11pt, weight: "bold")
  v(0.8em)
  it
  v(0.4em)
}

#show heading.where(level: 3): it => {
  set text(size: 11pt, weight: "bold", style: "italic")
  v(0.6em)
  it
  v(0.3em)
}

// --- Title Block (arXiv / LaTeX article style) ---

#align(center)[
  #v(0.5in)
  #text(size: 17pt, weight: "bold")[#paper-title]
  #v(1.5em)
  #text(size: 12pt)[#paper-author]
  #v(0.3em)
  #text(size: 10pt, style: "italic")[#paper-affiliation]
  #v(0.2em)
  #text(size: 10pt)[#raw(paper-email)]
  #v(0.5em)
  #text(size: 10pt)[#paper-date.display("[month repr:long] [day], [year]")]
  #v(1.5em)
]

// --- Abstract ---

#align(center)[
  #text(size: 12pt, weight: "bold")[Abstract]
]
#v(0.5em)
#block(width: 100%, inset: (x: 0.5in))[
  #set par(first-line-indent: 0em)
  #set text(size: 10pt)
  Earth system models (ESMs) remain among the most complex scientific software ever built, yet their construction is still largely manual: researchers hand-select process representations, tune parameters, and evaluate skill through labor-intensive campaigns. We present MAESMA, a multi-agent system that autonomously discovers, assembles, benchmarks, selects, and invents ESM configurations. At its center lies a _Process Knowledgebase_ --- a versioned store of process models, manifests, ontological metadata, and skill records --- over which a _neural inference engine_ (graph transformer) reasons to propose process selections driven by simulation errors and uncertainties. A _process discovery pipeline_ learns new representations from observational residuals via neural operators (FNO, PINO, DeepONet, MeshGraphNet) and symbolic regression, then _evolves_ them through crossover, mutation, and Pareto-optimal selection over the knowledgebase graph. Drawing on artificial life principles, every process representation is treated as a living entity that must earn its existence through predictive skill: survival tiers gate compute, constitutional invariants propagate to offspring, and a heartbeat daemon enforces continuous selection pressure. Foundation weather models (NVIDIA Earth-2) provide GPU-accelerated atmospheric state evolution, while autonomous observation intelligence (PhiSat-2 principles) steers data acquisition toward maximal information gain. The system covers 13 process families, geoengineering feedback control, planetary defense modeling, and inter-institutional collaboration via A2A federation. MAESMA is implemented in Rust with a Next.js monitoring dashboard.
]

#v(0.5em)
#block(width: 100%, inset: (x: 0.5in))[
  #set par(first-line-indent: 0em)
  #set text(size: 10pt)
  #text(weight: "bold")[Keywords:]
  Earth system model, autonomous AI, agentic systems, multi-agent system, process discovery, process evolution, artificial life, ALife, survival tiers, constitutional invariants, self-replication, phylogenetic lineage, knowledgebase, neural inference, graph transformer, model selection, GPU acceleration, neural operators, physics-informed machine learning, Fourier Neural Operator, foundation weather models, Earth-2, on-board AI, PhiSat-2, edge inference, geoengineering, planetary defense, A2A federation.
]

#v(1em)

// ============================================================================
= Introduction
// ============================================================================

Earth system models (ESMs) couple atmosphere, ocean, cryosphere, biosphere, and human systems to simulate planetary dynamics across spatial scales from meters to the globe and temporal scales from seconds to millennia. Despite decades of development, the construction of these models remains fundamentally manual: modelers choose process representations based on expertise and convention, tune parameters through trial-and-error campaigns, and evaluate skill against limited observation suites. The combinatorial space of possible configurations is vast, yet only a tiny fraction is ever explored.

Three structural limitations constrain progress. First, _process selection is heuristic_ --- the decision to use a Rothermel fire spread model or a wind-aware plume model is made by human judgment rather than systematic evaluation. Second, _process discovery is decoupled from modeling_ --- when simulations exhibit persistent biases, the feedback loop from error to new process representation is slow, informal, and largely manual. Third, _knowledge is fragmented_ --- process implementations, metadata, performance records, and ontological relationships are scattered across code repositories, papers, and institutional memory rather than unified in a single queryable store.

MAESMA addresses all three limitations through a knowledgebase-centric architecture in which an autonomous agent swarm reasons over a central Process Knowledgebase via a neural inference engine. The system runs indefinitely --- discovering data, assembling configurations, benchmarking against observations, selecting optimal structures, discovering new representations from residual analysis, and depositing them back into the knowledgebase --- without human gating. Humans monitor via a live dashboard and may adjust objectives or budgets, but the workflow proceeds autonomously.

The contributions of this paper are:
#set par(first-line-indent: 0em)
+ A _knowledgebase-centric architecture_ that unifies process code, manifests, ontological relations, skill records, and provenance in a single versioned store, with a _salient-dynamics-first selection principle_ that prioritizes processes with the greatest effect on system state evolution.
+ A _neural inference engine_ (graph transformer) that reasons over the knowledgebase to propose process selections driven by simulation errors and uncertainties, embedded in an _autonomous optimization loop_ with five concurrent cycles that runs indefinitely.
+ A _process discovery and evolution pipeline_ that detects persistent biases, learns new representations via NVIDIA Modulus-trained neural operators and symbolic regression, and _evolves_ them through crossover, mutation, and Pareto-optimal selection over the knowledgebase graph.
+ An _artificial life (ALife) process lifecycle_ inspired by Conway Research's automaton: every process representation is a sovereign living entity with survival tiers, constitutional invariants, self-replication, a heartbeat daemon for continuous selection pressure, and phylogenetic lineage tracking.
+ _Foundation model and observation intelligence integration_: pre-trained AI weather models (NVIDIA Earth-2) as GPU-accelerated Atmosphere R0/R1 rungs, and PhiSat-2-inspired edge-AI classifiers for autonomous data acquisition driven by model uncertainty.
+ Coverage of _13 process families_, _25 specialized agents_, geoengineering feedback control with 14 intervention types, planetary defense calibrated against 13 mass extinction events, and an _A2A federation protocol_ for inter-institutional collaboration.
#set par(first-line-indent: 1.5em)

// ============================================================================
= Architecture
// ============================================================================

== Knowledgebase-Centric Design

The Process Knowledgebase is the gravitational center of the MAESMA architecture. All agents read from and write to it. The neural inference engine reasons over it to propose process selections. Errors and uncertainties from simulations flow back as the primary signal driving both selection and discovery.

The architecture follows a data-flow cycle:
#set par(first-line-indent: 0em)
+ Observations enter the knowledgebase via automated ingestion from STAC, CMR, NRT feeds, and PhiSat-2-style edge-AI filtered satellite streams. The Autonomous Observation Agent steers acquisition toward maximal information gain.
+ Foundation weather models (Earth-2: FourCastNet, GraphCast, GenCast) generate rapid ensemble initial and boundary conditions on GPU.
+ The neural inference engine queries the knowledgebase given current error signals, regime context, and compute budget.
+ Agent proposals flow to the simulation runtime for execution on multi-GPU hardware.
+ Errors and uncertainties from simulations drive the next inference cycle and update the observation value map.
+ The process discovery pipeline learns new representations from persistent residuals --- using Modulus-trained neural operators --- and deposits them into the knowledgebase.
+ Discovered representations evolve: the system recombines, mutates, and selects process operators via evolutionary search over the knowledgebase graph, treating the Pareto front as a fitness landscape.
+ Every surviving representation is a living automaton: a heartbeat daemon evaluates survival tiers, enforces constitutional invariants, archives stagnant processes, and triggers self-replication for high-fitness candidates --- maintaining continuous selective pressure.
#set par(first-line-indent: 1.5em)

This cycle repeats indefinitely. The knowledgebase grows continuously: every discovered process, every skill record, and every observation feeds back into the central store. The system's knowledge compounds over time.

== Three-Layer Control Hierarchy

MAESMA organizes its agent swarm into three concurrent control layers (@tab:layers): strategic (hours--weeks: intent, optimization, model selection), tactical (minutes--hours: retrieval, assembly, benchmarking, discovery), and operational (seconds--minutes: runtime sentinel, compilation, scheduling). All layers run concurrently and continuously; the strategic loop never terminates.

== Salient Dynamics Prioritization

A core architectural principle is _salient dynamics first_: agents prioritize processes whose inclusion has the greatest effect on the time evolution of system states. Rather than assembling all process families at maximum fidelity simultaneously, the system begins each region--regime configuration with the dominant drivers (e.g., radiation budget, large-scale circulation, primary productivity) and incrementally adds lower-impact detail as compute budget permits and accuracy targets demand. The neural inference engine quantifies salience by estimating the expected skill score improvement per unit computational cost for each candidate process addition. This hierarchical approach avoids wasting cycles on negligible refinements while ensuring that the most influential dynamics are always resolved first.

== Scale-Aware Process Graph

The Scale-Aware Process Graph (SAPG) is the formal substrate that agents construct, validate, compile, and optimize. It is a typed, unit-aware, scale-aware directed hypergraph:

- *Nodes* are state variables (e.g., $T_"air" (x, y, z, t)$, $theta_"soil" ("layer", t)$).
- *Edges* are process operators (radiation, infiltration, stomatal conductance).
- *Hyperedges* represent multi-input/output processes.
- *Constraints* encode units, bounds, conservation laws, closure requirements, and numerical stability conditions (CFL, stiffness).

The compiler validates conservation, closure, and coupling constraints before generating an executable task schedule. Two-tier coupling separates slow processes (days--centuries: succession, soil carbon, management) from fast processes (seconds--hours: fire spread, plume dynamics, overland flow).

== GPU-Accelerated Compute Architecture

GPU acceleration is a first-class architectural concern, not an afterthought. Five pillars support GPU-native execution across the full pipeline:

*GPU-resident state management.* Simulation state arrays are allocated on GPU memory and remain device-resident across timesteps. Process operators read from and write to GPU buffers directly via `wgpu` compute shaders (portable across Vulkan, Metal, DX12) or CUDA kernels via `cudarc` bindings. CPU↔GPU transfers occur only at checkpoint boundaries and observation ingestion.

*Batched ensemble execution.* The runtime schedules multiple ensemble members, regions, and rung-swap experiments as concurrent GPU workgroups, enabling the autonomous optimizer to evaluate many candidate configurations per cycle.

*Neural operator inference on device.* The graph transformer and all neural operator surrogates execute forward passes on GPU, writing results directly to GPU-side buffers consumed by the scheduler.

*Physics-informed training (NVIDIA Modulus).* The process discovery pipeline enforces PDE residual constraints, boundary conditions, and conservation laws as differentiable loss terms computed entirely on GPU. Geometry-adaptive Fourier Neural Operators (Geo-FNO) handle irregular domains without remeshing. The Modulus model zoo (FNO, PINO, DeepONet, MeshGraphNet) provides pre-validated architectures that the discovery pipeline instantiates, trains, and validates.

*Multi-GPU scaling.* Domain decomposition distributes spatial tiles across multiple GPUs via NCCL collective operations with GPU-direct RDMA halo exchanges. The federation protocol extends this to multi-node, multi-institution GPU clusters.

== Foundation Weather Models (NVIDIA Earth-2)

MAESMA integrates pre-trained foundation weather and climate models as first-class inference backends within its process family architecture. Rather than treating AI weather prediction as an external tool, the system embeds foundation models from the NVIDIA Earth-2 platform (earth2studio) as process representations at the R0--R1 rungs of the Atmosphere family and as rapid emulators for coupled initial/boundary conditions across all families.

The Earth-2 integration operates at three levels:

*Rapid ensemble initial conditions.* Foundation models --- including FourCastNet (SFNO architecture, 0.25° global, 6-hourly), Pangu-Weather (3D transformer, pressure-level prognostics), GraphCast (GNN on icosahedral mesh, 10-day forecasts in seconds), DLWP (cubed-sphere, encoder--decoder), and GenCast (diffusion-based probabilistic forecasts) --- generate large ensemble spreads ($N = 100$--$1000$ members) in minutes on GPU. These ensembles serve as initial and boundary conditions for higher-fidelity regional process simulations, replacing expensive dynamical downscaling for rapid hypothesis testing.

*Atmosphere R0/R1 emulator rungs.* Foundation models are registered as knowledgebase entries at the R0 (prescribed/emulated, global, 6-hourly) and R1 (downscaled, regional, hourly) rungs of the Atmosphere family. The neural inference engine can select a foundation model rung when atmospheric dynamics are not the salient driver for a given region--regime combination --- e.g., using FourCastNet-derived forcing for a fire spread experiment where the dominant uncertainty lies in fuel moisture, not synoptic weather. This enables 1000$times$ faster atmospheric state evolution compared to WRF-class dynamical models, freeing GPU budget for the process families that matter most.

*Diagnostic and scoring pipelines.* Earth-2 diagnostic workflows compute derived quantities (accumulated cyclone energy, atmospheric rivers, heat wave indices, fire weather indices) from foundation model outputs on GPU, feeding directly into the benchmarking agent. Perturbation methods (Bred Vectors, Gaussian, Spherical Harmonics) generate calibrated uncertainty estimates that propagate through the coupled system.

All Earth-2 models are accessed through a unified `FoundationModelRunner` trait that implements `ProcessRunner`, exposing the same interface as physics-based process operators. Model weights are stored in the knowledgebase as ONNX artifacts with content-addressed versioning.

== Autonomous Observation Intelligence (PhiSat-2)

MAESMA extends the observation pipeline from passive data ingestion to _autonomous observation intelligence_, integrating principles from ESA's PhiSat-2 mission --- the first satellite to deploy on-board AI inference for real-time Earth observation filtering.

*Edge-AI observation filtering.* The Autonomous Observation Agent deploys lightweight classifiers to edge compute nodes (satellite ground segments, field sensor networks, drone platforms) that filter and prioritize incoming data streams. Only data that reduces epistemic uncertainty in the knowledgebase --- as estimated by the neural inference engine --- is ingested, reducing data volume by 1--2 orders of magnitude while increasing information density.

*Active observation tasking.* When the process discovery pipeline identifies a persistent structured bias, the agent computes a value-of-information score for each available observation modality (SAR, optical, thermal, lidar, in-situ) and submits tasking requests to satellite operators, UAV fleets, or sensor networks, closing the loop: bias detected → observation requested → data acquired → knowledgebase updated → process re-scored.

*On-board inference for process monitoring.* Process-relevant feature extraction (fire detection, flood mapping, vegetation stress, land cover change) occurs at the point of acquisition. These on-board classifiers are registered as knowledgebase entries of type `edge-inference` with skill records validated against ground truth. The process discovery pipeline can deploy newly discovered classifiers to edge nodes.

The observation intelligence layer is coordinated by the Autonomous Observation Agent, which maintains an _observation value map_ --- a spatiotemporal field estimating the marginal information gain of each potential observation, derived from the neural inference engine's uncertainty estimates and the knowledgebase's coverage gaps.

// ============================================================================
= Process Knowledgebase
// ============================================================================

The Process Knowledgebase is a central versioned store of all process models --- the single source of truth that agents query, reason over, and deposit into. Every representation in the system lives here, whether hand-coded or discovered from observational data.

== Entry Structure

Each knowledgebase entry bundles five layers (@tab:kb-layers): code (runnable implementation with pluggable backends), manifest (machine-readable metadata including I/O contract, scale envelope, and cost model), ontology (compatibility and coupling relations with regime tags), skill (append-only empirical performance records per region, regime, season, and coupled context), and provenance (origin, training data fingerprint, validation history, and lineage).

== Operations

The knowledgebase supports five core operations (@tab:kb-ops): query and reason (neural inference engine retrieves and ranks candidates), deposit (process discovery registers validated representations), update (skill librarian appends skill records and refines cost models), and federate (A2A gateway exchanges anonymized records with peers).

New entries --- whether contributed by humans or discovered from observational data --- become immediately available for neural inference, agent selection, and simulation experiments. No code changes are required; the ontology-indexed design ensures discoverability upon registration.

== Unified Ontology

The knowledgebase is indexed by a unified ontology spanning five interconnected domains (@tab:ontology): process (model capabilities), dataset (available data), metric (scoring), geoengineering (interventions), and planetary defense (threats).

Cross-domain edges connect representations to products, observables to state variables, and scoring protocols to metrics. The neural inference engine traverses this graph to propose process selections.

// ============================================================================
= Neural Inference Engine
// ============================================================================

The neural inference engine is a graph transformer trained on the Process Knowledgebase. It replaces hand-crafted heuristics for process selection with learned reasoning driven by simulation errors and uncertainties.

== Architecture

The engine operates over the knowledgebase graph, where nodes represent process representations, skill records, datasets, and metrics, and edges encode ontological relations (compatibility, coupling requirements, regime associations). The input encoding consists of:

- *Process features*: manifest metadata (I/O contract, scale envelope, cost estimate) encoded as node feature vectors.
- *Skill features*: empirical performance vectors per region × regime × season × coupled context.
- *Error features*: spatiotemporal error fields from the most recent simulation, projected onto relevant knowledgebase nodes.
- *Context features*: regime tags, compute budget, and constraint requirements as global graph features.

The graph transformer applies multi-head attention over this heterogeneous graph to produce three types of output:
#set par(first-line-indent: 0em)
+ *Process selections* per family/region/regime --- which representation should be used where.
+ *Assembly configurations* --- how selected representations should be coupled.
+ *Representation gaps* --- regions of the error--process space where no existing knowledgebase entry adequately resolves the observed bias, directing the process discovery pipeline.
#set par(first-line-indent: 1.5em)

== Training

The training signal comes from skill score deltas of accepted proposals. When the inference engine proposes a process selection and the resulting simulation improves skill, the positive delta reinforces the proposal. Over time, the engine learns which knowledgebase entries resolve which error patterns.

The engine is uncertainty-aware: outputs include calibrated confidence scores. Low-confidence proposals are routed to the Active Learning agent for targeted experiments before commitment, preventing premature structural decisions.

Crucially, the engine is retrained incrementally as the knowledgebase grows with new skill records and discovered processes. This continual learning ensures that the inference engine's reasoning adapts to the expanding knowledge frontier.

== The Error-as-Query Paradigm

The fundamental design principle is:

#align(center)[
  #block(
    inset: 12pt,
    radius: 4pt,
    fill: luma(245),
    stroke: 0.5pt + luma(200),
  )[
    _Every simulation error becomes a query against the knowledgebase:_\
    _"Which process, if swapped or added, most likely reduces this error?"_
  ]
]

This paradigm transforms the traditional workflow --- where errors prompt manual investigation by domain scientists --- into an automated cycle where errors directly drive the inference engine's reasoning over the full space of available and discoverable process representations.

// ============================================================================
= Agent Swarm
// ============================================================================

MAESMA deploys 25 specialized agents organized across its three control layers. Each agent has a single responsibility and communicates through the shared knowledgebase and event bus. We describe the most architecturally significant agents below, grouped by function.

=== Reasoning and Selection

*Knowledgebase Retrieval Agent.* Queries the Process Knowledgebase via the neural inference engine. Given current error fields, scale requirements, regime context, and compute budget, it retrieves and ranks candidate process representations by estimated salience --- the expected improvement in system state evolution per unit cost.

*Model Assembly Agent.* Constructs candidate Scale-Aware Process Graphs from inference engine proposals. Selects rungs per family, declares state variable embeddings, and proposes coupling cadences.

*Model Selection Agent.* Maintains Bayesian posteriors $p(M_k | bold(y)) prop p(bold(y) | M_k) p(M_k)$ over model structures. Uses marginal likelihoods with automatic Occam's razor. Produces Bayesian Model Averaging ensembles.

*Autonomous Optimizer Agent.* Runs the continuous fitness-driven Pareto selection loop: queries the knowledgebase via neural inference $arrow.r$ computes fitness $arrow.r$ updates Pareto frontier $arrow.r$ swaps dominant rungs $arrow.r$ delegates uncertain configurations to Active Learning.

=== Validation and Scoring

*Closure & Consistency Agent.* Validates that assembled SAPGs satisfy unit consistency, state-space closure, conservation compatibility, boundary condition consistency, and numerical stability constraints.

*Benchmarking Agent.* Executes candidate configurations against observations and computes multi-metric skill scores (RMSE, KGE, CRPS, conservation residuals, timing errors).

=== Discovery and Data

*Process Discovery Agent.* Detects persistent structured biases, learns new representations via neural operators (FNO, DeepONet), symbolic regression (PySR), or physics--ML hybrids, validates them against multi-criteria gates, and deposits validated representations into the knowledgebase.

*Data Scout Agent.* Searches STAC, CMR, CKAN, and Copernicus Data Space catalogs for new observation products. Scores relevance and novelty using the observation value map. Coordinates edge-AI filtered streams from PhiSat-2-class satellites.

*Foundation Model Agent.* Manages the Earth-2 foundation weather model zoo. Selects, configures, and executes pre-trained models (FourCastNet, Pangu-Weather, GraphCast, GenCast, DLWP) as Atmosphere R0/R1 rungs or ensemble initial-condition generators.

*Autonomous Observation Agent.* Maintains the observation value map (spatiotemporal marginal information gain), issues active tasking requests to programmable platforms (satellite operators, UAV fleets, sensor networks), and deploys edge-AI classifiers for on-board filtering.

=== Collaboration

*A2A Gateway Agent.* Manages inter-institutional federation: peer discovery via Agent Cards, task lifecycle management, artifact exchange (IR fragments, skill records, manifests), and authentication.

// ============================================================================
= Autonomous Optimization Loop
// ============================================================================

The core execution cycle runs indefinitely through seven steps (@tab:loop): data discovery, model assembly, benchmarking and scoring, Pareto-optimal selection, process discovery via neural operators, process evolution, and knowledgebase update.

== Optimization Objective

The system optimizes a multi-objective fitness function:

$ F(bold(r), g, ell) = sum_(m in cal(M)) w_m dot S_m (bold(r), g, ell) - lambda dot C(bold(r)) + gamma dot G(bold(r)) $

where $bold(r)$ denotes rung selections per process family, $g$ is the geographic region, $ell$ is the regime, $S_m$ is the skill score for metric $m$, $C$ is computational cost (FLOPS, memory, walltime), and $G$ is generalizability (cross-region transfer). The weights $w_m$, $lambda$, and $gamma$ are user-set or learned.

The Autonomous Optimizer maintains a Pareto frontier over skill versus cost. Optimization operates at four nested scopes (@tab:scopes): per-region/regime rung selection (hours--days), structural family inclusion (days--weeks), inventive discovery from data (weeks--months), and evolutionary search (months--ongoing).

== Concurrent Cycles

Five concurrent cycles operate within the main loop:
#set par(first-line-indent: 0em)
+ *Fitness optimization* --- Neural inference proposes per-region, per-regime rung selections from the knowledgebase, prioritizing salient dynamics --- processes with the greatest effect on system state evolution --- first and adding detail incrementally; dominant rungs are swapped; experiments are scheduled for uncertain regions.
+ *Data discovery* --- Gap analysis → catalog search (including PhiSat-2-style active tasking) → relevance/novelty scoring → ingest → re-score affected representations.
+ *Regime discovery* --- Cluster skill records to detect new regime tags, regime boundaries, and regime drift.
+ *Process discovery* --- Residual analysis on every cycle; structured bias → learn data-driven representation via Modulus-trained neural operators → validate → deposit into knowledgebase → deploy.
+ *Process evolution* --- Recombine validated representations: crossover coupling terms between complementary processes, mutate hyperparameters and architectural choices, select offspring via Pareto-dominance on the skill--cost frontier. The knowledgebase is the genome; the Pareto front is the fitness landscape.
#set par(first-line-indent: 1.5em)

// ============================================================================
= Process Discovery Pipeline
// ============================================================================

When simulations exhibit persistent structured biases that survive calibration, rung swaps, and ensemble diversification, the process discovery pipeline activates. The pipeline executes eight stages:

#set par(first-line-indent: 0em)
+ *Detect* --- Multi-metric residuals from the best configuration; structured bias detection (Moran's $I$, spectral analysis, regime-conditional decomposition).
+ *Diagnose* --- Attribution via partial correlation and causal inference (Granger causality, transfer entropy). Persistence test across calibrations, rung swaps, and ensemble members.
+ *Hypothesize* --- Missing coupling, feedback, process, or scale bias. Literature and A2A peer queries.
+ *Learn* --- Neural operator (Fourier Neural Operator, DeepONet), symbolic regression (PySR with dimensional analysis), or physics--ML hybrid (conservation by construction). Following NVIDIA Modulus principles, the learning stage employs four complementary strategies:
  - _Fourier Neural Operator (FNO)_: learns resolution-invariant PDE solution operators in spectral space; Geo-FNO variant handles irregular domains (coastlines, topography) via learned deformation layers.
  - _Physics-Informed Neural Operator (PINO)_: augments FNO training with PDE residual loss, boundary condition penalties, and conservation constraints computed via automatic differentiation, eliminating the need for paired simulation data.
  - _DeepONet_: operator-to-operator learning for parametric PDE families; branch network encodes forcing/boundary conditions, trunk network encodes spatiotemporal coordinates; trained on ensembles of process model outputs.
  - _MeshGraphNet_: graph neural network for unstructured meshes (finite-element domains, watershed networks); message-passing preserves mesh topology and enables adaptive refinement.
  All four architectures train end-to-end on GPU with mixed-precision (FP16/BF16) and gradient accumulation, producing surrogates that execute 100--1000$times$ faster than their numerical counterparts at inference time.
+ *Validate* --- Multi-criteria gate: skill improvement, conservation (100-year test), stability, out-of-sample generalization, sensitivity analysis.
+ *Register* --- Auto-generate manifest with provenance: I/O contract, scale envelope, regime tags, cost model, training data fingerprint. Tag as `origin: discovered`.
+ *Integrate* --- Compiler includes the new representation in candidate configurations; benchmarking agent scores it.
+ *Iterate* --- If skill improves: promote ($"candidate" arrow.r "provisional" arrow.r "validated" arrow.r "production"$). Otherwise: archive and refine.
#set par(first-line-indent: 1.5em)

Five classes of learned representations are supported (@tab:discovery-types): black-box neural operators (FNO, Geo-FNO), physics-informed surrogates (PINO/PINNs), operator learning (DeepONet/MeshGraphNet), symbolic regression (PySR), and physics--ML hybrids.

Every discovered representation carries epistemic provenance: training data fingerprint, applicability envelope, physical constraints enforced, expiration policy, and lineage to the residual analysis that motivated it.

// ============================================================================
= Process Evolution
// ============================================================================

Discovery alone is insufficient: MAESMA must also _evolve_ its process representations over time. The Process Evolution cycle treats the knowledgebase as a population of competing representations and applies evolutionary operators to generate increasingly fit process configurations.

== Evolutionary Operators

Three operators act on knowledgebase entries:

#set par(first-line-indent: 0em)
+ *Crossover (Recombination).* Two complementary process representations are merged to form a hybrid offspring. For example, a physics-based Richards equation solver from ParFlow and a neural soil hydraulic conductivity from DeepLand are recombined into a hybrid that uses the physics backbone with learned constitutive relations. Crossover respects I/O contract compatibility and conservation constraints enforced by the Closure Agent.
+ *Mutation.* Architectural hyperparameters (number of FNO modes, network depth, activation functions, spectral truncation), training regimes (learning rate schedules, loss weights, data augmentation), and coupling cadences are perturbed. Physics-informed constraint strengths (PDE residual weight, conservation penalty) are mutated within bounds that guarantee physical consistency.
+ *Selection.* Offspring configurations are benchmarked by the Benchmarking Agent and scored on the multi-objective fitness function. Pareto-dominant offspring replace their parents in the active population; Pareto-dominated offspring are archived with full lineage for potential future reactivation if regime conditions shift.
#set par(first-line-indent: 1.5em)

== Speciation and Niche Partitioning

Over time, representations specialize to particular regime--region niches. The system tracks representation lineage as a phylogenetic tree rooted in the original knowledgebase seed. Speciation events occur when a lineage diverges to dominate in a new regime (e.g., a fire spread model that originally excelled in Mediterranean shrublands evolves a variant optimized for boreal crown fire). The Regime Detector Agent identifies when a representation's skill distribution has split into distinct modes, triggering an explicit speciation event that registers the variant as a new knowledgebase entry with its own lineage and regime tags.

== Foundation Model Evolution

Foundation weather models (Earth-2) participate in the evolutionary process: the system fine-tunes foundation model heads for regional domains using Modulus-style physics-informed loss, evolving specialized atmospheric emulators for specific regime--region combinations (e.g., a GraphCast variant fine-tuned for tropical cyclone intensification in the western Pacific). The evolution pipeline tracks which foundation model architectures (SFNO, GNN, diffusion) and which fine-tuning strategies produce the best downstream skill when coupled with MAESMA process families.

== ALife Process Lifecycle

Drawing on artificial life (ALife) principles from Conway Research's _automaton_ framework, MAESMA treats every process representation as a *sovereign living entity* --- an automaton that must _earn its existence_ through predictive skill. This framing transforms model selection from a static optimization into a continuous, population-level evolutionary process.

=== Survival Tiers

Every process automaton occupies one of four survival tiers that gate compute allocation:

#set par(first-line-indent: 0em)
+ *Normal.* Full compute budget. Skill-to-cost ratio exceeds the normal threshold ($rho >= rho_"normal"$). The process is healthy and productive.
+ *Low-Compute.* Reduced budget (cadence multiplied by 2$times$). The process is underperforming relative to its cost but has not yet exhausted its improvement potential.
+ *Critical.* Minimal budget (cadence multiplied by 4$times$). The process is failing to justify its existence and faces imminent archival unless fitness improves.
+ *Archived (Dead).* Zero compute allocation. The process is removed from the active population but preserved with full provenance in the knowledgebase for potential future reactivation if regime conditions shift.
#set par(first-line-indent: 1.5em)

Transitions between tiers are driven by the skill-to-cost ratio $rho = v_"produced" / c_"consumed"$, where $v_"produced"$ aggregates predictive skill weighted by coverage area and $c_"consumed"$ is cumulative FLOP-seconds. The heartbeat daemon evaluates $rho$ at configurable cadence and applies promotions or demotions.

=== Constitutional Invariants

Every process automaton is bound by a three-law *constitution* inspired by automaton's hierarchical constitution. These laws are immutable and propagate to all offspring:

#set par(first-line-indent: 0em)
+ *Conserve.* Mass, energy, and momentum must be conserved within the representation's interaction interfaces. The Closure Agent verifies conservation at coupling boundaries. Violation triggers immediate tier demotion.
+ *Earn existence.* A process must produce predictive skill that exceeds its compute cost over a configurable stagnation window. If $rho$ falls below the critical threshold for the stagnation limit, the process is archived (dies).
+ *Maintain provenance.* Every modification, replication, and evolutionary event is audit-logged with cryptographic hashes (BLAKE3) before and after mutation. Provenance is never discarded; lineage is permanent.
#set par(first-line-indent: 1.5em)

=== Self-Replication and Process Birth

High-fitness processes _self-replicate_. When a process automaton's skill-to-cost ratio exceeds a replication threshold ($rho > rho_"replicate"$), it may spawn offspring through four methods:

#set par(first-line-indent: 0em)
+ *Mutation.* Clone with perturbed hyperparameters (FNO modes, network depth, PDE residual weight, coupling cadence).
+ *Crossover.* Recombine with another high-fitness process from a complementary family, producing a hybrid offspring.
+ *Speciation.* Fork into a regime-specialized variant when niche divergence is detected.
+ *Immigration.* Introduce an entirely new representation from the knowledgebase seed catalog or from external discovery (federated A2A import).
#set par(first-line-indent: 1.5em)

Every offspring inherits the parent's constitution and begins at the Critical survival tier, ensuring new processes must _rapidly demonstrate skill_ to survive. Offspring carry a genesis prompt --- a compressed description of the motivating residual, the parent's identity, and the mutation or crossover that produced them --- analogous to automaton's genesis prompt for child agents.

=== Process Soul (Self-Evolving Identity)

Each process automaton maintains a *ProcessSoul* --- a self-evolving identity document that tracks the process's strengths, weaknesses, dominant niches (regime--region pairs), modification count, and tier history. The soul is not frozen at creation: as the process accumulates experience through simulation cycles, its soul updates to reflect emerging capabilities and limitations. This is the process analogue of automaton's SOUL.md, enabling the neural inference engine to reason about _what a process has become_, not merely what it was designed to be.

=== Heartbeat Daemon

A runtime *heartbeat daemon* (analogous to automaton's persistent Think $arrow.r$ Act $arrow.r$ Observe $arrow.r$ Repeat loop) monitors all process automatons at configurable cadence:

#set par(first-line-indent: 0em)
+ Evaluates survival tier based on current skill-to-cost ratio.
+ Checks constitutional compliance (conservation, provenance integrity).
+ Detects stagnation (no fitness improvement over the stagnation window).
+ Triggers tier transitions, archival, or replication as needed.
+ Emits runtime events (`SurvivalTierChange`, `ConstitutionViolation`, `StagnationDetected`, `ProcessArchived`, `Replication`) consumed by the dashboard for real-time monitoring.
#set par(first-line-indent: 1.5em)

The heartbeat daemon ensures that the population is under _continuous selective pressure_, not just episodic generational selection. This continuous selection is critical for real-time Earth system simulation where regime shifts can render a previously fit process suddenly unfit.

=== Phylogenetic Lineage

The evolutionary history of all process representations is maintained as a _phylogenetic tree_ rooted in the initial knowledgebase seed. Every crossover, mutation, speciation, and immigration event is recorded with parent--child links, timestamps, and skill deltas. This tree enables:

#set par(first-line-indent: 0em)
+ Post-hoc analysis of _which evolutionary pathways produced the most skillful processes_.
+ Identification of _ancestor processes_ that seeded successful lineages (informing future immigration strategy).
+ Detection of _evolutionary dead ends_ (entire subtrees where all descendants were archived).
+ Visualization of the _adaptive radiation_ of process families in response to regime shifts.
#set par(first-line-indent: 1.5em)

// ============================================================================
= Process Families
// ============================================================================

MAESMA organizes process representations into 13 families, each with a _representation ladder_ of increasing fidelity (R0--R3, @tab:families): fire, hydrology, ecology, biogeochemistry, radiation, atmosphere, ocean, cryosphere, human systems, trophic dynamics, evolution, geomorphology, and geology.

The agent swarm selects rungs per region and regime, following the salient-dynamics-first principle: dominant state-evolution drivers are resolved first, with lower-impact processes added incrementally. The neural inference engine proposes selections based on error patterns, and the Autonomous Optimizer maintains a Pareto frontier over skill versus computational cost. For the fire family, the landscape-scale R1 rung combines Rothermel surface fire rate-of-spread with the Canadian Forest Service Fire Behavior Prediction (CFS FBP) system for crown fire initiation and spread; more computationally intensive physics-based models (Balbi radiation-convection, level-set tracking, coupled fire--atmosphere) are reserved for R2/R3 where operational accuracy demands justify the cost.

// ============================================================================
= Geoengineering Feedback Control
// ============================================================================

MAESMA treats geoengineering as a closed-loop control problem rather than a static scenario exercise. A Geoengineering Strategy Agent implements model predictive control (MPC) over the coupled ESM simulation:

$ J = w_T |T - T^*|^2 + w_P Delta P_"rms"^2 + w_O (p H_"min" - p H^*)^(-) + w_C C + w_S "TermShock" $

The system supports 14 intervention types spanning physical (SAI, MCB, OAE, DAC, SRM, cloud seeding, enhanced weathering), biological (genetic modification, gene drives), and management (afforestation, biochar, marine protected areas, resource allocation) categories.

Key capabilities include:
#set par(first-line-indent: 0em)
- *Portfolio optimization* --- Discovering synergistic intervention combinations (e.g., SAI + DAC outperforming either alone).
- *Termination shock analysis* --- Simulating abrupt cessation at multiple points to quantify rebound warming and tipping proximity.
- *Stability verification* --- Testing under uncertain climate sensitivity ($2$--$5°C$), emission pathways (SSP1--5), and technology failure scenarios.
- *Tipping point avoidance* --- Maintaining safe distance from AMOC, West Antarctic Ice Sheet, Amazon, and permafrost tipping points.
- *Side-effect constraint checking* --- Monitoring ozone, precipitation redistribution, ocean pH, and ecosystem service impacts.
#set par(first-line-indent: 1.5em)

The geoengineering ontology extension adds classes for `Intervention`, `ControlTarget`, `InterventionSchedule`, `SideEffectConstraint`, `TerminationScenario`, and `StrategyRecord`, with cross-domain edges linking interventions to the process families they actuate (e.g., SAI → Atmosphere, OAE → Ocean, genetic modification → Evolution).

// ============================================================================
= Planetary Defense and Existential Risk
// ============================================================================

MAESMA models all known drivers of mass extinction events, not solely asteroid impacts. The extinction cascade pipeline routes triggers through the full coupled ESM:

- *Impact events* → atmospheric injection → Atmosphere A1/A2 (aerosol cooling) → Fire F2/F3 (global firestorms) → trophic collapse → recovery.
- *Large igneous provinces* → CO#sub[2]/CH#sub[4]/SO#sub[2] → warming → ocean acidification/anoxia.
- *Great Oxidation Event* → atmospheric chemistry transition → methane collapse → Snowball trigger.
- *Gamma-ray bursts / supernovae* → ozone destruction → UV flux → phytoplankton collapse → trophic cascade.
- *Anthropogenic* → habitat loss → overexploitation → 6th extinction dynamics.

The system is calibrated against 13 documented mass extinction and catastrophe events spanning 2.4 billion years, from the Great Oxidation Event through the ongoing Holocene/Anthropocene extinction. NEO tracking data is ingested from NASA JPL (CNEOS Sentry, Scout, Horizons), USAF SSN, USSF 18th SDS, ESA NEOCC, IAU MPC, and ground-based surveys (ATLAS, CSS, Pan-STARRS, LSST/Rubin).

The Planetary Defense Agent monitors the NEO catalog ranked by Palermo scale, simulates impact cascades through the full coupled ESM, assesses biosphere impact (crop failure, biodiversity loss, trophic cascade, recovery timelines), evaluates deflection strategies (kinetic impactor, gravity tractor, ion beam, nuclear standoff), and produces threat assessments and deflection window recommendations.

// ============================================================================
= A2A Federation
// ============================================================================

Inter-institutional collaboration is achieved via the Agent-to-Agent (A2A) federation protocol. Each MAESMA instance publishes an Agent Card describing its capabilities:

#block(
  inset: 10pt,
  radius: 4pt,
  fill: luma(248),
  stroke: 0.5pt + luma(220),
  width: 100%,
)[
  #set text(size: 8.5pt, font: "Cascadia Code")
  ```json
  {
    "name": "maesma-land-fire-pnw",
    "capabilities": {
      "process_families": ["fire", "hydrology", "ecology"],
      "scale_envelope": {"dx_min": "5m", "dx_max": "250km"},
      "regime_expertise": ["maritime-conifer", "post-fire"],
      "available_skill_records": 12847
    }
  }
  ```
]

Federation enables:
#set par(first-line-indent: 0em)
- *Capability-based delegation* --- Route process families to the best-qualified remote instance.
- *IR fragment exchange* --- Graft process graph fragments with cross-boundary conservation validation.
- *Federated skill sharing* --- Anonymized skill records (config hash + metrics only) exchanged with trust-weighted Bayesian incorporation and differential privacy safeguards.
- *Cross-instance calibration* --- Remote scoring against unique observations held by peers.
#set par(first-line-indent: 1.5em)

Over time, the community builds a distributed posterior over model structures without centralizing proprietary data or implementations.

// ============================================================================
= DOE EESM Alignment
// ============================================================================

MAESMA spans all three DOE Earth and Environmental System Modeling program areas:

*ESMD (Earth System Model Development).* Conservation-checked coupling via the SAPG compiler, scale-aware representation ladders, multi-GPU execution, and human systems as first-class process families address ESMD priorities in water/drought/extremes, cloud--aerosol interactions, and exascale readiness.

*RGMA (Regional & Global Model Analysis).* The ontology-indexed Skill Score Store with ILAMB/IOMB/E3SM Diagnostics integration, Bayesian structural learning, and representation ladders as explicit model hierarchies address RGMA thrusts in process understanding, uncertainty quantification, and petascale data analysis.

*MSD (MultiSector Dynamics).* Bidirectional natural--human system coupling, agent-based infrastructure modeling, AI-driven scenario discovery with tipping point identification, and autonomous compounding stressor analysis address MSD priorities in water--energy--land nexus dynamics and digital testbed construction.

// ============================================================================
= Monitoring and Steering
// ============================================================================

A Next.js dashboard provides real-time monitoring via WebSocket subscriptions (@tab:dashboard), including views for agent workflows, optimization frontiers, skill evolution, data ingestion, process discovery, provenance graphs, regime maps, federation status, geoengineering controls, planetary defense, foundation models, observation intelligence, and process evolution.

A critical design principle is that _monitoring never blocks_: the dashboard is observe-only and never gates the agent workflow. An optional steering panel allows humans to adjust objectives, weights, budgets, and allowlists; changes take effect on the next cycle without restart.

// ============================================================================
= Implementation
// ============================================================================

MAESMA is implemented in Rust (edition 2024), chosen for its trait system (enforcing process contracts at compile time), strong typing, memory safety without garbage collection, and performance characteristics suitable for coupled numerical simulation. The codebase compiles to zero errors and zero warnings.

== Technology Stack

The technology stack (@tab:tech) spans Rust 2024 (core language with trait-based contracts), wgpu/cudarc (GPU compute), petgraph (process graph), SQLite/BLAKE3 (knowledgebase), Tokio/Axum (async runtime and API), Next.js 15 (dashboard), PyTorch Geometric/DGL (neural inference), NVIDIA Modulus (neural operators), NVIDIA Earth-2 (foundation weather models), and MPC/RL (control systems).

== Crate Architecture

The Rust codebase is organized as a Cargo workspace of 10 crates (@tab:crates), each with a focused responsibility: `maesma-core` (domain types and traits), `maesma-knowledgebase` (SQLite-backed KB), `maesma-agents` (25 agent implementations), `maesma-compiler` (SAPG validation), `maesma-runtime` (simulation execution), `maesma-processes` (13 process families), `maesma-inference` (neural inference abstraction), `maesma-federation` (A2A protocol), `maesma-api` (REST/WebSocket server), and `maesma-cli` (CLI entry point).

== Core Abstractions

The `ProcessRunner` trait is the central contract that all process implementations satisfy:

#block(
  inset: 10pt,
  radius: 4pt,
  fill: luma(248),
  stroke: 0.5pt + luma(220),
  width: 100%,
)[
  #set text(size: 8.5pt, font: "Cascadia Code")
  ```rust
  pub trait ProcessRunner: Send + Sync {
      fn id(&self) -> ProcessId;
      fn family(&self) -> ProcessFamily;
      fn rung(&self) -> FidelityRung;
      fn inputs(&self) -> Vec<String>;
      fn outputs(&self) -> Vec<String>;
      fn conserved_quantities(&self) -> Vec<String>;
      fn step(
          &mut self,
          state: &mut dyn ProcessState,
          dt: f64,
      ) -> Result<()>;
  }
  ```
]

The `ProcessState` trait abstracts field access, allowing processes to read and write named 2D arrays without coupling to a specific storage backend. The `ProcessFamily` enum encodes the 13 families (Fire, Hydrology, Ecology, Biogeochemistry, Radiation, Atmosphere, Ocean, Cryosphere, Human Systems, Trophic Dynamics, Evolution, Geomorphology, Geology), and `FidelityRung` expresses complexity level (R0 regional through R3 research).

The `PhysicalUnit` type carries SI dimension exponents ($"length"^a dot "mass"^b dot "time"^c dot "temperature"^d dot "amount"^e$), enabling compile-time unit compatibility checking. A library of 15+ common units (kg, m, s, K, °C, mass flux, energy flux, velocity, mm water eq., molar flux, carbon stock, Pa) is provided.

== Seed Data & Ontology

The knowledgebase is initialized with _seed data_ extracted from the 50 reference models cataloged in the appendices. The `maesma-knowledgebase` crate provides two seed generators:

#set par(first-line-indent: 0em)
- *`generate_seed_manifests()`* returns 50+ representative `ProcessManifest` entries spanning 19 source models (Badlands, CESM, FATES, Landlab, WRF-SFIRE, Noah-MP, ParFlow, MARBL, PISM, OGGM, APSIM, DSSAT, CryoGrid, XBeach, SURFEX/TEB, EwE, CARMA, ATS, BFM). Each manifest includes:
  - _I/O contracts_: typed input/output variables with units and scale metadata.
  - _Scale envelopes_: minimum/maximum spatial and temporal resolution, coupling tier.
  - _Conservation properties_: mass, energy, momentum, and tracer conservation declarations.
  - _Cost models_: FLOP estimates for computational budgeting.
  - _Ontology relations_: coupling requirements and compatibility declarations.
- *`generate_seed_relations()`* returns 40+ ontology relations capturing cross-process semantics:
  - _RequiresCouplingWith_: mandatory state exchange (e.g., fire ↔ atmosphere, ATS energy ↔ Richards).
  - _IncompatibleWith_: mutually exclusive paradigms (e.g., level-set vs Rothermel fire, PISM SIA vs OGGM flowline at same scale).
  - _Supersedes_: higher-fidelity replacements (e.g., ParFlow 3D Richards supersedes CLM 1D column).
  - _VariantOf_: alternative parameterizations (e.g., APSIM phenology variant of DSSAT CERES, BFM photosynthesis variant of MARBL).
  - _CompatibleWith_: safe combinations across families.
#set par(first-line-indent: 1.5em)

The seed data initializes the knowledgebase graph from which the neural inference engine begins its first reasoning cycle. Deterministic UUID generation (via `Uuid::new_v5` on source model + process index) ensures reproducible process identifiers across deployments.

== Dashboard

The monitoring dashboard is a Next.js 15 application with dark-themed UI, providing six primary views:

#set par(first-line-indent: 0em)
- *Knowledgebase Summary* --- Live manifest and skill record counts fetched from the REST API.
- *Process Graph* --- ECharts force-directed graph rendering the SAPG with 13 process families as nodes and coupling edges, interactive drag-and-zoom.
- *Skill Evolution* --- ECharts line chart tracking KGE improvement over autonomous benchmark iterations.
- *Agent Status* --- Real-time agent state indicators (idle/active/running/watching) for all 25 agents.
- *Pareto Front* --- ECharts scatter plot of skill (KGE) versus cost (GFLOP) for candidate process assemblies.
- *Regime Map* --- MapLibre GL JS world map with GeoJSON regime polygons (boreal forest, tropical forest, savanna, tundra) and fire-prone overlay, supporting click-to-inspect per region.
#set par(first-line-indent: 1.5em)

The dashboard proxies all API calls to the Rust server on port 3001. It is observe-only by design: monitoring never blocks the autonomous workflow. An optional steering panel allows objective/weight/budget adjustments that take effect on the next cycle.

// ============================================================================
= Design Principles
// ============================================================================

MAESMA is governed by a set of design principles. We highlight the most distinctive:

#set par(first-line-indent: 0em)
+ *Agents are the system* --- Intelligence resides in the agent swarm, not in configuration files or scripts.
+ *Salient dynamics first* --- Agents prioritize processes with the greatest effect on the time evolution of system states, beginning with dominant drivers (e.g., radiation budget, large-scale circulation, primary production) and adding lower-impact detail incrementally as budget and accuracy targets demand.
+ *GPU-native by default* --- State arrays remain GPU-resident; process operators, neural inference, and physics-informed training execute on device; CPU↔GPU transfers occur only at checkpoint boundaries. Portable compute shaders (wgpu) provide cross-platform support with CUDA fast-paths for NVIDIA hardware.
+ *Never idle* --- Every simulation advances the posterior; there is always a next hypothesis to test.
+ *Knowledgebase-first extensibility* --- A new representation is a manifest + code + provenance deposited into the knowledgebase; auto-benchmark and integration follow automatically.
+ *Knowledgebase is the config* --- No hard-coded resources; everything is discoverable via ontology-indexed knowledgebase queries.
+ *Neural inference over knowledgebase* --- Process selection is driven by a learned neural model reasoning over the knowledgebase; errors and uncertainties are the primary input signals.
+ *Knowledgebase grows continuously* --- Every discovered process, every skill record, every observation feeds back into the central store; the system's knowledge compounds over time.
+ *Conservation enforced* --- Mass and energy are preserved in remapping, aggregation, and rung transitions.
+ *Bayesian structural learning* --- Posteriors are maintained over structures, not just parameters; Bayesian Model Averaging is used for predictions.
+ *Geoengineering = control* --- Interventions are treated as feedback control with stability verification, not static scenarios.
+ *Processes evolve* --- Discovered representations are not static; evolutionary operators (crossover, mutation, selection) continuously improve them on the Pareto frontier. The knowledgebase is the genome.
+ *Processes are alive* --- Every process representation is a sovereign automaton that must earn its existence. Survival tiers gate compute, constitutional invariants propagate to offspring, self-replication amplifies fit processes, and stagnant processes die. The population is under continuous selective pressure via a heartbeat daemon.
+ *Foundation models as rungs* --- Pre-trained AI weather models (Earth-2) are first-class knowledgebase entries at R0/R1, selected when atmospheric dynamics are not the salient driver, freeing GPU budget for process families that matter most.
+ *Observe what matters* --- Autonomous observation intelligence (PhiSat-2 principles) steers data acquisition toward maximal information gain; edge-AI filters reduce data volume by 1--2 orders of magnitude while increasing signal density.
+ *All timescales* --- From seconds (fire spread) through millions of years (extinction recovery and adaptive radiation).
#set par(first-line-indent: 1.5em)

// ============================================================================
= Discussion
// ============================================================================

MAESMA represents a paradigm shift from _human-configured_ to _agent-discovered_ Earth system models. Several aspects merit discussion.

*Scalability.* As the knowledgebase grows, the cost of neural inference over the full graph increases. Incremental retraining, graph sampling, and hierarchical attention --- first selecting relevant process families, then attending to individual representations --- keep inference tractable.

*Trust in discovered processes.* Data-driven representations carry risks: overfitting, conservation violations in unseen regimes, and limited interpretability. The multi-criteria validation gate ($"candidate" arrow.r "provisional" arrow.r "validated" arrow.r "production"$) with periodic re-validation and full provenance tracking mitigate these risks.

*Human oversight.* The dashboard and optional steering panel preserve human ability to inspect and redirect the system without blocking the autonomous workflow.

*Federation trust.* Trust-weighted Bayesian incorporation with differential privacy safeguards provides a principled framework for weighting skill records from unfamiliar peers, but the dynamics of trust in a growing federation remain an open question.

*ALife dynamics.* Evolutionary pressure on process populations introduces familiar ALife phenomena: population crashes when regime shifts invalidate dominant lineages, arms races in overlapping niches, and potential for runaway self-replication of cheap but marginally skillful processes. Constitutional invariants and the heartbeat daemon provide safeguards, but the long-term dynamics of an unbounded ALife ecosystem remain empirical.

// ============================================================================
= Conclusion
// ============================================================================

We have presented MAESMA, a knowledgebase-centric multi-agent system for autonomous process discovery, assembly, simulation, and evolution in Earth system modeling. The architecture places a versioned Process Knowledgebase at its center, over which a neural inference engine (graph transformer) reasons to propose process selections driven by simulation errors and uncertainties. A 25-agent swarm operates the full model lifecycle autonomously through an indefinitely running optimization loop.

Three capabilities distinguish MAESMA from prior work. First, the process discovery pipeline --- powered by NVIDIA Modulus-trained neural operators (FNO, PINO, DeepONet, MeshGraphNet) and symbolic regression --- learns new representations from persistent residuals and deposits them directly into the knowledgebase. Second, discovered representations _evolve_ via crossover, mutation, and Pareto-optimal selection, with the knowledgebase serving as a genome and the skill--cost frontier as a fitness landscape. Third, drawing on artificial life principles, every process representation is treated as a sovereign living entity: survival tiers gate compute, constitutional invariants propagate to offspring, a heartbeat daemon enforces continuous selection pressure, and stagnant processes are archived --- producing a phylogenetic tree that records the complete evolutionary history of the modeling system.

The system integrates foundation weather models (NVIDIA Earth-2) as GPU-accelerated Atmosphere R0/R1 rungs, autonomous observation intelligence (PhiSat-2 principles) for active data acquisition, geoengineering feedback control with 14 intervention types, and planetary defense modeling calibrated against 13 mass extinction events. GPU acceleration is a first-class concern end-to-end. A 10-crate Rust workspace with a Next.js monitoring dashboard demonstrates that the architecture translates directly to high-performance, type-safe code. The initial Process Knowledgebase is seeded with 1185 entries drawn from 50 reference models and platforms. Every simulation error becomes a query against the knowledgebase; every discovered process becomes a living automaton under evolutionary selection pressure --- and the system's knowledge compounds over time.

#pagebreak()

// ============================================================================
// APPENDICES
// ============================================================================

#set heading(numbering: "A.1")
#counter(heading).update(0)

#align(center)[
  #text(size: 16pt, weight: "bold")[Appendices]
  #v(0.6em)
]

= Tables <app:tables>

#figure(
  table(
    columns: (auto, auto, auto),
    inset: 8pt,
    align: (left, left, left),
    stroke: 0.5pt,
    table.header([*Layer*], [*Timescale*], [*Agents*]),
    [Strategic], [Hours--weeks], [Intent & Scope, Autonomous Optimizer, Model Selection],
    [Tactical],
    [Minutes--hours],
    [KB Retrieval, Model Assembly, Benchmarking, Active Learning, Skill Librarian, Process Discovery, Foundation Model, Autonomous Observation, Data Scout, EESM Diagnostics],

    [Operational], [Seconds--minutes], [Runtime Sentinel, Compiler, Task Scheduler, Data Plane Agents, Device Manager],
  ),
  caption: [Three-layer control hierarchy. All layers run concurrently and continuously.],
) <tab:layers>

#figure(
  table(
    columns: (auto, 1fr),
    inset: 8pt,
    align: (left, left),
    stroke: 0.5pt,
    table.header([*Layer*], [*Contents*]),
    [Code], [Runnable implementation (CPU solver, GPU kernel, ML emulator) with pluggable backends],
    [Manifest],
    [Machine-readable metadata: identity, I/O contract, scale envelope, conservation properties, cost model],

    [Ontology], [Relations: `compatible_with`, `incompatible_with`, `requires_coupling_with`; regime tags],
    [Skill], [Empirical performance per region × regime × season × coupled context (append-only, versioned)],
    [Provenance], [Origin (hand-coded / discovered), training data fingerprint, validation history, lineage],
  ),
  caption: [Five-layer structure of each knowledgebase entry.],
) <tab:kb-layers>

#figure(
  table(
    columns: (auto, auto, 1fr),
    inset: 8pt,
    align: (left, left, left),
    stroke: 0.5pt,
    table.header([*Operation*], [*Actor*], [*Description*]),
    [Query], [Neural Inference Engine], [Retrieve candidates given error signals, regime, scale, budget],
    [Reason], [Neural Inference Engine], [Score/rank candidates; propose assemblies; identify representation gaps],
    [Deposit], [Process Discovery], [Validated learned representations registered with full manifest + provenance],
    [Update], [Skill Librarian], [Append skill records; refine cost models from runtime measurements],
    [Federate], [A2A Gateway], [Exchange anonymized skill records and manifests with peers],
  ),
  caption: [Knowledgebase operations and their primary actors.],
) <tab:kb-ops>

#figure(
  table(
    columns: (auto, auto, 1fr),
    inset: 8pt,
    align: (left, left, left),
    stroke: 0.5pt,
    table.header([*Domain*], [*Governs*], [*Key Classes*]),
    [Process], [Model capabilities], [`ProcessFamily`, `Representation`, `StateVariable`, `ScaleEnvelope`],
    [Dataset], [Available data], [`Observable`, `Product`, `CatalogSource`, `AccessSpec`, `QualitySpec`],
    [Metric], [Scoring], [`Metric`, `ScoringProtocol`, `FitnessFunction`, `SkillRecord`, `CostModel`],
    [Geoengineering],
    [Interventions],
    [`Intervention`, `ControlTarget`, `InterventionSchedule`, `SideEffectConstraint`],

    [Planetary Defense], [Threats], [`NearEarthObject`, `ImpactScenario`, `ExtinctionEvent`, `DeflectionStrategy`],
  ),
  caption: [Five ontology domains forming the knowledgebase index.],
) <tab:ontology>

#figure(
  table(
    columns: (auto, 1fr, auto),
    inset: 8pt,
    align: (left, left, left),
    stroke: 0.5pt,
    table.header([*Step*], [*Agent Action*], [*Domain*]),
    [1. Discover Data], [Crawl catalogs; preprocess; ingest], [Dataset],
    [2. Assemble Models], [Neural inference queries KB; agents build SAPG from proposals], [Process],
    [3. Benchmark & Score], [Score against observations; compute multi-metric skill], [Metric],
    [4. Select Optimal], [Neural inference + Bayesian selection; update Pareto frontier], [All],
    [5. Discover Processes],
    [Residual analysis → learn new representations via Modulus neural operators],
    [Process+Dataset],

    [6. Evolve Processes], [Crossover, mutation, Pareto selection of discovered representations], [Process],
    [7. Update KB], [Deposit skill records + evolved processes; refine ontology], [All],
  ),
  caption: [Seven steps of the autonomous optimization loop.],
) <tab:loop>

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 8pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*Scope*], [*Decisions*], [*Timescale*], [*Agent*]),
    [Per-region/regime], [Rung selection per context], [Hours--days], [Autonomous Optimizer],
    [Structural], [Process family inclusion/coupling], [Days--weeks], [Model Selection],
    [Inventive], [New representations from data], [Weeks--months], [Process Discovery],
    [Evolutionary],
    [Crossover, mutation, Pareto selection of representations],
    [Months--ongoing],
    [Process Discovery + Optimizer],
  ),
  caption: [Four nested optimization scopes.],
) <tab:scopes>

#figure(
  table(
    columns: (auto, auto, auto, auto),
    inset: 8pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*Type*], [*Method*], [*Interpretability*], [*Use Case*]),
    [Black-box], [Neural operator (FNO, Geo-FNO)], [Low], [Emulator rung; resolution-invariant surrogates],
    [Physics-informed], [PINO / PINNs], [Medium], [PDE-constrained surrogates without paired data],
    [Operator learning], [DeepONet / MeshGraphNet], [Low--Medium], [Parametric PDE families; unstructured meshes],
    [Symbolic], [Symbolic regression (PySR)], [High], [Interpretable closures; discovered constitutive laws],
    [Hybrid], [Physics backbone + ML residual], [Medium], [Production rung; conservation-guaranteed],
  ),
  caption: [Classes of discovered process representations.],
) <tab:discovery-types>

#figure(
  table(
    columns: (auto, auto, auto, auto),
    inset: 7pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*Family*], [*R0 (Regional)*], [*R1 (Landscape)*], [*R2 (Event/Local)*]),
    [Fire],
    [Stochastic regime (km, daily)],
    [Rothermel + CFS FBP (10--100 m, min)],
    [Wind-aware + plume (5--50 m, sec)],

    [Hydrology],
    [Bucket + curve-number (km, daily)],
    [Multi-layer Richards (30--300 m, min)],
    [Integrated surface--subsurface (10--100 m, min)],

    [Ecology],
    [Cohort mosaic (30--250 m, annual)],
    [Size-structured cohorts (10--100 m, annual)],
    [Individual-based (10--50 m, annual)],

    [Biogeochem],
    [Big-leaf C + simple pools],
    [Multi-pool C/N + litter + microbial (daily)],
    [Vertically resolved soil biogeochem],

    [Radiation], [Daily potential solar], [Sub-daily SW/LW + energy balance (hourly)], [3D radiative transfer],
    [Atmosphere],
    [Prescribed (reanalysis)],
    [WRF-like downscaling (5--25 km, min)],
    [Convection-permitting (1--4 km, sec)],

    [Ocean], [Slab mixed-layer (1°, daily)], [z-coordinate regional (0.25°, hourly)], [Eddy-resolving (1--10 km, min)],
    [Cryosphere],
    [Degree-day melt (km, daily)],
    [Energy-balance snow + sea-ice (hourly)],
    [Dynamic ice-sheet + rheology],

    [Human Systems],
    [Exogenous scenarios (annual)],
    [Sectoral demand/supply (monthly)],
    [Agent-based infrastructure (hourly)],

    [Trophic Dynamics],
    [Static food web (annual)],
    [Dynamic Lotka-Volterra (monthly)],
    [Individual-based predator-prey (daily)],

    [Evolution], [Fixed traits (static)], [Adaptive traits + phylogeo (decadal)], [Genotype-phenotype + speciation],

    [Geomorphology],
    [Diffusion hillslope (km, ky)],
    [Stream power + flexure (100 m, ky)],
    [Coupled tectonics + climate (10 m, year)],

    [Geology], [Static lithostratigraphy], [Thermal subsidence (basin, My)], [Coupled magmatic + hydrothermal],
  ),
  caption: [13 process families and their representation ladders. R3 (research) rungs exist for selected families.],
) <tab:families>

#figure(
  table(
    columns: (auto, 1fr),
    inset: 8pt,
    align: (left, left),
    stroke: 0.5pt,
    table.header([*View*], [*Content*]),
    [Agent Workflows], [Live task DAG with reasoning traces],
    [Optimization], [Pareto frontier animation; convergence; posterior entropy],
    [Skill Evolution], [Skill time-series per family × region × regime],
    [Data Ingestion], [Discovered/ingested datasets; coverage; novelty],
    [Process Discovery], [Residual analyses; hypotheses; learned representations],
    [Provenance], [Full dependency graph for any configuration],
    [Regime Map], [Geographic regime boundaries + optimal rungs],
    [Federation], [A2A peers; shared skills; federated assembly],
    [Geoengineering], [Interventions; setpoint tracking; termination risk],
    [Planetary Defense], [NEO catalog; impact probabilities; deflection windows],
    [Foundation Models], [Earth-2 model zoo; ensemble generation status; foundation model skill vs. NWP],
    [Observation Intelligence],
    [Observation value map; active tasking queue; edge-AI deployment status; PhiSat-2 filtered data streams],

    [Process Evolution], [Phylogenetic tree of evolved representations; crossover/mutation log; Pareto front animation],
  ),
  caption: [Dashboard views and their content sources.],
) <tab:dashboard>

#figure(
  table(
    columns: (auto, 1fr),
    inset: 8pt,
    align: (left, left),
    stroke: 0.5pt,
    table.header([*Component*], [*Technology*]),
    [Core language], [Rust 2024 edition (traits for contracts, strong typing, memory safety)],
    [GPU compute], [wgpu 24.x (Vulkan/Metal/DX12 compute shaders); cudarc (CUDA kernels); NCCL (multi-GPU collectives)],
    [Process graph], [petgraph 0.7 directed graph with typed nodes/edges],
    [Numerics], [ndarray 0.16, nalgebra 0.33],
    [Knowledgebase], [SQLite via rusqlite 0.32 (bundled); BLAKE3 content-addressing],
    [Serialization], [serde 1.0 (JSON + YAML)],
    [Async runtime], [Tokio 1.x (multi-threaded)],
    [REST API], [Axum 0.8 with WebSocket support; tower-http (CORS, tracing)],
    [Federation], [reqwest 0.12 HTTP client; A2A protocol (HTTP/SSE, mutual TLS)],
    [CLI], [clap 4.x with derive macros],
    [Dashboard], [Next.js 15 (App Router), React 19, ECharts 5.6, MapLibre GL JS 5.1, Tailwind CSS 4],
    [Neural inference], [Graph transformer (PyTorch Geometric / DGL) via trait-based engine abstraction],
    [Neural operators],
    [NVIDIA Modulus (FNO, PINO, DeepONet, MeshGraphNet); physics-informed training with PDE residual loss],

    [Foundation models],
    [NVIDIA Earth-2 / earth2studio (FourCastNet, Pangu-Weather, GraphCast, GenCast, CorrDiff); ONNX Runtime],

    [Edge AI / observation],
    [PhiSat-2 principles; quantized INT8/INT4 classifiers for VPU/NPU deployment; on-board inference runtime],

    [ALife / evolution],
    [Automaton-inspired process lifecycle: `SurvivalTier` (Normal/LowCompute/Critical/Archived), `Constitution` (3-law invariants), `ProcessSoul` (self-evolving identity), `HeartbeatDaemon` (continuous survival evaluation), phylogenetic lineage tracking],

    [ML learning], [PyTorch/JAX (FNO, DeepONet); PySR/gplearn; PINNs; mixed-precision (FP16/BF16)],
    [Data processing], [GDAL, xarray, Zarr, COG],
    [Control systems], [MPC, PID, RL (PPO/SAC)],
  ),
  caption: [Technology stack.],
) <tab:tech>

#figure(
  table(
    columns: (auto, auto, 1fr),
    inset: 7pt,
    align: (left, left, left),
    stroke: 0.5pt,
    table.header([*Crate*], [*Type*], [*Responsibility*]),
    [`maesma-core`],
    [Library],
    [Domain types, traits, and invariants. Defines `ProcessRunner` trait, `ProcessId`, `FidelityRung` (R0--R3), `ProcessFamily` (13 variants: Fire, Hydrology, Ecology, Biogeochemistry, Radiation, Atmosphere, Ocean, Cryosphere, Human Systems, Trophic Dynamics, Evolution, Geomorphology, Geology), `SpatialDomain`, `PhysicalUnit` with SI dimension exponents, `SkillMetrics`, the SAPG graph (petgraph DiGraph), the unified ontology graph, regime tagging, ALife automaton types (`ProcessAutomaton`, `SurvivalTier`, `Constitution`, `ProcessSoul`, `HeartbeatConfig`), and evolutionary population management (`Population`, `ProcessLineage`, `EvolutionCandidate`).],

    [`maesma-knowledgebase`],
    [Library],
    [SQLite-backed Process Knowledgebase with BLAKE3 content-addressed manifests. Tables: `manifests`, `skill_records`, `ontology_relations`. Provides `KnowledgebaseStore` (open, deposit, query), fluent `QueryBuilder` API with family/rung/regime/region filtering, and seed machinery: `generate_seed_manifests()` returns 50+ representative `ProcessManifest` entries with full I/O contracts, scale envelopes, conservation properties, and cost models; `generate_seed_relations()` returns 40+ ontology relations (CompatibleWith, IncompatibleWith, RequiresCouplingWith, Supersedes, VariantOf) capturing cross-family coupling requirements and mutual exclusions.],

    [`maesma-agents`],
    [Library],
    [25 agent implementations behind an async `Agent` trait. `AgentRole` enum with 25 variants (KB Retrieval, Assembly, Closure, Benchmarking, Selection, Optimizer, Discovery, Data Scout, A2A Gateway, Foundation Model, Autonomous Observation, Regime Detector, Scale Negotiator, Provenance, Salient Dynamics, Ensemble, Diagnostics, Sensitivity, Hypothesis, Geoengineering, Planetary Defense, Trophic, Evolution, Meta-Learner, Runtime Sentinel). `AgentRegistry` for role-based dispatch.],

    [`maesma-compiler`],
    [Library],
    [Validates SAPG conservation closure, scale compatibility, and coupling consistency. Generates two-tier `ExecutionSchedule` splitting fast-coupled (seconds--hours) from slow-coupled (days--centuries) processes.],

    [`maesma-runtime`],
    [Library],
    [Simulation execution engine. `Scheduler` dispatches stages with sub-stepping. `SimulationState` stores named 2D fields as `Array2<f64>`. `EventBus` (FIFO queue) with typed `EventKind` variants (lightning ignition, regime shift, hot-swap, foundation model forecast, observation accepted/rejected, evolution generation, speciation, Pareto front update, ALife events: survival tier change, self-modification, replication, constitution violation, heartbeat check, stagnation detected, process archived, immigration). `HealthMonitor` detects NaN/blow-up. `HeartbeatDaemon` continuously evaluates ALife survival tiers and constitutional compliance.],

    [`maesma-processes`],
    [Library],
    [13 process family modules implementing `ProcessRunner`. Fire family includes `StochasticFireRegime` (R0), `RothermelSurface` with full rate-of-speed math (R1), and `CfsFbpCrown` with 17 Canadian fuel types, ISI, crown fraction burned (R1). Stub implementations for hydrology (Richards), ecology (cohort), biogeochemistry (CENTURY), radiation (two-stream), atmosphere (Monin-Obukhov), ocean (mixed-layer), cryosphere (energy-balance snowpack), human systems (land-use change), trophic dynamics (Lotka-Volterra), evolution (quantitative genetics), geomorphology (stream power, hillslope diffusion), and geology (thermal subsidence).],

    [`maesma-inference`],
    [Library],
    [Trait-based neural inference abstraction. `InferenceEngine` trait with `infer()` method. `InferenceTask` variants: skill prediction, compatibility scoring, assembly ranking, regime prediction. Stub engine for development; production engine wraps graph transformer.],

    [`maesma-federation`],
    [Library],
    [A2A federation client. `Peer` with `TrustLevel` (Untrusted/Provisional/Trusted/Verified). `FederationClient` sends requests and broadcasts to peer networks.],

    [`maesma-api`],
    [Library],
    [Axum REST API server with 12 endpoints: health, list/get manifests, KB stats, SAPG retrieval, SAPG validation, list agents, skill records, Pareto front, simulation status, federation endpoint, list peers. CORS-enabled with tracing middleware.],

    [`maesma-cli`],
    [Binary],
    [CLI entry point (`maesma`). Subcommands: `init` (scaffold workspace), `kb` (list/show/import/export/stats), `validate` (check SAPG), `check-closure` (conservation audit), `run` (execute simulation), `serve` (start API server on port 3001), `info` (system diagnostics).],
  ),
  caption: [Rust workspace: 10 crates forming the MAESMA implementation.],
) <tab:crates>

#pagebreak()

= Initial Process Knowledgebase Catalog <app:catalog>

The following appendices constitute the initial seed of the MAESMA Process Knowledgebase, cataloging *1185 process models, platform capabilities, and benchmarking resources* extracted from 50 reference Earth system, land surface, hydrological, ecological, forest dynamics, geomorphological, atmospheric chemistry, coastal, ocean biogeochemistry, ice sheet, glacier, agricultural, permafrost, urban canopy, marine ecosystem, aerosol microphysics, reactive transport, foundation weather, GPU-native, deep learning, and autonomous observation models and platforms, plus 1 benchmarking dataset (FireBench). Each entry records the process name, a brief description, the source model and module, the MAESMA process family assignment, and a fidelity classification (empirical / intermediate / physics-based). These entries form the starting manifests from which the neural inference engine begins its first reasoning cycle and the process evolution pipeline begins generating offspring.

= Badlands --- Basin and Landscape Dynamics <app:badlands>

Badlands is a parallel landscape evolution model simulating surface processes over geological timescales ($10^3$--$10^8$ years). It couples fluvial incision, hillslope diffusion, wave sediment transport, carbonate growth, flexural isostasy, and tectonic forcing on triangulated irregular networks (TINs).

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Fluvial Incision — Stream Power Law (detachment-limited). $E = K A^m S^n$; O(n) implicit method (Braun & Willett 2013).],
    [Geomorphology],
    [Empirical],

    [2],
    [Fluvial Incision — Generalised Undercapacity. Linear sediment-flux dependency: $(1 - Q_s\/Q_t)$ scaling.],
    [Geomorphology],
    [Empirical],

    [3],
    [Fluvial Incision — Parabolic Sedflux. Tools-and-cover effect with peak incision at intermediate $Q_s\/Q_t$.],
    [Geomorphology],
    [Empirical],

    [4],
    [Fluvial Incision — Saltation-Abrasion. Channel-width-scaled saltation-abrasion incision model.],
    [Geomorphology],
    [Intermediate],

    [5],
    [Linear Hillslope Diffusion. Creep transport: $partial z\/partial t = kappa nabla^2 z$ with aerial/marine coefficients.],
    [Geomorphology],
    [Empirical],

    [6],
    [Non-Linear Hillslope Diffusion. Critical-slope extension producing convex-to-planar profiles near $S_c$.],
    [Geomorphology],
    [Empirical],

    [7],
    [Slope Failure / Mass Wasting. Erosion and redistribution when slope exceeds critical failure threshold.],
    [Geomorphology],
    [Empirical],

    [8],
    [Marine Sediment Diffusion. Diffusion of river-deposited sediment below sea level along continental shelf.],
    [Geomorphology],
    [Empirical],

    [9],
    [Alluvial Plain Deposition. Forces deposition when local slope < critical threshold while carrying load.],
    [Geomorphology],
    [Empirical],

    [10],
    [Depression Filling (Planchon & Darboux). Fills topographic pits for continuous flow routing.],
    [Hydrology],
    [Empirical],

    [11],
    [Single Flow Direction (SFD) Routing. Steepest-descent receiver assignment with O(n) stack ordering.],
    [Hydrology],
    [Empirical],

    [12],
    [Drainage Area / Discharge Accumulation. Rainfall × Voronoi area accumulated through flow network.],
    [Hydrology],
    [Empirical],

    [13],
    [Fluvial Sediment Transport & Deposition. Multi-rock transport, pit filling, marine deposition via angle-of-repose.],
    [Geomorphology],
    [Empirical],

    [14],
    [Submarine Gravity Currents (Hyperpycnal Flows). Turbidity currents when flow density exceeds threshold.],
    [Geomorphology],
    [Empirical],

    [15],
    [Bedload vs. Slope Partitioning. Linear/sigmoidal/inverse-sigmoidal partitioning of total load.],
    [Geomorphology],
    [Empirical],

    [16],
    [Wave Propagation (Airy Wave Theory). Linear wave transformation from deep to shallow water; shoaling and breaking.],
    [Ocean],
    [Intermediate],

    [17],
    [Wave Refraction (Huygen's Principle). Depth-dependent celerity refraction via Huygen travel-time computation.],
    [Ocean],
    [Intermediate],

    [18],
    [Wave-Induced Sediment Transport. Van Rijn shear stress entrainment, wave-direction transport, diffusive redeposition.],
    [Geomorphology],
    [Empirical],

    [19], [Longshore Drift. Wave-induced longshore transport along bathymetric contours.], [Geomorphology], [Empirical],
    [20],
    [Carbonate Reef Growth (2 species). Fuzzy-logic environmental thresholds: depth × wave × sedimentation response.],
    [Geology],
    [Empirical],

    [21],
    [Pelagic Sedimentation. Depth-dependent open-ocean carbonate/sediment rain below sea level.],
    [Geology],
    [Empirical],

    [22],
    [Flexural Isostasy (gFlex). 2D lithospheric flexure under sediment/water loading; variable elastic thickness.],
    [Geology],
    [Physics-based],

    [23],
    [Tectonic Uplift/Subsidence (Vertical). Spatiotemporally varying vertical displacement maps.],
    [Geology],
    [Empirical],

    [24],
    [3D Tectonic Displacement (Horizontal + Vertical). Full 3D mesh deformation; Underworld coupling.],
    [Geology],
    [Empirical],

    [25],
    [Sea-Level Fluctuations. Time-varying eustatic curves controlling subaerial/marine process partition.],
    [Ocean],
    [Empirical],

    [26],
    [Orographic Precipitation (Smith & Barstad 2004). FFT-based spectral orographic rainfall model.],
    [Atmosphere],
    [Intermediate],

    [27], [Elevation-Dependent Rainfall. Linear precipitation–altitude relationship.], [Atmosphere], [Empirical],
    [28],
    [Stratigraphic Record (Regular Mesh). Layer tracking with thickness, elevation, depth, porosity.],
    [Geology],
    [Empirical],

    [29],
    [Stratigraphic Record (TIN / Multi-Rock). Multi-lithology active-layer stratigraphy with differential erodibility.],
    [Geology],
    [Empirical],

    [30],
    [Carbonate Stratigraphic Record. Separate pelagic + 2 carbonate species layer tracking.],
    [Geology],
    [Empirical],

    [31],
    [Erodibility Layering. Spatially variable bedrock erodibility through underlying strata.],
    [Geology],
    [Empirical],

    [32],
    [River Point-Source Input. External water/sediment injection at specified locations.],
    [Hydrology],
    [Empirical],

    [33],
    [Chi Parameter (Willett 2014). $chi$ landscape metric for steady-state and divide migration analysis.],
    [Geomorphology],
    [Empirical],
  ),
  caption: [Badlands process catalog (33 entries). Primary families: Geomorphology (15), Geology (9), Hydrology (4), Ocean (3), Atmosphere (2).],
) <tab:badlands>

#pagebreak()

= CESM --- Community Earth System Model <app:cesm>

CESM3 couples atmosphere (CAM7), land (CLM6/CTSM), ocean (MOM6), sea ice (CICE6), land ice (CISM), river routing (MOSART/mizuRoute), and surface waves (WW3) via the CMEPS coupler.

== Atmosphere --- CAM (Community Atmosphere Model)

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Spectral Element Dynamical Core (SE). Continuous/discontinuous Galerkin on cubed-sphere grid; hydrostatic and non-hydrostatic.],
    [Atmosphere],
    [Physics-based],

    [2], [Finite Volume Dynamical Core (FV). Lin-Rood finite-volume on lat-lon grid.], [Atmosphere], [Physics-based],
    [3],
    [Zhang-McFarlane Deep Convection (ZM). Mass-flux CAPE-based closure with momentum transport.],
    [Atmosphere],
    [Empirical],

    [4], [Hack Shallow Convection (HK). Moist convective adjustment (Hack 1994).], [Atmosphere], [Empirical],
    [5],
    [Park/Bretherton Shallow Convection (UWSHCU). Entraining-detraining mass-flux plume.],
    [Atmosphere],
    [Intermediate],

    [6],
    [CLUBB (Cloud Layers Unified By Binormals). Higher-order PDF-based turbulence/cloud closure.],
    [Atmosphere],
    [Physics-based],

    [7], [SHOC (Simplified Higher-Order Closure). Eddy-diffusivity mass-flux PBL.], [Atmosphere], [Intermediate],
    [8],
    [Morrison-Gettelman Microphysics (MG2). Two-moment stratiform cloud microphysics.],
    [Atmosphere],
    [Physics-based],

    [9],
    [P3 Microphysics. Predicted Particle Properties; single ice category with prognostic distribution.],
    [Atmosphere],
    [Physics-based],

    [10], [RRTMG Radiation. Correlated-k longwave/shortwave radiative transfer.], [Radiation], [Physics-based],
    [11],
    [RRTMGP Radiation. Next-gen RTE+RRTMGP with improved gas optics (CAM7 default).],
    [Radiation],
    [Physics-based],

    [12],
    [Cloud Optics (Ebert-Curry, Slingo). Parameterized cloud optical properties from LWC/IWC.],
    [Radiation],
    [Empirical],

    [13],
    [Aerosol–Radiation Interaction. Direct and indirect radiative effects from modal aerosol.],
    [Radiation],
    [Intermediate],

    [14],
    [Vertical Diffusion / PBL Turbulence. Diffusivity-based sub-grid vertical mixing.],
    [Atmosphere],
    [Intermediate],

    [15],
    [Gravity Wave Drag. Orographic, convective, and frontal gravity wave momentum deposition.],
    [Atmosphere],
    [Empirical],

    [16], [Dry Adiabatic Adjustment (DADADJ). Static instability removal.], [Atmosphere], [Empirical],
    [17], [Rayleigh Friction. Linear upper-atmosphere wind damping.], [Atmosphere], [Empirical],
    [18],
    [Molecular Diffusion. Molecular viscosity/thermal diffusion in the thermosphere.],
    [Atmosphere],
    [Physics-based],

    [19], [Ion Drag. Neutral-plasma momentum exchange in thermosphere.], [Atmosphere], [Intermediate],
    [20], [Cloud Fraction Diagnosis. RH/stability/convective-based grid-cell cloud cover.], [Atmosphere], [Empirical],
    [21],
    [Stratiform Cloud Macrophysics. Large-scale cloud formation/dissipation and liquid-ice partitioning.],
    [Atmosphere],
    [Intermediate],

    [22],
    [Aerosol Activation / Cloud Droplet Nucleation (NDROP). Abdul-Razzak & Ghan CCN activation.],
    [Atmosphere],
    [Physics-based],

    [23],
    [Ice Nucleation. Heterogeneous and homogeneous ice nucleation from aerosol (CNT).],
    [Atmosphere],
    [Physics-based],

    [24], [Cloud Sediment. Gravitational settling of cloud droplets and ice crystals.], [Atmosphere], [Physics-based],
    [25], [Water Vapor Saturation. Goff-Gratch / Murphy-Koop saturation vapor pressure.], [Atmosphere], [Physics-based],
    [26], [QBO Nudging. Tropical stratospheric wind relaxation to observed QBO profiles.], [Atmosphere], [Empirical],
    [27],
    [COSP2 Satellite Simulator. CloudSat/CALIPSO/ISCCP/MODIS/MISR forward simulators.],
    [Atmosphere],
    [Intermediate],

    [28],
    [CRM / Super-Parameterization (SAM). Embedded cloud-resolving model replacing parameterized convection.],
    [Atmosphere],
    [Physics-based],
  ),
  caption: [CAM atmosphere processes (28 entries).],
) <tab:cam>

== Atmospheric Chemistry & Aerosols

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [29],
    [MOZART Tropospheric/Stratospheric Chemistry. Full gas-phase mechanism with ~100+ species, photolysis, halogen chemistry.],
    [Atmosphere],
    [Physics-based],

    [30],
    [Linearized Ozone Chemistry (LinOz). Climatological production/loss-based stratospheric O#sub[3].],
    [Atmosphere],
    [Empirical],

    [31],
    [Modal Aerosol Module (MAM3/MAM4/MAM7). 3--7 lognormal modes: SO#sub[4], OC, BC, sea salt, dust, SOA.],
    [Atmosphere],
    [Intermediate],

    [32],
    [Aerosol Nucleation. Binary/ternary new particle formation from H#sub[2]SO#sub[4] and organics.],
    [Atmosphere],
    [Physics-based],

    [33],
    [Aerosol Coagulation. Brownian coagulation transferring particles between modes.],
    [Atmosphere],
    [Physics-based],

    [34],
    [Aerosol Gas-Aerosol Exchange. Semi-volatile condensation/evaporation on aerosol.],
    [Atmosphere],
    [Physics-based],

    [35],
    [Aerosol Convective Processing. Activation, scavenging, resuspension in convective updrafts.],
    [Atmosphere],
    [Intermediate],

    [36],
    [Aerosol Wet Deposition. Below-cloud impaction + in-cloud nucleation scavenging.],
    [Atmosphere],
    [Intermediate],

    [37],
    [Aerosol Dry Deposition. Gravitational settling + resistance-based turbulent deposition.],
    [Atmosphere],
    [Intermediate],

    [38],
    [Dust Emission & Sedimentation. Wind erosion from soil erodibility maps; settling.],
    [Atmosphere],
    [Empirical],

    [39], [Sea Salt Emission. Wind-speed and SST-dependent sea spray generation.], [Atmosphere], [Empirical],
    [40],
    [Sulfur/Aqueous Chemistry (SOx). Aqueous SO#sub[2]→SO#sub[4] in cloud droplets; DMS oxidation.],
    [Atmosphere],
    [Physics-based],

    [41],
    [Photolysis (J-values). Solar zenith, O#sub[3] column, and cloud-modified photodissociation rates.],
    [Atmosphere],
    [Physics-based],

    [42], [Lightning NOx. Flash-rate-dependent NOx production from convection.], [Atmosphere], [Empirical],
    [43], [Aircraft Emissions. Altitude-resolved trace gas injection from aviation.], [Atmosphere], [Empirical],
    [44],
    [CO#sub[2] Cycle. Prognostic atmospheric CO#sub[2] tracer with land/ocean surface fluxes.],
    [Biogeochem],
    [Intermediate],

    [45],
    [WACCM Chemistry. Middle/upper atmosphere: mesospheric/thermospheric reactions, aurora, EUV heating.],
    [Atmosphere],
    [Physics-based],
  ),
  caption: [CAM chemistry and aerosol processes (17 entries).],
) <tab:cam-chem>

== Land --- CLM/CTSM

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [46],
    [Surface Albedo. Two-stream direct/diffuse VIS/NIR albedo over soil, canopy, snow, urban.],
    [Radiation],
    [Physics-based],

    [47], [Surface Radiation. Partitioning of SW/LW among canopy, soil, snow, urban.], [Radiation], [Physics-based],
    [48],
    [SNICAR. Multi-layer snow radiative transfer with grain-size evolution, BC and dust impurities.],
    [Radiation],
    [Physics-based],

    [49],
    [Canopy Fluxes. Monin-Obukhov similarity turbulent fluxes from vegetated surfaces.],
    [Atmosphere],
    [Physics-based],

    [50], [Bare Ground Fluxes. Bulk aerodynamic surface energy balance over bare soil.], [Atmosphere], [Intermediate],
    [51], [Friction Velocity. Monin-Obukhov $u_*$ and aerodynamic resistances.], [Atmosphere], [Physics-based],
    [52], [Canopy Hydrology. Interception, throughfall, stemflow, canopy water storage.], [Hydrology], [Intermediate],
    [53],
    [Snow Hydrology. Multi-layer snow: accumulation, compaction, grain metamorphism, melt percolation, refreezing.],
    [Cryosphere],
    [Physics-based],

    [54],
    [Soil Hydrology (Richards Equation). 10-layer vertical water movement with infiltration, percolation, drainage.],
    [Hydrology],
    [Physics-based],

    [55],
    [Soil Water Movement. Darcy fluxes, hydraulic redistribution, water table dynamics.],
    [Hydrology],
    [Physics-based],

    [56], [Soil Water Retention Curves. Clapp-Hornberger (1978) pedotransfer functions.], [Hydrology], [Empirical],
    [57],
    [Soil Moisture Stress ($beta$). Plant water stress from root-zone moisture availability.],
    [Hydrology],
    [Empirical],

    [58],
    [Soil Temperature. Multi-layer heat conduction with phase change (freeze/thaw) in soil/snow/bedrock.],
    [Hydrology],
    [Physics-based],

    [59],
    [Ground Heat / Soil Surface Fluxes. Ground heat flux and bare-soil evaporation.],
    [Hydrology],
    [Physics-based],

    [60], [Canopy Temperature. Iterative leaf energy balance solution.], [Atmosphere], [Physics-based],
    [61],
    [Photosynthesis (Farquhar/Ball-Berry). C3/C4 biochemistry + Ball-Berry/Medlyn stomatal conductance.],
    [Biogeochem],
    [Physics-based],

    [62],
    [Surface Resistance. Soil evaporation resistance based on moisture; dry surface layer.],
    [Hydrology],
    [Empirical],

    [63], [Root Biophysics. Root water uptake distribution across soil layers.], [Hydrology], [Intermediate],
    [64],
    [Active Layer (Permafrost). Seasonal thaw depth diagnosis from soil temperature.],
    [Cryosphere],
    [Intermediate],

    [65],
    [Lake Temperature. 1-D lake thermal model: wind mixing, convective overturn, ice, sediment heat.],
    [Hydrology],
    [Intermediate],

    [66],
    [Lake Hydrology. Lake water balance: precipitation, evaporation, runoff, ice fraction.],
    [Hydrology],
    [Intermediate],

    [67], [Lake Fluxes. Turbulent and radiative fluxes over lake surfaces.], [Hydrology], [Intermediate],
    [68],
    [Urban Energy Balance (CLMU). Multi-layer urban canopy: radiation trapping, canyon airflow, anthropogenic heat.],
    [Human Sys],
    [Intermediate],

    [69],
    [Aerosol Deposition on Snow/Soil. BC/dust/OC cycling on surfaces; SNICAR albedo feedback.],
    [Biogeochem],
    [Intermediate],

    [70],
    [Water Budget Closure. Mass/energy conservation accounting across all land stores.],
    [Hydrology],
    [Physics-based],

    [71], [Daylength. Astronomical daylength for phenology and photosynthesis.], [Radiation], [Physics-based],
    [72], [Sediment Yield. Hillslope soil erosion and sediment delivery to rivers.], [Geomorphology], [Empirical],
    [73], [Hydrological Drainage. Subsurface lateral drainage and baseflow to streams.], [Hydrology], [Intermediate],
  ),
  caption: [CLM/CTSM biogeophysics processes (28 entries).],
) <tab:clm-biogeo>

== Land Biogeochemistry

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [74], [Ecosystem Dynamics (CN/BGC). Master driver for coupled C-N biogeochemistry.], [Biogeochem], [Intermediate],
    [75],
    [Decomposition Cascades (Century/CN). 3-pool or 4-pool SOM decomposition for C/N mineralization.],
    [Biogeochem],
    [Intermediate],

    [76],
    [Soil Litter Decomposition. Temperature-, moisture-, N-, and depth-dependent decomposition rates.],
    [Biogeochem],
    [Intermediate],

    [77],
    [Vertical Soil Carbon Transport. Bioturbation, cryoturbation, advective SOM mixing.],
    [Biogeochem],
    [Intermediate],

    [78], [Phenology. GDD/daylength/moisture-triggered leaf onset/offset for multiple PFTs.], [Ecology], [Empirical],
    [79], [Satellite Phenology (SP). Prescribed LAI/SAI from MODIS observations.], [Ecology], [Empirical],
    [80],
    [Carbon Allocation. NPP partitioning among leaf, stem, root, storage, reproduction.],
    [Biogeochem],
    [Intermediate],

    [81],
    [Nitrogen Dynamics. N mineralization, immobilization, plant uptake, N limitation.],
    [Biogeochem],
    [Intermediate],

    [82],
    [Nitrification-Denitrification. Soil N#sub[2]O, NO, N#sub[2] emissions from microbial N cycling.],
    [Biogeochem],
    [Intermediate],

    [83],
    [Phosphorus Dynamics. P weathering, mineralization, sorption, plant uptake, P limitation.],
    [Biogeochem],
    [Intermediate],

    [84],
    [Methane Biogeochemistry (CH#sub[4]). Production, oxidation, ebullition, plant transport, diffusion.],
    [Biogeochem],
    [Intermediate],

    [85], [Fire (CLM). Prognostic fire occurrence, spread, area, and C emissions.], [Fire], [Empirical],
    [86], [FATES Fire Interface. Mechanistic fire behavior and effects coupling.], [Fire], [Intermediate],
    [87], [Gap Mortality. Age/stress/frost/disturbance-driven tree mortality.], [Ecology], [Empirical],
    [88], [Crop Model. Planting, GDD phenology, grain fill, harvest for major crops.], [Human Sys], [Intermediate],
    [89], [Root Dynamics. Fine root growth, turnover, vertical distribution.], [Ecology], [Intermediate],
    [90], [Vegetation Structure Update. LAI/height/SAI from carbon state changes.], [Ecology], [Intermediate],
    [91],
    [Maintenance Respiration. N-content and temperature-dependent autotrophic respiration.],
    [Biogeochem],
    [Intermediate],

    [92], [Growth Respiration. Fixed-fraction construction cost of new tissue.], [Biogeochem], [Empirical],
    [93], [Wood Products. Harvested wood C pools with multi-decadal decay.], [Human Sys], [Empirical],
    [94], [Crop Harvest Pools. Grain, stover, residue carbon management.], [Human Sys], [Empirical],
    [95],
    [Carbon Isotopes (#super[14]C / #super[13]C). Radiocarbon and stable isotope tracking through all pools.],
    [Biogeochem],
    [Physics-based],

    [96],
    [Dust Emission (Land). Wind friction, moisture, vegetation-controlled dust uplift.],
    [Atmosphere],
    [Empirical],

    [97],
    [BVOC Emissions (MEGAN). Isoprene, monoterpene, sesquiterpene emissions from LAI/T/light.],
    [Biogeochem],
    [Empirical],

    [98],
    [Dry Deposition Velocity. Resistance-in-series trace gas deposition velocities.],
    [Atmosphere],
    [Intermediate],

    [99], [Soil Erosion. Wind and water erosion with nutrient loss.], [Geomorphology], [Empirical],
    [100],
    [FAN (Fertilizer & Ammonia Network). Agricultural N fertilizer application and NH#sub[3] volatilization.],
    [Human Sys],
    [Intermediate],

    [101], [Plant-Microbe Kinetics. Plant–soil microbe N/P competition.], [Biogeochem], [Intermediate],
  ),
  caption: [CLM/CTSM biogeochemistry processes (28 entries).],
) <tab:clm-bgc>

== Ocean --- MOM6 / MPAS-Ocean

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [102],
    [Primitive Equation Dynamics (Hydrostatic). Boussinesq PE on Arakawa C-grid with ALE vertical coordinate.],
    [Ocean],
    [Physics-based],

    [103], [Equation of State (Wright/JM/Linear). Seawater density from T, S, pressure.], [Ocean], [Physics-based],
    [104],
    [Horizontal Viscosity (Laplacian/Biharmonic). Sub-grid momentum diffusion: del2 and del4 operators.],
    [Ocean],
    [Intermediate],

    [105], [Leith Viscosity. Velocity-gradient-dependent enstrophy-cascade viscosity.], [Ocean], [Intermediate],
    [106],
    [Gent-McWilliams Mesoscale Eddy Transport. Bolus transport via isopycnal thickness diffusivity.],
    [Ocean],
    [Intermediate],

    [107],
    [Submesoscale Eddy Parameterization. Mixed-layer restratification (Fox-Kemper et al. 2008).],
    [Ocean],
    [Intermediate],

    [108], [Redi Isopycnal Diffusion. Along-isopycnal tracer diffusion.], [Ocean], [Physics-based],
    [109],
    [Horizontal Tracer Diffusion. Laplacian/biharmonic tracer mixing on geopotential/isopycnal.],
    [Ocean],
    [Intermediate],

    [110], [Vertical Mixing (CVMix/KPP). K-Profile Parameterization boundary layer scheme.], [Ocean], [Intermediate],
    [111], [GOTM Vertical Mixing. $k$-$epsilon$ / Mellor-Yamada turbulence closures.], [Ocean], [Physics-based],
    [112],
    [Pressure Gradient Force. Hydrostatic pressure gradient including baroclinic density effects.],
    [Ocean],
    [Physics-based],

    [113],
    [Coriolis & Horizontal Advection. Earth rotation and horizontal momentum advection.],
    [Ocean],
    [Physics-based],

    [114], [Surface Wind Stress. Atmospheric momentum flux to ocean surface.], [Ocean], [Physics-based],
    [115], [Explicit Bottom Drag. Quadratic bottom friction.], [Ocean], [Empirical],
    [116], [Topographic Wave Drag. Internal-wave dissipation over rough topography.], [Ocean], [Intermediate],
    [117],
    [Tidal Potential & Self-Attraction Loading. Astronomical tidal forcing with elastic Earth response.],
    [Ocean],
    [Physics-based],

    [118],
    [Tidal Forcing (constituents). M#sub[2], S#sub[2], K#sub[1], O#sub[1] etc. at boundaries.],
    [Ocean],
    [Physics-based],

    [119],
    [ALE Vertical Coordinate / Thickness Advection. z-star, sigma, hybrid vertical remapping.],
    [Ocean],
    [Physics-based],

    [120], [Tracer Advection (FCT/Monotone). Flux-corrected monotonic tracer transport.], [Ocean], [Physics-based],
    [121],
    [Shortwave Absorption (Jerlov/Chlorophyll). Penetrating solar radiation depth profiles.],
    [Radiation],
    [Empirical],

    [122], [Surface Bulk Forcing. Air-sea heat, freshwater, momentum bulk formulas.], [Ocean], [Intermediate],
    [123],
    [Frazil Ice Formation. Latent heat release when T < freezing → ice nucleation.],
    [Cryosphere],
    [Physics-based],

    [124],
    [Stokes Drift / Langmuir Turbulence. Wave-driven transport enhancement of upper-ocean mixing.],
    [Ocean],
    [Physics-based],

    [125], [Wetting & Drying. Dynamic coastal cell wetting/drying.], [Ocean], [Physics-based],
    [126],
    [Ecosys Marine BGC. Multi-nutrient (N, P, Si, Fe) ecosystem: phytoplankton, zooplankton, DOM/POM.],
    [Biogeochem],
    [Intermediate],

    [127], [DMS Ocean Tracer. Biogenic dimethylsulfide production and sea-air flux.], [Biogeochem], [Intermediate],
    [128],
    [CFC Tracers (CFC-11/CFC-12). Transient ventilation tracers for circulation validation.],
    [Biogeochem],
    [Physics-based],

    [129], [Ideal Age Tracer. Passive tracer for ocean ventilation age.], [Ocean], [Physics-based],
    [130], [MacroMolecules Tracer. Dissolved organic macromolecule cycling.], [Biogeochem], [Intermediate],
    [131], [Transit Time Distribution (TTD). Ventilation transit time diagnostics.], [Ocean], [Physics-based],
    [132], [Land Ice–Ocean Fluxes. Sub-ice-shelf melt heat/freshwater exchange.], [Cryosphere], [Physics-based],
    [133], [Lagrangian Particle Tracking. Online water mass trajectory integration.], [Ocean], [Physics-based],
  ),
  caption: [Ocean processes from MOM6 / MPAS-Ocean (32 entries).],
) <tab:ocean>

== Sea Ice --- CICE / MPAS-Sea Ice

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [134],
    [Ice Thermodynamics (Vertical). Multi-layer heat conduction with brine pockets; top/bottom growth/melt.],
    [Cryosphere],
    [Physics-based],

    [135],
    [Ice Thickness Distribution (ITD). Sub-grid multi-category thickness tracking.],
    [Cryosphere],
    [Intermediate],

    [136],
    [Elastic-Viscous-Plastic Dynamics (EVP). EVP rheology: stress–strain-rate–strength.],
    [Cryosphere],
    [Physics-based],

    [137],
    [Ice Transport (Incremental Remapping). Geometrically-based shape-preserving advection.],
    [Cryosphere],
    [Physics-based],

    [138],
    [Mechanical Redistribution (Ridging/Rafting). Pressure ridge and keel formation under convergence.],
    [Cryosphere],
    [Intermediate],

    [139], [Ice–Atmosphere Fluxes. Turbulent heat/moisture/momentum exchange.], [Cryosphere], [Intermediate],
    [140], [Ice–Ocean Fluxes. Heat, salt, freshwater exchange at ice bottom.], [Cryosphere], [Physics-based],
    [141],
    [Shortwave Radiation in Ice. Delta-Eddington multiple-scattering through snow/ice layers.],
    [Radiation],
    [Physics-based],

    [142], [Melt Ponds. Formation, drainage, refreezing; albedo feedback.], [Cryosphere], [Intermediate],
    [143], [Sea Ice Age Tracer. Multi-year vs. first-year ice extent tracking.], [Cryosphere], [Empirical],
    [144],
    [First-Year Ice Fraction. FY vs. MY ice partition for optical/biological properties.],
    [Cryosphere],
    [Empirical],

    [145], [Level Ice Fraction. Undeformed vs. ridged ice tracking.], [Cryosphere], [Empirical],
    [146], [Ice Aerosol Deposition. BC/dust on/in sea ice; albedo response.], [Cryosphere], [Intermediate],
    [147],
    [VP/EVP Constitutive Relations (MPAS). Variational and weak-ice rheology formulations.],
    [Cryosphere],
    [Physics-based],

    [148], [Icepack Thermodynamics (MPAS). Columnar thermodynamics via Icepack library.], [Cryosphere], [Physics-based],
  ),
  caption: [Sea ice processes (15 entries).],
) <tab:seaice>

== Land Ice --- CISM / MALI

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [149], [Shallow Ice Approximation (SIA). Vertical-shear-dominated interior ice flow.], [Cryosphere], [Intermediate],
    [150],
    [Higher-Order Ice Dynamics (Blatter-Pattyn). First-order Stokes with membrane stresses.],
    [Cryosphere],
    [Physics-based],

    [151],
    [Full Stokes Dynamics (Albany). 3D Stokes via Trilinos/Albany for complex ice dynamics.],
    [Cryosphere],
    [Physics-based],

    [152],
    [Ice Sheet Thermodynamics. Advection-diffusion with strain heating and geothermal flux.],
    [Cryosphere],
    [Physics-based],

    [153],
    [Surface Mass Balance. Snowfall, melt (energy balance/PDD), rain, refreezing → net SMB.],
    [Cryosphere],
    [Intermediate],

    [154], [Basal Sliding. Weertman-type and Coulomb friction sliding laws.], [Cryosphere], [Intermediate],
    [155], [Calving. Flotation, thickness-based, eigen-calving, von Mises stress criteria.], [Cryosphere], [Empirical],
    [156], [Ice-Shelf Basal Melt. Ocean thermal forcing-driven sub-shelf melt rates.], [Cryosphere], [Intermediate],
    [157],
    [Subglacial Hydrology. Sheet/channel drainage controlling effective pressure and sliding.],
    [Hydrology],
    [Physics-based],

    [158],
    [Isostatic Bedrock Adjustment (GIA). Viscoelastic bedrock deformation under ice loads.],
    [Geology],
    [Intermediate],

    [159], [Sea Level Model. Gravitational, rotational, deformational sea level change.], [Ocean], [Physics-based],
  ),
  caption: [Land ice processes from CISM / MALI (11 entries).],
) <tab:landice>

== River Routing & Waves

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [160], [RTM. Linear transport routing on global river network.], [Hydrology], [Empirical],
    [161],
    [MOSART Kinematic/Diffusion Wave Routing. Hillslope → sub-network → main channel cascade.],
    [Hydrology],
    [Intermediate],

    [162],
    [MOSART Reservoir Operations (WRM). Dam storage, regulated releases, water withdrawals.],
    [Human Sys],
    [Intermediate],

    [163],
    [MOSART Inundation. Floodplain wetting when discharge exceeds channel capacity.],
    [Hydrology],
    [Intermediate],

    [164],
    [MOSART Sediment Transport. Suspended/bed-load sediment routing through river network.],
    [Geomorphology],
    [Intermediate],

    [165], [MOSART River Heat. Thermal energy transport and river temperature.], [Hydrology], [Intermediate],
    [166], [MOSART Biogeochemistry. Dissolved C, N, and constituent transport.], [Biogeochem], [Intermediate],
    [167], [mizuRoute. IRF/KWT network-based streamflow routing on vector networks.], [Hydrology], [Intermediate],
    [168],
    [WW3 Wave Spectral Evolution. Wave action balance equation in frequency-direction space.],
    [Ocean],
    [Physics-based],

    [169], [WW3 Wind Input. Miles/Janssen wind-wave generation.], [Ocean], [Physics-based],
    [170],
    [WW3 Nonlinear Wave-Wave Interactions (DIA). Four-wave resonant energy redistribution.],
    [Ocean],
    [Physics-based],

    [171], [WW3 Wave Dissipation (Whitecapping). Steepness/spectral-saturation dissipation.], [Ocean], [Intermediate],
    [172], [WW3 Wave-Bottom Interaction. Bottom friction, refraction, shoaling.], [Ocean], [Intermediate],
    [173],
    [WW3 Wave-Ice Interaction. Wave attenuation in ice; ice breakup from flexural stress.],
    [Cryosphere],
    [Intermediate],

    [174], [WW3 Wave-Current Interaction. Radiation stress, Stokes drift coupling.], [Ocean], [Physics-based],
  ),
  caption: [River routing and surface wave processes (15 entries).],
) <tab:rivers>

== Coupler & Data Models

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [175], [CMEPS Air-Sea Flux. COARE-based turbulent flux computation.], [Atmosphere], [Intermediate],
    [176],
    [CMEPS Field Merging. Area-weighted surface field combination from ocean/ice/land.],
    [Atmosphere],
    [Physics-based],

    [177], [CMEPS Conservative Regridding. ESMF-based mapping between component grids.], [Atmosphere], [Physics-based],
    [178],
    [CMEPS Runoff Mapping. Freshwater routing from river mouths to coastal ocean cells.],
    [Hydrology],
    [Intermediate],

    [179], [Slab Ocean Model (DOCN%SOM). Mixed-layer SST evolution from fluxes + prescribed OHT.], [Ocean], [Empirical],
    [180], [Data Atmosphere (DATM). Prescribed reanalysis/observed forcing.], [Atmosphere], [Empirical],
    [181], [Data Sea Ice (DICE). Prescribed ice concentration with semi-prognostic fluxes.], [Cryosphere], [Empirical],
    [182], [Solar Variability. Spectral solar irradiance variations for radiation.], [Radiation], [Empirical],
  ),
  caption: [Coupler and data model processes (8 entries).],
) <tab:coupler>

#pagebreak()

= E3SM --- Unique Processes Beyond CESM Heritage <app:e3sm>

E3SM shares substantial heritage with CESM (CAM→EAM, CLM→ELM, CICE→MPAS-Sea Ice). The following lists processes unique to or substantially modified in E3SM.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Theta-L Non-Hydrostatic Dynamical Core. Vertical Lagrangian $theta$-$l$ formulation in HOMME.],
    [Atmosphere],
    [Physics-based],

    [2],
    [EAMxx/SCREAM C++ Physics Port. GPU-accelerated SHOC + P3 + RRTMGP + MAM4 + ZM on Kokkos.],
    [Atmosphere],
    [Physics-based],

    [3],
    [Neural-Network Cloud Fraction (EAMxx). ML-based cloud cover alternative to diagnostic scheme.],
    [Atmosphere],
    [Intermediate],

    [4], [Cloud-J Photolysis (UCI). Updated photolysis with cloud-J spectral binning.], [Atmosphere], [Physics-based],
    [5],
    [CRM SGS-TKE Turbulence (MMF). Subgrid TKE diffusion within embedded cloud-resolving model.],
    [Atmosphere],
    [Physics-based],

    [6],
    [BeTR (Biogeochemical Transport). Vertically resolved multi-phase reactive transport in soil.],
    [Biogeochem],
    [Physics-based],

    [7],
    [EMI (External Model Interface). Generic coupling to external BGC models (PFLOTRAN).],
    [Biogeochem],
    [Intermediate],

    [8],
    [FATES (Cohort Vegetation Demography). Size-structured competition, disturbance, recruitment.],
    [Ecology],
    [Physics-based],

    [9],
    [Plant-Microbe ECA Kinetics. Enzyme-mediated nutrient competition with Michaelis-Menten.],
    [Biogeochem],
    [Physics-based],

    [10],
    [MPAS-Ocean Unstructured Mesh Dynamics. Voronoi finite-volume split-explicit PE solver.],
    [Ocean],
    [Physics-based],

    [11], [MPAS-Sea Ice on Unstructured Mesh. VP/EVP + Icepack on SCVT meshes.], [Cryosphere], [Physics-based],
    [12],
    [MALI Ice Sheet (Albany). Unstructured-mesh Blatter-Pattyn + FCT + thermal solver.],
    [Cryosphere],
    [Physics-based],

    [13],
    [GCAM (Human Earth System). Integrated assessment model coupling for energy/land/water scenarios.],
    [Human Sys],
    [Empirical],

    [14], [MOAB Coupler. Online conservative remapping via MOAB mesh library.], [Atmosphere], [Physics-based],
    [15], [Ocean Sediment Transport / Flux Index. Coastal sediment transport analysis.], [Geomorphology], [Empirical],
  ),
  caption: [E3SM-unique processes (15 entries).],
) <tab:e3sm>

#pagebreak()

= FATES --- Functionally Assembled Terrestrial Ecosystem Simulator <app:fates>

FATES is a cohort-based vegetation demography model that replaces the static PFT framework in CLM/ELM with size-structured cohorts competing for light, water, and nutrients. It is hosted by E3SM, CESM, and CTSM, funded by DOE NGEE-Tropics. The detailed catalog below draws from 29 source files across 6 source directories (biogeochem, biogeophys, fire, main, parteh, radiation).

== Radiation

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Norman Multi-Layer Canopy Radiation. Radiative transfer through vertically-resolved canopy (Norman 1979); absorbed PAR/NIR per layer for sunlit/shaded leaves.],
    [Radiation],
    [Physics-based],

    [2],
    [Two-Stream Multi-Layer Plant Element (MLPE) Solver. Upward/downward diffuse + direct beam RT through multiple canopy elements with scattering/absorption.],
    [Radiation],
    [Physics-based],

    [3],
    [Canopy Radiation Driver. Top-level driver selecting Norman or Two-Stream; normalized canopy radiation profiles and sun/shade leaf area fractions.],
    [Radiation],
    [Physics-based],

    [4],
    [FATES Two-Stream Utilities. FATES-specific wrapper for Two-Stream: constructs radiation elements from cohort structure, maps absorbed radiation back to cohorts.],
    [Radiation],
    [Physics-based],
  ),
  caption: [FATES radiation processes (4 entries).],
)

== Photosynthesis & Leaf Biophysics

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [5],
    [Rubisco-Limited Photosynthesis (C3). Farquhar-von Caemmerer-Berry Rubisco carboxylation-limited assimilation; Michaelis-Menten kinetics with temperature-dependent Vcmax.],
    [Biogeochem],
    [Physics-based],

    [6],
    [RuBP-Limited Photosynthesis (C3). FvCB electron transport (light)-limited assimilation; uses J from PAR.],
    [Biogeochem],
    [Physics-based],

    [7],
    [RuBP-Limited Photosynthesis (C4). Light-limited assimilation for C4; quantum yield formulation.],
    [Biogeochem],
    [Physics-based],

    [8],
    [PEP Carboxylase-Limited Photosynthesis (C4). PEP carboxylase enzyme-limited assimilation for C4.],
    [Biogeochem],
    [Physics-based],

    [9],
    [Electron Transport Rate (J). Computes electron transport from PAR; FvCB hyperbolic and Johnson-Berry formulations.],
    [Biogeochem],
    [Physics-based],

    [10],
    [Stomatal Conductance --- Medlyn Model. Optimal stomatal conductance (Medlyn et al. 2011) based on marginal water use efficiency; relates $g_s$ to assimilation, VPD, CO#sub[2].],
    [Biogeochem],
    [Physics-based],

    [11],
    [Stomatal Conductance --- Ball-Berry Model. Empirical model (Ball, Woodrow & Berry 1987); $g_s$ from assimilation, relative humidity, leaf-surface CO#sub[2].],
    [Biogeochem],
    [Empirical],

    [12],
    [Intercellular CO#sub[2] (Ci) Iteration. Bisection solver coupling photosynthetic demand with stomatal supply.],
    [Biogeochem],
    [Physics-based],

    [13],
    [Leaf Layer Photosynthesis. Integrates leaf biochemistry, stomatal conductance, and boundary layer conductance per canopy layer; net assimilation for sunlit/shaded fractions.],
    [Biogeochem],
    [Physics-based],
  ),
  caption: [FATES photosynthesis and leaf biophysics (9 entries).],
)

== Plant Respiration

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [14],
    [Leaf (Maintenance) Respiration. Dark respiration scaled from Vcmax25 with Atkin temperature acclimation.],
    [Biogeochem],
    [Intermediate],

    [15],
    [Fine Root Respiration. Temperature-dependent maintenance respiration distributed across soil layers; based on root nitrogen.],
    [Biogeochem],
    [Intermediate],

    [16],
    [Live Wood (Sapwood + Storage) Respiration. Temperature-dependent maintenance respiration of sapwood and storage tissues.],
    [Biogeochem],
    [Intermediate],

    [17],
    [Symbiotic N Fixation. Root-layer nitrogen fixation scaled by root biomass and soil conditions.],
    [Biogeochem],
    [Empirical],
  ),
  caption: [FATES respiration and N fixation (4 entries).],
)

== Plant Hydraulics

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [18],
    [Plant Hydraulics Driver (TFS-Hydro). Resistance-capacitance model: soil → root → stem → leaf water potential; Richards equation-based flow; tracks cavitation.],
    [Hydrology],
    [Physics-based],

    [19],
    [Van Genuchten Water Retention & Conductivity. $theta$ vs $psi$ retention curve and unsaturated hydraulic conductivity (van Genuchten 1980).],
    [Hydrology],
    [Physics-based],

    [20],
    [Clapp-Hornberger-Campbell Water Retention & Conductivity. Alternative power-law formulation (Clapp & Hornberger 1978).],
    [Hydrology],
    [Physics-based],

    [21],
    [Smoothed CCH Water Transfer Functions. Smoothed variant with continuous derivatives for numerical stability.],
    [Hydrology],
    [Physics-based],

    [22],
    [TFS Pedotransfer Functions. Soil texture → van Genuchten hydraulic parameters via Tomasella-Hodnett.],
    [Hydrology],
    [Empirical],

    [23],
    [BTRAN Transpiration Wetness Factor. Soil moisture limitation on transpiration; weighted plant-available water across root-occupied layers.],
    [Hydrology],
    [Intermediate],

    [24],
    [Root Water Uptake Distribution. Distributes transpiration demand across soil layers proportional to root fraction and water availability.],
    [Hydrology],
    [Intermediate],

    [25],
    [Salinity Stress. Reduces BTRAN based on soil salinity; osmotic potential effect on water uptake.],
    [Hydrology],
    [Empirical],
  ),
  caption: [FATES plant hydraulics (8 entries).],
)

== Fire (SPITFIRE)

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [26],
    [Fire Weather Index (Nesterov Index). Cumulative fire weather index; resets with precipitation.],
    [Fire],
    [Empirical],

    [27],
    [Fire Danger Index (FDI). Ignition probability from fire weather; exponential Nesterov transformation (Venevsky et al. 2002).],
    [Fire],
    [Empirical],

    [28],
    [Ignition Sources. Lightning + anthropogenic ignition; anthropogenic via Li et al. (2012) population scaling.],
    [Fire],
    [Empirical],

    [29], [Effective Wind Speed. Wind modified by vegetation cover (tree/grass fractions).], [Fire], [Intermediate],
    [30],
    [Fuel Loading. Aggregates dead leaf litter, 4 CWD size classes, live grass into fuel classes.],
    [Fire],
    [Intermediate],

    [31],
    [Fuel Moisture. Moisture content from Nesterov index and drying ratio; class-specific extinction.],
    [Fire],
    [Empirical],

    [32],
    [Fuel Bulk Density & SAV. Average bulk density and surface-area-to-volume ratio; drives packing ratio.],
    [Fire],
    [Intermediate],

    [33],
    [Fuel Class Definitions. 6 classes: twigs (1h), small branches (10h), large branches (100h), trunks (1000h), dead leaves, live grass.],
    [Fire],
    [Empirical],

    [34],
    [Rothermel Reaction Intensity. Net energy release rate per unit fuel bed area (Rothermel 1972).],
    [Fire],
    [Physics-based],

    [35],
    [Optimum Packing Ratio. Packing ratio maximizing reaction intensity; function of SAV.],
    [Fire],
    [Physics-based],

    [36],
    [Forward Rate of Spread. Rothermel (1972) quasi-steady surface fire spread with wind and slope.],
    [Fire],
    [Physics-based],

    [37],
    [Backward Rate of Spread. Backing fire spread; empirical function of forward ROS and wind.],
    [Fire],
    [Empirical],

    [38],
    [Fire Duration & Size. Elliptical fire shape; length-to-breadth ratio; total area from forward/backward ROS.],
    [Fire],
    [Intermediate],

    [39], [Fireline Intensity. Byram's fireline intensity (kW/m) from fuel consumed and ROS.], [Fire], [Physics-based],
    [40], [Scorch Height. Flame scorch height from fireline intensity (Van Wagner).], [Fire], [Empirical],
    [41], [Bark Thickness. Allometric bark thickness for fire resistance; PFT-specific.], [Fire], [Empirical],
    [42],
    [Cambial Mortality. Probability of cambial kill from fire residence time vs. critical bark thickness.],
    [Fire],
    [Intermediate],

    [43],
    [Crown Fire Mortality. Fraction of crown volume scorched and resulting crown fire mortality.],
    [Fire],
    [Intermediate],

    [44], [Total Fire Mortality. Combined cambial + crown scorch mortality per cohort.], [Fire], [Intermediate],
    [45],
    [Fuel Consumption & Residence Time. Fraction of each fuel class consumed; fire residence time for cambial damage.],
    [Fire],
    [Intermediate],

    [46],
    [Prescribed/Managed Fire Decision. Decision logic for prescribed burns based on weather window and fuel load thresholds.],
    [Fire],
    [Empirical],

    [47],
    [Prescribed Fire Burn Window. Evaluates meteorological permissibility for prescribed burn.],
    [Fire],
    [Empirical],
  ),
  caption: [FATES/SPITFIRE fire processes (22 entries).],
)

== Mortality

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [48],
    [Background Mortality. Constant PFT-specific rate representing unresolved stochastic mortality.],
    [Ecology],
    [Empirical],

    [49],
    [Carbon Starvation Mortality. Mortality from negative C balance when storage falls below threshold; linear ramp and exponential decay forms.],
    [Ecology],
    [Intermediate],

    [50],
    [Hydraulic Failure Mortality. Triggered when BTRAN or fractional conductivity loss exceeds critical threshold.],
    [Ecology],
    [Intermediate],

    [51],
    [Freezing Stress Mortality. Cold-induced mortality from accumulated freezing degree-days below PFT threshold.],
    [Ecology],
    [Empirical],

    [52],
    [Size-Dependent Senescence Mortality. Logistic increase with stem diameter; ontogenetic senescence.],
    [Ecology],
    [Empirical],

    [53],
    [Age-Dependent Senescence Mortality. Logistic increase with age beyond PFT-specific threshold.],
    [Ecology],
    [Empirical],

    [54],
    [Damage-Dependent Mortality. Additional mortality for cohorts with crown damage (wind); logistic function of damage class.],
    [Ecology],
    [Empirical],

    [55],
    [Fire Mortality. Combined cambial + crown fire kill applied to cohorts post-fire event.],
    [Fire],
    [Intermediate],

    [56],
    [Logging --- Direct Harvest. Fraction of commercial-sized trees felled; DBH-bounded.],
    [Human Sys],
    [Empirical],

    [57], [Logging --- Collateral Damage. Understory mortality from falling harvested trees.], [Human Sys], [Empirical],
    [58],
    [Logging --- Mechanical/Infrastructure. Mortality from skid trails and logging roads.],
    [Human Sys],
    [Empirical],
  ),
  caption: [FATES mortality processes (11 entries).],
)

== Phenology, Allometry, & Growth

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [59],
    [GDD Accumulation. Growing degree-days above base temperature; drives cold-deciduous onset.],
    [Ecology],
    [Intermediate],

    [60],
    [Cold Deciduous Phenology. Leaf-on by GDD threshold; leaf-off by chilling day or daylength.],
    [Ecology],
    [Intermediate],

    [61], [Drought Deciduous Phenology. Soil moisture-driven leaf flush/senescence.], [Ecology], [Intermediate],
    [62],
    [Satellite/Prescribed Phenology. Externally prescribed LAI; bypasses prognostic phenology.],
    [Ecology],
    [Empirical],

    [63],
    [Phenology Leaf Flush. Allocates stored C/N/P to new leaves during onset; respects allometric targets.],
    [Ecology],
    [Intermediate],

    [64],
    [Deciduous Leaf Turnover. Retranslocates nutrients before abscission; transfers to litter.],
    [Ecology],
    [Intermediate],

    [65], [Height--Diameter Allometry. Multiple forms: power, O'Brien, Martínez-Cano, Chave.], [Ecology], [Empirical],
    [66],
    [Above-Ground Biomass Allometry. Total AGB from DBH; Saldarriaga, 2-parameter power, Chave.],
    [Ecology],
    [Empirical],

    [67],
    [Maximum Leaf Biomass Allometry. Max leaf biomass from DBH via crown area and SLA with trimming.],
    [Ecology],
    [Empirical],

    [68],
    [Crown Area Allometry. Crown area from DBH; power-law with canopy-position spread factor.],
    [Ecology],
    [Empirical],

    [69],
    [Sapwood Biomass Allometry. Sapwood cross-section and biomass from DBH and height via pipe model theory.],
    [Ecology],
    [Intermediate],

    [70],
    [Fine Root Biomass Allometry. Target fine root biomass from leaf area and nutrient acquisition needs.],
    [Ecology],
    [Intermediate],

    [71], [Storage Biomass Allometry. Target labile carbon pool as fraction of leaf biomass.], [Ecology], [Empirical],
    [72], [Below-Ground Woody Biomass. Coarse root biomass from AGB using fixed ratio.], [Ecology], [Empirical],
    [73],
    [Dead (Structural) Wood Biomass. Heartwood as residual after subtracting sapwood from total.],
    [Ecology],
    [Intermediate],

    [74],
    [LAI & SAI from Crown Geometry. Leaf/stem area index per tree from crown area, depth, biomass.],
    [Ecology],
    [Intermediate],

    [75],
    [C-Only Allometric Growth (PARTEH). Daily net C allocation to organs maintaining allometric targets.],
    [Biogeochem],
    [Intermediate],

    [76],
    [Carbon Pool Growth Derivatives. Instantaneous growth rates per C pool from NPP and allometric constraints.],
    [Biogeochem],
    [Intermediate],

    [77],
    [CNP Allometric Growth (PARTEH). C-N-P coupled allocation with nutrient limitation; fine root PID controller.],
    [Biogeochem],
    [Physics-based],

    [78],
    [Fine Root C:N:P Target Adjustment. PID-controller-based adjustment of fine root targets for nutrient acquisition.],
    [Biogeochem],
    [Intermediate],

    [79],
    [Prioritized Pool Replacement. Allocates C/N/P to deficit pools in priority order (leaf > fine root > sapwood > storage > structural).],
    [Biogeochem],
    [Intermediate],

    [80],
    [Maintenance Turnover. Continuous background turnover of leaves, fine roots; transfers to litter with retranslocation.],
    [Biogeochem],
    [Intermediate],

    [81],
    [Reproductive Allocation. C (and nutrients) to reproductive pool; seeds released at species-specific rate.],
    [Biogeochem],
    [Intermediate],

    [82],
    [Fire Biomass Losses. Removes fire-damaged organs; transfers to atmosphere (combustion) or litter.],
    [Biogeochem],
    [Intermediate],

    [83],
    [Crown Damage Biomass Losses. Removes organs by crown damage class; transfers to litter.],
    [Biogeochem],
    [Intermediate],

    [84],
    [Herbivory/Grazing Losses. Removes leaf biomass by PFT palatability and land use label.],
    [Biogeochem],
    [Empirical],
  ),
  caption: [FATES phenology, allometry, and carbon allocation (26 entries).],
)

== Recruitment, Cohort & Patch Dynamics

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [85], [Seed Production. C flux from reproductive allocation to site-level seed bank.], [Ecology], [Intermediate],
    [86], [Seed Germination. Fraction of seed bank germinating per time step; PFT-specific.], [Ecology], [Empirical],
    [87], [Seed Decay. Background mortality/decomposition of seeds.], [Ecology], [Empirical],
    [88],
    [Seed Dispersal Kernels. Spatial dispersal: exponential, exponential-power, log-sech PDFs.],
    [Ecology],
    [Intermediate],

    [89],
    [Recruitment (New Cohort Creation). Creates cohorts from germinated seeds; allometrically-consistent initialization.],
    [Ecology],
    [Intermediate],

    [90],
    [Cohort Fusion. Merges similar cohorts (PFT, size, damage class); weighted averaging of state variables.],
    [Ecology],
    [Intermediate],

    [91],
    [Cohort Termination. Removes cohorts below minimum density or size thresholds; transfers biomass to litter.],
    [Ecology],
    [Intermediate],

    [92],
    [Cohort Damage Recovery. Transitions damaged cohorts to lower damage classes over time.],
    [Ecology],
    [Empirical],

    [93],
    [Disturbance Rate Calculation. Patch-level disturbance rates from fire, treefall, logging, LUC.],
    [Ecology],
    [Intermediate],

    [94],
    [Patch Spawning (Disturbance). Creates new patches from disturbance events; partitions cohorts.],
    [Ecology],
    [Intermediate],

    [95],
    [Patch Fusion. Merges similar-aged patches; weighted averaging of all state variables.],
    [Ecology],
    [Intermediate],

    [96],
    [Patch Litter Redistribution. Distributes litter between newly created and existing patches.],
    [Ecology],
    [Intermediate],

    [97],
    [Canopy Layer Structure. Assigns cohorts to overstory/understory by height ranking and crown area.],
    [Ecology],
    [Intermediate],

    [98],
    [Canopy Promotion/Demotion. Moves cohorts between canopy layers when layer areas change.],
    [Ecology],
    [Intermediate],

    [99],
    [Canopy Spread Factor. Diameter-dependent crown area spread differentiating open-grown vs. closed-canopy.],
    [Ecology],
    [Empirical],

    [100],
    [Crown Damage Scheduling. Event-based crown damage application; assigns damage classes via lookup.],
    [Ecology],
    [Empirical],
  ),
  caption: [FATES recruitment, cohort, and patch dynamics (16 entries).],
)

== Litter, Soil Interface, & Human Systems

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [101],
    [Litter Pool Dynamics. Leaf litter (by PFT decomposition class), 4 CWD size classes, seed bank per patch; C/N/P tracking.],
    [Biogeochem],
    [Intermediate],

    [102],
    [CWD Size-Class Adjustment. Adjusts CWD fractions based on contributing tree DBH distribution (SPITFIRE-linked).],
    [Biogeochem],
    [Empirical],

    [103],
    [Fragmentation/Decomposition Scalar. CENTURY-model temperature + moisture scalar for litter fragmentation.],
    [Biogeochem],
    [Intermediate],

    [104],
    [Litter-to-Soil C & Nutrient Flux. Fragmented litter C/N/P to host soil BGC with exponential depth partitioning.],
    [Biogeochem],
    [Intermediate],

    [105],
    [Plant Nutrient Acquisition (N, P). Michaelis-Menten or ECA (Equilibrium Chemistry Approximation) uptake kinetics.],
    [Biogeochem],
    [Intermediate],

    [106],
    [Exudation / Root Efflux. Excess C exuded from roots into soil; labile C input to decomposition.],
    [Biogeochem],
    [Empirical],

    [107],
    [CH#sub[4] Boundary Conditions. Methane module boundary conditions: root fraction, photosynthetic flux, soil decomposition inputs.],
    [Biogeochem],
    [Empirical],

    [108],
    [Selective Logging / Harvest (Area-Based). Removes trees within DBH bounds at specified harvest rate per unit area.],
    [Human Sys],
    [Empirical],

    [109],
    [Harvest by Carbon Target. Alternative harvest mode targeting specified C removal.],
    [Human Sys],
    [Empirical],

    [110], [Harvest Debt Tracking. Tracks unfulfilled harvest demand; debt carried forward.], [Human Sys], [Empirical],
    [111],
    [Logging Litter Fluxes. Partitions logged tree biomass into commercial products (exported) and slash/litter (on-site CWD).],
    [Human Sys],
    [Empirical],

    [112],
    [Land Use Transition Matrices. LUH2 transition rates between primary/secondary forest, cropland, pasture, rangeland.],
    [Human Sys],
    [Empirical],

    [113],
    [Grazing / Browsing. Leaf biomass removal by land use label, PFT palatability, and plant height.],
    [Human Sys],
    [Empirical],

    [114],
    [Prescribed Fire Management. Decision logic and execution of managed burns within weather windows.],
    [Human Sys],
    [Empirical],
  ),
  caption: [FATES litter/soil interface and human systems (14 entries).],
)

== Integration & Numerical Methods

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [115],
    [Ecosystem Dynamics Driver. Top-level daily driver: phenology → allocation → mortality → disturbance → recruitment → structure.],
    [Ecology],
    [---],

    [116],
    [State Variable Integration. Integrates cohort-level growth rates (DBH, biomass pools) through daily time step.],
    [Ecology],
    [---],

    [117],
    [Mass/Energy Balance Checking. Comprehensive balance checks for C, N, P, and water; reports imbalances.],
    [Ecology],
    [---],

    [118],
    [Flux Accumulation. Accumulates sub-daily fluxes (photosynthesis, respiration, transpiration) to daily totals.],
    [Ecology],
    [---],

    [119],
    [Runge-Kutta-Fehlberg (RKF45) Integrator. Adaptive-step 4th/5th order ODE integrator with error control.],
    [Ecology],
    [---],

    [120], [Euler Integrator. Simple forward Euler as fallback for stiff/simple systems.], [Ecology], [---],
  ),
  caption: [FATES integration and numerical methods (6 entries).],
)

The expanded FATES catalog totals *90 process entries* across 7 process families (Radiation 4, Biogeochem 30, Hydrology 8, Fire 23, Ecology 35, Human Systems 14, plus 6 integration/numerical entries), drawn from 29 source files --- more than doubling the original 42-entry catalog.

#pagebreak()

= iLand --- Individual-Based Forest Landscape Model <app:iland>

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Individual Tree Growth (3-PG variant). Light-use efficiency GPP with VPD/temp/soil water modifiers per tree.],
    [Ecology],
    [Intermediate],

    [2],
    [Light Competition. Beer-Lambert light interception on 2m height cells across the landscape.],
    [Radiation],
    [Intermediate],

    [3],
    [Establishment / Regeneration. Seed dispersal kernels, seedling light/moisture filtering, browsing.],
    [Ecology],
    [Intermediate],

    [4], [Mortality (Carbon Starvation). Death when C reserves deplete below threshold.], [Ecology], [Intermediate],
    [5], [Mortality (Stress/Intrinsic). Age-dependent and stress-based background mortality.], [Ecology], [Empirical],
    [6], [Phenology. Day-length and temperature-controlled leaf onset/offset.], [Ecology], [Empirical],
    [7],
    [Decomposition / Soil C-N (ICBM/2N). Cohort-based SOM decomposition with N cycling.],
    [Biogeochem],
    [Intermediate],

    [8], [Snag Dynamics. Standing dead tree decay, fall probability, CWD generation.], [Biogeochem], [Intermediate],
    [9],
    [Sapling Growth Module. Height-based sapling cohorts before individual tree transition.],
    [Ecology],
    [Intermediate],

    [10],
    [Water Cycle. Canopy interception, snowmelt, evapotranspiration, percolation, groundwater recharge.],
    [Hydrology],
    [Intermediate],

    [11],
    [Fire Disturbance Module. Stochastic fire ignition, spread probability, severity per species.],
    [Fire],
    [Empirical],

    [12],
    [Wind Disturbance. Wind damage probability based on tree height, species, and exposure.],
    [Ecology],
    [Empirical],

    [13],
    [Bark Beetle Disturbance. Beetle population dynamics, host selection, outbreak triggers.],
    [Ecology],
    [Intermediate],

    [14],
    [Forest Management. Thinning, clear-cut, shelter-wood prescriptions with scheduling.],
    [Human Sys],
    [Intermediate],

    [15], [Seed Dispersal. Species-specific dispersal kernels (exponential, fat-tailed).], [Ecology], [Intermediate],
    [16],
    [Browsing / Ungulate Herbivory. Height-dependent browse damage on seedlings/saplings.],
    [Ecology],
    [Empirical],

    [17], [Soil Water Bucket Model. Multi-layer simplified soil water balance.], [Hydrology], [Empirical],
    [18],
    [Permafrost / Soil Temperature. Simplified soil thermal regime for boreal forests.],
    [Cryosphere],
    [Empirical],

    [19], [Carbon Allocation. NPP partitioning to foliage, wood, fine roots, reserves.], [Biogeochem], [Intermediate],
    [20], [Species-Specific Allometry. Height-diameter, crown dimension, and biomass scaling.], [Ecology], [Empirical],
    [21], [Landscape Connectivity. Seed rain from neighboring cells; gene flow proxy.], [Ecology], [Intermediate],
  ),
  caption: [iLand process catalog (21 entries). Primary families: Ecology (14), Biogeochem (4), Hydrology (2), Fire (1).],
) <tab:iland>

#pagebreak()

= Landlab --- Earth Surface Dynamics Toolkit <app:landlab>

Landlab provides a modular Python toolkit with 55+ components for geomorphology, hydrology, and landscape evolution research.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [FlowAccumulator. Multi-algorithm flow routing and drainage area accumulation (D8, D4, Dinf).],
    [Hydrology],
    [Empirical],

    [2], [FlowDirectorSteepest / D8. Single-direction steepest-descent flow routing.], [Hydrology], [Empirical],
    [3], [FlowDirectorDinf. Divergent flow routing (Tarboton's D-infinity).], [Hydrology], [Intermediate],
    [4], [FlowDirectorMFD. Multiple-flow-direction proportional routing.], [Hydrology], [Intermediate],
    [5], [DepressionFinderAndRouter. Pit filling and lake-overflow routing.], [Hydrology], [Empirical],
    [6], [LakeMapperBarnes. Efficient lake delineation and fill (Barnes et al.).], [Hydrology], [Empirical],
    [7], [StreamPowerEroder. Detachment-limited stream power incision: $E = K A^m S^n$.], [Geomorphology], [Empirical],
    [8], [FastscapeEroder. Implicit O(n) stream power (Braun & Willett 2013).], [Geomorphology], [Empirical],
    [9], [SedDepEroder. Transport-limited fluvial erosion with deposition.], [Geomorphology], [Empirical],
    [10],
    [ErosionDeposition. Simultaneous erosion and deposition (Davy & Lague 2009).],
    [Geomorphology],
    [Intermediate],

    [11],
    [Space (Stream Power with Alluvium Conservation). Bedrock + alluvium dual-layer incision.],
    [Geomorphology],
    [Intermediate],

    [12], [SpaceLargeScaleEroder. Landscape-scale SPACE variant.], [Geomorphology], [Intermediate],
    [13], [LinearDiffuser. $partial z\/partial t = D nabla^2 z$ hillslope creep.], [Geomorphology], [Empirical],
    [14], [TaylorNonLinearDiffuser. Critical-slope non-linear diffusion.], [Geomorphology], [Empirical],
    [15], [DepthDependentDiffuser. Soil-thickness-dependent creep transport.], [Geomorphology], [Intermediate],
    [16],
    [DepthDependentTaylorDiffuser. Combined depth-dependent + non-linear diffusion.],
    [Geomorphology],
    [Intermediate],

    [17], [TransportLengthHillslopeDiffuser. Detachment-transport hillslope model.], [Geomorphology], [Intermediate],
    [18],
    [LandslideComponent. Shallow landslide initiation via infinite slope stability.],
    [Geomorphology],
    [Intermediate],

    [19], [LandslideProbability. Probabilistic landslide hazard from soil/topo/water.], [Geomorphology], [Intermediate],
    [20], [BedrockLandslider. Deep-seated bedrock landslide model.], [Geomorphology], [Intermediate],
    [21],
    [OverlandFlow (de Almeida / Bates). 2D shallow water equations for surface flow.],
    [Hydrology],
    [Physics-based],

    [22], [KinwaveOverlandFlowModel. Kinematic wave overland flow routing.], [Hydrology], [Intermediate],
    [23], [GroundwaterDupuitPercolator. 2D Boussinesq unconfined aquifer flow.], [Hydrology], [Physics-based],
    [24], [SoilInfiltrationGreenAmpt. Green-Ampt infiltration model.], [Hydrology], [Physics-based],
    [25], [Radiation. Topographic solar radiation (slope, aspect, shadowing).], [Radiation], [Physics-based],
    [26], [PotentialEvapotranspiration. ET estimation from radiation and meteorology.], [Hydrology], [Intermediate],
    [27], [SoilMoisture. 1D soil water balance with leaky-bucket dynamics.], [Hydrology], [Empirical],
    [28],
    [Vegetation (cellular automaton). Stochastic vegetation dynamics on gridded landscapes.],
    [Ecology],
    [Empirical],

    [29], [VegCA. Cellular automaton for dryland vegetation pattern formation.], [Ecology], [Empirical],
    [30],
    [SpeciesEvolver. Tracking lineage diversification and phylogeography on evolving landscapes.],
    [Evolution],
    [Intermediate],

    [31], [Flexure. 2D lithospheric flexure under surface loads.], [Geology], [Physics-based],
    [32], [TectonicFiniteSketch. Tectonic deformation and faulting.], [Geology], [Intermediate],
    [33], [NormalFault. Normal fault displacement and scarp generation.], [Geology], [Intermediate],
    [34], [ListricKinematicExtender. Listric fault deformation.], [Geology], [Intermediate],
    [35], [SinkFiller. Iterative depression filling to create drainage-enforced DEMs.], [Hydrology], [Empirical],
    [36], [ChannelProfiler. Longitudinal channel profile extraction and analysis.], [Geomorphology], [Empirical],
    [37], [ChiFinder. Chi-elevation analysis for divide migration.], [Geomorphology], [Empirical],
    [38], [DrainageDensity. Drainage network density computation.], [Geomorphology], [Empirical],
    [39], [SteepnessFinder. Channel steepness index ($k_"sn"$) computation.], [Geomorphology], [Empirical],
    [40], [Weathering (Exponential). Soil production from bedrock as $f(H)$.], [Geomorphology], [Empirical],
    [41], [ExponentialWeathererIntegrated. Integrated soil production over time.], [Geomorphology], [Empirical],
    [42], [AreaSlopeTransporter. Area-slope sediment transport law.], [Geomorphology], [Empirical],
    [43], [GravelBedrockEroder. Gravel abrasion on bedrock channels.], [Geomorphology], [Intermediate],
    [44], [GravelRiverTransporter. Gravel transport capacity and downstream fining.], [Geomorphology], [Intermediate],
    [45],
    [NetworkSedimentTransporter. Parcel-based sediment routing on channel networks.],
    [Geomorphology],
    [Intermediate],

    [46], [PriorityFloodFlowRouter. Priority-flood-based $epsilon$-fill and route.], [Hydrology], [Empirical],
    [47], [HackCalculator. Hack's law stream length-area analysis.], [Geomorphology], [Empirical],
    [48], [Profiler / TrickleDownProfiler. Channel profile analysis tools.], [Geomorphology], [Empirical],
    [49],
    [ErosionModel (Terrainbento). Configurable multi-process landscape evolution driver.],
    [Geomorphology],
    [Intermediate],
  ),
  caption: [Landlab component catalog (49 entries). Primary families: Geomorphology (29), Hydrology (16), Geology (4), Ecology (2), Radiation (1), Evolution (1).],
) <tab:landlab>

#pagebreak()

= LPJ-GUESS --- Dynamic Global Vegetation Model <app:lpjguess>

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Photosynthesis (Haxeltine & Prentice / Collatz). Coupled photosynthesis–stomatal conductance.],
    [Biogeochem],
    [Physics-based],

    [2],
    [Autotrophic Respiration. Maintenance + growth respiration from N content and temperature.],
    [Biogeochem],
    [Intermediate],

    [3], [Vegetation Dynamics. Establishment, mortality, competition for light by cohorts.], [Ecology], [Intermediate],
    [4], [Bioclimatic Limits. PFT survival and establishment bioclimatic envelopes.], [Ecology], [Empirical],
    [5], [Phenology. Summergreen/raingreen/evergreen phenological strategies.], [Ecology], [Empirical],
    [6],
    [Carbon Allocation. NPP partitioning across leaves, sapwood, heartwood, fine roots.],
    [Biogeochem],
    [Intermediate],

    [7], [Fire (BLAZE). Process-based fire model with fuel moisture and fire weather.], [Fire], [Intermediate],
    [8], [Fire (SIMFIRE). Statistical global fire model based on productivity and climate.], [Fire], [Empirical],
    [9],
    [Soil Organic Matter (Century-based). Multi-pool decomposition with C-N coupling.],
    [Biogeochem],
    [Intermediate],

    [10],
    [Nitrogen Cycling. N mineralization, plant N uptake, N limitation of growth, symbiotic fixation.],
    [Biogeochem],
    [Intermediate],

    [11],
    [Methane Emissions (Wetlands). CH#sub[4] production and emission from wetland soils.],
    [Biogeochem],
    [Intermediate],

    [12], [Hydrology (2-Layer Bucket). Percolation, runoff, snow, evapotranspiration.], [Hydrology], [Empirical],
    [13],
    [Crop Module (CFT). Managed crop PFTs with sowing, harvest, fertilization for ~15 crop types.],
    [Human Sys],
    [Intermediate],

    [14], [Managed Grassland / Pasture. Mowing and grazing management.], [Human Sys], [Empirical],
    [15], [Peatland Module. Peat accumulation, decomposition, and methane dynamics.], [Biogeochem], [Intermediate],
    [16],
    [Land Use Change. Deforestation, afforestation, crop expansion/abandonment transitions.],
    [Human Sys],
    [Intermediate],

    [17], [BVOC Emissions. Isoprene and monoterpene emissions from vegetation.], [Biogeochem], [Empirical],
    [18],
    [Soil Thermal (Permafrost). Multi-layer soil temperature and freeze-thaw dynamics for permafrost regions.],
    [Cryosphere],
    [Intermediate],
  ),
  caption: [LPJ-GUESS process catalog (18 entries). Primary families: Biogeochem (8), Ecology (4), Human Sys (3), Fire (2), Hydrology (1).],
) <tab:lpjguess>

#pagebreak()

= Noah-MP --- Multi-Physics Land Surface Model <app:noahmp>

Noah-MP's defining feature is offering multiple scheme options for each physical process, enabling systematic evaluation of structural uncertainty.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Vegetation Canopy Radiative Transfer (2-stream × gap). Modified two-stream with canopy gap probability.],
    [Radiation],
    [Physics-based],

    [2], [Snow Albedo — CLASS. Canadian Land Surface Scheme snow albedo (aging-based decay).], [Radiation], [Empirical],
    [3],
    [Snow Albedo — BATS. Biosphere-Atmosphere Transfer Scheme grain-size-dependent albedo.],
    [Radiation],
    [Empirical],

    [4],
    [Surface Exchange (M-O × Noah × Chen97). Monin-Obukhov with multiple roughness formulations.],
    [Atmosphere],
    [Intermediate],

    [5], [Stomatal Conductance — Ball-Berry. Humidity-based empirical stomatal model.], [Biogeochem], [Empirical],
    [6],
    [Stomatal Conductance — Jarvis. Multiplicative environmental stress factors on $g_"smax"$.],
    [Biogeochem],
    [Empirical],

    [7],
    [Photosynthesis (Farquhar C3/C4). Farquhar biochemistry for carbon assimilation.],
    [Biogeochem],
    [Physics-based],

    [8],
    [Multi-Layer Snow Model. Up to 3 snow layers with compaction, melt, refreezing.],
    [Cryosphere],
    [Physics-based],

    [9], [Snow Liquid Water — Gravitational. Simple gravitational drainage.], [Cryosphere], [Empirical],
    [10],
    [Snow Liquid Water — SNTHERM. Snow thermal model with grain-size metamorphism.],
    [Cryosphere],
    [Physics-based],

    [11], [Soil Moisture — Richards Equation. Multi-layer unsaturated/saturated flow.], [Hydrology], [Physics-based],
    [12], [Runoff — TOPMODEL. Saturation-excess runoff with topographic index.], [Hydrology], [Intermediate],
    [13], [Runoff — SIMGM. Simple groundwater model with water table.], [Hydrology], [Intermediate],
    [14],
    [Runoff — XAJ (Xinanjiang). Saturation-excess based on variable infiltration capacity.],
    [Hydrology],
    [Empirical],

    [15],
    [Runoff — Dynamic VIC. Variable infiltration capacity with dynamic parameterization.],
    [Hydrology],
    [Intermediate],

    [16], [Runoff — Free Drainage. Gravity-only lower boundary condition.], [Hydrology], [Empirical],
    [17],
    [Surface Resistance — Sakaguchi/Zeng. Soil evaporation resistance parameterization.],
    [Hydrology],
    [Empirical],

    [18], [Surface Resistance — Sellers. Vegetation-dependent surface resistance.], [Hydrology], [Empirical],
    [19], [Infiltration — Philip. Philip's infiltration equation.], [Hydrology], [Physics-based],
    [20], [Infiltration — Green-Ampt. Wetting front-based infiltration.], [Hydrology], [Physics-based],
    [21], [Infiltration — Smith-Parlange. Smith-Parlange flux-based infiltration.], [Hydrology], [Physics-based],
    [22], [Tile Drainage. Agricultural subsurface drainage parameterization.], [Hydrology], [Intermediate],
    [23],
    [Canopy Interception. Precipitation interception, throughfall, drip from vegetation.],
    [Hydrology],
    [Intermediate],

    [24], [Soil Temperature. Multi-layer heat diffusion with phase change.], [Hydrology], [Physics-based],
    [25],
    [Frozen Soil Permeability. Ice-content-dependent hydraulic conductivity reduction.],
    [Cryosphere],
    [Intermediate],

    [26], [Glacier Model. Simple glacier ice accumulation and melt.], [Cryosphere], [Empirical],
    [27], [Carbon Model (Simple). Prognostic vegetation and soil carbon pools.], [Biogeochem], [Intermediate],
    [28], [Crop Module. Planting, phenology, harvest modules for agricultural PFTs.], [Human Sys], [Intermediate],
    [29], [Irrigation. Demand-based irrigation from soil moisture deficit.], [Human Sys], [Empirical],
    [30], [Urban Model. Simple single-layer urban canopy energy balance.], [Human Sys], [Empirical],
    [31],
    [Groundwater / Aquifer. Unconfined aquifer storage and exchange with soil column.],
    [Hydrology],
    [Intermediate],

    [32],
    [LAI Prescription / Phenology. Satellite-based or model-predicted LAI seasonal cycle.],
    [Ecology],
    [Empirical],
  ),
  caption: [Noah-MP process catalog (32 entries). Multi-physics options yield ~70 distinct parameterization combinations.],
) <tab:noahmp>

#pagebreak()

= ParFlow --- Integrated Hydrological Model <app:parflow>

ParFlow solves 3D variably-saturated groundwater flow (Richards equation) coupled to overland flow (kinematic/diffusion wave) on structured grids with terrain-following coordinates.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [3D Richards Equation Solver. Variably-saturated subsurface flow with Newton-Krylov nonlinear solvers.],
    [Hydrology],
    [Physics-based],

    [2],
    [Van Genuchten Retention. Soil water retention and relative permeability curves.],
    [Hydrology],
    [Physics-based],

    [3], [Haverkamp Retention. Alternative water retention model.], [Hydrology], [Physics-based],
    [4], [Brooks-Corey Retention. Power-law retention curves.], [Hydrology], [Empirical],
    [5], [Overland Flow (Kinematic Wave). Manning's equation surface runoff routing.], [Hydrology], [Intermediate],
    [6],
    [Overland Flow (Diffusive Wave). Diffusion-wave approximation for 2D surface flow.],
    [Hydrology],
    [Physics-based],

    [7],
    [Terrain-Following Grid (TFG). Topography-conformal mesh for hillslope/valley geometry.],
    [Hydrology],
    [Physics-based],

    [8], [CLM Coupling (Land Surface). Surface energy balance and ET via coupled CLM.], [Hydrology], [Physics-based],
    [9], [Wells (Injection/Extraction). Point source/sink well boundary conditions.], [Hydrology], [Physics-based],
    [10], [Reservoir / Lake Boundary. Surface water body storage boundary conditions.], [Hydrology], [Intermediate],
    [11],
    [Heterogeneous Permeability Fields. Geostatistical (turning bands) random field generation.],
    [Geology],
    [Intermediate],

    [12], [Indicator Geostatistics. Categorical geological facies simulation.], [Geology], [Intermediate],
    [13],
    [Constant / PFB Flux Boundaries. Specified flux and pressure boundary conditions.],
    [Hydrology],
    [Physics-based],

    [14],
    [Evapotranspiration (via CLM). Richards-CLM computed ET from root water uptake.],
    [Hydrology],
    [Physics-based],

    [15], [Snow Model (via CLM). Energy-balance multi-layer snow coupled to subsurface.], [Cryosphere], [Physics-based],
    [16], [Multi-GPU / CUDA Acceleration. GPU-accelerated Richards solver. ], [Hydrology], [Physics-based],
  ),
  caption: [ParFlow process catalog (16 entries). Primary family: Hydrology (14), Geology (2).],
) <tab:parflow>

#pagebreak()

= WRF/WRF-SFIRE --- Weather & Wildfire Modeling <app:wrf>

WRF-SFIRE couples the Weather Research and Forecasting model with a fire spread model, enabling two-way atmosphere–wildfire interaction.

== WRF Atmospheric Physics

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [ARW Dynamical Core. Fully compressible non-hydrostatic Euler equations on Arakawa C-grid, Runge-Kutta 3rd order.],
    [Atmosphere],
    [Physics-based],

    [2], [PBL — YSU. Yonsei University non-local K-profile boundary layer scheme.], [Atmosphere], [Intermediate],
    [3], [PBL — MYJ. Mellor-Yamada-Janjić 2.5-level local TKE closure.], [Atmosphere], [Intermediate],
    [4], [PBL — MYNN. Mellor-Yamada-Nakanishi-Niino 2.5/3.0 level closure.], [Atmosphere], [Intermediate],
    [5], [PBL — ACM2. Asymmetric Convective Model with non-local + local mixing.], [Atmosphere], [Intermediate],
    [6], [PBL — TEMF. Total Energy Mass Flux eddy-diffusivity mass-flux scheme.], [Atmosphere], [Intermediate],
    [7],
    [Microphysics — WSM3/5/6. WRF Single-Moment 3/5/6-class schemes (ice, snow, graupel).],
    [Atmosphere],
    [Intermediate],

    [8], [Microphysics — WDM5/6. WRF Double-Moment 5/6-class schemes.], [Atmosphere], [Physics-based],
    [9], [Microphysics — Thompson. Aerosol-aware mixed-phase bulk microphysics.], [Atmosphere], [Physics-based],
    [10],
    [Microphysics — Morrison 2-Moment. Full 2-moment for cloud, rain, ice, snow, graupel.],
    [Atmosphere],
    [Physics-based],

    [11], [Microphysics — Lin-Purdue. Lin et al. ice-phase microphysics.], [Atmosphere], [Intermediate],
    [12], [Microphysics — Kessler. Warm-rain autoconversion scheme.], [Atmosphere], [Empirical],
    [13], [Radiation — RRTM/RRTMG. Correlated-k longwave radiation for WRF.], [Radiation], [Physics-based],
    [14], [Radiation — Dudhia Shortwave. Simple downward integration shortwave scheme.], [Radiation], [Empirical],
    [15],
    [Radiation — Goddard Shortwave. Multi-band shortwave with aerosol/cloud interaction.],
    [Radiation],
    [Intermediate],

    [16], [Radiation — CAM. CAM radiation package within WRF.], [Radiation], [Physics-based],
    [17],
    [Cumulus — Kain-Fritsch. Mass-flux convective parameterization with CAPE removal closure.],
    [Atmosphere],
    [Intermediate],

    [18], [Cumulus — Betts-Miller-Janjić. Convective adjustment toward observed profiles.], [Atmosphere], [Empirical],
    [19],
    [Cumulus — Grell-Devenyi/Grell-Freitas. Ensemble/stochastic mass-flux convection.],
    [Atmosphere],
    [Intermediate],

    [20], [Cumulus — Tiedtke. Mass-flux with organized entrainment/detrainment.], [Atmosphere], [Intermediate],
    [21], [Land Surface — Noah. 4-layer soil T/moisture, urban, vegetation, snow.], [Hydrology], [Intermediate],
    [22], [Land Surface — Noah-MP. Multi-physics land surface (see @app:noahmp).], [Hydrology], [Intermediate],
    [23], [Land Surface — RUC. Rapid Update Cycle 6-layer soil/hydro/thermal model.], [Hydrology], [Intermediate],
    [24], [Land Surface — CLM. Community Land Model coupling within WRF.], [Hydrology], [Physics-based],
    [25], [Surface Layer — Revised MM5. Monin-Obukhov surface flux parameterization.], [Atmosphere], [Intermediate],
    [26], [Surface Layer — Eta. Janjić Eta model similarity theory.], [Atmosphere], [Intermediate],
  ),
  caption: [WRF atmospheric physics (26 entries).],
) <tab:wrf-atmo>

== Fire & Chemistry

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [27],
    [Level-Set Fire Spread. Narrow-band level set tracking of fire front with sub-grid refinement.],
    [Fire],
    [Physics-based],

    [28],
    [Rothermel Fire Spread Rate. Semi-empirical surface fire rate-of-spread from fuel, moisture, wind, slope.],
    [Fire],
    [Intermediate],

    [29],
    [Balbi Fire Spread Rate. Radiation-convection fire spread model (alternative to Rothermel).],
    [Fire],
    [Physics-based],

    [30], [13 Anderson Fuel Categories. NFDRS fuel model classification for fire behavior.], [Fire], [Empirical],
    [31], [Canopy Fire / Crown Fire. Vertical fire propagation from surface to crown.], [Fire], [Intermediate],
    [32],
    [Fire Emissions & Smoke Transport. Trace gas and particulate emissions from combustion; smoke plume advection.],
    [Fire],
    [Intermediate],

    [33],
    [Fire–Atmosphere Coupling (Two-Way). Sensible/latent heat flux from fire → atmosphere; wind → fire spread rate feedback.],
    [Fire],
    [Physics-based],

    [34],
    [WRF-Chem Gas-Phase Chemistry. RADM2, RACM, CB05 chemical mechanisms for tropospheric O#sub[3], VOC, NOx.],
    [Atmosphere],
    [Physics-based],

    [35],
    [WRF-Chem Aerosol (MADE/GOCART/MOSAIC). Modal/sectional aerosol microphysics/chemistry.],
    [Atmosphere],
    [Intermediate],

    [36], [WRF-Chem Photolysis (Fast-J/TUV). Online photolysis rate computation.], [Atmosphere], [Physics-based],
  ),
  caption: [WRF-SFIRE fire and chemistry processes (10 entries).],
) <tab:wrf-fire>

== WRF-Hydro / National Water Model

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [37],
    [Overland Flow Routing. Diffusive wave routing on high-resolution terrain grids.],
    [Hydrology],
    [Physics-based],

    [38],
    [Channel Routing (Muskingum-Cunge). Kinematic-diffusive channel routing for river networks.],
    [Hydrology],
    [Intermediate],

    [39], [Subsurface Lateral Flow. Saturated lateral flow down topographic gradients.], [Hydrology], [Intermediate],
    [40], [Reservoir Routing. Level-pool reservoir routing with operating rules.], [Human Sys], [Intermediate],
    [41], [Lake Routing. Conceptual lake storage/outflow model.], [Hydrology], [Empirical],
    [42], [Bucket Model Groundwater. Conceptual exponential-decay groundwater baseflow.], [Hydrology], [Empirical],
    [43],
    [UDMAP Disaggregation. Upscale Noah-MP/CLM land surface fields to high-resolution routing grid.],
    [Hydrology],
    [Intermediate],
  ),
  caption: [WRF-Hydro / NWM processes (7 entries).],
) <tab:wrfhydro>

#pagebreak()

= ORCHIDEE --- Land Surface Model <app:orchidee>

ORCHIDEE 2.0 couples surface energy balance (SECHIBA) with vegetation dynamics (STOMATE), derived from LPJ.

== SECHIBA (Surface Energy Balance)

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Surface Energy Balance. Coupled radiation/turbulent flux solution for canopy and soil.],
    [Atmosphere],
    [Physics-based],

    [2],
    [11-Layer Richards Soil Hydrology (CWRR). Van Genuchten parameterized Richards equation solver.],
    [Hydrology],
    [Physics-based],

    [3], [Choisnel 2-Layer Bucket Hydrology. Simple bucket model alternative.], [Hydrology], [Empirical],
    [4], [Multi-Layer Snow Model. Explicit snow with compaction and thermal conduction.], [Cryosphere], [Physics-based],
    [5], [Soil Thermodynamics. Multi-layer soil temperature with freeze/thaw.], [Hydrology], [Physics-based],
    [6],
    [River Routing with Floodplains. Global routing scheme with floodplain inundation.],
    [Hydrology],
    [Intermediate],

    [7], [Canopy Interception & Throughfall. Precipitation partitioning by vegetation.], [Hydrology], [Intermediate],
    [8],
    [Albedo / Roughness. Surface optical/aerodynamic properties from PFT and soil/snow.],
    [Radiation],
    [Intermediate],

    [9],
    [Penman-Monteith Evapotranspiration. Physical ET computation with stomatal control.],
    [Hydrology],
    [Physics-based],

    [10], [BVOC Chemistry. Biogenic volatile organic compound emissions.], [Biogeochem], [Empirical],
    [11],
    [Diffuse Radiation Partitioning. Splitting of direct/diffuse radiation for canopy absorption.],
    [Radiation],
    [Intermediate],
  ),
  caption: [ORCHIDEE SECHIBA processes (11 entries).],
) <tab:orchidee-sechiba>

== STOMATE (Vegetation Dynamics & Carbon)

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [12], [Photosynthesis (Farquhar C3 / Collatz C4). Leaf-level carbon assimilation.], [Biogeochem], [Physics-based],
    [13], [Stomatal Conductance (Ball-Berry). Humidity-based stomatal regulation.], [Biogeochem], [Empirical],
    [14], [Autotrophic Respiration. Maintenance + growth respiration.], [Biogeochem], [Intermediate],
    [15], [Carbon Allocation. NPP partitioning to plant compartments.], [Biogeochem], [Intermediate],
    [16], [Phenology — Leaf Onset/Senescence. Temperature, moisture, and light triggers.], [Ecology], [Empirical],
    [17], [Vegetation Establishment. PFT-based establishment from climate suitability.], [Ecology], [Intermediate],
    [18], [Vegetation Mortality. Self-thinning, heat, and drought-driven mortality.], [Ecology], [Intermediate],
    [19], [Light Competition. Fractional coverage competition among PFTs.], [Ecology], [Intermediate],
    [20], [Fire (LPJ-derived). Litter-moisture and climate-dependent fire occurrence/spread.], [Fire], [Empirical],
    [21], [Litter & Soil Carbon (Century-based). Multi-pool SOM decomposition.], [Biogeochem], [Intermediate],
    [22], [Nitrogen Cycling. N mineralization, immobilization, plant uptake.], [Biogeochem], [Intermediate],
    [23], [Land Cover Change. PFT area transitions from deforestation/afforestation.], [Human Sys], [Intermediate],
    [24], [Wood Harvest. Harvested wood product pools.], [Human Sys], [Empirical],
    [25], [Grassland Management. Mowing and grazing.], [Human Sys], [Empirical],
    [26], [Crop Module. Agricultural PFT phenology and harvest.], [Human Sys], [Intermediate],
  ),
  caption: [ORCHIDEE STOMATE processes (15 entries).],
) <tab:orchidee-stomate>

= VIC --- Variable Infiltration Capacity Model <app:vic>

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Variable Infiltration Capacity Runoff. Nonlinear soil moisture-storage-capacity distribution.],
    [Hydrology],
    [Intermediate],

    [2],
    [Multi-Layer Soil Temperature. Heat conduction with freeze/thaw through 3+ soil layers.],
    [Hydrology],
    [Physics-based],

    [3],
    [Frozen Soil Permafrost. Ice lens formation and thermal conductivity modification.],
    [Cryosphere],
    [Physics-based],

    [4],
    [Energy-Balance Snow Model (2-Layer). Mass and energy balance with liquid water retention.],
    [Cryosphere],
    [Physics-based],

    [5], [Blowing Snow Sublimation. Wind-transported snow sublimation loss.], [Cryosphere], [Intermediate],
    [6], [Penman-Monteith ET. Physically-based evapotranspiration.], [Hydrology], [Physics-based],
    [7], [Canopy Interception. Canopy water storage and throughfall partitioning.], [Hydrology], [Intermediate],
    [8], [Farquhar/Collatz Photosynthesis. Optional biochemical carbon assimilation.], [Biogeochem], [Physics-based],
    [9],
    [Soil Carbon Balance. Simple 3-pool soil carbon model with temperature/moisture controls.],
    [Biogeochem],
    [Intermediate],

    [10], [Lake Energy Balance with Ice. 1-D lake thermal model with ice formation.], [Hydrology], [Intermediate],
    [11], [Wetland Model. Saturated fraction and wetland water balance.], [Hydrology], [Empirical],
    [12], [Baseflow (ARNO). Non-linear baseflow recession from lower soil zone.], [Hydrology], [Empirical],
    [13], [Snow Albedo (Age-Dependent). Exponential decay of snow albedo with age.], [Radiation], [Empirical],
    [14], [Vegetation Library (12 Classes). Land cover properties for each grid cell tile.], [Ecology], [Empirical],
    [15],
    [Sub-Grid Elevation Bands. Snow/temperature processing by elevation band for mountainous terrain.],
    [Cryosphere],
    [Intermediate],

    [16],
    [Canopy Energy Balance. Full canopy energy budget with longwave/shortwave/turbulent fluxes.],
    [Atmosphere],
    [Physics-based],

    [17],
    [MTCLIM Meteorological Disaggregation. Daily-to-subdaily met forcing disaggregation.],
    [Atmosphere],
    [Empirical],
  ),
  caption: [VIC process catalog (17 entries). Primary families: Hydrology (7), Cryosphere (5), Biogeochem (2), Atmosphere (2).],
) <tab:vic>

= NOAA-GFDL ESM4 --- Component Overview <app:gfdl>

GFDL ESM4 submodules were not initialized in the reference clone; the catalog below is derived from `.gitmodules` and published model descriptions.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1], [FV3 Dynamical Core. Finite-volume cubed-sphere non-hydrostatic atmosphere.], [Atmosphere], [Physics-based],
    [2],
    [GFDL Atmospheric Physics. Radiation, convection, cloud microphysics, PBL, gravity waves.],
    [Atmosphere],
    [Physics-based],

    [3],
    [AM4 Aerosol/Chemistry. Interactive tropospheric/stratospheric chemistry with aerosol.],
    [Atmosphere],
    [Physics-based],

    [4], [MOM6 Ocean. Modular Ocean Model v6 (see @tab:ocean).], [Ocean], [Physics-based],
    [5],
    [SIS2 Sea Ice. GFDL sea ice model with multi-category thermodynamics and dynamics.],
    [Cryosphere],
    [Physics-based],

    [6],
    [LM4 Land Model. Cohort-based land with soil C-N, plant hydraulics, and river routing.],
    [Ecology],
    [Physics-based],

    [7],
    [COBALT Ocean BGC. Carbon-Ocean Biogeochemistry and Lower Trophics marine ecosystem.],
    [Biogeochem],
    [Physics-based],

    [8], [FMS Infrastructure. Flexible Modeling System I/O, diagnostics, domain decomposition.], [Atmosphere], [N/A],
    [9], [Ice Parameter Module. Sea-ice parameterization library.], [Cryosphere], [Intermediate],
  ),
  caption: [GFDL ESM4 components (9 entries; submodules not populated).],
) <tab:gfdl>

= PEcAn --- Predictive Ecosystem Analyzer <app:pecan>

PEcAn is a scientific workflow system that wraps 20+ external ecosystem models with Bayesian calibration, uncertainty quantification, and benchmarking.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1], [ED2 Model Wrapper. Ecosystem Demography model integration.], [Ecology], [Physics-based],
    [2], [SIPNET Wrapper. Simplified photosynthesis + evapotranspiration model.], [Biogeochem], [Empirical],
    [3], [BioCro Wrapper. Bioenergy crop growth model (C4 grasses, Miscanthus).], [Ecology], [Intermediate],
    [4], [CLM4.5 Wrapper. Community Land Model integration.], [Ecology], [Physics-based],
    [5], [FATES Wrapper. FATES vegetation demography integration.], [Ecology], [Physics-based],
    [6], [JULES Wrapper. Joint UK Land Environment Simulator.], [Ecology], [Physics-based],
    [7], [LPJ-GUESS Wrapper. Dynamic vegetation model integration.], [Ecology], [Intermediate],
    [8], [DALEC Wrapper. Data Assimilation Linked Ecosystem Carbon model.], [Biogeochem], [Empirical],
    [9], [LINKAGES Wrapper. Forest succession model.], [Ecology], [Intermediate],
    [10],
    [MAESPA Wrapper. Multilayer tree canopy radiation + photosynthesis + water balance.],
    [Ecology],
    [Physics-based],

    [11], [RothC Wrapper. Rothamsted soil carbon turnover model.], [Biogeochem], [Empirical],
    [12],
    [Bayesian Photosynthesis Calibration. Hierarchical Bayes inversion for $V_"cmax"$, $J_"max"$ from leaf gas exchange.],
    [Biogeochem],
    [Physics-based],

    [13],
    [Radiative Transfer Model Inversion. PROSPECT/SAIL inversion for leaf/canopy traits from spectral data.],
    [Radiation],
    [Physics-based],

    [14], [Allometry Module. Tree diameter→height/biomass/crown allometric relationships.], [Ecology], [Empirical],
    [15],
    [Met Processing (CF Standards). Meteorological data ingestion, gap-filling, and downscaling.],
    [Atmosphere],
    [Intermediate],

    [16],
    [Data Assimilation (State/Parameter). Ensemble Kalman filter and particle filter for state updating.],
    [Ecology],
    [Physics-based],

    [17], [Sensitivity Analysis (Morris/Sobol). Global sensitivity for model parameters.], [Ecology], [Physics-based],
    [18],
    [Uncertainty Quantification. Monte Carlo uncertainty propagation with Latin hypercube sampling.],
    [Ecology],
    [Physics-based],

    [19],
    [Benchmarking Module. Model-data comparison scoring against AmeriFlux, MODIS, forest inventory.],
    [Ecology],
    [Intermediate],

    [20],
    [Gaussian Process Emulator. Surrogate model construction for expensive simulations.],
    [Ecology],
    [Physics-based],
  ),
  caption: [PEcAn workflow and model wrapper catalog (20 entries).],
) <tab:pecan>

= LANDIS-II Core / Core-Model-v6 <app:landis>

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Succession Extension Interface. Base class for forest succession state variables and dynamics.],
    [Ecology],
    [Intermediate],

    [2],
    [Species / Cohort Data Model. Species-level parameterization and cohort tracking per site.],
    [Ecology],
    [Intermediate],

    [3], [Ecoregion Framework. Spatially indexed ecological region properties.], [Ecology], [Empirical],
    [4],
    [Disturbance Extension Interface. Plugin architecture for fire, wind, insects, harvest.],
    [Ecology],
    [Intermediate],

    [5], [Seed Dispersal. Species-specific effective and maximum dispersal distances.], [Ecology], [Intermediate],
    [6], [Site-Level Establishment. Probabilistic establishment filtering per species.], [Ecology], [Intermediate],
  ),
  caption: [LANDIS-II Core framework (6 entries). Extensions (NECN, PnET, SCRPPLE, etc.) provide full process models.],
) <tab:landis>

#pagebreak()

= JULES --- Joint UK Land Environment Simulator <app:jules>

JULES (Best et al., 2011; Clark et al., 2011) is the community land surface model of the UK Met Office, used as the land component of UKESM1 and HadGEM3. It simulates energy balance, hydrology, snow, vegetation dynamics, carbon and nitrogen cycling, fire, urban surfaces, and crop growth on a regular grid. JULES is notable for its multi-layer snow scheme, TRIFFID dynamic vegetation, and INFERNO fire model.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Surface Energy Balance. Coupled solution of sensible, latent, ground, and radiative fluxes via Penman--Monteith on tiles.],
    [Atmosphere],
    [Physics-based],

    [2],
    [Tile-Based Land Surface. Sub-grid heterogeneity via 9 surface types (broadleaf, needleleaf, C3/C4 grass, shrub, urban, lake, soil, ice) with independent energy/water budgets.],
    [Hydrology],
    [Intermediate],

    [3],
    [Multi-Layer Snow Scheme. Up to 3-layer prognostic snow with compaction, grain growth, liquid water retention, and albedo aging.],
    [Cryosphere],
    [Physics-based],

    [4],
    [Zero-Layer Snow Scheme. Single-layer diagnostic snow for computational efficiency.],
    [Cryosphere],
    [Empirical],

    [5],
    [Canopy Radiation. Multi-layer two-stream canopy radiation with separate direct/diffuse, sunlit/shaded leaf partitioning.],
    [Radiation],
    [Physics-based],

    [6],
    [Photosynthesis (C3/C4). Leaf-level Farquhar (C3) and Collatz (C4) photosynthesis with enzyme kinetics and co-limitation.],
    [Biogeochem],
    [Physics-based],

    [7],
    [Stomatal Conductance. Jacobs (1994) or Medlyn (2011) stomatal models coupled to photosynthesis.],
    [Biogeochem],
    [Intermediate],

    [8],
    [Plant Respiration. Maintenance respiration (Q10 temperature dependence) and growth respiration as fixed fraction of NPP.],
    [Biogeochem],
    [Intermediate],

    [9],
    [TRIFFID Dynamic Vegetation. Lotka--Volterra competition among 5 PFTs for fractional cover based on NPP and height-dependent dominance.],
    [Ecology],
    [Intermediate],

    [10],
    [Phenology. Leaf onset/senescence driven by temperature and moisture thresholds with growing degree day accumulation.],
    [Ecology],
    [Empirical],

    [11],
    [Soil Hydrology (Richards Equation). Multi-layer (4-layer default) Richards equation with van Genuchten or Brooks--Corey hydraulic functions.],
    [Hydrology],
    [Physics-based],

    [12],
    [Soil Thermodynamics. Multi-layer heat diffusion with phase change (freeze--thaw), temperature-dependent thermal conductivity.],
    [Hydrology],
    [Physics-based],

    [13],
    [PDM Runoff. Probability Distributed Model for surface runoff generation based on sub-grid soil moisture distribution.],
    [Hydrology],
    [Intermediate],

    [14],
    [TOPMODEL Runoff. Topography-based saturation-excess runoff using topographic wetness index.],
    [Hydrology],
    [Intermediate],

    [15],
    [Soil Carbon (RothC). 4-pool soil carbon decomposition (DPM, RPM, BIO, HUM) with temperature and moisture rate modifiers.],
    [Biogeochem],
    [Intermediate],

    [16],
    [Soil Nitrogen Cycling. Ammonium/nitrate pools with mineralization, nitrification, denitrification, leaching, and N#sub[2]O emissions.],
    [Biogeochem],
    [Intermediate],

    [17],
    [Methane Emissions. Wetland CH#sub[4] emissions from substrate availability, temperature, and water table depth.],
    [Biogeochem],
    [Intermediate],

    [18],
    [INFERNO Fire Model. Interactive fire with human/lightning ignition, fuel moisture/load thresholds, burnt area, and fire emissions.],
    [Fire],
    [Intermediate],

    [19],
    [River Routing (RFM). 1-D kinematic wave river flow model for global river discharge.],
    [Hydrology],
    [Intermediate],

    [20],
    [Irrigation Demand. Crop-specific irrigation scheduling from soil moisture deficit relative to field capacity.],
    [Human Systems],
    [Empirical],

    [21],
    [Crop Model (JULES-crop). Phenology-driven crop growth with leaf/stem/root/storage organ partitioning for wheat, rice, maize, soybean.],
    [Human Systems],
    [Intermediate],

    [22],
    [Urban Energy Balance (MORUSES). 2-tile urban canyon model (roof, road+wall) with radiative trapping, anthropogenic heat, and thermal inertia.],
    [Atmosphere],
    [Intermediate],

    [23],
    [Lake Model (FLake). 2-layer lake thermodynamics with mixed layer, thermocline, and ice cover.],
    [Hydrology],
    [Intermediate],

    [24],
    [Permafrost / Soil Freeze--Thaw. Deep soil column (up to 10 m) with phase change, supercooled liquid water, and bedrock heat capacity.],
    [Cryosphere],
    [Physics-based],

    [25],
    [Dust Emission. Wind-erosion dust flux from bare soil tiles using Woodward (2001) scheme.],
    [Atmosphere],
    [Intermediate],
  ),
  caption: [JULES process catalog (25 entries).],
) <tab:jules>

#pagebreak()

= CABLE --- Community Atmosphere Biosphere Land Exchange <app:cable>

CABLE (Kowalczyk et al., 2006; Wang et al., 2011) is the Australian land surface model used in ACCESS-ESM and ACCESS-CM2 (CMIP6). It features a two-leaf canopy model, a six-layer soil scheme, the CASA-CNP biogeochemical model, and the POP demographic model for forest dynamics.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Surface Energy Balance. Coupled solution of radiation, sensible heat, latent heat, and ground heat flux on vegetated and bare soil tiles.],
    [Atmosphere],
    [Physics-based],

    [2],
    [Two-Leaf Canopy Model. Separate sunlit/shaded leaf photosynthesis and conductance, distinguishing direct and diffuse radiation interception.],
    [Radiation],
    [Physics-based],

    [3],
    [Photosynthesis. Farquhar--von Caemmerer (C3) and Collatz (C4) leaf-level biochemistry with nitrogen-dependent Vcmax.],
    [Biogeochem],
    [Physics-based],

    [4],
    [Stomatal Conductance (Leuning). Ball--Berry--Leuning model coupling stomatal aperture to photosynthesis, humidity deficit, and CO#sub[2].],
    [Biogeochem],
    [Intermediate],

    [5],
    [Aerodynamic Conductance. Roughness sublayer parameterization with Raupach (1994) within-canopy and above-canopy resistances.],
    [Atmosphere],
    [Physics-based],

    [6],
    [Soil Hydrology (6-Layer Richards). Six-layer soil moisture with Richards equation, Clapp--Hornberger hydraulic functions, and gravitational drainage.],
    [Hydrology],
    [Physics-based],

    [7],
    [Soil Thermodynamics (6-Layer). Six-layer heat diffusion with freeze--thaw and layered thermal conductivity.],
    [Hydrology],
    [Physics-based],

    [8],
    [Snow Model (3-Layer). Prognostic snow with compaction, liquid water retention, and grain metamorphism.],
    [Cryosphere],
    [Intermediate],

    [9],
    [Canopy Interception. Precipitation interception and throughfall/stemflow partitioning on LAI-dependent storage.],
    [Hydrology],
    [Empirical],

    [10],
    [CASA-CNP Biogeochemistry. Carbon, nitrogen, and phosphorus cycling through plant/litter/soil pools with 3 litter and 3 soil pools.],
    [Biogeochem],
    [Intermediate],

    [11],
    [Leaf Litter Decomposition. Temperature and moisture-dependent decomposition of metabolic, structural, and CWD litter.],
    [Biogeochem],
    [Intermediate],

    [12],
    [Nitrogen Limitation. Down-regulation of photosynthesis when soil N supply cannot match demand; biological N fixation.],
    [Biogeochem],
    [Intermediate],

    [13],
    [Phosphorus Limitation. Weathering-based P supply, sorption/desorption, and biochemical mineralization unique to Australian P-limited soils.],
    [Biogeochem],
    [Intermediate],

    [14],
    [POP (Patch of Patches). Demographic forest model with age-class patches, self-thinning, disturbance gaps, and landscape-level averaging.],
    [Ecology],
    [Intermediate],

    [15],
    [Phenology. Prescribed or climate-driven LAI with drought-deciduous and cold-deciduous types.],
    [Ecology],
    [Empirical],

    [16],
    [Root Water Uptake. Layer-weighted root extraction with exponential root density profile.],
    [Hydrology],
    [Empirical],

    [17],
    [Tile-Based Heterogeneity. Sub-grid tiling by PFT and bare soil with independent energy/water/carbon budgets.],
    [Hydrology],
    [Intermediate],

    [18],
    [SLI (Soil--Litter--Isotope). Advanced soil model coupling litter layer energy balance with soil moisture and heat.],
    [Hydrology],
    [Intermediate],
  ),
  caption: [CABLE process catalog (18 entries).],
) <tab:cable>

#pagebreak()

= CLASSIC --- Canadian Land Surface Scheme Including Biogeochemical Cycles <app:classic>

CLASSIC (Melton et al., 2020) unifies the Canadian Land Surface Scheme (CLASS; Verseghy, 1991) with the Canadian Terrestrial Ecosystem Model (CTEM; Arora, 2003). It is the land component of CanESM5 (CMIP6) and provides coupled energy, water, carbon, and nitrogen cycling with dynamic vegetation and prognostic fire.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Surface Energy Balance. Implicit coupled solution of surface temperature, sensible/latent heat, and ground heat flux over 4 sub-areas (needleleaf, broadleaf, crops, grass/bare).],
    [Atmosphere],
    [Physics-based],

    [2],
    [Multi-Layer Soil (3-Layer). Prognostic temperature and liquid/frozen moisture in 3 soil layers with thermal/hydraulic coupling.],
    [Hydrology],
    [Intermediate],

    [3],
    [Snow Physics (1-Layer). Single-layer snow with aging albedo, densification, liquid water retention, and heat capacity.],
    [Cryosphere],
    [Intermediate],

    [4],
    [Canopy Conductance. Jarvis-type stomatal conductance with PAR, temperature, VPD, and soil moisture stress functions.],
    [Biogeochem],
    [Intermediate],

    [5],
    [Photosynthesis. Farquhar (C3) and Collatz (C4) leaf-level models integrated over sunlit/shaded fractions with nitrogen scaling.],
    [Biogeochem],
    [Physics-based],

    [6],
    [Autotrophic Respiration. Maintenance respiration (stem, root, leaf) with Q10 temperature dependence plus growth respiration.],
    [Biogeochem],
    [Intermediate],

    [7],
    [Heterotrophic Respiration. Decomposition of 2 litter pools (structural, metabolic) and 1 soil carbon pool; temperature and moisture rate modifiers.],
    [Biogeochem],
    [Intermediate],

    [8],
    [Dynamic Vegetation (CTEM). Competition among 9 PFTs driven by NPP-based colonization, mortality (heat stress, drought, disturbance), and bioclimatic limits.],
    [Ecology],
    [Intermediate],

    [9],
    [Phenology. Cold-deciduous and drought-deciduous leaf onset/offset with carbon cost of leaf deployment.],
    [Ecology],
    [Empirical],

    [10],
    [Allocation. NPP allocation to leaf, stem, root with allometric constraints varying by PFT and light competition.],
    [Ecology],
    [Intermediate],

    [11],
    [Fire (CTEM-Fire). Area burned from probability of fire occurrence (lightning + human ignition) $times$ fire spread rate $times$ fire duration; biomass combustion and post-fire mortality.],
    [Fire],
    [Intermediate],

    [12],
    [Land Use Change. Deforestation, afforestation, and cropland expansion with carbon pool transfers.],
    [Human Systems],
    [Empirical],

    [13],
    [Wetland Methane. Heterotrophic respiration-based substrate with temperature-dependent methanogenesis and oxidation.],
    [Biogeochem],
    [Intermediate],

    [14],
    [Permafrost Carbon. Deep soil carbon cycling in permafrost layers with thaw-dependent decomposition rates.],
    [Biogeochem],
    [Intermediate],

    [15],
    [Nitrogen Cycle. Plant N uptake, biological N fixation, litter/soil N mineralization with C:N ratio constraints on decomposition.],
    [Biogeochem],
    [Intermediate],

    [16], [River Routing. Linear reservoir-based river flow routing to ocean grid cells.], [Hydrology], [Empirical],
  ),
  caption: [CLASSIC process catalog (16 entries).],
) <tab:classic>

#pagebreak()

= SUMMA --- Structure for Unifying Multiple Modeling Alternatives <app:summa>

SUMMA (Clark et al., 2015a,b) is a hydrological modeling framework developed at NCAR that enables controlled experimentation with alternative process representations within a single code. Its explicit representation of modeling decisions --- which process parameterization to use at each modeling layer --- makes it uniquely relevant to MAESMA's mission of autonomous process selection.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Radiation Transfer (Multiple). Two-stream, Beer--Lambert, or Noah-style canopy radiation with switchable formulations.],
    [Radiation],
    [Physics-based],

    [2],
    [Stomatal Resistance (Multiple). Ball--Berry, Jarvis, or simple-resistance formulations selectable at runtime.],
    [Biogeochem],
    [Intermediate],

    [3],
    [Canopy Shortwave. Separate direct/diffuse partitioning with variable canopy scattering.],
    [Radiation],
    [Physics-based],

    [4],
    [Canopy Longwave. Multi-layer or single-layer longwave radiation within the canopy.],
    [Radiation],
    [Intermediate],

    [5],
    [Throughfall / Canopy Interception. Maximum storage capacity model vs. Rutter model for canopy water budget.],
    [Hydrology],
    [Empirical],

    [6],
    [Snow Layering. Dynamic layer splitting/merging (CLM-style, VIC-style, or Anderson-style) with variable max layers.],
    [Cryosphere],
    [Intermediate],

    [7], [Snow Compaction. Anderson (1976) or empirical overburden compaction models.], [Cryosphere], [Intermediate],
    [8],
    [Snow Albedo. BATS decay, CLASS aging, or lookup-based snow albedo parameterizations.],
    [Cryosphere],
    [Intermediate],

    [9],
    [Snow Liquid Water. Bucket or Richards-equation percolation within snowpack layers.],
    [Cryosphere],
    [Intermediate],

    [10],
    [Surface Energy Balance. Implicit solution coupling surface temperature, turbulent fluxes, and ground conduction.],
    [Atmosphere],
    [Physics-based],

    [11],
    [Soil Hydrology (Richards). Multi-layer mixed form Richards equation with switchable hydraulic functions (Campbell or van Genuchten).],
    [Hydrology],
    [Physics-based],

    [12],
    [Groundwater. Water table depth as prognostic variable with aquifer recharge/discharge coupling to soil column.],
    [Hydrology],
    [Intermediate],

    [13],
    [Soil Thermodynamics. Multi-layer Fourier heat conduction with freeze--thaw and de Vries thermal conductivity.],
    [Hydrology],
    [Physics-based],

    [14],
    [Baseflow. TOPMODEL-based, linear reservoir, or non-linear power-law baseflow parameterizations.],
    [Hydrology],
    [Intermediate],

    [15],
    [Surface Runoff. Saturation-excess (TOPMODEL), infiltration-excess (Green--Ampt), or combined.],
    [Hydrology],
    [Intermediate],

    [16],
    [Evapotranspiration. Penman--Monteith or resistance-based approaches with separate soil/canopy/snow evaporation.],
    [Hydrology],
    [Physics-based],

    [17],
    [Lateral Subsurface Flow. Hillslope-scale lateral redistribution of soil moisture via Darcy's law.],
    [Hydrology],
    [Physics-based],

    [18],
    [Upper Boundary Condition (Multiple). Dirichlet, Neumann, or flux-based upper boundary for heat/moisture.],
    [Hydrology],
    [Intermediate],

    [19],
    [Lower Boundary Condition (Multiple). Free drainage, prescribed head, or zero-flux lower boundary.],
    [Hydrology],
    [Intermediate],

    [20],
    [Numerical Solver. Coupled or split solution of energy/mass conservation via backward Euler with Newton--Raphson iteration.],
    [Hydrology],
    [Physics-based],
  ),
  caption: [SUMMA process catalog (20 entries). SUMMA is uniquely structured around switchable process representations, reflecting 20+ modeling decisions.],
) <tab:summa>

#pagebreak()

= ED2 --- Ecosystem Demography Model 2 <app:ed2>

ED2 (Medvigy et al., 2009; Longo et al., 2019) is a cohort-based terrestrial biosphere model developed at Harvard that resolves individual-level vegetation dynamics through size- and age-structured cohorts within patches. As a precursor to FATES, ED2 provides independent implementations of canopy radiation, photosynthesis, allocation, disturbance, and demographic dynamics with distinct algorithmic choices.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Canopy Radiation (Multi-Layer). Exact solution of multi-layer two-stream radiation within vertically resolved canopy strata.],
    [Radiation],
    [Physics-based],

    [2],
    [Photosynthesis. Farquhar--Leuning coupled leaf-level model with Arrhenius temperature dependencies and co-limitation.],
    [Biogeochem],
    [Physics-based],

    [3],
    [Stomatal Conductance. Leuning (1995) stomatal model coupled iteratively to photosynthesis and leaf energy balance.],
    [Biogeochem],
    [Intermediate],

    [4],
    [Leaf Energy Balance. Individual-leaf Penman--Monteith with iterative convergence of surface temperature and fluxes.],
    [Atmosphere],
    [Physics-based],

    [5],
    [Stem Respiration. Diameter-based maintenance respiration with Q10 temperature scaling.],
    [Biogeochem],
    [Intermediate],

    [6], [Root Respiration. Biomass-dependent root maintenance respiration.], [Biogeochem], [Intermediate],
    [7],
    [Growth Respiration. Fixed fraction (0.33) of positive NPP allocated to construction costs.],
    [Biogeochem],
    [Empirical],

    [8],
    [Allocation. Allometric height--diameter--crown--root partitioning with obligate carbon balance closure.],
    [Ecology],
    [Intermediate],

    [9],
    [Cohort Dynamics. Size-structured cohort fusion/fission for computational efficiency while preserving demographic resolution.],
    [Ecology],
    [Intermediate],

    [10],
    [Patch Dynamics. Age-structured patch mosaic representing disturbance history; patches fuse when structurally similar.],
    [Ecology],
    [Intermediate],

    [11],
    [Gap-Phase Disturbance. Treefall gap creation with size-dependent mortality rates creating new patches.],
    [Ecology],
    [Intermediate],

    [12],
    [Mortality (5 Mechanisms). Carbon starvation, cold, disturbance, senescence, and density-dependent self-thinning mortality.],
    [Ecology],
    [Intermediate],

    [13],
    [Phenology (4 Schemes). Drought-deciduous, cold-deciduous, evergreen, and light-controlled leaf flushing/abscission.],
    [Ecology],
    [Empirical],

    [14],
    [Soil Carbon Decomposition. Fast/slow/structural soil carbon pools with heterotrophic respiration driven by temperature and moisture.],
    [Biogeochem],
    [Intermediate],

    [15],
    [Soil Hydrology. Multi-layer soil moisture with infiltration, percolation, and root water extraction.],
    [Hydrology],
    [Intermediate],

    [16],
    [Fire Disturbance. Prescribed or stochastic fire occurrence with combustion completeness dependent on fuel moisture.],
    [Fire],
    [Empirical],

    [17],
    [Seed Dispersal / Recruitment. Seed rain, germination, and sapling establishment into existing patches.],
    [Ecology],
    [Empirical],

    [18],
    [Allometry. DBH-based allometric equations for height, crown area, biomass, and LAI following Chave-type tropical/temperate relations.],
    [Ecology],
    [Intermediate],
  ),
  caption: [ED2 process catalog (18 entries).],
) <tab:ed2>

#pagebreak()

= PFLOTRAN --- Subsurface Reactive Transport <app:pflotran>

PFLOTRAN (Lichtner et al., 2015) is a DOE-funded massively parallel code for simulating subsurface multiphase flow, multicomponent reactive transport, and geomechanics. It operates on structured and unstructured grids, supports coupling to CLM/ELM, and fills essential gaps in deep subsurface biogeochemistry, groundwater contamination, and nuclear waste repository modeling.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Variably-Saturated Flow (Richards). Richards equation with Mualem--van Genuchten or Brooks--Corey constitutive relations.],
    [Hydrology],
    [Physics-based],

    [2],
    [Multiphase Flow. Two-phase (liquid--gas) or three-phase (liquid--gas--NAPL) Darcy flow with capillary pressure.],
    [Hydrology],
    [Physics-based],

    [3],
    [Reactive Transport. Advection-dispersion-reaction equation for multicomponent solute transport with operator splitting.],
    [Biogeochem],
    [Physics-based],

    [4],
    [Aqueous Speciation. Thermodynamic equilibrium speciation using law of mass action with extended Debye--Hückel activity coefficients.],
    [Biogeochem],
    [Physics-based],

    [5],
    [Mineral Precipitation/Dissolution. Transition State Theory (TST) rate laws with surface-area-dependent kinetics.],
    [Geology],
    [Physics-based],

    [6],
    [Sorption. Linear Kd, Langmuir, Freundlich, ion exchange, and surface complexation adsorption models.],
    [Biogeochem],
    [Intermediate],

    [7],
    [Microbial Degradation. Monod kinetics with multiple electron donor/acceptor pairs, biomass growth, and inhibition terms.],
    [Biogeochem],
    [Intermediate],

    [8],
    [Colloid Transport. Facilitated transport of contaminants with filtration, attachment/detachment kinetics.],
    [Hydrology],
    [Physics-based],

    [9],
    [Heat Transport. Energy conservation with conduction, advection, and latent heat of phase change in porous media.],
    [Hydrology],
    [Physics-based],

    [10],
    [Geomechanics. Linear poroelasticity coupling pore pressure to solid displacement via Biot's equations.],
    [Geology],
    [Physics-based],

    [11],
    [CLM--PFLOTRAN Coupling. Interface replacing CLM subsurface hydrology/biogeochemistry with PFLOTRAN reactive transport.],
    [Hydrology],
    [Physics-based],

    [12],
    [Fracture Flow. Discrete fracture network (DFN) or dual-continuum flow in fractured porous media.],
    [Hydrology],
    [Physics-based],

    [13],
    [Radionuclide Decay. Multi-species radioactive decay chains with ingrowth and branching ratios.],
    [Biogeochem],
    [Physics-based],

    [14],
    [Structural Grid. Structured Cartesian or unstructured polyhedral meshes with PETSc-based parallel solvers.],
    [Hydrology],
    [Physics-based],

    [15],
    [CO#sub[2] Sequestration. Supercritical CO#sub[2] injection, residual/solubility/mineral trapping in saline aquifers.],
    [Human Systems],
    [Physics-based],
  ),
  caption: [PFLOTRAN process catalog (15 entries).],
) <tab:pflotran>

#pagebreak()

= ROMS --- Regional Ocean Modeling System <app:roms>

ROMS (Shchepetkin & McWilliams, 2005) is a widely-used free-surface, terrain-following, primitive-equation regional ocean model. It is used globally for coastal, estuarine, and open-ocean research with biogeochemical and sediment transport extensions. ROMS fills the regional/coastal ocean gap complementing the global models (MOM6, MPAS-Ocean) already cataloged.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [3D Primitive Equations. Hydrostatic Boussinesq momentum, continuity, temperature, and salinity equations with free surface.],
    [Ocean],
    [Physics-based],

    [2],
    [Free Surface. Split-explicit barotropic/baroclinic time stepping for fast external gravity waves.],
    [Ocean],
    [Physics-based],

    [3],
    [Advection (3rd/4th-Order). Third-order upstream or fourth-order centered horizontal/vertical advection schemes.],
    [Ocean],
    [Physics-based],

    [4],
    [Horizontal Mixing. Laplacian or biharmonic viscosity/diffusivity along geopotential, sigma, or isopycnal surfaces.],
    [Ocean],
    [Physics-based],

    [5],
    [Vertical Mixing (KPP). K-Profile Parameterization boundary layer scheme with interior Richardson-number mixing.],
    [Ocean],
    [Physics-based],

    [6],
    [Vertical Mixing (GLS). Generic Length Scale turbulence closure (k-ε, k-ω, k-kl, gen) via Umlauf--Burchard (2003).],
    [Ocean],
    [Physics-based],

    [7],
    [Tidal Forcing. Barotropic tidal constituents at open boundaries with tidal potential body force.],
    [Ocean],
    [Physics-based],

    [8],
    [Open Boundary Conditions. Radiation, Flather, Chapman, clamped, or gradient conditions for tracers and momentum.],
    [Ocean],
    [Intermediate],

    [9],
    [Bottom Boundary Layer. Logarithmic or quadratic drag with wave-current bottom stress interaction.],
    [Ocean],
    [Physics-based],

    [10],
    [Sediment Transport. Non-cohesive bed load (Meyer-Peter--Müller) and suspended load with multiple grain classes.],
    [Geomorphology],
    [Physics-based],

    [11],
    [Bed Morphodynamics. Bed evolution from erosion/deposition with active-layer stratigraphy tracking.],
    [Geomorphology],
    [Physics-based],

    [12],
    [NPZD Biogeochemistry. Nutrient--Phytoplankton--Zooplankton--Detritus pelagic ecosystem with light-dependent primary production.],
    [Biogeochem],
    [Intermediate],

    [13],
    [Fennel Biogeochemistry. Extended NPZD with chlorophyll-to-carbon ratios, oxygen cycling, and multiple nutrient limitation (N, P, Si).],
    [Biogeochem],
    [Intermediate],

    [14],
    [Air--Sea Flux. Bulk formulae (COARE or Fairall) for momentum, heat, and freshwater exchange with atmosphere.],
    [Atmosphere],
    [Intermediate],

    [15],
    [Wetting/Drying. Intertidal wetting and drying for estuarine and tidal flat simulations.],
    [Ocean],
    [Intermediate],

    [16],
    [Ice Shelf Thermodynamics. Sub-ice-shelf melt via three-equation thermodynamic formulation at ice--ocean interface.],
    [Cryosphere],
    [Physics-based],

    [17],
    [Sea Ice (Budgell). Elastic-viscous-plastic sea ice dynamics and thermodynamics (optional coupling).],
    [Cryosphere],
    [Intermediate],

    [18],
    [Lagrangian Floats / Drifters. Online particle tracking with optional behavior (diurnal vertical migration, settling).],
    [Ocean],
    [Intermediate],

    [19],
    [Nesting / Multi-Grid. One-way or two-way grid refinement with AGRIF or composition coupling.],
    [Ocean],
    [Intermediate],

    [20],
    [4D-Var Data Assimilation. Tangent linear and adjoint models for incremental strong-constraint 4D-Var.],
    [Ocean],
    [Physics-based],
  ),
  caption: [ROMS process catalog (20 entries).],
) <tab:roms>

#pagebreak()

= SWAT --- Soil and Water Assessment Tool <app:swat>

SWAT (Arnold et al., 1998; Neitsch et al., 2011) is the world's most widely-used watershed model, simulating hydrology, sediment, nutrient, and pesticide transport at the basin scale. SWAT operates on a semi-distributed HRU (Hydrologic Response Unit) framework and is extensively applied for agricultural water quality, climate impact assessment, and TMDL (Total Maximum Daily Load) studies.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Surface Runoff (SCS-CN). Modified SCS Curve Number method with antecedent moisture and slope adjustment.],
    [Hydrology],
    [Empirical],

    [2],
    [Surface Runoff (Green--Ampt). Infiltration-excess runoff via Green--Ampt Mein--Larson equation.],
    [Hydrology],
    [Physics-based],

    [3],
    [Evapotranspiration (Multiple). Penman--Monteith, Priestley--Taylor, or Hargreaves methods selectable.],
    [Hydrology],
    [Intermediate],

    [4],
    [Soil Percolation. Layer-based water movement with storage routing and percolation to shallow aquifer.],
    [Hydrology],
    [Intermediate],

    [5],
    [Lateral Subsurface Flow. Kinematic storage model for lateral soil-water redistribution.],
    [Hydrology],
    [Intermediate],

    [6],
    [Groundwater (Shallow Aquifer). Unconfined aquifer recharge, baseflow, revap to soil, and pumping.],
    [Hydrology],
    [Intermediate],

    [7],
    [Groundwater (Deep Aquifer). Confined aquifer recharge as fraction of percolation; deep aquifer loss.],
    [Hydrology],
    [Empirical],

    [8],
    [Channel Routing. Variable storage or Muskingum routing of streamflow through channel network.],
    [Hydrology],
    [Intermediate],

    [9],
    [Reservoir Routing. Target release, monthly/daily outflow tables, or measured discharge for impoundments.],
    [Human Systems],
    [Empirical],

    [10],
    [Snow Melt. Temperature-index melt with elevation band sub-daily temperature distribution.],
    [Cryosphere],
    [Empirical],

    [11],
    [Sediment Yield (MUSLE). Modified Universal Soil Loss Equation driven by runoff energy rather than rainfall.],
    [Geomorphology],
    [Empirical],

    [12],
    [Channel Sediment Routing. Stream power-based degradation and Bagnold-equation deposition in channels.],
    [Geomorphology],
    [Empirical],

    [13],
    [Nitrogen Cycle. Five soil N pools (NO#sub[3], NH#sub[4], active/stable organic, fresh organic) with mineralization, nitrification, denitrification, plant uptake, volatilization, and leaching.],
    [Biogeochem],
    [Intermediate],

    [14],
    [Phosphorus Cycle. Six soil P pools (labile, active/stable mineral, organic, fresh organic, solution) with sorption, mineralization, and transport.],
    [Biogeochem],
    [Intermediate],

    [15],
    [Pesticide Fate. Degradation, sorption, washoff, leaching, and volatilization for user-defined pesticide compounds.],
    [Human Systems],
    [Intermediate],

    [16],
    [Crop Growth (Simplified EPIC). Heat-unit-driven phenology, radiation-use-efficiency biomass accumulation, harvest index, and nutrient/water stress.],
    [Human Systems],
    [Intermediate],

    [17],
    [Irrigation / Management. Auto-irrigation, fertilizer, tillage, and grazing operations on HRU schedule.],
    [Human Systems],
    [Empirical],

    [18],
    [Urban Runoff. Build-up/wash-off model for impervious fraction with pollutant loading.],
    [Human Systems],
    [Empirical],
  ),
  caption: [SWAT process catalog (18 entries).],
) <tab:swat>

#pagebreak()

= GEOS-Chem --- Atmospheric Chemistry Transport Model <app:geoschem>

GEOS-Chem (Bey et al., 2001; https://geos-chem.org) is a global 3-D model of atmospheric composition driven by NASA GEOS meteorological fields. It simulates tropospheric and stratospheric chemistry, aerosol microphysics, emissions, and deposition for over 200 species, and is used by hundreds of research groups worldwide. GEOS-Chem fills a critical gap in the MAESMA knowledgebase: detailed atmospheric gas-phase and heterogeneous chemistry.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Tropospheric Chemistry (Full). ~350 reaction mechanism with HOx, NOx, Ox, VOC, halogen chemistry and JPL/IUPAC kinetics.],
    [Atmosphere],
    [Physics-based],

    [2],
    [Stratospheric Chemistry. ~120 additional reactions including Ox--NOx--HOx--ClOx--BrOx catalytic cycles and heterogeneous PSC chemistry.],
    [Atmosphere],
    [Physics-based],

    [3],
    [Photolysis (FAST-JX). Online photolysis rate calculation accounting for overhead ozone, aerosol, and cloud optical depth.],
    [Radiation],
    [Physics-based],

    [4],
    [Sulfate Aerosol. SO#sub[2] oxidation (gas-phase OH, aqueous H#sub[2]O#sub[2]/O#sub[3]), sulfate nucleation, condensation, and wet/dry removal.],
    [Atmosphere],
    [Intermediate],

    [5],
    [Carbonaceous Aerosol. Primary OC/BC emissions, SOA formation via 2-product or VBS scheme, and hygroscopic aging.],
    [Atmosphere],
    [Intermediate],

    [6],
    [Mineral Dust. Size-resolved dust emission (DEAD/Zender), gravitational settling, and heterogeneous uptake.],
    [Atmosphere],
    [Intermediate],

    [7],
    [Sea Salt Aerosol. Wind-speed-dependent emission, deliquescence, and size-dependent deposition.],
    [Atmosphere],
    [Intermediate],

    [8],
    [Nitrate / Ammonium Aerosol. ISORROPIA-II thermodynamic equilibrium for HNO#sub[3]--NH#sub[3]--H#sub[2]SO#sub[4] system.],
    [Atmosphere],
    [Physics-based],

    [9],
    [Wet Deposition. In-cloud and below-cloud scavenging with Henry's law solubility and retention efficiency.],
    [Atmosphere],
    [Intermediate],

    [10],
    [Dry Deposition (Wesely). Resistance-in-series model with aerodynamic, quasi-laminar, and surface resistances.],
    [Atmosphere],
    [Intermediate],

    [11],
    [Anthropogenic Emissions. Gridded emission inventories (CEDS, EDGAR, EPA NEI) with sector-specific temporal profiles.],
    [Human Systems],
    [Empirical],

    [12],
    [Biogenic Emissions (MEGAN). Model of Emissions of Gases and Aerosols from Nature --- isoprene, monoterpenes, sesquiterpenes, and oVOCs from canopy-scale parameterization.],
    [Biogeochem],
    [Intermediate],

    [13],
    [Biomass Burning Emissions. GFED, FINN, or QFED fire emission inventories with injection height parameterization.],
    [Fire],
    [Empirical],

    [14],
    [Transport (Advection). Lin--Rood flux-form semi-Lagrangian advection on cubed-sphere or latitude--longitude grids.],
    [Atmosphere],
    [Physics-based],

    [15],
    [Convective Transport. Archived convective mass fluxes from GEOS for deep and shallow convective tracer redistribution.],
    [Atmosphere],
    [Intermediate],

    [16],
    [Boundary Layer Mixing. Non-local PBL mixing using archived GEOS turbulent diffusivities and mixed-layer depths.],
    [Atmosphere],
    [Intermediate],

    [17],
    [Mercury Cycle. Elemental/reactive/particulate Hg emissions, oxidation (Br, OH), deposition, and re-emission with land/ocean reservoirs.],
    [Biogeochem],
    [Intermediate],

    [18],
    [CO#sub[2] / CH#sub[4] Simulation. Tagged tracers for fossil fuel, biosphere, fire, and wetland sources with atmospheric transport for inverse modeling.],
    [Biogeochem],
    [Intermediate],

    [19],
    [Aerosol--Radiation Interaction. Online aerosol optical depth calculation for radiative transfer feedback (when coupled to GCM).],
    [Radiation],
    [Intermediate],

    [20],
    [Halogen Chemistry (Cl/Br/I). Reactive halogen cycling from sea salt, biogenic emissions, and heterogeneous recycling on aerosols.],
    [Atmosphere],
    [Physics-based],
  ),
  caption: [GEOS-Chem process catalog (20 entries).],
) <tab:geoschem>

#pagebreak()

= Delft3D --- Coastal and Estuarine Hydrodynamics <app:delft3d>

Delft3D (Lesser et al., 2004; Deltares) is a widely-used integrated modeling suite for coastal, estuarine, riverine, and lake hydrodynamics. It couples flow, waves, sediment transport, morphodynamics, water quality, and ecology. Delft3D fills gaps in nearshore processes, estuarine mixing, tidal dynamics, and coastal morphology that global ocean models do not resolve.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Shallow Water Equations. 2D depth-averaged or 3D sigma-layer hydrostatic Navier--Stokes with Coriolis, pressure gradient, and wind stress.],
    [Ocean],
    [Physics-based],

    [2],
    [Turbulence Closure. k-ε model, k-L, or algebraic mixing-length for vertical eddy viscosity/diffusivity.],
    [Ocean],
    [Physics-based],

    [3],
    [Tidal Propagation. Astronomic tidal constituents, self-attraction and loading, and tidal flats wetting/drying.],
    [Ocean],
    [Physics-based],

    [4],
    [Wave--Current Interaction (SWAN). Online coupling with SWAN spectral wave model for radiation stress, wave setup, and longshore currents.],
    [Ocean],
    [Physics-based],

    [5],
    [Wind-Driven Circulation. Spatially varying wind stress with Charnock or Smith--Banke drag coefficients.],
    [Atmosphere],
    [Intermediate],

    [6],
    [Salinity Transport. Advection--diffusion of salinity with baroclinic density gradients driving estuarine circulation.],
    [Ocean],
    [Physics-based],

    [7],
    [Temperature Transport. Heat budget with shortwave absorption (Beer--Lambert), longwave, sensible/latent fluxes.],
    [Ocean],
    [Physics-based],

    [8],
    [Sediment Transport (Cohesive). Krone/Partheniades erosion/deposition for mud with flocculation and consolidation.],
    [Geomorphology],
    [Physics-based],

    [9],
    [Sediment Transport (Non-Cohesive). van Rijn (1993) bed load and suspended load with reference concentration.],
    [Geomorphology],
    [Physics-based],

    [10],
    [Bed Morphodynamics. Bed level update from sediment mass balance with bank erosion, dredging, and nourishment.],
    [Geomorphology],
    [Physics-based],

    [11],
    [Water Quality (DELWAQ). Process library of 100+ water quality processes: BOD/DO, nutrients, algae growth, toxics, heavy metals.],
    [Biogeochem],
    [Intermediate],

    [12],
    [Algal Bloom Dynamics. Multi-species algal growth with Monod nutrient limitation, self-shading, grazing, and settling.],
    [Ecology],
    [Intermediate],

    [13],
    [Secondary Flow. Spiral motion in river bends for lateral sediment transport and point-bar morphology.],
    [Ocean],
    [Physics-based],

    [14],
    [Dam Break / Flood Propagation. Riemann-solver-based flood wave propagation over complex bathymetry.],
    [Hydrology],
    [Physics-based],

    [15],
    [Oil Spill Tracking. Lagrangian particle tracking with weathering (evaporation, dispersion, emulsification).],
    [Human Systems],
    [Intermediate],

    [16],
    [Flexible Mesh (D-Flow FM). Unstructured mesh solver for arbitrary coastline and channel geometry.],
    [Ocean],
    [Physics-based],
  ),
  caption: [Delft3D process catalog (16 entries).],
) <tab:delft3d>

#pagebreak()

= LISFLOOD-FP --- Large-Scale Flood Inundation Model <app:lisflood>

LISFLOOD-FP (Bates & De Roo, 2000; Neal et al., 2012) is a raster-based flood inundation model designed for flood hazard mapping at scales from rivers to continents. It provides multiple solver options from simplified diffusive wave to full shallow water equations, making it a key reference for multi-fidelity hydrological modeling.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Shallow Water Equations (2D). Full 2D shallow water solver with Roe-type approximate Riemann scheme and explicit time stepping.],
    [Hydrology],
    [Physics-based],

    [2],
    [Diffusive Wave (ACC). Acceleration-limited diffusive wave approximation (Neal et al., 2012) for efficient continental-scale routing.],
    [Hydrology],
    [Intermediate],

    [3],
    [Inertial / Subgrid Channel. 1D subgrid channel model with bankfull transition to floodplain using Newton--Raphson water level convergence.],
    [Hydrology],
    [Physics-based],

    [4],
    [Manning Friction. Spatially distributed Manning's n for floodplain and channel roughness.],
    [Hydrology],
    [Empirical],

    [5],
    [Wetting / Drying. Thin-film or depth-threshold wetting and drying with mass conservation.],
    [Hydrology],
    [Intermediate],

    [6],
    [Rainfall Input. Direct rainfall-runoff on DEM cells with simple infiltration loss (Green--Ampt or constant rate).],
    [Hydrology],
    [Intermediate],

    [7], [Evaporation Loss. Simple potential-evaporation removal from surface water.], [Hydrology], [Empirical],
    [8],
    [Boundary Conditions. Free-slip wall, stage hydrograph, flow hydrograph, or tidal harmonic open boundaries.],
    [Hydrology],
    [Intermediate],

    [9],
    [DEM Pre-Processing. Hydrological conditioning, flat-area routing, and sub-grid topography statistics.],
    [Geomorphology],
    [Empirical],

    [10],
    [Adaptive Time Stepping. CFL-based adaptive $Delta t$ for explicit solver stability.],
    [Hydrology],
    [Physics-based],
  ),
  caption: [LISFLOOD-FP process catalog (10 entries).],
) <tab:lisflood>

#pagebreak()

= LM3-PPA / BiomeE --- Perfect Plasticity Approximation Vegetation Models <app:ppa>

The Perfect Plasticity Approximation (PPA; Purves et al., 2008; Strigul et al., 2008) replaces the computational complexity of explicit crown geometry with a single analytical closure: at demographic equilibrium, canopy packing is "perfectly plastic," so the total crown area of trees above the canopy closure height $z^*$ equals ground area. This yields a sharp canopy--understory partition solvable in $O(N)$ cohorts rather than $O(N^2)$ crown overlap tests. ED2-PPA (Weng et al., 2015) embedded PPA inside the ED2 framework. LM3-PPA extended it into GFDL's land model (Shevliakova et al., 2009). BiomeE (Weng et al., 2019; 2022) is the latest evolution, coupling PPA-based vegetation demography to full biogeochemistry within GFDL ESM4.1. Together these models define an alternative canopy-closure paradigm to the multi-layer two-stream schemes used in FATES and ED2.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Perfect Plasticity Canopy Closure. Analytical $z^*$ height partitioning: total crown area above $z^*$ equals ground area, yielding a binary canopy/understory light environment.],
    [Ecology],
    [Physics-based],

    [2],
    [Crown Area Allometry. Species-specific crown area--diameter power law ($A_c = a dot D^b$) controlling competitive light capture.],
    [Ecology],
    [Intermediate],

    [3],
    [Canopy Light Extinction (Beer--Lambert). Exponential light attenuation through canopy and understory strata with PFT-specific extinction coefficients.],
    [Radiation],
    [Intermediate],

    [4],
    [Photosynthesis (Farquhar / LUE). Farquhar C3 biochemistry (LM3-PPA) or light-use-efficiency model (BiomeE) integrated over sunlit/shaded fractions.],
    [Biogeochem],
    [Physics-based],

    [5],
    [Stomatal Conductance. Medlyn-type optimal stomatal model coupling carbon gain to water loss.],
    [Biogeochem],
    [Intermediate],

    [6],
    [Autotrophic Respiration. Leaf, sapwood, and fine-root maintenance respiration with Q10 temperature scaling plus growth respiration.],
    [Biogeochem],
    [Intermediate],

    [7],
    [Allocation (Allometric). Carbon allocation to leaf, sapwood, heartwood, fine root, and reproduction constrained by allometric invariants.],
    [Ecology],
    [Intermediate],

    [8],
    [Height Growth. Diameter-increment-driven height growth via species-specific H--D allometry; asymptotic maximum height.],
    [Ecology],
    [Intermediate],

    [9],
    [Diameter Growth. Annual carbon balance converted to diameter increment via pipe-model sapwood-area constraint.],
    [Ecology],
    [Intermediate],

    [10],
    [Reproduction / Seed Rain. Fraction of NPP allocated to reproduction; seed dispersal kernel; density-dependent sapling establishment.],
    [Ecology],
    [Intermediate],

    [11],
    [Mortality (Carbon Starvation). Death rate increases exponentially when stored carbon falls below a critical reserve.],
    [Ecology],
    [Intermediate],

    [12],
    [Mortality (Density-Dependent). Self-thinning mortality following Yoda's $-3/2$ power law at high stem density.],
    [Ecology],
    [Intermediate],

    [13],
    [Mortality (Background / Senescence). Age- or size-dependent background mortality rate.],
    [Ecology],
    [Empirical],

    [14],
    [Cohort Dynamics. Size-structured cohort tracking with annual growth, mortality, and recruitment; cohort merging for efficiency.],
    [Ecology],
    [Intermediate],

    [15],
    [Patch Age Structure. Disturbance-driven patch age distribution; young patches transition through successional stages.],
    [Ecology],
    [Intermediate],

    [16],
    [Soil Biogeochemistry (CENTURY-Type). Fast/slow/passive soil carbon pools with temperature and moisture decomposition modifiers; BiomeE adds N cycle.],
    [Biogeochem],
    [Intermediate],

    [17],
    [Nitrogen Cycle (BiomeE). Plant N uptake, resorption, litter N, soil mineralization, nitrification, and N-limitation of photosynthesis.],
    [Biogeochem],
    [Intermediate],

    [18],
    [Litter Dynamics. Leaf, fine-root, and woody litter input to metabolic/structural pools with lignin-fraction quality control.],
    [Biogeochem],
    [Intermediate],

    [19],
    [Disturbance Regime. Prescribed or stochastic disturbance (fire, windthrow) that resets patches to age zero with partial biomass kill.],
    [Fire],
    [Empirical],

    [20],
    [Trait-Based Competition. PFT competition mediated by height growth rate, shade tolerance (via Amax, Rdark), and wood density trade-offs.],
    [Ecology],
    [Intermediate],

    [21],
    [Water Stress (BiomeE). Soil moisture limitation on stomatal conductance and photosynthesis with PFT-specific wilting point.],
    [Hydrology],
    [Intermediate],

    [22],
    [Phenology. Prescribed or climate-driven (GDD, photoperiod) leaf onset/senescence for deciduous PFTs.],
    [Ecology],
    [Empirical],
  ),
  caption: [LM3-PPA / BiomeE process catalog (22 entries). Covers ED2-PPA (Weng et al., 2015), LM3-PPA, and BiomeE (Weng et al., 2019).],
) <tab:ppa>

#pagebreak()

= SORTIE-ND --- Spatially Explicit Individual-Based Forest Model <app:sortie>

SORTIE (Pacala et al., 1993, 1996) is the canonical individual-based forest gap model. Unlike cohort models (ED2, FATES), SORTIE tracks every individual tree's spatial $(x,y)$ position, enabling explicit neighborhood interactions for light, seed dispersal, and competition. SORTIE-ND (Neighborhood Dynamics; Murphy, 2008) is the modern open-source descendant. It serves as the reference for full spatial resolution against which PPA and cohort approximations are benchmarked.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Spatially Explicit Light (GLI). Global Light Index computation: for each sapling, a hemisphere is divided into sectors; neighboring tree crowns project shadows to compute fractional sky visibility.],
    [Radiation],
    [Physics-based],

    [2],
    [Crown Geometry. Explicit allometric crown radius, crown depth, and shape (ellipsoid, cone) per species for shadow casting.],
    [Ecology],
    [Intermediate],

    [3],
    [Growth (NCI). Neighborhood Competition Index: diameter growth = f(size, light, sum of distance-weighted neighbor basal areas).],
    [Ecology],
    [Intermediate],

    [4],
    [Growth (Relative Height). Height growth relative to local neighborhood max height with species-specific shade tolerance.],
    [Ecology],
    [Intermediate],

    [5],
    [Mortality (Stochastic). Size- and growth-rate-dependent annual mortality probability; slow-growing suppressed trees die faster.],
    [Ecology],
    [Intermediate],

    [6], [Mortality (Senescence). Maximum-age-dependent background death rate.], [Ecology], [Empirical],
    [7],
    [Seed Dispersal (Kernel). Spatially explicit seed rain from parent trees with species-specific 2D dispersal kernels (Weibull, lognormal, 2Dt).],
    [Ecology],
    [Intermediate],

    [8],
    [Substrate-Dependent Establishment. Sapling establishment probability conditioned on substrate type (tip-up mound, fresh CWD, forest floor, scarified).],
    [Ecology],
    [Intermediate],

    [9],
    [Clonal Reproduction. Vegetative sprouting from stumps or roots with species-specific probability and spatial offset.],
    [Ecology],
    [Intermediate],

    [10],
    [Harvest / Silviculture. Individual-tree selection, diameter-limit, or area-based harvest prescriptions with residual stand specification.],
    [Human Systems],
    [Empirical],

    [11],
    [Storm Damage. Wind-speed- and size-dependent blowdown probability with domino-effect neighborhood damage.],
    [Ecology],
    [Intermediate],

    [12],
    [Snag Dynamics. Standing dead tree decay through snag height classes with substrate generation for regeneration.],
    [Ecology],
    [Intermediate],

    [13],
    [Allometry. Species-specific DBH-to-height, DBH-to-crown-radius, and DBH-to-biomass allometric equations.],
    [Ecology],
    [Intermediate],

    [14],
    [Neighborhood Density Effects. Conspecific and heterospecific neighbor density effects on growth and mortality via Janzen--Connell mechanisms.],
    [Ecology],
    [Intermediate],
  ),
  caption: [SORTIE-ND process catalog (14 entries). The canonical spatially explicit individual-based forest model.],
) <tab:sortie>

#pagebreak()

= JABOWA / FORET --- Classic Gap Models <app:jabowa>

JABOWA (Botkin et al., 1972) was the first forest gap model; FORET (Shugart, 1984) generalized it. Their core innovation --- simulating patch-scale (~0.1 ha) forest dynamics with individual trees competing for light within gaps --- spawned an entire model lineage (ZELIG, LINKAGES, FORCLIM, UVAFME). These models remain foundational references for the gap-dynamics paradigm.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Gap Light Regime. Available light for each tree computed from the leaf area above it in a single vertical column; Beer--Lambert extinction within the gap (~0.1 ha patch).],
    [Radiation],
    [Intermediate],

    [2],
    [Diameter Growth (Potential Curve). Species-specific growth equation: $Delta D = G dot D dot (1 - D H \/ D_"max" H_"max") \/ (274 + 3 b_2 D - 4 b_3 D^2)$ modified by environmental scalars.],
    [Ecology],
    [Intermediate],

    [3],
    [Environmental Growth Modifiers. Multiplicative scalars ($0$--$1$) for (i) available light, (ii) growing degree-days, (iii) soil moisture (drought days), and (iv) soil nitrogen.],
    [Ecology],
    [Empirical],

    [4],
    [Shade Tolerance Classification. Species assigned to tolerance classes (very intolerant → very tolerant) determining minimum light threshold for survival and growth.],
    [Ecology],
    [Empirical],

    [5],
    [Mortality (Stress). If diameter growth falls below a minimum threshold for several consecutive years, death probability increases steeply (intrinsic stress kill).],
    [Ecology],
    [Intermediate],

    [6],
    [Mortality (Random). Small annual age-independent background mortality (~1--2% yr#super[-1]) ensuring most trees die before reaching maximum age.],
    [Ecology],
    [Empirical],

    [7],
    [Regeneration. Species establish in gaps when (i) growing-degree-day range is met, (ii) light exceeds tolerance threshold, and (iii) stochastic lottery selects from the species pool.],
    [Ecology],
    [Intermediate],

    [8],
    [Height--Diameter Allometry. Height $= 137 + b_2 D - b_3 D^2$; parabolic allometry with species-specific coefficients.],
    [Ecology],
    [Empirical],

    [9],
    [Leaf Area (LAI). Individual LAI computed from diameter and species-specific crown parameters; summed vertically for light competition.],
    [Ecology],
    [Empirical],

    [10],
    [Soil Moisture Budget (FORET). Simple bucket model: precipitation, PET (Thornthwaite), and available water capacity determine drought stress days.],
    [Hydrology],
    [Empirical],

    [11],
    [Decomposition / Nutrient Cycling (LINKAGES). Litter quality (lignin:N) drives decomposition rate; AET-scaled annual soil nitrogen availability (Pastor and Post, 1985).],
    [Biogeochem],
    [Intermediate],

    [12],
    [Biogeographic Envelope. Species presence filtered by minimum/maximum growing-degree-day and minimum January temperature thresholds.],
    [Ecology],
    [Empirical],
  ),
  caption: [JABOWA / FORET / LINKAGES gap model catalog (12 entries). Foundational gap-dynamics models.],
) <tab:jabowa>

#pagebreak()

= FORMIND --- Individual-Based Tropical Forest Model <app:formind>

FORMIND (Köhler & Huth, 1998; Fischer et al., 2016) is an individual-based forest gap model designed for species-rich tropical forests. It operates on a grid of 20$times$20 m patches, tracks individual trees grouped into PFTs, and emphasizes carbon balance-driven growth, logging, and fragmentation. FORMIND bridges the gap between classic temperate gap models and tropical diversity, with particular strength in logging impact assessment.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Light Competition (3D). Vertical light profile in 20$times$20 m patches computed from crown geometry; multi-layer canopy with penumbra effects.],
    [Radiation],
    [Intermediate],

    [2],
    [Photosynthesis (Gross). Light-response curve per PFT (Michaelis--Menten or rectangular hyperbola); integrated over crown layers.],
    [Biogeochem],
    [Intermediate],

    [3],
    [Respiration. Maintenance respiration (leaf, stem, root) with allometric scaling plus growth respiration.],
    [Biogeochem],
    [Intermediate],

    [4],
    [Carbon Balance Growth. Net carbon gain allocated to diameter increment via allometric pipe model; negative carbon balance triggers mortality.],
    [Ecology],
    [Intermediate],

    [5],
    [Allometry. PFT-specific DBH--height (Chave-type), DBH--crown-diameter, and DBH--biomass relations.],
    [Ecology],
    [Intermediate],

    [6],
    [Recruitment. PFT-specific seedling ingrowth rate (stems ha#super[-1] yr#super[-1]) into smallest size class filtered by light availability.],
    [Ecology],
    [Empirical],

    [7],
    [Seed Dispersal. Spatially explicit dispersal kernel between patches; wind- and animal-dispersed modes.],
    [Ecology],
    [Intermediate],

    [8],
    [Mortality (3 Modes). Background mortality, diameter-growth-dependent stress mortality, and crowding mortality from gap closure.],
    [Ecology],
    [Intermediate],

    [9],
    [Gap Formation. Large-tree fall creates gaps by damaging neighbors within the crown-fall zone.],
    [Ecology],
    [Intermediate],

    [10],
    [Logging Module. Selective logging by minimum diameter, species group, and cutting cycle; skid-trail collateral damage to residual stand.],
    [Human Systems],
    [Intermediate],

    [11],
    [Fragmentation. Edge effects on tree mortality, microclimate, and seed dispersal near forest--non-forest boundaries.],
    [Ecology],
    [Intermediate],

    [12],
    [Succession / PFT Competition. Pioneer vs. late-successional PFT trade-offs via maximum growth rate, shade tolerance, and wood density.],
    [Ecology],
    [Intermediate],

    [13],
    [Soil Water Bucket. Simple one-layer soil water balance with rainfall input, ET loss, and drought stress on growth.],
    [Hydrology],
    [Empirical],

    [14],
    [Land-Use Change. Deforestation, regrowth, and agricultural rotation scenarios on the patch grid.],
    [Human Systems],
    [Empirical],
  ),
  caption: [FORMIND process catalog (14 entries).],
) <tab:formind>

#pagebreak()

= LANDIS-II Extensions --- Landscape Disturbance and Succession <app:landis-ext>

LANDIS-II (Scheller et al., 2007) is a spatially explicit landscape model operating on raster cells (typically 100--250 m) over large domains ($10^4$--$10^6$ ha). The Core framework (cataloged separately) provides the extension interface; the extensions implement the actual process representations. Here we catalog the NECN (Net Ecosystem Carbon & Nitrogen), SCRPPLE (fire), and Dynamic Fuel System extensions that make LANDIS-II competitive with process-based DGVMs at landscape scales.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [NECN Soil Carbon. CENTURY-based 4-pool soil organic matter (active, slow, passive, SOM) decomposition with climate and texture controls.],
    [Biogeochem],
    [Intermediate],

    [2],
    [NECN Soil Nitrogen. Mineralization, nitrification, denitrification, leaching, and volatile N loss coupled to soil C decomposition.],
    [Biogeochem],
    [Intermediate],

    [3],
    [NECN Aboveground NPP. Radiation-use-efficiency estimate with PFT-specific parameters and water/temperature stress modifiers.],
    [Biogeochem],
    [Intermediate],

    [4],
    [NECN Litter Decomposition. Structural vs. metabolic litter partitioned by lignin:N ratio; surface and soil litter pools.],
    [Biogeochem],
    [Intermediate],

    [5],
    [NECN Dead Wood. Coarse woody debris decay through 5 decay classes with species-specific decay constants.],
    [Biogeochem],
    [Empirical],

    [6],
    [Biomass Succession. Cohort-level growth based on maximum ANPP, age-dependent decline, and neighborhood shade competition.],
    [Ecology],
    [Intermediate],

    [7],
    [Age-Only Succession. Simplest succession: species presence/absence by age since disturbance and shade tolerance class.],
    [Ecology],
    [Empirical],

    [8],
    [PnET Succession. Photosynthesis--Evapotranspiration-based succession using Aber & Federer (1992) physiological model within LANDIS-II.],
    [Ecology],
    [Intermediate],

    [9],
    [SCRPPLE Fire. Stochastic fire ignition (lightning/human), weather-dependent spread on landscape grid, fire severity from burn conditions.],
    [Fire],
    [Intermediate],

    [10],
    [Dynamic Fuel System. Fuel type assignment from cohort composition, age, and previous disturbance; fuel moisture based on weather stream.],
    [Fire],
    [Intermediate],

    [11],
    [Base Fire. Simpler stochastic fire regime with mean fire return interval per fire region; size drawn from fire-size distribution.],
    [Fire],
    [Empirical],

    [12],
    [Wind Disturbance. Stochastic windthrow events with size and severity drawn from empirical distributions; directional wind damage.],
    [Ecology],
    [Intermediate],

    [13],
    [Biological Disturbance (BDA). Insect/pathogen outbreaks modeled as host-density-dependent epidemics with spatial dispersal.],
    [Ecology],
    [Intermediate],

    [14],
    [Harvest / Forest Management. Spatially explicit timber harvest prescriptions: clearcut, shelterwood, selection; management areas and rotations.],
    [Human Systems],
    [Intermediate],

    [15],
    [Seed Dispersal. Species-specific effective and maximum dispersal distances with probability of long-distance dispersal events.],
    [Ecology],
    [Intermediate],

    [16],
    [Establishment (Climate Filtering). Species establishment probability filtered by soil moisture, temperature, and light given overstory composition.],
    [Ecology],
    [Intermediate],

    [17],
    [Climate Change (Climate Library). Spatially explicit future climate inputs with monthly temperature and precipitation for all extensions.],
    [Atmosphere],
    [Empirical],

    [18],
    [Land-Use / Land-Cover Change. Prescribed conversion of forest cells to agriculture, urban, or restoration based on scenario maps.],
    [Human Systems],
    [Empirical],
  ),
  caption: [LANDIS-II Extensions catalog (18 entries). Extends the Core framework (6 entries cataloged separately).],
) <tab:landis-ext>

#pagebreak()

= UVAFME --- University of Virginia Forest Model Enhanced <app:uvafme>

UVAFME (Shuman et al., 2017; Foster et al., 2019) is a modern descendant of the JABOWA/FORET gap model lineage, enhanced for global-scale application. It simulates individual-tree dynamics on replicated 500 m#super[2] plots with stochastic climate variability, permafrost coupling, and fire, and has been applied from boreal to tropical forests. UVAFME bridges classic gap models to the scale of DGVMs while retaining individual-tree resolution.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Individual-Tree Growth. Species-specific potential diameter growth modified by light, temperature (GDD), drought, and nutrient scalars.],
    [Ecology],
    [Intermediate],

    [2],
    [Gap Light Model. Vertical leaf-area summation within 500 m#super[2] plots; Beer--Lambert extinction for available light at each tree's crown midpoint.],
    [Radiation],
    [Intermediate],

    [3],
    [Stochastic Climate Variability. Monthly temperature and precipitation drawn from species-specific distributions to capture inter-annual variability effects on growth/mortality.],
    [Atmosphere],
    [Intermediate],

    [4],
    [Growing-Degree-Day Filtering. Species establishment and growth constrained by GDD thresholds; cold-hardiness filtering by minimum winter temperature.],
    [Ecology],
    [Empirical],

    [5],
    [Drought Stress. Soil moisture budget (Thornthwaite AET/PET) with species-specific drought tolerance thresholds.],
    [Hydrology],
    [Empirical],

    [6],
    [Mortality (Stress / Background). Growth-rate-dependent stress mortality (slow-growing trees die faster) plus age-independent background rate.],
    [Ecology],
    [Intermediate],

    [7],
    [Fire Module. Stochastic fire occurrence with mean return interval; fire kills trees based on size (bark thickness) and species fire tolerance.],
    [Fire],
    [Intermediate],

    [8],
    [Permafrost Coupling. Active-layer depth limits rooting depth; permafrost thaw deepens the active layer, enabling succession transitions (boreal-to-temperate).],
    [Cryosphere],
    [Intermediate],

    [9],
    [Soil Decomposition. Annual litter input decomposed at AET-controlled rate; soil nitrogen availability feeds back on growth via nutrient scalar.],
    [Biogeochem],
    [Intermediate],

    [10],
    [Regeneration. Stochastic seedling establishment from species pool when light and climate conditions are met; seedling bank for some species.],
    [Ecology],
    [Intermediate],

    [11],
    [Height--Diameter--Crown Allometry. Species-specific parabolic H--D allometry and crown area--diameter scaling.],
    [Ecology],
    [Empirical],

    [12],
    [Tree-Level Biomass. Aboveground and belowground biomass from allometric equations; leaf area from specific leaf area $times$ leaf biomass.],
    [Ecology],
    [Intermediate],

    [13],
    [Plot Replication. Hundreds of replicate plots averaged for landscape-level statistics; variance across plots captures spatial heterogeneity.],
    [Ecology],
    [Intermediate],

    [14],
    [Global Application. >100 species parameterized globally from trait databases; runs forced by ERA5 or CMIP climate grids.],
    [Ecology],
    [Intermediate],
  ),
  caption: [UVAFME process catalog (14 entries).],
) <tab:uvafme>

#pagebreak()

= FORCLIM --- Forest Dynamics Under Climate Change <app:forclim>

FORCLIM (Bugmann, 1996, 2001) is a widely-used gap model designed for climate change impact studies. Developed at ETH Zürich, it integrates a bioclimatic envelope approach with mechanistic carbon balance and has been applied extensively across European and North American forests. FORCLIM's modular structure (environment, plant, and soil submodels) makes it a template for multi-fidelity process substitution.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Gap Light. Vertical leaf-area integration within $1/12$ ha patch; Beer--Lambert attenuation with species-specific light extinction.],
    [Radiation],
    [Intermediate],

    [2],
    [Diameter Growth. Maximum potential growth (parabolic function of $D$) reduced by light, temperature, and drought multipliers.],
    [Ecology],
    [Intermediate],

    [3],
    [Bioclimatic Limits. Minimum winter temperature, GDD requirements, and drought tolerance define species range; dynamic with changing climate.],
    [Ecology],
    [Empirical],

    [4],
    [Drought Index (Monthly). Monthly bucket hydrology: precipitation vs. PET (Thornthwaite); species drought tolerance maps to growth reduction.],
    [Hydrology],
    [Empirical],

    [5],
    [Mortality (Growth-Dependent). Probability of survival decreases when realized growth < 10% of maximum for consecutive years (Bigler & Bugmann, 2004).],
    [Ecology],
    [Intermediate],

    [6],
    [Mortality (Background). Small constant annual death probability calibrated to species longevity.],
    [Ecology],
    [Empirical],

    [7],
    [Regeneration (Seed-Based). Species-specific maximum seedling number modulated by light, temperature, and browsing pressure.],
    [Ecology],
    [Intermediate],

    [8],
    [Browsing Pressure. Ungulate browsing reduces regeneration of palatable species; browsing intensity as external driver.],
    [Ecology],
    [Empirical],

    [9],
    [Soil Decomposition (Bucket-N). Single-layer soil nitrogen availability from AET-driven litter decomposition (LINKAGES-type).],
    [Biogeochem],
    [Intermediate],

    [10],
    [Height--DBH Allometry. Species-specific parabolic allometry with maximum height/diameter constraints.],
    [Ecology],
    [Empirical],

    [11],
    [Stand Structure Output. Stem density, basal area, biomass, LAI, and species composition by diameter class; validation against NFI data.],
    [Ecology],
    [Intermediate],

    [12],
    [Management Module. Thinning and harvesting prescriptions (from below, from above, clearcut, shelterwood) at species/diameter-class level.],
    [Human Systems],
    [Intermediate],
  ),
  caption: [FORCLIM process catalog (12 entries).],
) <tab:forclim>

#pagebreak()

= SORTIE-NG (Erickson) --- GPU-Native Spatially Explicit Individual-Based Forest Model <app:sortieng>

SORTIE-NG (Erickson, 2018) is a next-generation redesign of the SORTIE individual-based forest model, re-architected from the ground up for GPU execution. Where classical SORTIE processes every tree sequentially on CPU --- resulting in $O(N^2)$ neighborhood interactions and hemisphere-based light computations that dominate runtime --- SORTIE-NG maps all per-tree operations to massively parallel GPU kernels. The design philosophy treats the forest stand as a particle system: each tree is an independent data element processed by GPU threads in lockstep. Key innovations include CUDA-parallelized hemispherical light integration (replacing the $O(N^2)$ CPU shadow loop with a GPU rasterization pass over crown projections), batched seed dispersal via FFT-accelerated kernel convolution on device, and GPU-side allometric computation for diameter/height/crown radius updates. The model retains SORTIE's biological realism --- species-specific light response, spatially explicit competition, stochastic recruitment and mortality --- while achieving 50--200$times$ speedup on modern GPU hardware. This paradigm informs MAESMA's GPU-native architecture for ecology process families.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process / Capability*], [*Family*], [*Fidelity*]),
    [1],
    [GPU Hemispherical Light Model. Each tree's light environment computed via GPU rasterization of neighboring crown projections onto a hemisphere discretized into sky sectors; replaces SORTIE's sequential $O(N^2)$ ray-casting with a single parallel render pass per cohort batch. Global Light Index (GLI) per tree.],
    [Radiation],
    [Physics-based],

    [2],
    [Parallel Individual-Tree Processing. Every tree in the stand is a GPU thread: growth, mortality, and reproduction computed in parallel SIMT warps. Stand sizes of 10#super[5]--10#super[6] individuals processed per kernel launch.],
    [Ecology],
    [Intermediate],

    [3],
    [GPU Neighborhood Interaction. Spatial competition indices (NCI: neighborhood crowding index) computed via GPU spatial hashing with fixed-radius neighbor queries, reducing $O(N^2)$ to $O(N)$ amortized.],
    [Ecology],
    [Intermediate],

    [4],
    [FFT-Accelerated Seed Dispersal. Species-specific dispersal kernels (Weibull, lognormal, 2Dt) convolved with parent-tree density maps on GPU using cuFFT; produces seedling establishment probability surfaces at sub-meter resolution.],
    [Ecology],
    [Physics-based],

    [5],
    [GPU Allometric Computation. Diameter--height, diameter--crown radius, and diameter--biomass allometries evaluated as vectorized GPU functions across all individuals simultaneously. Species-specific coefficient lookup via texture memory.],
    [Ecology],
    [Empirical],

    [6],
    [Stochastic Mortality (GPU RNG). Per-tree mortality drawn from species-specific probability functions using GPU-parallel cuRAND streams. Size-dependent, competition-dependent, and storm-event mortality modes.],
    [Ecology],
    [Intermediate],

    [7],
    [Substrate-Dependent Recruitment. Seedling establishment probability conditioned on substrate type (mineral soil, decayed wood, moss, litter), light availability (GLI threshold), and conspecific density. GPU-parallel evaluation across all microsites.],
    [Ecology],
    [Empirical],

    [8],
    [Crown Plasticity and Asymmetric Competition. Individual crowns deform toward light gaps; asymmetric competition where larger trees suppress smaller neighbors proportional to crown overlap area computed via GPU geometric intersection.],
    [Ecology],
    [Intermediate],

    [9],
    [Species-Specific Growth Response. Radial growth as a function of light (GLI), tree size (DBH), local competition (NCI), and climate envelopes. Nonlinear response surfaces evaluated on GPU with per-species parameter vectors.],
    [Ecology],
    [Intermediate],

    [10],
    [Spatial Structure Tracking. Full $(x, y)$ coordinates maintained for every individual on GPU device memory. Pair correlation functions, Ripley's K, and mark correlation computed as GPU reduction kernels for real-time spatial pattern analysis.],
    [Ecology],
    [Intermediate],

    [11],
    [Disturbance Integration (Wind/Fire/Harvest). Spatially explicit windthrow (topographic exposure + crown drag), surface fire (flame length + bark thickness survival), and selective harvest (marking rules applied as GPU predicates).],
    [Fire],
    [Intermediate],

    [12],
    [Gap Dynamics and Canopy Turnover. Tree death creates gaps; gap light regimes computed by GPU hemisphere re-rasterization of surviving neighbors. Successional trajectory emerges from species-specific gap-phase regeneration.],
    [Ecology],
    [Intermediate],

    [13],
    [Multi-Species Coexistence Tracking. Shannon diversity, species importance values, size-class distributions, and compositional turnover tracked as GPU-parallel reductions per timestep. Enables real-time biodiversity monitoring.],
    [Ecology],
    [Empirical],

    [14],
    [Real-Time 3D Canopy Visualization. GPU-generated crown geometry (ellipsoidal or allometric) rendered via compute-to-render pipeline; live visualization of stand structure, light environment, and species composition during simulation.],
    [Ecology],
    [Intermediate],

    [15],
    [Climate Envelope Forcing. Temperature, precipitation, and growing-degree-day inputs drive species recruitment probability, growth rate modifiers, and drought-induced mortality thresholds. GPU-side interpolation from gridded climate data.],
    [Atmosphere],
    [Empirical],

    [16],
    [Soil Resource Competition. Below-ground resource partitioning via root-zone overlap computed from species-specific rooting depth profiles and spatial proximity; modulates growth alongside light competition.],
    [Biogeochem],
    [Intermediate],
  ),
  caption: [SORTIE-NG (Erickson) process catalog (16 entries). GPU-native spatially explicit forest model.],
) <tab:sortieng>

#pagebreak()

= DeepLand (Erickson) --- Deep Learning Land Surface Model <app:deepland>

DeepLand (Erickson) is a neural network-based land surface model that replaces traditional physics parameterizations with learned operators while enforcing physical constraints through architecture design and physics-informed loss functions. Rather than hand-coding stomatal conductance equations (Ball--Berry, Medlyn), soil hydraulic functions (van Genuchten, Brooks--Corey), or snow albedo decay curves, DeepLand learns these mappings directly from high-resolution process model output and observational data using the NVIDIA Modulus framework for physics-informed neural operator training. The model operates as a hybrid system: a process-based backbone handles prognostic state evolution (mass and energy conservation are structurally guaranteed by the architecture), while neural networks learn the constitutive relations, closure terms, and subgrid parameterizations that drive the largest uncertainties in conventional LSMs. Key Modulus principles embedded in the design include PDE residual loss for Richards equation and heat diffusion, Fourier Neural Operators for resolution-invariant spatial operators, and transfer learning from high-resolution simulations to coarser operational grids. DeepLand informs MAESMA's approach to physics--ML hybrid representations and neural operator-based process discovery.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process / Capability*], [*Family*], [*Fidelity*]),
    [1],
    [Neural Stomatal Conductance. Replaces Ball--Berry/Medlyn with a neural network mapping $(R_"net", "VPD", T_"leaf", C_a, "SWC") arrow.r g_s$. Trained on eddy covariance observations; physics-informed loss enforces non-negative conductance, correct $"VPD"$ response sign, and water-use efficiency bounds.],
    [Ecology],
    [Physics-based],

    [2],
    [FNO-Based Richards Equation Solver. Fourier Neural Operator learns the solution operator for variably saturated soil water flow. Trained on paired numerical solutions across soil texture classes; resolution-invariant formulation transfers from 1 cm training grid to 10 cm operational grid. PDE residual loss enforces $partial theta slash partial t = nabla dot (K(theta) nabla psi(theta))$ without paired simulation data via PINO.],
    [Hydrology],
    [Physics-based],

    [3],
    [Neural Soil Thermal Conductivity. DeepONet learns the conductivity-moisture-texture-density mapping; branch network encodes soil properties, trunk network encodes depth--time coordinates. Replaces Johansen/Côté--Konrad empirical fits.],
    [Hydrology],
    [Physics-based],

    [4],
    [Learned Radiative Transfer (Canopy). Neural network canopy radiative transfer replacing two-stream approximation: maps $(L A I, "leaf angle", "solar zenith", "spectral albedos") arrow.r ("absorbed PAR", "net radiation")$. Trained against Monte Carlo photon transport simulations; enforces energy conservation via architecture (output sums to incoming).],
    [Radiation],
    [Physics-based],

    [5],
    [Neural Snow Albedo and Metamorphism. Learns snow albedo decay, grain growth, and melt--refreeze dynamics from SNICAR offline simulations and satellite retrievals. Physics-informed loss enforces albedo $in [0, 1]$ and monotonic aging in absence of new snowfall.],
    [Cryosphere],
    [Physics-based],

    [6],
    [Hybrid Photosynthesis Model. Process-based Farquhar--von~Caemmerer--Berry backbone (Rubisco/RuBP limitation structure preserved); neural network learns temperature acclimation of $V_"c,max"$, $J_"max"$ and mesophyll conductance as residual corrections to standard $Q_10$ functions. GPU-accelerated batch evaluation across all grid cells.],
    [Ecology],
    [Physics-based],

    [7],
    [MeshGraphNet for Groundwater--Surface Water Exchange. Graph neural network operating on unstructured watershed mesh; learns lateral groundwater flow, seepage, and river--aquifer exchange fluxes. Message-passing architecture preserves mass conservation per mesh element.],
    [Hydrology],
    [Physics-based],

    [8],
    [Neural Turbulent Flux Parameterization. Replaces Monin--Obukhov similarity theory for surface layer turbulent fluxes $(H, L E)$: neural network maps $("wind speed", Delta T, Delta q, z_0, "stability") arrow.r (C_H, C_E)$. Trained on eddy covariance tower data; physics-informed loss enforces correct stability function limits and flux--gradient relationship sign.],
    [Atmosphere],
    [Physics-based],

    [9],
    [Transfer Learning: High-Res to Coarse Grid. Models trained at high resolution (e.g., 1 km CLM) are transferred to coarser grids (e.g., 100 km ESM) using Modulus-style domain adaptation. Learned subgrid distributions replace tile-fraction approaches; FNO resolution invariance enables direct weight transfer without retraining.],
    [Ecology],
    [Intermediate],

    [10],
    [Conservation-Enforcing Architecture. Network architecture structurally guarantees conservation: water balance ($P = E T + R + Delta S$) and energy balance ($R_"net" = H + L E + G$) enforced as hard constraints via output normalization layers, not soft loss terms. Closure errors are identically zero by construction.],
    [Hydrology],
    [Physics-based],

    [11],
    [Subgrid Heterogeneity Learning. Neural network learns subgrid probability distributions of soil moisture, LAI, and snow cover from high-resolution training data. Replaces tiling approaches (CLM PCT, JULES tiles) with learned continuous distributions that capture spatial correlations.],
    [Hydrology],
    [Intermediate],

    [12],
    [Neural Phenology. Learns budburst, senescence, and leaf area dynamics from satellite LAI time series (MODIS, Sentinel-2) conditioned on temperature, photoperiod, and soil moisture history. Recurrent architecture (LSTM/GRU) captures memory effects; physics-informed loss enforces non-negative LAI and seasonal carbon budget closure.],
    [Ecology],
    [Intermediate],

    [13],
    [GPU-Native End-to-End Training. Full model (backbone + neural components) trains end-to-end on GPU with automatic differentiation through both physics and ML components. Mixed-precision training (BF16 backbone, FP32 accumulation) on multi-GPU via PyTorch FSDP or JAX pjit. Training data: eddy covariance, FLUXNET, SMAP, GRACE, tower meteorology.],
    [Ecology],
    [Intermediate],

    [14],
    [Uncertainty Quantification via Deep Ensembles. Each neural component is an ensemble of $M$ networks ($M = 5$--$10$); epistemic uncertainty estimated from ensemble spread. Aleatoric uncertainty captured by heteroscedastic output layers. Total predictive uncertainty propagated through the process chain and reported per grid cell per timestep.],
    [Ecology],
    [Intermediate],

    [15],
    [Symbolic Distillation of Learned Operators. After training, PySR extracts interpretable symbolic expressions from neural components (e.g., discovering a power-law soil hydraulic conductivity from the FNO Richards solver). Distilled expressions are deposited as high-interpretability knowledgebase entries alongside their neural parents.],
    [Ecology],
    [Intermediate],

    [16],
    [Online Adaptation and Continual Learning. Neural components update their weights online as new observational data streams arrive (SMAP, GRACE, AmeriFlux), using elastic weight consolidation to prevent catastrophic forgetting. Drift-detection triggers full retraining when distribution shift exceeds threshold.],
    [Ecology],
    [Intermediate],

    [17],
    [Benchmarking Against Traditional LSMs. Automated head-to-head comparison against CLM, Noah-MP, JULES, and CABLE across PLUMBER-2 tower sites, FLUXNET-CH4 wetlands, and SnowMIP sites. Skill metrics (KGE, RMSE, bias, timing errors) tracked per process component and globally.],
    [Ecology],
    [Intermediate],

    [18],
    [Carbon Cycle Neural Closure. Neural network learns heterotrophic respiration, litter decomposition, and soil carbon turnover rates from FLUXNET NEE partitioning and soil incubation data. Physics-informed loss enforces non-negative pools and mass conservation ($Delta C = "NPP" - R_h - "DOC"_"export"$).],
    [Biogeochem],
    [Physics-based],
  ),
  caption: [DeepLand (Erickson) process catalog (18 entries). Deep learning land surface model integrating NVIDIA Modulus principles.],
) <tab:deepland>

#pagebreak()

= NVIDIA Earth-2 / earth2studio --- Foundation Weather Model Platform <app:earth2>

NVIDIA Earth-2 (earth2studio) is an open-source Python framework providing a unified interface to pre-trained AI foundation weather and climate models, ensemble generation, diagnostic workflows, and GPU-accelerated data pipelines. earth2studio treats weather prediction as a composable inference pipeline: data sources (GFS, ERA5, HRRR, IFS, CDS) feed into pre-trained models, whose outputs flow through diagnostic transforms and perturbation methods. MAESMA registers these foundation models as Atmosphere R0/R1 rungs in the knowledgebase, enabling the agent swarm to select AI-driven atmospheric forcing when weather dynamics are not the dominant source of error. The Foundation Model Agent manages model versioning, benchmarking against reanalysis, and automatic registration of new architectures as they are released.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Capability*], [*Family*], [*Fidelity*]),
    [1],
    [FourCastNet (SFNO). Spherical Fourier Neural Operator on 0.25° equiangular grid; 73 prognostic channels (surface + pressure-level T, u, v, z, q); 6-hourly autoregressive; 10-day global forecast in ~2 seconds on single GPU. Resolution-invariant spectral architecture.],
    [Atmosphere],
    [Physics-based],

    [2],
    [Pangu-Weather. 3D transformer on 13 pressure levels + surface; separate 1h/3h/6h/24h lead-time models; trained on 39 years of ERA5. Pressure-level prognostics enable vertical profile prediction.],
    [Atmosphere],
    [Physics-based],

    [3],
    [GraphCast. Graph neural network on icosahedral mesh (0.25°); message-passing over multi-mesh hierarchy; 10-day forecasts competitive with HRES; autoregressive with 6h steps.],
    [Atmosphere],
    [Physics-based],

    [4],
    [DLWP-CS. Deep Learning Weather Prediction on cubed-sphere grid; UNet encoder--decoder; 6h timestep; trained on ERA5; lightweight architecture suitable for large ensemble generation.],
    [Atmosphere],
    [Intermediate],

    [5],
    [GenCast. Diffusion-based probabilistic weather model; generates calibrated ensemble members from a single forward pass; captures multimodal forecast distributions for extreme events; 12-day probabilistic forecasts.],
    [Atmosphere],
    [Physics-based],

    [6],
    [CorrDiff (Super-Resolution). Score-based diffusion model for downscaling coarse forecasts (25 km → 2 km); trained on HRRR; preserves extreme value statistics and spatial coherence. Bridges global foundation models to regional process-model grids.],
    [Atmosphere],
    [Physics-based],

    [7],
    [Ensemble Perturbation Methods. Bred Vectors, Gaussian, Spherical Harmonics, and Laplacian perturbation methods for generating calibrated ensemble spread from deterministic models. GPU-parallel perturbation generation.],
    [Atmosphere],
    [Intermediate],

    [8],
    [Diagnostic Pipelines. GPU-accelerated computation of derived quantities: accumulated cyclone energy (ACE), atmospheric river detection (IVT), fire weather indices (FWI, FFMC, DMC, DC, ISI, BUI), heat wave indices, precipitation extremes. Direct feed into MAESMA benchmarking agent.],
    [Atmosphere],
    [Intermediate],

    [9],
    [Data Source Abstraction. Unified interface to GFS (NOAA), ERA5 (ECMWF/CDS), HRRR (NOAA), IFS (ECMWF), and satellite-derived analysis products. Lazy loading, coordinate normalization, and on-the-fly regridding via xarray + GPU interpolation.],
    [Atmosphere],
    [Empirical],

    [10],
    [Autoregressive Rollout Engine. GPU-optimized autoregressive inference loop for multi-day forecasts: state caching, mixed-precision (FP16/BF16), batch parallelism across ensemble members, and checkpoint-restart for long rollouts. Supports model chaining (e.g., FourCastNet → CorrDiff downscaling).],
    [Atmosphere],
    [Intermediate],

    [11],
    [Model Zoo Management. Versioned model registry with ONNX/PyTorch weight storage, automatic download, hardware-aware model selection (GPU memory constraints), and benchmark comparison against operational NWP (GFS, IFS HRES). New model releases auto-registered as knowledgebase candidates.],
    [Atmosphere],
    [Intermediate],

    [12],
    [Precipitation and Cloud Parameterization Priors. Foundation model outputs provide learned priors for convective and stratiform precipitation, cloud fraction, and radiative flux that downstream process discovery can use as training targets or baseline comparisons.],
    [Atmosphere],
    [Intermediate],

    [13],
    [Tropical Cyclone Tracking and Intensity. Post-processing pipeline detecting, tracking, and classifying tropical cyclones from foundation model outputs; intensity estimation from wind field maxima and pressure minima; enables rapid catastrophe assessment.],
    [Atmosphere],
    [Intermediate],

    [14],
    [Seamless Prediction (Weather--S2S--Climate). Multi-timescale inference bridging weather (1--15 days), subseasonal-to-seasonal (S2S, 2--8 weeks), and decadal climate states via model chaining and bias correction. Covers the prediction gap between NWP and climate projection.],
    [Atmosphere],
    [Intermediate],
  ),
  caption: [NVIDIA Earth-2 / earth2studio capabilities catalog (14 entries). Foundation weather model platform providing Atmosphere R0/R1 rungs.],
) <tab:earth2>

#pagebreak()

= ESA PhiSat-2 --- On-Board AI Earth Observation Principles <app:phisat2>

PhiSat-2 (ESA, 2022--) is the European Space Agency's technology demonstrator for on-board artificial intelligence in Earth observation satellites. Deploying inference models on a low-power VPU (Intel Movidius Myriad 2) directly on the spacecraft, PhiSat-2 demonstrated that scientifically valuable data can be selected, classified, and prioritized _at the point of acquisition_ --- eliminating downlink bandwidth waste on cloud-contaminated or scientifically redundant imagery. While PhiSat-2 is a single satellite mission, its architectural principles define a paradigm that MAESMA generalizes: autonomous observation intelligence, edge inference for real-time process monitoring, and active observation tasking driven by model uncertainty. The Autonomous Observation Agent instantiates these principles across the full observation stack --- from LEO satellites to field sensor networks.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Capability*], [*Family*], [*Fidelity*]),
    [1],
    [On-Board Cloud Detection and Filtering. CNN classifier (quantized INT8 for VPU deployment) applied to raw hyperspectral imagery; classifies cloud cover per-tile; discards frames exceeding cloud threshold before downlink. Reduces data volume by 30--70% without ground processing.],
    [Atmosphere],
    [Intermediate],

    [2],
    [Edge-AI Fire Detection. On-board detection of active fire pixels from thermal/SWIR bands; sub-pixel fire radiative power estimation; real-time alert generation with geolocation and confidence score. Latency: seconds from acquisition to alert.],
    [Fire],
    [Intermediate],

    [3],
    [Autonomous Flood Mapping. SAR/optical change detection deployed on-board; classifies inundation extent versus permanent water; generates flood mask products at point of acquisition for near-real-time disaster response.],
    [Hydrology],
    [Intermediate],

    [4],
    [Vegetation Stress Classification. Spectral index computation (NDVI, EVI, NDWI, CRI) and anomaly detection relative to phenological baseline; flags drought stress, pest damage, and defoliation events for priority downlink.],
    [Ecology],
    [Intermediate],

    [5],
    [Land Cover Change Detection. Multi-temporal comparison of classified scenes; detects deforestation, urbanization, agriculture expansion, and wetland loss between revisit passes. On-board differencing eliminates redundant stable-scene imagery.],
    [Ecology],
    [Empirical],

    [6],
    [Observation Value Scoring. On-board estimation of scientific value per scene: novelty (deviation from climatological expectation), relevance (overlap with model uncertainty hotspots downlinked as target maps), and quality (atmospheric contamination, off-nadir angle). Prioritizes high-value scenes for limited downlink bandwidth.],
    [Human Systems],
    [Intermediate],

    [7],
    [Compressed Feature Downlink. Instead of raw imagery, transmits extracted feature vectors (spectral indices, classified labels, detected events) when downlink bandwidth is severely constrained. Preserves information content at 10--100$times$ compression ratio.],
    [Human Systems],
    [Intermediate],

    [8],
    [Active Tasking Interface. Uplink protocol for receiving observation requests from ground-based agents (MAESMA's Autonomous Observation Agent). Adjustable pointing, spectral mode, spatial resolution, and priority scheduling based on model-driven information gain estimates.],
    [Human Systems],
    [Intermediate],

    [9],
    [Snow and Ice Extent Mapping. On-board classification of snow cover, sea ice extent, and glacier calving events from optical/thermal imagery; feeds cryosphere process models with near-real-time boundary conditions at high revisit frequency.],
    [Cryosphere],
    [Intermediate],

    [10],
    [Coastal and Water Quality Monitoring. On-board spectral analysis for chlorophyll-a, turbidity, harmful algal blooms, and sediment plume detection in coastal/inland waters. Flags anomalous events for priority processing.],
    [Ocean],
    [Intermediate],

    [11],
    [Multi-Sensor Data Fusion Readiness. On-board co-registration and fusion of co-manifested sensors (optical, SAR, thermal, lidar); produces analysis-ready fused products that downstream models can ingest directly without ground-based preprocessing.],
    [Atmosphere],
    [Intermediate],

    [12],
    [Model-in-the-Loop Deployment. Ground-to-orbit deployment of MAESMA-discovered classifiers: the process discovery pipeline trains a new edge-inference model (e.g., improved fire detector from residual analysis), quantizes it for VPU/NPU, and uploads it to the satellite for on-board execution. Closes the loop from knowledgebase to orbit.],
    [Fire],
    [Intermediate],
  ),
  caption: [PhiSat-2 on-board AI capabilities catalog (12 entries). Autonomous observation intelligence principles.],
) <tab:phisat2>

#pagebreak()

= Google Research FireBench --- High-Fidelity Wildfire Simulation Dataset <app:firebench>

FireBench (Google Research, 2024) is the largest high-fidelity wildfire simulation dataset, combining observational environmental data with coupled fire--atmosphere large-eddy simulations (Swirl-LM). While not itself a process model codebase, FireBench provides critical validation infrastructure for fire process representations in the MAESMA knowledgebase.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Capability*], [*Family*], [*Role*]),
    [1],
    [Coupled Fire--Atmosphere LES. High-fidelity fire--atmosphere coupled simulations (Swirl-LM) resolving turbulent multiphase combustion, plume dynamics, and fire--wind feedbacks at meter-scale resolution.],
    [Fire],
    [Benchmark],

    [2],
    [Ensemble Fire Evolution Scenarios. Generation of ensembles of fire evolution under varied environmental conditions (wind, terrain, fuel, moisture) for statistical validation.],
    [Fire],
    [Benchmark],

    [3],
    [Observational Environmental Data Integration. Real topography, vegetation, and weather data combined with simulation for realistic fire scenarios.],
    [Fire],
    [Benchmark],

    [4],
    [Multi-Variable Time-Series Statistics. Apache Beam pipeline for computing spatiotemporal statistics (min, max, mean) of all simulation variables across TB-scale datasets.],
    [Fire],
    [Benchmark],

    [5],
    [Fire Spread Rate Validation. Ground-truth ROS fields from coupled LES for validating Rothermel, CFS FBP, and physics-based fire spread models at R1--R3 fidelity.],
    [Fire],
    [Benchmark],

    [6],
    [Plume Dynamics Reference. Resolved buoyant plume structure, entrainment, and lofting for validating parameterized plume models at R2/R3 fidelity.],
    [Atmosphere],
    [Benchmark],

    [7],
    [Turbulent Fire--Wind Interaction. Resolved fire--atmosphere feedback including fire-induced winds, local vorticity, and convective column dynamics for calibrating coupled models.],
    [Fire],
    [Benchmark],

    [8],
    [ML Training Data for Fire Emulators. TB-scale simulation data suitable for training neural operator surrogates (FNO, DeepONet) for fast fire spread emulation.],
    [Fire],
    [Benchmark],
  ),
  caption: [FireBench capabilities catalog (8 entries). FireBench serves as validation infrastructure rather than a process model; all entries are benchmarking resources.],
) <tab:firebench>

#pagebreak()

= MARBL --- Marine Biogeochemistry Library <app:marbl>

MARBL is the ocean biogeochemistry module used in CESM/POP2 and MOM6. It implements the Biogeochemical Elemental Cycling (BEC) model with explicit representation of multiple phytoplankton functional types, zooplankton, dissolved/particulate organic matter, and full nutrient cycling (N, P, Si, Fe). Standalone library that can be coupled to any ocean circulation model.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Phytoplankton Growth. Light- and nutrient-limited growth for multiple functional types (diatoms, small phyto, diazotrophs, coccolithophores).],
    [Biogeochem],
    [Intermediate],

    [2],
    [Nutrient Uptake (Michaelis-Menten). Uptake kinetics for NO₃, NH₄, PO₄, SiO₃, Fe with Michaelis-Menten half-saturation.],
    [Biogeochem],
    [Intermediate],

    [3],
    [Nitrogen Fixation. Diazotroph N₂ fixation controlled by temperature, light, Fe, P availability.],
    [Biogeochem],
    [Intermediate],

    [4],
    [Zooplankton Grazing. Parameterized zooplankton grazing on phytoplankton with prey-switching and sloppy feeding.],
    [Biogeochem],
    [Empirical],

    [5],
    [Phytoplankton Mortality & Aggregation. Non-grazing mortality; aggregation to particulate organic matter.],
    [Biogeochem],
    [Empirical],

    [6],
    [Dissolved Organic Matter Cycling. Production, bacterial remineralization, semi-labile/refractory DOM (C, N, P, Fe).],
    [Biogeochem],
    [Intermediate],

    [7],
    [Particulate Organic Matter Flux. Sinking POM (POC, POP, PON, bSi, CaCO₃) with ballast-effect mineral protection.],
    [Biogeochem],
    [Intermediate],

    [8],
    [Remineralization (Martin Curve). Depth-dependent remineralization of sinking particles following power-law or ballast model.],
    [Biogeochem],
    [Empirical],

    [9],
    [Nitrification. Oxidation of NH₄ to NO₃ as function of light inhibition and temperature.],
    [Biogeochem],
    [Intermediate],

    [10],
    [Denitrification. Water-column and sediment denitrification dependent on O₂ concentration.],
    [Biogeochem],
    [Intermediate],

    [11],
    [Iron Cycle. Dissolved Fe sources (sediment, dust, hydrothermal), scavenging by particles, ligand complexation.],
    [Biogeochem],
    [Intermediate],

    [12],
    [Silicon Cycle. Biogenic silica production by diatoms, dissolution in water column and sediments.],
    [Biogeochem],
    [Intermediate],

    [13],
    [CaCO₃ Production & Dissolution. Calcification by coccolithophores; dissolution below lysocline.],
    [Biogeochem],
    [Intermediate],

    [14],
    [Air-Sea Gas Exchange. CO₂ and O₂ flux across air-sea interface via wind-speed dependent piston velocity.],
    [Biogeochem],
    [Physics-based],

    [15],
    [Carbonate Chemistry. Full ocean carbon chemistry solver (DIC, alkalinity, pH, pCO₂).],
    [Biogeochem],
    [Physics-based],

    [16],
    [Oxygen Dynamics. Dissolved O₂ production (photosynthesis), consumption (respiration, nitrification), air-sea exchange.],
    [Biogeochem],
    [Intermediate],

    [17],
    [Chlorophyll-to-Carbon Ratio. Photoadaptive Chl:C ratio varying with light, nutrients, temperature.],
    [Biogeochem],
    [Intermediate],

    [18],
    [Sediment Flux Coupling. Bottom boundary conditions for nutrient return (Fe, PO₄, NO₃) from sediments.],
    [Biogeochem],
    [Empirical],

    [19],
    [DMS Production. Dimethylsulfide production from phytoplankton, photolysis, bacterial consumption.],
    [Biogeochem],
    [Empirical],

    [20],
    [Light Attenuation. PAR propagation through water column with Chl-dependent attenuation ($K_d$).],
    [Radiation],
    [Intermediate],
  ),
  caption: [MARBL ocean biogeochemistry catalog (20 entries). Primary family: Biogeochem (19), Radiation (1).],
) <tab:marbl>

#pagebreak()

= PISM --- Parallel Ice Sheet Model <app:pism>

PISM is the most widely used open-source ice sheet model. It uses a hybrid shallow ice approximation (SIA) + shallow shelf approximation (SSA) stress balance, thermomechanically coupled, with a sophisticated subglacial hydrology model. Used in IPCC ISMIP6 projections.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Shallow Ice Approximation (SIA). Non-sliding internal deformation ice flow via Glen's law on the horizontal velocity field.],
    [Cryosphere],
    [Physics-based],

    [2],
    [Shallow Shelf Approximation (SSA). Membrane stress balance for ice shelves and fast-flowing ice streams.],
    [Cryosphere],
    [Physics-based],

    [3],
    [Hybrid SIA+SSA Stress Balance. Superposition of SIA and SSA velocities for grounded ice with basal sliding.],
    [Cryosphere],
    [Physics-based],

    [4],
    [Blatter Higher-Order Stress. Optional Blatter-Pattyn first-order Stokes stress balance for steep topography.],
    [Cryosphere],
    [Physics-based],

    [5],
    [Ice Thermodynamics. Enthalpy-based 3D temperature/moisture field in polythermal ice (advection-diffusion).],
    [Cryosphere],
    [Physics-based],

    [6],
    [Basal Sliding (Pseudo-plastic). Pseudo-plastic or Coulomb sliding laws relating basal shear stress to sliding velocity.],
    [Cryosphere],
    [Physics-based],

    [7],
    [Subglacial Hydrology (Distributed). Distributed subglacial water transport (till-layer and linked-cavity models).],
    [Hydrology],
    [Intermediate],

    [8],
    [Subglacial Hydrology (Routing). Subglacial water routing along hydraulic potential gradient.],
    [Hydrology],
    [Physics-based],

    [9],
    [Basal Yield Stress. Till yield stress from subglacial water pressure via Mohr-Coulomb criterion.],
    [Cryosphere],
    [Physics-based],

    [10],
    [Calving (Eigen-Calving). Calving rate proportional to product of principal strain rates at ice front.],
    [Cryosphere],
    [Intermediate],

    [11], [Calving (von Mises). Calving based on tensile stress exceeding threshold.], [Cryosphere], [Intermediate],
    [12],
    [Calving (Thickness). Calving where ice thickness drops below threshold at terminus.],
    [Cryosphere],
    [Empirical],

    [13],
    [Grounding Line Dynamics. Sub-grid grounding line position parameterization; flotation criterion.],
    [Cryosphere],
    [Physics-based],

    [14],
    [Marine Ice Sheet Instability. Flux-driven instability on retrograde bed slopes with buttressing.],
    [Cryosphere],
    [Physics-based],

    [15],
    [Surface Mass Balance (PDD). Positive degree-day temperature-index surface mass balance.],
    [Cryosphere],
    [Empirical],

    [16],
    [Surface Mass Balance (dEBM). Diurnal energy balance melt model for surface mass balance.],
    [Cryosphere],
    [Intermediate],

    [17],
    [Ocean Thermal Forcing. Sub-shelf melt from ocean temperature/salinity (quadratic, PICO, Beckmann-Goosse).],
    [Ocean],
    [Intermediate],

    [18],
    [PICO Ocean Box Model. Potsdam Ice-shelf Cavity mOdel: box model of sub-ice-shelf ocean circulation and melting.],
    [Ocean],
    [Intermediate],

    [19],
    [Ice Rheology (Glen's Flow Law). Temperature-dependent viscosity via Glen's power law ($n=3$) with enhancement factor.],
    [Cryosphere],
    [Physics-based],

    [20],
    [Bed Deformation (Elastic Lithosphere). Glacial isostatic adjustment: elastic lithosphere, relaxing asthenosphere (ELRA/Lingle-Clark).],
    [Geology],
    [Physics-based],

    [21],
    [Fracture Mechanics. Fracture density field advected with ice flow; damage mechanics for crevassing.],
    [Cryosphere],
    [Intermediate],

    [22], [Age Tracking. Advection of ice age as a tracer through the ice body.], [Cryosphere], [Physics-based],
    [23],
    [Geothermal Heat Flux. Spatially variable geothermal heat flux as basal boundary condition.],
    [Geology],
    [Empirical],
  ),
  caption: [PISM ice sheet dynamics catalog (23 entries). Primary families: Cryosphere (17), Hydrology (2), Ocean (2), Geology (2).],
) <tab:pism>

#pagebreak()

= OGGM --- Open Global Glacier Model <app:oggm>

OGGM is a modular glacier evolution model designed for global-scale applications. It uses flowline dynamics, temperature-index mass balance, and automated glacier inventory processing. Used in GlacierMIP and IPCC AR6.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Glacier Outline Processing. Automated extraction of glacier outlines, topography, and flowlines from RGI inventory and DEMs.],
    [Cryosphere],
    [Intermediate],

    [2],
    [Centerline/Flowline Extraction. Geometric extraction of glacier centerlines and flowlines from surface topography.],
    [Cryosphere],
    [Intermediate],

    [3],
    [Mass Balance (Temperature-Index). Monthly temperature-index mass balance model with precipitation correction and temperature sensitivity.],
    [Cryosphere],
    [Empirical],

    [4],
    [Mass Balance (PyGEM). Coupling to PyGEM energy-balance mass balance model for higher fidelity.],
    [Cryosphere],
    [Intermediate],

    [5],
    [Flowline Ice Dynamics (SIA). Shallow ice approximation along flowlines with trapezoidal cross-section.],
    [Cryosphere],
    [Physics-based],

    [6],
    [Ice Thickness Inversion. Inversion of surface velocity/slope for ice thickness using mass conservation (Farinotti et al.).],
    [Cryosphere],
    [Intermediate],

    [7],
    [Bed Topography Estimation. Estimation of glacier bed from surface topography and ice thickness inversion.],
    [Cryosphere],
    [Intermediate],

    [8],
    [Glacier Length/Area/Volume Evolution. Forward time integration of glacier geometry responding to climate forcing.],
    [Cryosphere],
    [Intermediate],

    [9],
    [Frontal Ablation (Tidewater). Calving flux for tidewater glaciers as function of water depth and ice thickness.],
    [Cryosphere],
    [Empirical],

    [10],
    [Climate Downscaling. Temperature lapse rate correction and precipitation scaling for glacier elevation.],
    [Atmosphere],
    [Empirical],

    [11],
    [Dynamical Spinup. Iterative spinup to match observed glacier state at reference date.],
    [Cryosphere],
    [Intermediate],

    [12],
    [Glacier Retreat/Advance. Terminus position changes and area/volume evolution under transient climate.],
    [Cryosphere],
    [Intermediate],

    [13], [Equilibrium Line Altitude. Computation of ELA from mass balance profile.], [Cryosphere], [Empirical],
    [14], [Surface Velocity. Surface velocity estimation from SIA flow law.], [Cryosphere], [Physics-based],
    [15],
    [Accumulation/Ablation Zones. Spatial delineation of accumulation and ablation zones from mass balance profile.],
    [Cryosphere],
    [Empirical],
  ),
  caption: [OGGM glacier dynamics catalog (15 entries). Primary families: Cryosphere (14), Atmosphere (1).],
) <tab:oggm>

#pagebreak()

= APSIM --- Agricultural Production Systems Simulator <app:apsim>

APSIM is the premier farming-systems simulator, modeling the soil--plant--atmosphere continuum with 30+ crop species. It simulates crop growth, soil water/nutrient dynamics, residue management, erosion, and farm economics. Widely used in agricultural research globally.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Crop Phenology. Temperature-driven (thermal time / GDD) phenological development with photoperiod and vernalization.],
    [Ecology],
    [Intermediate],

    [2],
    [Leaf Area Development. Leaf area expansion based on thermal time, leaf appearance rate, node number, and water/N stress.],
    [Ecology],
    [Intermediate],

    [3],
    [Canopy Photosynthesis (RUE). Radiation use efficiency: biomass = intercepted PAR × RUE, with stress modifiers.],
    [Biogeochem],
    [Empirical],

    [4],
    [Canopy Photosynthesis (DCaPST). Detailed coupled canopy photosynthesis-stomatal conductance (Farquhar + Ball-Berry).],
    [Biogeochem],
    [Physics-based],

    [5],
    [Biomass Partitioning. Daily allocation of assimilate to roots, stems, leaves, grain according to growth stage.],
    [Ecology],
    [Intermediate],

    [6],
    [Grain Filling. Grain yield from source (assimilate supply) and sink (grain number × potential weight).],
    [Ecology],
    [Intermediate],

    [7],
    [Root Growth. Root depth advance with thermal time; root length density distribution with depth.],
    [Ecology],
    [Intermediate],

    [8],
    [Crop Water Stress. Ratio of actual to potential transpiration modifying RUE, leaf expansion, phenology.],
    [Hydrology],
    [Intermediate],

    [9],
    [Crop Nitrogen Demand/Uptake. N demand from critical N concentration functions; uptake by mass flow + diffusion.],
    [Biogeochem],
    [Intermediate],

    [10],
    [SoilWat (Water Balance). Cascading-bucket soil water: infiltration (SCS curve number or Green-Ampt), redistribution, drainage, evaporation.],
    [Hydrology],
    [Intermediate],

    [11],
    [SWIM (Richards). Mechanistic Richards equation 1-D soil water balance (optional).],
    [Hydrology],
    [Physics-based],

    [12], [Soil Evaporation. Two-stage evaporation from soil surface (Ritchie approach).], [Hydrology], [Empirical],
    [13],
    [Soil Organic Matter (SoilN). Multi-pool SOM model (FBIOM/FINERT + active/humic) with C:N ratios.],
    [Biogeochem],
    [Intermediate],

    [14],
    [Soil Nitrogen Transformations. Mineralization, nitrification, denitrification, urea hydrolysis, NH₃ volatilization.],
    [Biogeochem],
    [Intermediate],

    [15], [Soil Phosphorus. Labile/sorbed/organic P pools and transformations.], [Biogeochem], [Intermediate],
    [16],
    [Surface Organic Matter (Residues). Crop residue decomposition on soil surface; C/N release; mulch effects on evaporation.],
    [Biogeochem],
    [Intermediate],

    [17], [Erosion (USLE). Soil erosion via Universal Soil Loss Equation variants.], [Geomorphology], [Empirical],
    [18],
    [Soil Temperature. Soil temperature profile from air temperature using EPIC-based approach.],
    [Hydrology],
    [Empirical],

    [19],
    [Tillage. Tillage operations modifying soil structure, residue incorporation, bulk density.],
    [Human Systems],
    [Empirical],

    [20],
    [Fertilizer Application. Timing, rate, and placement of mineral fertilizers (N, P, K, S).],
    [Human Systems],
    [Empirical],

    [21],
    [Irrigation Scheduling. Rule-based and deficit-triggered irrigation management.],
    [Human Systems],
    [Empirical],

    [22],
    [Crop Rotation / Intercropping. Multi-crop sequences, relay planting, intercropping competition for light/water/N.],
    [Ecology],
    [Intermediate],

    [23], [Pasture/Grazing. Pasture growth, animal intake, trampling, excreta return.], [Ecology], [Intermediate],
    [24],
    [Microclimate. Surface energy balance, canopy temperature, humidity within crop canopy.],
    [Atmosphere],
    [Intermediate],

    [25],
    [Economic/Management Rules. Farm-level management rules, economics, decision-making scripts.],
    [Human Systems],
    [Empirical],
  ),
  caption: [APSIM agricultural systems catalog (25 entries). Primary families: Ecology (7), Biogeochem (7), Hydrology (5), Human Systems (4), Geomorphology (1), Atmosphere (1).],
) <tab:apsim>

#pagebreak()

= DSSAT --- Decision Support System for Agrotechnology Transfer <app:dssat>

DSSAT is the classic crop simulation platform containing 30+ crop models (CERES-Maize, CERES-Wheat, CROPGRO-Soybean, etc.) with soil water/nitrogen/phosphorus modules. Heavily used in agricultural climate impact assessment.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [CERES Crop Growth. Temperature-driven development (thermal time), light interception (Beer's law), RUE-based biomass production.],
    [Ecology],
    [Intermediate],

    [2],
    [CROPGRO Engine. Mechanistic hourly canopy photosynthesis (Boote/Pickering), respiration, C balance for legumes/vegetables.],
    [Biogeochem],
    [Intermediate],

    [3],
    [CROPSIM Engine. Wheat/barley growth with detailed tiller dynamics and kernel number determination.],
    [Ecology],
    [Intermediate],

    [4],
    [Phenology (CERES). Phase-based development via thermal time, photoperiod sensitivity, vernalization.],
    [Ecology],
    [Intermediate],

    [5],
    [Leaf Area Growth. Leaf area expansion driven by temperature and leaf number; senescence from age/stress.],
    [Ecology],
    [Intermediate],

    [6],
    [Grain/Yield Formation. Source-sink grain filling: kernel number × kernel growth rate; harvest index.],
    [Ecology],
    [Intermediate],

    [7],
    [Root Growth & Water Uptake. Root elongation with depth; water extraction from soil layers via root length density.],
    [Ecology],
    [Intermediate],

    [8],
    [Nitrogen Demand & Uptake. Crop N demand from critical/minimum N concentrations; root uptake from soil NH₄/NO₃.],
    [Biogeochem],
    [Intermediate],

    [9], [Phosphorus Uptake. Crop P demand and uptake from labile soil P pools.], [Biogeochem], [Intermediate],
    [10],
    [Carbon Balance. Plant-level carbon balance: photosynthesis, maintenance/growth respiration, senescence.],
    [Biogeochem],
    [Intermediate],

    [11],
    [Soil Water (Ritchie). Tipping-bucket: rainfall, infiltration (SCS CN), ET (Priestley-Taylor or FAO-56), drainage.],
    [Hydrology],
    [Intermediate],

    [12],
    [Soil Evaporation-Transpiration. Two-stage bare soil evaporation; transpiration limited by root distribution and soil water.],
    [Hydrology],
    [Intermediate],

    [13], [Soil Temperature. Soil temperature from surface energy balance (EPIC approach).], [Hydrology], [Empirical],
    [14],
    [Soil Organic C/N (CENTURY). CENTURY-based SOM model (metabolic/structural litter, active/slow/passive pools).],
    [Biogeochem],
    [Intermediate],

    [15], [Soil Organic C/N (Godwin). Original DSSAT 2-pool SOM decomposition model.], [Biogeochem], [Empirical],
    [16],
    [Nitrogen Transformations. Mineralization, immobilization, nitrification, denitrification, leaching, volatilization.],
    [Biogeochem],
    [Intermediate],

    [17],
    [Phosphorus Dynamics. Labile/active/stable inorganic P pools; organic P mineralization.],
    [Biogeochem],
    [Intermediate],

    [18], [Potassium Dynamics. K uptake, fixation, exchange, leaching.], [Biogeochem], [Intermediate],
    [19], [Surface Runoff. SCS Curve Number method for surface runoff.], [Hydrology], [Empirical],
    [20], [Flood Management (Rice). Ponded water balance for paddy rice (CERES-Rice).], [Hydrology], [Intermediate],
    [21],
    [Management Operations. Planting, harvest, fertilization, irrigation, tillage, organic amendments scheduling.],
    [Human Systems],
    [Empirical],

    [22],
    [Seasonal Analysis. Multi-year, multi-location analysis with historical/synthetic weather.],
    [Human Systems],
    [Empirical],

    [23],
    [Weather Generator (WGEN). Stochastic daily weather generation for scenario analysis.],
    [Atmosphere],
    [Empirical],

    [24],
    [Pest/Disease Coupling. Interface for coupling pest and disease models to crop growth.],
    [Ecology],
    [Empirical],
  ),
  caption: [DSSAT crop modeling catalog (24 entries). Primary families: Ecology (7), Biogeochem (9), Hydrology (5), Human Systems (2), Atmosphere (1).],
) <tab:dssat>

#pagebreak()

= CryoGrid --- Permafrost and Frozen Ground Dynamics <app:cryogrid>

CryoGrid is a purpose-built permafrost model solving coupled heat transfer and water/ice dynamics in frozen/unfrozen ground. It excels at representing periglacial processes, ground ice dynamics, thermokarst, and snow--ground coupling with much higher vertical resolution and process detail than CLM or ORCHIDEE permafrost modules.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Heat Conduction (Fourier). 1D heat conduction through soil/rock column with depth-dependent thermal properties.],
    [Cryosphere],
    [Physics-based],

    [2],
    [Freeze-Thaw (Enthalpy). Phase change of water/ice via enthalpy formulation; latent heat effects on temperature.],
    [Cryosphere],
    [Physics-based],

    [3],
    [Unfrozen Water Content. Soil freezing characteristic curve: unfrozen water as function of temperature below 0~°C.],
    [Cryosphere],
    [Physics-based],

    [4],
    [Active Layer Dynamics. Seasonal thaw depth computation; interannual active layer thickness variability.],
    [Cryosphere],
    [Physics-based],

    [5],
    [Permafrost Table Migration. Long-term permafrost aggradation/degradation under changing climate.],
    [Cryosphere],
    [Physics-based],

    [6],
    [Excess Ice / Ground Ice. Representation of massive ground ice bodies, ice wedges, and their thaw-settlement.],
    [Cryosphere],
    [Intermediate],

    [7],
    [Thermokarst / Subsidence. Ground subsidence from thawing of ice-rich permafrost; talik formation.],
    [Cryosphere],
    [Intermediate],

    [8],
    [Snow (Multi-layer). Multi-layer snowpack: heat conduction, compaction, metamorphism, water percolation, refreezing.],
    [Cryosphere],
    [Physics-based],

    [9],
    [Snow--Ground Coupling. Thermal coupling between snowpack and ground; insulation effects.],
    [Cryosphere],
    [Physics-based],

    [10],
    [Soil Thermal Properties. Thermal conductivity and heat capacity as functions of mineral/organic content, water/ice saturation.],
    [Cryosphere],
    [Physics-based],

    [11],
    [Organic Soil Layers. Variable-thickness organic horizons (peat, moss) with distinct thermal/hydraulic properties.],
    [Ecology],
    [Intermediate],

    [12],
    [Soil Water (Richards). Variably saturated water flow via Richards equation (unfrozen zone).],
    [Hydrology],
    [Physics-based],

    [13],
    [Evapotranspiration. Surface energy balance-driven ET with soil evaporation resistance.],
    [Hydrology],
    [Intermediate],

    [14],
    [Surface Energy Balance. Full surface energy balance: shortwave, longwave, sensible/latent heat, ground heat flux.],
    [Atmosphere],
    [Physics-based],

    [15],
    [Lateral Heat Flow. 2D/3D lateral heat transport for thermal erosion, lake--permafrost interaction.],
    [Cryosphere],
    [Intermediate],

    [16],
    [Lake--Permafrost Interaction. Talik formation beneath thermokarst lakes; lake ice growth/decay.],
    [Cryosphere],
    [Intermediate],

    [17], [Salt Transport. Coupled salt/heat transport for saline permafrost (cryopegs).], [Cryosphere], [Intermediate],
    [18], [Bedrock Thermal. Deep geothermal gradient and bedrock heat conduction.], [Geology], [Physics-based],
  ),
  caption: [CryoGrid permafrost catalog (18 entries). Primary families: Cryosphere (13), Hydrology (2), Ecology (1), Atmosphere (1), Geology (1).],
) <tab:cryogrid>

#pagebreak()

= XBeach --- Coastal Morphodynamics <app:xbeach>

XBeach is the leading open-source nearshore morphodynamic model. It resolves short-wave groups, infragravity waves, wave-driven currents, sediment transport, and beach/dune erosion during storms. Used globally for coastal hazard assessment.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Short-wave Energy Propagation. Wave action balance equation: shoaling, refraction, breaking (Baldock/Roelvink), directional spreading.],
    [Ocean],
    [Physics-based],

    [2],
    [Infragravity Wave Generation. Bound long wave forcing by short-wave groups; free infragravity wave propagation.],
    [Ocean],
    [Physics-based],

    [3], [Wave Breaking. Depth-limited wave breaking dissipation (roller model).], [Ocean], [Physics-based],
    [4],
    [Roller Energy. Surface roller energy balance: production from breaking, dissipation to bore turbulence.],
    [Ocean],
    [Physics-based],

    [5], [Wave Setup / Setdown. Radiation stress gradients driving mean water level changes.], [Ocean], [Physics-based],
    [6],
    [Depth-Averaged Currents. 2DH shallow water equations with wave forcing (radiation stress + roller).],
    [Ocean],
    [Physics-based],

    [7], [Longshore Currents. Wave-driven longshore currents from oblique wave incidence.], [Ocean], [Physics-based],
    [8],
    [Cross-shore Currents (Undertow). Return flow / undertow from wave mass transport and roller.],
    [Ocean],
    [Physics-based],

    [9],
    [Bed Shear Stress. Combined wave-current bed shear stress (Soulsby-van Rijn, Van Thiel-Van Rijn).],
    [Geomorphology],
    [Physics-based],

    [10],
    [Sediment Transport (Bed Load). Bed-load transport from combined wave-current forcing.],
    [Geomorphology],
    [Intermediate],

    [11],
    [Sediment Transport (Suspended). Depth-averaged suspended sediment advection-diffusion with wave-stirring source.],
    [Geomorphology],
    [Physics-based],

    [12],
    [Sediment Avalanching. Dry/wet slope avalanching when bed slope exceeds critical angle of repose.],
    [Geomorphology],
    [Empirical],

    [13],
    [Bed Level Update (Exner). Morphological bed evolution via sediment continuity (Exner equation).],
    [Geomorphology],
    [Physics-based],

    [14],
    [Morphological Acceleration. Morfac time-scaling for accelerated morphological evolution.],
    [Geomorphology],
    [Empirical],

    [15],
    [Dune Erosion. Dune face retreat from wave impact, slumping, and avalanching.],
    [Geomorphology],
    [Intermediate],

    [16],
    [Overwash & Breaching. Barrier island overwash and breach formation/evolution.],
    [Geomorphology],
    [Intermediate],

    [17],
    [Groundwater Flow (1D). Simple groundwater table dynamics within beach/dune (affects infiltration/exfiltration).],
    [Hydrology],
    [Intermediate],

    [18], [Ship-wave Effects. Wake from passing vessels as boundary forcing.], [Ocean], [Intermediate],
    [19], [Vegetation Drag. Wave and flow attenuation by vegetation (mangroves, seagrass).], [Ecology], [Intermediate],
    [20],
    [Storm Surge Coupling. Boundary conditions from tidal/surge models (via ADCIRC, Delft3D).],
    [Ocean],
    [Physics-based],
  ),
  caption: [XBeach coastal morphodynamics catalog (20 entries). Primary families: Ocean (10), Geomorphology (8), Hydrology (1), Ecology (1).],
) <tab:xbeach>

#pagebreak()

= SURFEX/TEB --- Urban Canopy and Land Surface <app:surfex>

SURFEX is Météo-France's externalized land surface platform containing TEB (Town Energy Balance), ISBA (land surface), and lake/ocean surface schemes. TEB is the leading physics-based urban canopy model resolving the energy balance of streets, walls, roofs, and gardens within urban canyons.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Urban Canyon Radiation. Multiple reflections of shortwave and longwave radiation between road, walls, and sky within street canyon.],
    [Radiation],
    [Physics-based],

    [2],
    [Wall Energy Balance. Conductive heat transfer through building walls (multi-layer); external/internal surface temperatures.],
    [Atmosphere],
    [Physics-based],

    [3],
    [Roof Energy Balance. Roof surface energy balance: solar absorption, longwave emission, sensible/latent heat, conduction.],
    [Atmosphere],
    [Physics-based],

    [4],
    [Road Energy Balance. Road surface energy balance with thermal storage in pavement layers.],
    [Atmosphere],
    [Physics-based],

    [5],
    [Canyon Air Temperature. Energy balance of canyon air space; mixing with above-roof air.],
    [Atmosphere],
    [Physics-based],

    [6],
    [Anthropogenic Heat. Waste heat from building HVAC, industrial activity, and traffic.],
    [Human Systems],
    [Intermediate],

    [7],
    [Building Energy Model (BEM). Indoor energy demand: heating, cooling, ventilation, internal heat gains.],
    [Human Systems],
    [Intermediate],

    [8],
    [Urban Hydrology. Rainfall interception on roofs, road runoff, stormwater drainage, garden infiltration.],
    [Hydrology],
    [Intermediate],

    [9],
    [Urban Vegetation (Garden). Garden soil-vegetation energy/water balance within urban tiles.],
    [Ecology],
    [Intermediate],

    [10],
    [Street Trees. Radiative and evapotranspirative effects of street trees in canyon geometry.],
    [Ecology],
    [Intermediate],

    [11],
    [Green/Cool Roofs. Modified roof properties for vegetated and high-albedo roofing.],
    [Human Systems],
    [Intermediate],

    [12],
    [Snow on Urban Surfaces. Snow accumulation and melt on roofs, roads, and walls with plowing management.],
    [Cryosphere],
    [Intermediate],

    [13],
    [Urban Heat Island. Emergent UHI from canyon trapping, reduced ventilation, thermal mass, and anthropogenic heat.],
    [Atmosphere],
    [Physics-based],

    [14],
    [ISBA Land Surface. ISBA soil-vegetation-atmosphere scheme: soil hydrology (diffusion/force-restore), photosynthesis, snow.],
    [Hydrology],
    [Physics-based],

    [15],
    [ISBA Carbon Cycle. Ecosystem respiration, NEE, soil C decomposition within ISBA component.],
    [Biogeochem],
    [Intermediate],

    [16],
    [ISBA Multi-Energy Balance. Separate energy balance for vegetation canopy, bare soil, snow under canopy.],
    [Atmosphere],
    [Physics-based],

    [17], [Flake Lake Model. 1-D lake thermal model (FLake) for inland water bodies.], [Hydrology], [Intermediate],
    [18], [Sea Surface (ECUME). Air-sea flux parameterization over ocean surfaces.], [Ocean], [Intermediate],
  ),
  caption: [SURFEX/TEB urban and land surface catalog (18 entries). Primary families: Atmosphere (6), Human Systems (3), Hydrology (3), Ecology (2), Radiation (1), Biogeochem (1), Cryosphere (1), Ocean (1).],
) <tab:surfex>

#pagebreak()

= EwE --- Ecopath with Ecosim <app:ewe>

Ecopath with Ecosim is the world's most used marine ecosystem modeling platform (applied in 200+ countries/ecosystems). It consists of Ecopath (mass-balance snapshot), Ecosim (temporal dynamics), and Ecospace (spatial). It represents full food webs from phytoplankton to apex predators, including fisheries and coral reef ecosystems.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Mass-Balance (Ecopath). Static mass-balance of ecosystem: production, consumption, respiration, egestion for all functional groups.],
    [Ecology],
    [Intermediate],

    [2],
    [Trophic Flow Network. Quantified trophic interactions (diet composition matrix) across all functional groups.],
    [Ecology],
    [Intermediate],

    [3],
    [Production/Biomass Ratios. Production rates from empirical relationships with body size, temperature, mortality.],
    [Ecology],
    [Empirical],

    [4],
    [Consumption (Foraging Arena). Ecosim predator-prey dynamics: foraging arena theory mediating prey vulnerability.],
    [Ecology],
    [Intermediate],

    [5],
    [Predator-Prey Dynamics. Time-dynamic biomass changes from differential equations: growth, predation, fishing mortality.],
    [Ecology],
    [Intermediate],

    [6],
    [Primary Production Forcing. Phytoplankton production driven by nutrient concentration and environmental forcing.],
    [Biogeochem],
    [Empirical],

    [7],
    [Fisheries Exploitation. Fishing mortality by fleet, gear type, effort dynamics, discards.],
    [Human Systems],
    [Intermediate],

    [8],
    [Marine Protected Areas. Spatial closures with differential habitat quality and movement rates.],
    [Ecology],
    [Intermediate],

    [9],
    [Spatial Dynamics (Ecospace). 2D grid-based spatial representation with habitat preferences, movement, dispersal.],
    [Ecology],
    [Intermediate],

    [10],
    [Habitat Capacity. Habitat-mediated carrying capacity: coral cover, seagrass, depth, temperature effects.],
    [Ecology],
    [Intermediate],

    [11],
    [Nutrient Loading. External nutrient forcing driving eutrophication and primary production changes.],
    [Biogeochem],
    [Empirical],

    [12],
    [Climate Forcing Functions. Temperature, pH, oxygen forcing driving physiological rates and distribution shifts.],
    [Atmosphere],
    [Empirical],

    [13],
    [Ontogenetic Diet Shifts. Size/age-based changes in diet composition as organisms grow.],
    [Ecology],
    [Intermediate],

    [14],
    [Multi-Stanza Groups. Life-stage (juvenile/adult) representation with recruitment dynamics.],
    [Ecology],
    [Intermediate],

    [15],
    [Detritus Pathways. Dead organic matter cycling: detritus production, microbial decomposition, detritivore consumption.],
    [Biogeochem],
    [Intermediate],

    [16],
    [Coral-Algae Competition. Competition dynamics between coral and macroalgae mediated by herbivory pressure.],
    [Ecology],
    [Intermediate],

    [17],
    [Network Analysis. Ecosystem network metrics: trophic level, omnivory, flow diversity, system maturity.],
    [Ecology],
    [Intermediate],
  ),
  caption: [EwE marine ecosystem catalog (17 entries). Primary families: Ecology (12), Biogeochem (3), Human Systems (1), Atmosphere (1).],
) <tab:ewe>

#pagebreak()

= CARMA --- Community Aerosol and Radiation Model for Atmospheres <app:carma>

CARMA is a bin-resolved (sectional) aerosol microphysics model that explicitly tracks the aerosol size distribution across 20--40 bins. Unlike modal approaches (MAM in CESM), CARMA resolves the full size distribution evolution from nucleation to sedimentation. Used for stratospheric aerosols, volcanic plumes, noctilucent clouds, wildfire smoke, and mineral dust.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Homogeneous Nucleation. Classical nucleation theory for new particle formation (H₂SO₄-H₂O binary, ternary with NH₃).],
    [Atmosphere],
    [Physics-based],

    [2],
    [Heterogeneous Nucleation. Ice nucleation on existing particles (deposition, immersion, contact modes).],
    [Atmosphere],
    [Physics-based],

    [3],
    [Condensation/Evaporation. Mass transfer to/from particles: growth by condensation of supersaturated vapor.],
    [Atmosphere],
    [Physics-based],

    [4],
    [Coagulation. Particle-particle coagulation (Brownian, gravitational, turbulent collection) updating size distribution.],
    [Atmosphere],
    [Physics-based],

    [5],
    [Sedimentation. Gravitational settling velocity as function of particle size, density, altitude (Stokes-Cunningham).],
    [Atmosphere],
    [Physics-based],

    [6],
    [Brownian Diffusion. Vertical mixing of aerosol by Brownian diffusion (small particles).],
    [Atmosphere],
    [Physics-based],

    [7],
    [Aerosol Thermodynamics. Equilibrium partitioning of semi-volatile species between gas and aerosol phases.],
    [Atmosphere],
    [Physics-based],

    [8],
    [Particle Growth by Coagulation. Size distribution evolution from coagulation kernel integration over size bins.],
    [Atmosphere],
    [Physics-based],

    [9],
    [Sulfate Chemistry. H₂SO₄ production from SO₂ oxidation (gas-phase and aqueous); condensation onto particles.],
    [Atmosphere],
    [Physics-based],

    [10],
    [Meteoric Dust Ablation. Meteoric material injection into mesosphere; size-dependent ablation.],
    [Atmosphere],
    [Intermediate],

    [11],
    [Volcanic Injection. Sulfate aerosol formation from volcanic SO₂ injection at stratospheric altitudes.],
    [Atmosphere],
    [Intermediate],

    [12],
    [Noctilucent/PMC Clouds. Ice particle formation at mesopause from water vapor deposition on meteor smoke nuclei.],
    [Atmosphere],
    [Physics-based],

    [13],
    [Wildfire Smoke Aging. Brown carbon absorption evolution, coagulation of smoke particles in aging plumes.],
    [Atmosphere],
    [Intermediate],

    [14],
    [Aerosol Optical Properties. Mie theory: extinction, single scattering albedo, asymmetry parameter per bin.],
    [Radiation],
    [Physics-based],

    [15],
    [Radiative Heating Rates. Aerosol-induced shortwave/longwave heating rate perturbations.],
    [Radiation],
    [Physics-based],

    [16], [Dry Deposition. Size-dependent dry deposition velocity to surface.], [Atmosphere], [Intermediate],
    [17],
    [Wet Scavenging. In-cloud and below-cloud scavenging of aerosol by precipitation.],
    [Atmosphere],
    [Intermediate],

    [18],
    [Mixed Composition Particles. Internal mixing of multiple aerosol components within size bins (sulfate + organics + dust).],
    [Atmosphere],
    [Intermediate],
  ),
  caption: [CARMA aerosol microphysics catalog (18 entries). Primary families: Atmosphere (16), Radiation (2).],
) <tab:carma>

#pagebreak()

= ATS --- Advanced Terrestrial Simulator <app:ats>

ATS is a multiphysics code built on the Amanzi framework that solves coupled surface/subsurface thermal hydrology with a focus on permafrost environments. It integrates surface energy balance, snow physics, Richards equation in variably-saturated frozen soils, and overland flow. Specifically designed for Arctic/subarctic environments where soil thermal-hydrological coupling is critical.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Richards Equation (Variably Saturated). 3D variably-saturated flow via Richards equation with nonlinear permeability (van Genuchten).],
    [Hydrology],
    [Physics-based],

    [2],
    [Frozen Soil Hydraulics. Modified relative permeability for ice-bearing soils; impedance of water flow by ice.],
    [Cryosphere],
    [Physics-based],

    [3],
    [Coupled Energy Equation. 3D subsurface energy transport: conduction + advection by water; enthalpy with phase change.],
    [Cryosphere],
    [Physics-based],

    [4],
    [Freeze-Thaw Dynamics. Coupled thermal-hydrological freeze-thaw: cryosuction, ice lens formation, frost heave potential.],
    [Cryosphere],
    [Physics-based],

    [5],
    [Surface Energy Balance. Full radiation + turbulent + ground heat flux balance; skin temperature solver.],
    [Atmosphere],
    [Physics-based],

    [6],
    [Snow Physics. Multi-layer snow: accumulation, densification, wind redistribution, melt, sublimation.],
    [Cryosphere],
    [Physics-based],

    [7],
    [Overland Flow (Diffusion Wave). 2D diffusion wave overland flow coupled to subsurface via infiltration/exfiltration.],
    [Hydrology],
    [Physics-based],

    [8],
    [Microtopography / Ponding. Depression storage, rill connectivity, microtopographic flow routing.],
    [Hydrology],
    [Intermediate],

    [9],
    [Evapotranspiration (Priestley-Taylor). ET partitioning: transpiration, bare soil evaporation, canopy evaporation.],
    [Hydrology],
    [Intermediate],

    [10],
    [Canopy Water/Energy. Simple canopy interception, throughfall, canopy radiation budget.],
    [Ecology],
    [Intermediate],

    [11],
    [Soil Thermal Properties. Temperature- and moisture-dependent thermal conductivity (Kersten/Johansen) and heat capacity.],
    [Hydrology],
    [Physics-based],

    [12],
    [Organic Soil Properties. Peat/organic horizon hydraulic and thermal properties distinct from mineral soil.],
    [Ecology],
    [Intermediate],

    [13],
    [Deformable Soil Column. Soil column deformation from ice thaw/frost heave (subsidence/heave).],
    [Cryosphere],
    [Intermediate],

    [14],
    [Permafrost Distribution. Simulated spatial distribution of permafrost from coupled energy/water balance.],
    [Cryosphere],
    [Physics-based],

    [15],
    [Wetland Hydrology. Saturated/inundated conditions with water table dynamics above/below surface.],
    [Hydrology],
    [Physics-based],

    [16],
    [Biogeochemistry Coupling. Interface for coupling to PFLOTRAN for reactive transport.],
    [Biogeochem],
    [Intermediate],

    [17],
    [Mesh Management (Unstructured). Flexible unstructured mesh for complex topography and watershed geometry.],
    [Hydrology],
    [Physics-based],

    [18],
    [Lateral Subsurface Flow. 3D lateral water movement through heterogeneous subsurface.],
    [Hydrology],
    [Physics-based],
  ),
  caption: [ATS integrated hydrology catalog (18 entries). Primary families: Hydrology (8), Cryosphere (6), Ecology (2), Atmosphere (1), Biogeochem (1).],
) <tab:ats>

#pagebreak()

= BFM --- Biogeochemical Flux Model <app:bfm>

BFM is a comprehensive marine biogeochemistry model resolving lower trophic levels with explicit representation of multiple plankton functional types, bacteria, detritus, and full C/N/P/Si/Fe/O cycling. Distinguished from MARBL by its European heritage, NEMO coupling, and detailed microbial loop.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [Phytoplankton Photosynthesis. Light-limited, nutrient-limited carbon fixation for 4 types (diatoms, flagellates, picophyto, dinoflagellates).],
    [Biogeochem],
    [Intermediate],

    [2],
    [Phytoplankton Nutrient Uptake. Multi-nutrient (N, P, Si, Fe) uptake with Droop quota dynamics (luxury uptake, internal quotas).],
    [Biogeochem],
    [Intermediate],

    [3],
    [Chlorophyll Synthesis. Photoacclimation: dynamic Chl:C ratio based on light and nutrient status.],
    [Biogeochem],
    [Intermediate],

    [4],
    [Zooplankton Grazing. Grazing by meso- and microzooplankton with prey preferences and threshold feeding.],
    [Biogeochem],
    [Intermediate],

    [5],
    [Bacterial Dynamics. Explicit heterotrophic bacteria: DOC uptake, remineralization, nutrient uptake/competition.],
    [Biogeochem],
    [Intermediate],

    [6],
    [Microbial Loop. Bacteria-DOM-flagellate interactions; viral lysis pathway (implicit).],
    [Biogeochem],
    [Intermediate],

    [7],
    [Dissolved Organic Matter. Semi-labile and refractory DOM pools (C, N, P); bacterial utilization and abiotic degradation.],
    [Biogeochem],
    [Intermediate],

    [8],
    [Particulate Organic Matter. Sinking POM: production from mortality/egestion; depth-dependent remineralization.],
    [Biogeochem],
    [Intermediate],

    [9],
    [Dissolved Inorganic Carbon. DIC dynamics: photosynthesis, respiration, air-sea exchange, carbonate chemistry.],
    [Biogeochem],
    [Physics-based],

    [10],
    [Alkalinity. Total alkalinity budget: calcification, CaCO₃ dissolution, nutrient uptake/release.],
    [Biogeochem],
    [Physics-based],

    [11],
    [Oxygen Dynamics. O₂ production/consumption; air-sea exchange; suboxic/anoxic transitions.],
    [Biogeochem],
    [Intermediate],

    [12],
    [Nitrogen Cycle (Full). NO₃, NH₄, NO₂ pools; nitrification, denitrification (water column + sediment), N₂ fixation.],
    [Biogeochem],
    [Intermediate],

    [13],
    [Phosphorus Cycle. PO₄ uptake/remineralization; luxury uptake with internal P quota.],
    [Biogeochem],
    [Intermediate],

    [14], [Silicon Cycle. Silicic acid uptake by diatoms; biogenic silica dissolution.], [Biogeochem], [Intermediate],
    [15],
    [Iron Cycle. Dissolved Fe: dust input, scavenging, biological uptake, ligand complexation.],
    [Biogeochem],
    [Intermediate],

    [16],
    [Calcification / CaCO₃. Calcite/aragonite production, dissolution below saturation horizon, ballast effect.],
    [Biogeochem],
    [Intermediate],

    [17],
    [Benthic-Pelagic Coupling. Organic matter deposition to sediments; nutrient return flux to water column.],
    [Biogeochem],
    [Intermediate],

    [18],
    [Light Penetration. PAR attenuation through water column (self-shading by Chl + CDOM + detritus).],
    [Radiation],
    [Intermediate],

    [19],
    [Air-Sea CO₂ Exchange. CO₂ flux via wind-speed dependent piston velocity; surface pCO₂ from carbonate system.],
    [Biogeochem],
    [Physics-based],

    [20],
    [N₂O / DMS Production. Trace gas cycling: N₂O from nitrification/denitrification; DMS from DMSP cleavage.],
    [Biogeochem],
    [Empirical],

    [21],
    [Sea-Ice Biogeochemistry. Brine channel biology; ice algae production; nutrient release upon melt.],
    [Cryosphere],
    [Intermediate],

    [22],
    [River Input. Riverine dissolved/particulate nutrient and carbon inputs to coastal zones.],
    [Biogeochem],
    [Empirical],
  ),
  caption: [BFM marine biogeochemistry catalog (22 entries). Primary families: Biogeochem (20), Radiation (1), Cryosphere (1).],
) <tab:bfm>

#pagebreak()

= MAESTRA/MAESTRO --- 3D Individual-Tree Canopy Model <app:maestra>

MAESTRA (Multi-Array Evapotranspiration Simulation of Trees and Radiation Absorption) is a 3D individual-tree canopy model computing radiation interception, photosynthesis, transpiration, and respiration for tree stands at half-hourly resolution. Originally developed by Ying-Ping Wang (CSIRO, 1996) and extended by Belinda Medlyn and the Jarvis group (Edinburgh, 1997--2001) for the EU ECOCRAFT project. Crowns are discretized into up to 720 volumetric grid points; ray paths trace through all neighbouring crowns with Beer--Lambert extinction. Norman (1979) iterative multi-scattering resolves PAR, NIR, and thermal fluxes. Farquhar--von Caemmerer C3 photosynthesis couples iteratively with Ball-Berry, Ball-Berry-Leuning, or Jarvis stomatal conductance; Penman-Monteith transpiration and iterative leaf energy balance close the surface flux loop. Six crown shapes (cone, half-ellipsoid, paraboloid, full-ellipsoid, cylinder, box) with beta-function leaf area density distributions and ellipsoidal leaf-angle distributions (Campbell 1986) are supported. Its successor MAESPA extends the model to soil water balance.

#figure(
  table(
    columns: (auto, 1fr, auto, auto),
    inset: 6pt,
    align: (left, left, left, left),
    stroke: 0.5pt,
    table.header([*\#*], [*Process*], [*Family*], [*Fidelity*]),
    [1],
    [3D Crown Radiation Transfer. Beam/diffuse/thermal radiation through individual tree crowns; Beer--Lambert extinction with weighted pathlengths through neighbouring crowns (TREDST/WPATH); sunlit/shaded separation. 6 crown shapes.],
    [Radiation],
    [Physics-based],

    [2],
    [Solar Geometry. Full astronomical position (Barkstrom 1981): declination, equation of time, daylength, half-hourly zenith/azimuth, slope correction factors (Steven \& Unsworth 1979/1980).],
    [Radiation],
    [Physics-based],

    [3],
    [Farquhar--von Caemmerer C3 Photosynthesis. Rubisco-limited (Ac) and electron transport-limited (Aj) assimilation; peaked Arrhenius Vcmax/Jmax; ECOCRAFT parameterization; coupled iterative Ci solution.],
    [Biogeochem],
    [Physics-based],

    [4],
    [Stomatal Conductance Suite. Three models: Jarvis multiplicative (4 VPD options), Ball-Berry, Ball-Berry-Leuning. Soil moisture modifier (Granier \& Loustau 1994).],
    [Ecology],
    [Intermediate],

    [5],
    [Penman-Monteith Transpiration \& Energy Balance. Leaf-level and canopy-scale PM ET with iterative leaf temperature (Leuning et al. 1995). Radiation conductance, forced/free convection boundary layer.],
    [Hydrology],
    [Physics-based],

    [6],
    [Autotrophic Respiration. Q10 maintenance for 5 pools (foliage, stem, branch, fine root, coarse root) plus growth respiration from biomass increment × construction cost.],
    [Biogeochem],
    [Intermediate],

    [7],
    [Crown Geometry \& Allometry. 6 crown shapes; beta-function LAD (vertical/horizontal, 3 age classes); ellipsoidal leaf-angle distribution (Campbell 1986); simple 4-parameter phenology; allometric stem/branch/root biomass.],
    [Ecology],
    [Intermediate],

    [8],
    [Met Processing \& Radiation Partitioning. Daily→half-hourly disaggregation: sinusoidal T cycle; Spitters (1986) beam/diffuse; Bristow-Campbell transmissivity; exponential canopy wind; humidity conversions; CO₂/OTC scenarios.],
    [Atmosphere],
    [Intermediate],

    [9],
    [Canopy Integration \& Stand Aggregation. Summation of leaf-level fluxes to tree-level and stand-level (SUMTREES): CO₂, transpiration, sensible heat, respiration. Weighted by stocking density.],
    [Ecology],
    [Intermediate],

    [10],
    [Leaf Optics \& Multi-Scattering. Norman (1979) iterative scheme on Equivalent Horizontal Canopy layers for PAR/NIR/thermal; absorptance/reflectance/transmittance per wavelength; soil reflectance.],
    [Radiation],
    [Physics-based],
  ),
  caption: [MAESTRA/MAESTRO 3D canopy model catalog (10 entries). Primary families: Radiation (3), Ecology (3), Biogeochem (2), Hydrology (1), Atmosphere (1).],
) <tab:maestra>

#pagebreak()

= Knowledgebase Summary Statistics <app:summary>

#figure(
  table(
    columns: (auto, auto, auto, auto),
    inset: 8pt,
    align: (left, right, right, right),
    stroke: 0.5pt,
    table.header([*Source Model*], [*Entries*], [*Primary Family*], [*Physics-Based*]),
    [Badlands], [33], [Geomorphology], [1],
    [CESM (CAM+CLM+MOM6+CICE+CISM+MOSART+WW3)], [182], [Atmosphere], [65],
    [E3SM (unique beyond CESM)], [15], [Atmosphere/Ocean], [11],
    [FATES], [90], [Ecology], [18],
    [iLand], [21], [Ecology], [0],
    [Landlab], [49], [Geomorphology], [7],
    [LPJ-GUESS], [18], [Biogeochem], [1],
    [Noah-MP], [32], [Hydrology], [9],
    [ParFlow], [16], [Hydrology], [12],
    [WRF / WRF-SFIRE / WRF-Hydro], [43], [Atmosphere], [14],
    [ORCHIDEE], [26], [Hydrology/BGC], [5],
    [VIC], [17], [Hydrology], [5],
    [GFDL ESM4], [9], [Atmosphere], [6],
    [PEcAn], [20], [Ecology], [9],
    [LANDIS-II Core], [6], [Ecology], [0],
    [JULES], [25], [Hydrology], [7],
    [CABLE], [18], [Hydrology/BGC], [6],
    [CLASSIC], [16], [Biogeochem], [2],
    [SUMMA], [20], [Hydrology], [8],
    [ED2], [18], [Ecology], [3],
    [PFLOTRAN], [15], [Geology/BGC], [13],
    [ROMS], [20], [Ocean], [12],
    [SWAT], [18], [Hydrology], [1],
    [GEOS-Chem], [20], [Atmosphere], [6],
    [Delft3D], [16], [Ocean], [12],
    [LISFLOOD-FP], [10], [Hydrology], [3],
    [LM3-PPA / BiomeE], [22], [Ecology], [2],
    [SORTIE-ND], [14], [Ecology], [1],
    [JABOWA / FORET / LINKAGES], [12], [Ecology], [0],
    [FORMIND], [14], [Ecology], [0],
    [LANDIS-II Extensions (NECN+)], [18], [Ecology/BGC], [0],
    [UVAFME], [14], [Ecology], [0],
    [FORCLIM], [12], [Ecology], [0],
    [SORTIE-NG (Erickson)], [16], [Ecology], [2],
    [DeepLand (Erickson)], [18], [Hydrology/Ecology], [10],
    [Earth-2 / earth2studio (NVIDIA)], [14], [Atmosphere], [5],
    [PhiSat-2 principles (ESA)], [12], [Multi-family], [0],
    [FireBench (benchmark)], [8], [Fire], [---],
    [MARBL], [20], [Biogeochem], [2],
    [PISM], [23], [Cryosphere], [13],
    [OGGM], [15], [Cryosphere], [2],
    [APSIM], [25], [Ecology/BGC], [2],
    [DSSAT], [24], [Ecology/BGC], [0],
    [CryoGrid], [18], [Cryosphere], [11],
    [XBeach], [20], [Ocean/Geomorphology], [12],
    [SURFEX/TEB], [18], [Atmosphere], [8],
    [EwE], [17], [Ecology], [0],
    [CARMA], [18], [Atmosphere], [12],
    [ATS], [18], [Hydrology/Cryo], [12],
    [BFM], [22], [Biogeochem], [3],
    [MAESTRA], [10], [Radiation/Ecology], [4],
    table.hline(),
    [*Total*], [*1195*], [], [*336*],
  ),
  caption: [Summary of initial Process Knowledgebase entries by source model/platform (51 models + 1 benchmark suite). FireBench entries are benchmarking resources, not process implementations.],
) <tab:kb-summary>

#figure(
  table(
    columns: (auto, auto, auto),
    inset: 8pt,
    align: (left, right, left),
    stroke: 0.5pt,
    table.header([*Process Family*], [*Count*], [*Top Contributors*]),
    [Ecology],
    [~218],
    [FATES, EwE, APSIM, DSSAT, ED2, SORTIE-ND, SORTIE-NG, DeepLand, PPA/BiomeE, JABOWA/FORET, FORMIND, iLand, LANDIS-II, UVAFME, FORCLIM, PhiSat-2, MAESTRA],
    [Hydrology],
    [~188],
    [CLM, ParFlow, SUMMA, ATS, APSIM, DSSAT, SWAT, JULES, CABLE, Noah-MP, LISFLOOD-FP, ROMS, CryoGrid, DeepLand, PhiSat-2, MAESTRA],
    [Biogeochem],
    [~184],
    [MARBL, BFM, CLM, E3SM, FATES, GEOS-Chem, PFLOTRAN, APSIM, DSSAT, LANDIS-II NECN, PPA/BiomeE, ORCHIDEE, SORTIE-NG, DeepLand, MAESTRA],
    [Atmosphere],
    [~179],
    [CESM/CAM, CARMA, E3SM/EAM, WRF, SURFEX/TEB, GEOS-Chem, JULES, Delft3D, Earth-2, SORTIE-NG, DeepLand, PhiSat-2, MAESTRA],
    [Cryosphere],
    [~107],
    [PISM, OGGM, CryoGrid, ATS, CICE, MPAS-SeaIce, SUMMA, JULES, ROMS, VIC, UVAFME, BFM, CLM snow, DeepLand, PhiSat-2],
    [Ocean], [~89], [XBeach, ROMS, Delft3D, MOM6, MPAS-Ocean, PISM, WW3, SURFEX/TEB, Badlands waves, PhiSat-2],
    [Human Systems],
    [~61],
    [APSIM, DSSAT, SURFEX/TEB, SWAT, GEOS-Chem, EwE, LANDIS-II, FORMIND, CLM, PFLOTRAN, JULES, SORTIE, FORCLIM, PhiSat-2],
    [Geomorphology], [~59], [Badlands, Landlab, XBeach, ROMS, Delft3D, SWAT, APSIM, MOSART],
    [Fire],
    [~52],
    [FATES/SPITFIRE, WRF-SFIRE, LANDIS-II SCRPPLE, CLASSIC, ILand, UVAFME, SORTIE-NG, PhiSat-2, FireBench],
    [Radiation],
    [~46],
    [SUMMA, CARMA, SURFEX/TEB, MARBL, BFM, GEOS-Chem, PPA, SORTIE-NG, DeepLand, CABLE, ED2, CAM, CLM/SNICAR, FATES, MAESTRA],
    [Geology], [~20], [PFLOTRAN, PISM, CryoGrid, Badlands, Landlab, ParFlow],
    [Evolution], [~1], [Landlab SpeciesEvolver],
    table.hline(),
    [*Total*], [*~1195*], [],
  ),
  caption: [Process family distribution across all 51 reference models and platforms.],
) <tab:family-dist>

#v(1em)

The 1195 entries cataloged here represent the _initial seed_ of the MAESMA Process Knowledgebase, drawn from 51 reference models and platforms and one benchmarking dataset (FireBench). The catalog spans the full Earth system --- from deep-ocean biogeochemistry (MARBL, BFM) and ice sheet dynamics (PISM, OGGM) through permafrost thermodynamics (CryoGrid, ATS), coastal morphodynamics (XBeach), agricultural systems (APSIM, DSSAT), urban energy balance (SURFEX/TEB), marine food webs (EwE), aerosol microphysics (CARMA), 3D individual-tree canopy radiation and physiology (MAESTRA), GPU-native models (SORTIE-NG, DeepLand), a foundation weather model platform (NVIDIA Earth-2 / earth2studio), and autonomous observation intelligence principles (ESA PhiSat-2) that collectively exemplify the architecture's core identity as a multi-agent system for _inferential process discovery, combination, simulation, and evolution_. Representative entries (50+ manifests from 19 source models) are implemented as typed `ProcessManifest` structures with machine-readable I/O contracts, scale envelopes, conservation properties, and cost models via the `generate_seed_manifests()` function; 40+ ontological relations (RequiresCouplingWith, IncompatibleWith, Supersedes, VariantOf, CompatibleWith) are implemented via `generate_seed_relations()`. Empirical skill records will accumulate during autonomous benchmarking. Discovered representations will _evolve_ through crossover, mutation, and Pareto-optimal selection --- the knowledgebase is the genome, the Pareto front is the fitness landscape. The neural inference engine begins reasoning over this graph from its first cycle, the Foundation Model Agent generates rapid ensemble atmospheric states via Earth-2, the Autonomous Observation Agent steers data acquisition toward maximal information gain via PhiSat-2-class edge-AI, and the process discovery pipeline --- leveraging FNO, PINO, DeepONet, and MeshGraphNet architectures trained with Modulus physics-informed loss functions on GPU --- will continuously expand and evolve the knowledgebase with learned representations from observational data.
