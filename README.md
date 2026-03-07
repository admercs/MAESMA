<p align="center">
  <img src="media/banner_logo.png" alt="MAESMA logo" width="100%">
</p>

<h3 align="center">An autonomous system for Earth observation and system modeling (EOSM). The final Earth system modeling framework.</h3>

Modular Agentic Earth System Modeling Arena (MAESMA) is an autonomous agentic AI system for Earth system observation and model discovery. A multi-agent system for inferential process discovery, combination, simulation, and evolution. A central Process Knowledgebase stores all process models, implementations, and ontological metadata. Agents reason over the knowledgebase via a neural inference engine — selecting, composing, and inventing process representations driven by simulation errors and uncertainties. Process models are treated as living organisms: they compete for compute, earn survival through skill, replicate via mutation and crossover, and face extinction when stagnant — an artificial life (ALife) framework for autonomous model construction.

## Overview

1. **Human-out-of-loop** — Runs autonomously and indefinitely; humans monitor via dashboard, never gate workflow
2. **Salient dynamics first** — Agents prioritize processes with the greatest effect on the time evolution of system states; lower-impact processes are added incrementally as budget and accuracy targets demand
3. **Automated data discovery** — Agents crawl STAC/CMR/CKAN for observations, preprocess, ingest, expand validation coverage
4. **Central Process Knowledgebase** — Single versioned store of all process models (code, manifests, ontologies, skill records); agents query, deposit, and reason over this knowledgebase for all model construction decisions
5. **Neural inference engine** — Graph transformer over the knowledgebase (process embeddings, skill records, error fields, regime context) proposes process selections; replaces hand-crafted heuristics with learned reasoning
6. **Error/uncertainty-driven selection** — Simulation errors and posterior uncertainties propagate to the inference engine; every model–observation mismatch reshapes process selection and triggers discovery
7. **Process discovery → knowledgebase** — Core workflow step: residual analysis → learn new representations from observations → validate → deposit into knowledgebase for subsequent model integration and experiments
8. **Infinite loop** — Discover data → query knowledgebase → assemble → benchmark → select → discover processes → deposit into knowledgebase → repeat
9. **Ontology-indexed knowledgebase** — Unified ontology indexes all knowledgebase contents; every process, dataset, and metric is agent-discoverable; new entries available upon registration
10. **Live dashboard** — Next.js real-time monitoring: agent workflows, Pareto frontiers, skill evolution, data ingestion
11. **Scale-aware compilation** — Compiler validates conservation, closure, coupling constraints across assembled process graphs
12. **Full EESM coverage** — All DOE EESM program areas: ESMD, RGMA, MSD
13. **A2A federation** — Inter-institutional agent collaboration via A2A protocol; federated assembly and skill sharing
14. **Geoengineering control** — Closed-loop: physical (SAI, MCB, OAE, DAC, SRM, cloud seeding, enhanced weathering), biological (genetic modification, gene drives), and management (afforestation, MPAs, resource allocation) interventions
15. **Planetary defense** — All mass extinction drivers: impacts, LIP volcanism, GOE, Snowball Earth, GRBs, supernovae, pandemics, anthropogenic; NEO tracking from JPL/USAF/USSF/ESA
16. **Trophic dynamics** — Multi-trophic food webs as first-class processes: allometric scaling, metabolic theory, functional responses
17. **Evolution & phylogeography** — Trait evolution, speciation, extinction, adaptive radiation; gene flow over space/time via migration corridors, dispersal barriers, vicariance
18. **Population dynamics** — Human demographics + land use + resource consumption coupled with species density-dependence, Allee effects, dispersal, metapopulation structure
19. **GPU acceleration** — Multi-GPU execution via wgpu/cudarc with NVIDIA Modulus neural operators (FNO, PINO, DeepONet, MeshGraphNet) for physics-informed emulation
20. **Foundation models** — NVIDIA Earth-2/earth2studio integration (FourCastNet, Pangu-Weather, GraphCast, GenCast, CorrDiff) as ensemble backbone; Foundation Model Agent orchestrates inference
21. **Autonomous observation** — PhiSat-2-inspired edge AI for satellite-side anomaly detection, adaptive tasking, and real-time observation triggering
22. **Artificial life process evolution** — Processes are living organisms with survival tiers, constitutional invariants, heartbeat monitoring, process souls, self-replication, and phylogenetic lineage tracking
23. **Inferential process discovery** — Multi-agent system for inferential process discovery, combination, simulation, and evolution — errors drive the entire lifecycle

## Architecture

### Three-Layer Control

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                      STRATEGIC LAYER (hours–weeks)                     │
│                                                                         │
│  Intent & Scope ──► Autonomous Optimizer ──► Model Selection           │
│       │                    │                       │                    │
│       │    ┌───────────────┘                       │                    │
│       ▼    ▼                                       ▼                    │
│  ┌──────────────────────────────────────────────────────────┐           │
│  │           TACTICAL LAYER (minutes–hours)                 │           │
│  │                                                          │           │
│  │  KB Retrieval ──────► Model Assembly ──► Benchmarking  │           │
│  │       │                      │                  │        │           │
│  │  Active Learning ◄───── Closure Check ──── Skill Librarian          │
│  │       │                      │                  │        │           │
│  │  Process Discovery ◄── Data Scout ◄──── EESM Diagnostics│           │
│  └──────────────────────────────────────────────────────────┘           │
│       │                    │                       │                    │
│       ▼                    ▼                       ▼                    │
│  ┌──────────────────────────────────────────────────────────┐           │
│  │            OPERATIONAL LAYER (seconds–minutes)            │           │
│  │                                                          │           │
│  │  Runtime Sentinel ──► Compiler ──► Task Scheduler        │           │
│  │       │                  │              │                 │           │
│  │  Data Plane Agents ◄─── Event Bus ◄─── Device Manager    │           │
│  └──────────────────────────────────────────────────────────┘           │
└─────────────────────────────────────────────────────────────────────────┘
```

All layers run concurrently and continuously. The strategic loop never terminates.

### Knowledgebase-Centric Architecture

```text
┌──────────────────────────────────────────────────────────────────────────┐
│                    KNOWLEDGEBASE-CENTRIC ARCHITECTURE                    │
│                                                                          │
│  Observations ──►  ┌──────────────────────────────┐  ◄── Discovered     │
│  (STAC/CMR/NRT)    │     PROCESS KNOWLEDGEBASE     │      Processes      │
│                     │                              │                     │
│                     │  Process code + manifests    │                     │
│                     │  Ontology graph (5 domains)  │                     │
│                     │  Skill records (append-only) │                     │
│                     │  Learned representations     │                     │
│                     └──────────────┬───────────────┘                     │
│                                    │ query                               │
│                                    ▼                                     │
│                     ┌──────────────────────────────┐                     │
│                     │    NEURAL INFERENCE ENGINE    │                     │
│                     │                              │                     │
│                     │  Graph transformer over KB   │                     │
│                     │  Inputs: errors, uncertainty,│                     │
│                     │    regime context, budgets   │                     │
│                     │  Output: process selections, │                     │
│                     │    assembly proposals        │                     │
│                     └──────────────┬───────────────┘                     │
│                                    │ proposals                           │
│                                    ▼                                     │
│  ┌─────────────┐    ┌──────────────────────────────┐    ┌─────────────┐ │
│  │   PROCESS   │◄───┤       AGENT SWARM (22)       ├───►│  SIMULATION │ │
│  │  DISCOVERY  │    │                              │    │   RUNTIME   │ │
│  │  PIPELINE   │    │  Assemble, compile, execute, │    │             │ │
│  │             │    │  benchmark, score, calibrate  │    │  Multi-GPU  │ │
│  │  Residual → │    └──────────────────────────────┘    │  execution  │ │
│  │  Learn →    │                                        └──────┬──────┘ │
│  │  Validate → │                                               │        │
│  │  Deposit    │    ┌──────────────────────────────┐           │        │
│  │  into KB    │◄───┤    ERRORS & UNCERTAINTIES     │◄──────────┘        │
│  └─────────────┘    │  (drive next inference cycle) │                    │
│                     └──────────────────────────────┘                     │
└──────────────────────────────────────────────────────────────────────────┘
```

The Process Knowledgebase is the gravitational center of the architecture. All agents read from and write to it. The neural inference engine reasons over it to propose process selections. Errors and uncertainties from simulations flow back as the primary signal driving both selection and discovery.

### Autonomous Optimization Loop

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│                    AUTONOMOUS CLOSED LOOP (runs indefinitely)               │
│                                                                              │
│     ┌────────────┐     ┌────────────┐     ┌────────────┐     ┌──────────┐   │
│     │ 1. DISCOVER │────►│ 2. ASSEMBLE│────►│ 3. BENCHMARK────►│ 4. SELECT│   │
│     │    DATA     │     │   MODELS   │     │   & SCORE  │     │  OPTIMAL │   │
│     └─────▲───────┘     └────────────┘     └────────────┘     └────┬─────┘   │
│           │                                                        │         │
│           │         ┌──────────────────────────────────┐            │         │
│           │         │  5. DISCOVER PROCESSES            │◄───────────┘         │
│           │         │     Residual → learn → validate  │                     │
│           │         │     → deposit into Knowledgebase  │                     │
│           │         └──────────────┬───────────────────┘                     │
│           │                        │                                         │
│     ┌─────┴────────────────────────┘                                         │
│     │  6. UPDATE KNOWLEDGEBASE ──► skills + ontology + posteriors            │
│     └──────────────────── repeat forever ────────────────────────────────────┘
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │  MONITORING PLANE: Next.js dashboard (observe-only, never blocks)     │  │
│  │  Optional: humans adjust objectives/weights/budgets at any time       │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────────┘
```

| Step                        | Agent Action                                                    | Ontology Domain |
| --------------------------- | --------------------------------------------------------------- | --------------- |
| **1. Discover Data**        | Crawl catalogs; preprocess; ingest                              | Dataset         |
| **2. Assemble Models**      | Neural inference queries KB; agents build SAPG from proposals   | Process         |
| **3. Benchmark & Score**    | Score against observations; compute multi-metric skill          | Metric          |
| **4. Select Optimal**       | Neural inference + Bayesian selection; update Pareto frontier   | All             |
| **5. Discover Processes**   | Residual analysis → learn new representations from observations | Process+Dataset |
| **6. Update Knowledgebase** | Deposit skill records + learned processes; refine ontology      | All             |

Four concurrent cycles within this loop:

1. **Fitness optimization** — Neural inference proposes per-region, per-regime rung selections from knowledgebase, prioritizing salient dynamics (processes with the greatest effect on system state evolution) first and adding detail incrementally; swap dominant rungs; schedule experiments for uncertain regions
2. **Data discovery** — Gap analysis → catalog search → relevance/novelty scoring → ingest → re-score affected representations
3. **Regime discovery** — Cluster skill records to detect new regime tags, regime boundaries, and regime drift
4. **Process discovery** — Residual analysis on every cycle; structured bias → learn data-driven representation → validate → deposit into knowledgebase → deploy

### Optimization Objective

$$F(\mathbf{r}, g, \ell) = \sum_{m \in \mathcal{M}} w_m \cdot S_m(\mathbf{r}, g, \ell) - \lambda \cdot C(\mathbf{r}) + \gamma \cdot G(\mathbf{r})$$

- $\mathbf{r}$ — rung selections per process family; $g$ — region; $\ell$ — regime
- $S_m$ — skill score for metric $m$; $C$ — cost (FLOPS, memory, walltime); $G$ — generalizability (cross-region transfer)
- $w_m, \lambda, \gamma$ — weights (user-set or learned)

The Autonomous Optimizer maintains a Pareto frontier over skill vs. cost. Optimization operates at three nested scopes:

| Scope                 | What                                        | Timescale    | Agent                |
| --------------------- | ------------------------------------------- | ------------ | -------------------- |
| **Per-region/regime** | Rung selection per context                  | Hours–days   | Autonomous Optimizer |
| **Structural**        | Process family inclusion/coupling decisions | Days–weeks   | Model Selection      |
| **Inventive**         | New representations from data               | Weeks–months | Process Discovery    |

### Scale-Aware Process Graph (SAPG)

Typed, unit-aware, scale-aware directed hypergraph — the substrate agents read, write, compile, and optimize:

- **Nodes** — State variables (e.g., `T_air(x,y,z,t)`, `soil_moisture(layer,t)`)
- **Edges** — Process operators (radiation, infiltration, stomatal conductance, ...)
- **Hyperedges** — Multi-input/output processes
- **Constraints** — Units, bounds, conservation, closure, stability

### Two-Tier Coupling

| Tier     | Timestep       | Processes                                                 |
| -------- | -------------- | --------------------------------------------------------- |
| **Slow** | Days–centuries | Succession, competition, soil C/N, ecohydrology, mgmt     |
| **Fast** | Seconds–hours  | Fire spread, plume feedback, canopy energy, overland flow |

## Process Families

| Family                    | R0 (Regional)                            | R1 (Landscape)                                                    | R2 (Event/Local)                                              | R3 (Research)            |
| ------------------------- | ---------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------- | ------------------------ |
| **Fire**                  | Stochastic regime (km, daily)            | Rothermel + CFS FBP (10–100 m, min)                               | Wind-aware + plume (5–50 m, sec)                              | Fire–atmosphere (m, sec) |
| **Hydrology**             | Bucket + curve-number (km, daily)        | Multi-layer Richards (30–300 m, min)                              | Integrated surface–subsurface (10–100 m, min)                 | —                        |
| **Ecology**               | Cohort mosaic (30–250 m, annual)         | Size-structured cohorts (10–100 m, annual)                        | Individual-based (10–50 m, annual)                            | —                        |
| **Biogeochem**            | Big-leaf C + simple pools                | Multi-pool C/N + litter + microbial (daily)                       | Vertically resolved soil biogeochem                           | —                        |
| **Radiation**             | Daily potential solar + empirical canopy | Sub-daily SW/LW + energy balance (hourly)                         | 3D radiative transfer                                         | —                        |
| **Atmosphere**            | Prescribed (reanalysis)                  | WRF-like downscaling (5–25 km, min)                               | Convection-permitting (1–4 km, sec)                           | E3SM-Omega               |
| **Ocean**                 | Slab mixed-layer (1°, daily)             | z-coordinate regional (0.25°, hourly)                             | Eddy-resolving MPAS-Ocean (1–10 km, min)                      | —                        |
| **Cryosphere**            | Degree-day melt (km, daily)              | Energy-balance snow + sea-ice (30–300 m, hourly)                  | Dynamic ice-sheet + rheology (10 km, min)                     | —                        |
| **Human Systems**         | Exogenous scenarios (national, annual)   | Sectoral demand/supply (regional, monthly)                        | Agent-based infrastructure (county, hourly)                   | Coupled IAM (global)     |
| **Trophic Dynamics**      | Static food web (biome, annual)          | Dynamic Lotka-Volterra (landscape, monthly)                       | Individual-based predator-prey (patch, daily)                 | —                        |
| **Evolution & Phylogeo.** | Fixed traits (PFT, static)               | Adaptive traits + phylogeographic dispersal (population, decadal) | Genotype-phenotype + speciation + gene flow (individual, gen) | —                        |

## Process Knowledgebase

Central versioned store of all process models — the single source of truth that agents query, reason over, and deposit into ([manifest spec](process_registry/README.md)). Every representation in the system lives here, whether hand-coded or discovered from observational data.

Each knowledgebase entry bundles:

| Layer          | Contents                                                                                      |
| -------------- | --------------------------------------------------------------------------------------------- |
| **Code**       | Runnable implementation (CPU solver, GPU kernel, ML emulator) with pluggable backends         |
| **Manifest**   | Machine-readable metadata: identity, I/O contract, scale envelope, conservation, cost model   |
| **Ontology**   | Relations: `compatible_with`, `incompatible_with`, `requires_coupling_with`, regime tags      |
| **Skill**      | Empirical performance per region × regime × season × coupled context (append-only, versioned) |
| **Provenance** | Origin (hand-coded / discovered), training data fingerprint, validation history, lineage      |

### Knowledgebase Operations

| Operation    | Actor                   | Description                                                                  |
| ------------ | ----------------------- | ---------------------------------------------------------------------------- |
| **Query**    | Neural Inference Engine | Retrieve candidates given error signals, regime, scale, budget               |
| **Reason**   | Neural Inference Engine | Score/rank candidates; propose assemblies; identify representation gaps      |
| **Deposit**  | Process Discovery       | Validated learned representations registered with full manifest + provenance |
| **Update**   | Skill Librarian         | Append skill records; refine cost models from runtime measurements           |
| **Federate** | A2A Gateway             | Exchange anonymized skill records and manifests with peers                   |

New entries — whether contributed by humans or discovered from observational data — become immediately available for neural inference, agent selection, and simulation experiments.

## Unified Ontology

The knowledgebase's indexing and reasoning substrate — five interconnected domains in a single queryable graph ([full spec](ontology/README.md)):

| Domain                | Governs            | Key Classes                                                                                        |
| --------------------- | ------------------ | -------------------------------------------------------------------------------------------------- |
| **Process**           | Model capabilities | `ProcessFamily`, `Representation`, `StateVariable`, `ScaleEnvelope`, `Constraint`                  |
| **Dataset**           | Available data     | `Observable`, `Product`, `CatalogSource`, `AccessSpec`, `QualitySpec`                              |
| **Metric**            | Scoring            | `Metric`, `ScoringProtocol`, `FitnessFunction`, `SkillRecord`, `CostModel`                         |
| **Geoengineering**    | Interventions      | `Intervention`, `ControlTarget`, `InterventionSchedule`, `SideEffectConstraint`, `StrategyRecord`  |
| **Planetary Defense** | Threats            | `NearEarthObject`, `ImpactScenario`, `ExtinctionEvent`, `DeflectionStrategy`, `RecoveryTrajectory` |

Cross-domain edges connect representations → products → observables → state variables; scoring protocols → metrics; skill records → skill models. The neural inference engine traverses this graph to propose process selections. New entries become agent-discoverable upon registration — no code changes required.

## ALife Process Lifecycle

Process models in MAESMA are not static configurations — they are living organisms governed by artificial life (ALife) principles inspired by [automaton](https://github.com/Conway-Research/automaton). Every process representation is wrapped in a `ProcessAutomaton` that manages its lifecycle from birth through competition, replication, and potential extinction.

### Survival Tiers

| Tier           | Budget Multiplier | Heartbeat Cadence | Condition                             |
| -------------- | ----------------- | ----------------- | ------------------------------------- |
| **Normal**     | 1.0×              | 1×                | skill/cost > threshold; not stagnant  |
| **LowCompute** | 0.5×              | 2×                | Marginal value; under observation     |
| **Critical**   | 0.1×              | 4×                | Near extinction; last chance to prove |
| **Archived**   | 0×                | —                 | Preserved for lineage; no execution   |

Processes earn their right to exist by producing predictive value (skill) relative to their compute cost. The heartbeat daemon periodically re-evaluates every process, promoting survivors and demoting underperformers.

### Constitutional Invariants

Three hierarchical, immutable laws that no agent or optimizer may violate:

1. **Law of Conservation** — Every process must satisfy mass/energy/charge conservation within tolerance
2. **Law of Earned Existence** — A process must demonstrate positive skill-to-cost ratio to persist; no entitlements
3. **Law of Provenance** — Every modification, replication, and tier transition is recorded; lineage is immutable

Constitutional violations trigger immediate demotion or archival — the constitution supersedes fitness optimization.

### Process Soul

Each process carries a `ProcessSoul` — its identity beyond parameters:

- **Strengths/Weaknesses** — Empirical characterization of where the process excels or fails
- **Dominant niches** — Regions, regimes, seasons where it outperforms alternatives
- **Modification history** — Complete record of parameter edits, structural changes, hybridizations
- **Tier history** — Timeline of survival tier transitions

### Self-Replication & Phylogenetic Lineage

Processes reproduce via four methods:

| Method          | Description                                            |
| --------------- | ------------------------------------------------------ |
| **Mutation**    | Clone with random parameter or structural perturbation |
| **Crossover**   | Combine components from two parent processes           |
| **Immigration** | Import from federated peer via A2A                     |
| **Speciation**  | Diverge into new family when niche separation is large |

Every replication event is recorded with parent IDs, method, and timestamp. The resulting phylogenetic tree tracks the evolutionary history of all process models, enabling lineage analysis, ancestral rollback, and evolutionary trend detection.

### Heartbeat Daemon

A continuous background daemon (`HeartbeatDaemon`) ticks through all living process automatons:

1. **Compute skill-to-cost ratio** — Is this process earning its compute budget?
2. **Evaluate survival tier** — Promote, demote, or archive based on thresholds
3. **Check constitutional compliance** — Conservation residuals within tolerance?
4. **Detect stagnation** — Has fitness improved in the last N generations?
5. **Report outcomes** — Emit `HeartbeatCheck`, `SurvivalTierChange`, `StagnationDetected` events

## Agent Swarm

25 agents own the full model lifecycle:

| Agent                          | Role                                                                |
| ------------------------------ | ------------------------------------------------------------------- |
| **Intent & Scope**             | User objectives → observable requirements + error bands             |
| **Knowledgebase Retrieval**    | Query KB via neural inference given errors, scale, regime, budget   |
| **Model Assembly**             | Build SAPG from inference engine proposals; pick rungs, coupling    |
| **Closure & Consistency**      | Validate variables, physics, units, conservation, CFL               |
| **Data & Calibration**         | Determine datasets, parameter priors, calibration targets           |
| **Runtime Sentinel**           | Monitor execution; trigger rung upgrades/downgrades                 |
| **Provenance & Audit**         | Decision reports: what was selected, rejected, why                  |
| **Benchmarking**               | Run configurations against observations; compute skill scores       |
| **Model Selection**            | Neural inference + Bayesian selection; update posterior weights     |
| **Active Learning**            | Identify most informative experiments and under-observed regimes    |
| **Skill Librarian**            | Manage Skill Score Store: write, query, version, aggregate          |
| **Autonomous Optimizer**       | Continuous fitness-driven Pareto selection loop                     |
| **Data Scout**                 | Search catalogs; score relevance/novelty; ingest new products       |
| **A2A Gateway**                | Peer discovery, task lifecycle, artifact exchange, authentication   |
| **MSD Coupling**               | Bidirectional natural ↔ human system coupling                       |
| **Scenario Discovery**         | AI-driven scenario exploration; tipping points; cascading failures  |
| **EESM Diagnostics**           | ILAMB/IOMB/E3SM Diags; RGMA-aligned evaluation campaigns            |
| **Process Discovery**          | Residual analysis → ML learning → validation → deposit into KB      |
| **Geoengineering Strategy**    | Multi-intervention optimization; termination shock; Pareto frontier |
| **Planetary Defense**          | NEO tracking; all-source extinction modeling; deflection assessment |
| **Trophic Dynamics**           | Food web assembly, calibration, energy flow validation              |
| **Evolution & Phylogeography** | Trait evolution, speciation, gene flow, vicariance, range dynamics  |
| **Foundation Model**           | Orchestrate Earth-2 foundation model ensemble; bias-correct; fuse   |
| **Autonomous Observation**     | Edge AI observation tasking; anomaly detection; adaptive scheduling |
| **Process Evolution**          | ALife-driven population management; survival tiers; replication     |

## Neural Inference & Knowledge Engine

### Neural Inference Engine

A graph transformer trained on the Process Knowledgebase reasons over process embeddings, skill records, simulation error fields, and regime context to drive process selection:

- **Input encoding** — Process manifests, skill vectors, spatiotemporal error fields, regime tags, compute budget encoded as node/edge features on the knowledgebase graph
- **Inference** — Transformer attention over the graph proposes: (1) process selections per family/region/regime, (2) assembly configurations, (3) representation gaps where discovery should focus
- **Training signal** — Skill score deltas from accepted proposals; the engine learns which knowledgebase entries resolve which error patterns
- **Uncertainty-aware** — Outputs calibrated confidence; low-confidence proposals route to Active Learning for targeted experiments before commitment
- **Continual learning** — Retrained incrementally as the knowledgebase grows with new skill records and discovered processes

The inference engine replaces hand-crafted heuristics for process selection. Every simulation error becomes a query against the knowledgebase: *"which process, if swapped or added, most likely reduces this error?"*

### Skill Score Store

Performance records indexed by rung × region × regime × season × coupled context. Multi-metric (RMSE, KGE, CRPS, conservation residuals, timing errors). Versioned and append-only. Primary training data for the neural inference engine.

### Bayesian Model Selection

Posterior $p(M_k | \mathbf{y}) \propto p(\mathbf{y} | M_k) \, p(M_k)$ over model structures. Marginal likelihoods penalize over-complexity (automatic Occam's razor). Bayesian Model Averaging for predictions. Active Learning identifies high-uncertainty configurations, under-observed regimes, and sensitivity frontiers.

### Combinatorial Hypothesis Engine

- **Structured enumeration** — Vary one family's rung while holding others fixed
- **Factorial experiments** — Test interaction effects (e.g., F1+H1 vs. F1+H0)
- **Budget-aware scheduling** — Prioritize experiments within idle GPU time
- **Emulator screening** — Surrogate models pre-screen unpromising configurations

### Ontology Feedback

Each simulation updates: skill models (empirical posteriors replace expert priors), cost models (actual walltime/memory), compatibility constraints, default rung preferences, regime tags, and parameter priors.

### Process Discovery Pipeline

```text
┌─────────────────────────────────────────────────────────────────────────┐
│  1. DETECT     — Structured bias persisting across calibrations        │
│  2. DIAGNOSE   — Attribute to variables, regions, seasons, regimes     │
│  3. HYPOTHESIZE — Missing coupling / feedback / process / scale bias   │
│  4. LEARN      — Neural operator, symbolic regression, or hybrid       │
│  5. VALIDATE   — Conservation, stability, out-of-sample generalization │
│  6. REGISTER   — Auto-generate manifest + register with provenance     │
│  7. INTEGRATE  — Compiler includes in candidates; benchmarking scores  │
│  8. ITERATE    — Improves skill → promote; else → archive + refine     │
└─────────────────────────────────────────────────────────────────────────┘
```

| Type          | Method                | Interpretability | Use Case               |
| ------------- | --------------------- | ---------------- | ---------------------- |
| **Black-box** | Neural operator (FNO) | Low              | Emulator rung          |
| **Symbolic**  | Symbolic regression   | High             | Interpretable closures |
| **Hybrid**    | Physics + ML residual | Medium           | Production rung        |

Every discovered representation carries epistemic provenance: training data fingerprint, applicability envelope, physical constraints enforced, expiration policy, and lineage to the residual analysis that motivated it.

## Geoengineering Feedback Control

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│                   GEOENGINEERING FEEDBACK CONTROL LOOP                       │
│                                                                              │
│   SETPOINTS                    PLANT                      OBSERVATIONS      │
│   ┌──────────────────┐        ┌──────────────────┐        ┌──────────────┐  │
│   │ T_global ≤ 1.5°C │        │                  │        │ Satellite    │  │
│   │ ΔP_regional < 5% │───►    │  MAESMA coupled   │───────►│ In-situ     │  │
│   │ pH_ocean > 8.0    │  ┌──► │  ESM simulation  │        │ Reanalysis  │  │
│   │ RF_target = W/m²  │  │    │                  │        │              │  │
│   └──────────────────┘  │    └──────────────────┘        └──────┬───────┘  │
│                          │                                       │           │
│   ACTUATORS              │    CONTROLLER                         │           │
│   ┌──────────────────┐  │    ┌──────────────────────────────────┴────────┐  │
│   │ SAI, MCB, OAE,   │  │    │  Geoengineering Strategy Agent            │  │
│   │ DAC, SRM, cloud  │◄─┤    │  Error → predict → optimize → simulate  │  │
│   │ seeding, enhanced│  │    │  → verify → update strategy → repeat     │  │
│   │ weathering, gene │  │    └─────────────────────────────────────────┘  │
│   │ mod, afforestation│◄─┘                                                   │
│   └──────────────────┘                                                       │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Interventions

| Intervention               | Mechanism                                 | Control Variables                    | Side Effects                              |
| -------------------------- | ----------------------------------------- | ------------------------------------ | ----------------------------------------- |
| **SAI**                    | Stratospheric aerosol injection           | Rate, latitude, altitude, season     | Ozone, acid deposition, monsoons          |
| **MCB**                    | Marine cloud brightening                  | Spray rate, regions, season          | Precipitation redistribution              |
| **Cloud seeding**          | AgI/dry ice nucleation                    | Rate, regions, storm criteria        | Downwind redistribution, AgI accumulation |
| **OAE**                    | Ocean alkalinity enhancement              | Mineral flux, regions, particle size | pH spikes, marine ecology                 |
| **Enhanced weathering**    | Basalt/olivine on land/coast              | Type, rate, area                     | Soil chemistry, heavy metals              |
| **DAC**                    | Direct air CO₂ capture                    | Rate, scale, energy source           | Energy demand, land use, cost             |
| **SRM**                    | Surface albedo modification               | Area, albedo delta, persistence      | Ecosystem disruption                      |
| **Iron fertilization**     | Ocean phytoplankton stimulation           | Flux, region, timing                 | Anoxia, trophic cascades, N₂O             |
| **Afforestation**          | Tree planting for sequestration           | Species, density, area               | Albedo (boreal), water use                |
| **Biochar**                | Pyrolyzed biomass → soil carbon           | Feedstock, rate, soils               | Nutrients, water, PAH risk                |
| **Genetic modification**   | Engineered organisms for C fixation       | Organism, trait, scale               | Gene flow, ecosystem disruption           |
| **Gene drives**            | Engineered alleles for ecosystem mgmt     | Species, mechanism, containment      | Uncontrolled spread, resistance           |
| **Marine protected areas** | Fishing restriction for biomass rebuild   | Boundaries, restriction level        | Displaced effort, socioeconomic           |
| **Resource management**    | Optimized water/land/fisheries allocation | Rules, quotas, targets               | Equity, enforcement                       |

### Strategy Discovery

1. **Forward simulation** — 50–500 yr trajectories through coupled ESM
2. **Multi-objective evaluation** — $J = w_T |T - T^*|^2 + w_P \Delta P_{rms}^2 + w_O (pH_{min} - pH^*)^- + w_C C + w_S \text{TermShock}$
3. **Termination shock** — Simulate abrupt cessation; quantify rebound warming and tipping proximity
4. **Stability** — Test under climate sensitivity, emission pathway, and technology failure uncertainty
5. **Portfolio optimization** — Combine interventions; discover synergies (SAI+DAC) and antagonisms
6. **Adaptive scheduling** — Dynamic controllers adjust intensity to observed system response
7. **Tipping point avoidance** — Maintain safe distance from AMOC/ice-sheet/permafrost tipping

### Geoengineering Ontology Extension

| Class                    | Description                                                   |
| ------------------------ | ------------------------------------------------------------- |
| `Intervention`           | Method with mechanism, control variables, side-effect profile |
| `BiologicalIntervention` | Genetic modification, gene drives, organism engineering       |
| `ManagementIntervention` | Resource allocation, protected areas, land management         |
| `ControlTarget`          | Variable + setpoint + tolerance + measurement source          |
| `InterventionSchedule`   | Time-varying control law over planning horizon                |
| `SideEffectConstraint`   | Max allowable deviation in non-target variable                |
| `TerminationScenario`    | Abrupt cessation specification for stability testing          |
| `StrategyRecord`         | Evaluated strategy with trajectory, cost, stability score     |
| `InterventionCostModel`  | Economic + energy cost per unit intervention                  |
| `GeneFlowRisk`           | Gene flow risk for biological interventions                   |
| `EcosystemServiceImpact` | Projected impact on ecosystem services                        |

Cross-domain edges: SAI → Atmosphere, OAE → Ocean, SRM → Radiation, genetic modification → Evolution, resource management → Human Systems, afforestation → Ecology.

## Planetary Defense & Existential Risk

### Data Sources

| Source                   | Provider    | Data                                             | Cadence    |
| ------------------------ | ----------- | ------------------------------------------------ | ---------- |
| **CNEOS Sentry**         | NASA JPL    | NEO orbits, impact probabilities, Palermo/Torino | Continuous |
| **Scout**                | NASA JPL    | New NEO trajectory predictions                   | Real-time  |
| **Horizons**             | NASA JPL    | High-precision ephemerides                       | On-demand  |
| **SSN**                  | USAF        | Object tracking, deep space                      | Continuous |
| **18th SDS**             | USSF        | Space object catalog, conjunctions               | Continuous |
| **NEOCC**                | ESA         | Independent risk assessment                      | Continuous |
| **MPC**                  | IAU         | Designations, orbital elements                   | Daily      |
| **ATLAS/CSS/Pan-STARRS** | NASA-funded | Survey detections                                | Nightly    |
| **LSST/Rubin**           | NSF/DOE     | Deep survey (future)                             | Nightly    |

### Mass Extinction & Catastrophe Database

| Event                     | Age (Ma)    | Cause                        | Key Processes Tested                                      |
| ------------------------- | ----------- | ---------------------------- | --------------------------------------------------------- |
| **GOE**                   | 2,400       | Cyanobacterial O₂            | Atmospheric chemistry, methane collapse, snowball trigger |
| **Huronian glaciation**   | 2,400–2,100 | GOE methane loss + albedo    | Snowball Earth, ocean chemistry under ice                 |
| **Sturtian Snowball**     | 717–660     | Low CO₂ + continental config | Global glaciation, ocean anoxia, deglaciation pulse       |
| **Marinoan Snowball**     | 650–635     | Ice-albedo feedback          | Snowball collapse, cap carbonates, Ediacaran radiation    |
| **End-Ordovician**        | 445         | Glaciation + volcanism       | Cooling, marine habitat loss, two-phase extinction        |
| **Late Devonian**         | 372         | Volcanism, anoxia, plants    | Ocean anoxia, reef collapse, nutrient loading             |
| **Capitanian**            | 260         | Emeishan Traps               | Ocean acidification, tropical reef loss                   |
| **End-Permian**           | 252         | Siberian Traps cascade       | CO₂/CH₄, anoxia, ozone destruction, >90% loss             |
| **End-Triassic**          | 201         | CAMP volcanism               | 3–4°C warming, acidification, ecosystem turnover          |
| **K-Pg**                  | 66          | Chicxulub + Deccan Traps     | Impact winter, acid rain, food web collapse               |
| **PETM**                  | 56          | Rapid carbon release         | 5–8°C warming, deep-sea anoxia, mammalian dispersal       |
| **Quaternary megafauna**  | 0.05–0.01   | Human hunting + climate      | Overkill, trophic downgrading, vegetation shifts          |
| **Holocene/Anthropocene** | 0           | Human activity               | Habitat loss, overexploitation, 6th extinction            |

Generic scenarios: GRB (ozone → UV → phytoplankton), supernova (cosmic rays → ozone), supervolcano (Toba-class cooling), pandemic (population → trophic → land use).

### Extinction Cascade Pipeline

```text
EXTINCTION TRIGGER
    │
    ├─► [IMPACT] Atmospheric injection ──► Atmosphere A1/A2 (aerosol cooling)
    ├─► [IMPACT] Thermal pulse ──► Fire F2/F3 (global firestorms)
    ├─► [VOLCANIC/LIP] CO₂/CH₄/SO₂ ──► Atmosphere (warming) + Ocean (acidification/anoxia)
    ├─► [ATMOSPHERIC] O₂ revolution ──► Biogeochem (redox) + Evolution (anaerobe extinction)
    ├─► [GLACIATION] Ice-albedo cascade ──► Cryosphere (Snowball) + Ocean (sub-ice chemistry)
    ├─► [RADIATION] GRB/supernova ──► Atmosphere (ozone) + Trophic (phytoplankton collapse)
    ├─► [ALL DRIVERS] Biosphere response ──► Trophic (food web collapse) + Evolution (mass extinction)
    └─► Long-term recovery ──► Ecology + Biogeochem + Evolution (niche refilling, recolonization)
```

### Planetary Defense Agent

1. **Monitor** — NEO catalog ranked by Palermo scale
2. **Simulate** — Impact cascades through full coupled ESM
3. **Model all drivers** — Volcanic, atmospheric, glaciation, cosmic, anthropogenic
4. **Assess** — Biosphere impact: crop failure, biodiversity loss, trophic cascade, recovery timeline
5. **Deflect** — Kinetic impactor, gravity tractor, ion beam; optimize intercept
6. **Calibrate** — Historical extinctions (all types) validate cascade modeling against paleo records
7. **Report** — Threat assessments, deflection windows, observation recommendations

## DOE EESM Alignment

### ESMD — Earth System Model Development

| Priority                | Capability                                             |
| ----------------------- | ------------------------------------------------------ |
| Coupled simulations     | SAPG compiler; conservation-checked coupling           |
| Scale-aware resolution  | Representation ladders: coarse → convection-permitting |
| Exascale readiness      | Multi-GPU; pluggable backends                          |
| Water/drought/extremes  | H0–H2, A0–A2, C0–C2                                    |
| Cloud–aerosol           | A1/A2 microphysics; R1/R2 feedbacks                    |
| Human systems           | HS0–HS3                                                |
| Performance portability | Rust traits; GPU/CPU/emulator backends                 |

### RGMA — Regional & Global Model Analysis

| Thrust                   | Capability                                                                 |
| ------------------------ | -------------------------------------------------------------------------- |
| Cloud processes          | Atmosphere + radiation ladders; A2A federation with cloud-resolving models |
| Biogeochemical feedbacks | B0–B2 coupled to hydrology + ecology; factorial experiments                |
| High-latitude            | C0–C2: permafrost, ice-sheet, sea-ice; Arctic regime tags                  |
| Variability & change     | Multi-decadal hindcasts with BMA; ensemble weighting                       |
| Extreme events           | Event-driven embedding; high-fidelity solvers                              |
| Water cycle              | H0–H2 + routing + snow + human water management                            |
| Model hierarchy          | Representation ladders ARE a model hierarchy                               |
| Uncertainty              | Bayesian structural learning; CRPS; information-loss tracking              |
| Benchmarking             | Ontology + Skill Store + Benchmarking Agent                                |
| Petascale data           | Zarr/COG; MPAS unstructured mesh                                           |

### MSD — MultiSector Dynamics

| Focus                 | Capability                                        |
| --------------------- | ------------------------------------------------- |
| Energy                | HS1/HS2: supply/demand, grid, generation, storage |
| Resources             | Coupled hydrology + ecology + human extraction    |
| Infrastructure        | HS2: power grid, water systems, cascading failure |
| Water–energy–land     | H↔E↔HS coupling with conservative exchange        |
| Supply chains         | HS2/HS3: commodity flow, trade routes             |
| Land use              | HS1+: transition matrices, urbanization feedback  |
| Compounding stressors | Event embedding + MSD coupling                    |
| Scenario discovery    | Active Learning + Optimizer; tipping points       |
| Digital testbeds      | Compile-and-adapt: region → coupled model         |

## A2A Federation

```text
┌──────────────────────────────────────────────────────────────────────┐
│                       A2A FEDERATION LAYER                           │
│                                                                      │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐             │
│  │  MAESMA       │   │  MAESMA       │   │  External     │            │
│  │  Instance A   │◄─►│  Instance B   │◄─►│  Agent        │            │
│  │  (Land/Fire)  │   │  (Ocean/Ice)  │   │  (IAM/Econ)   │            │
│  └──────┬───────┘   └──────┬───────┘   └──────┬───────┘             │
│         │                  │                   │                      │
│         ▼                  ▼                   ▼                      │
│  ┌─────────────────────────────────────────────────────────┐         │
│  │              A2A Gateway Agent                           │         │
│  │  • Agent Card registry (publish + discover)             │         │
│  │  • Task lifecycle (submitted → working → completed)     │         │
│  │  • Artifact exchange (IR, skill records, manifests)     │         │
│  │  • Authentication + authorization per peer              │         │
│  └─────────────────────────────────────────────────────────┘         │
└──────────────────────────────────────────────────────────────────────┘
```

### Agent Cards

```json
{
  "name": "maesma-land-fire-pnw",
  "description": "PNW land surface + fire specialist",
  "url": "https://lab-a.example.org/.well-known/agent.json",
  "capabilities": {
    "process_families": ["fire", "hydrology", "ecology", "biogeochem", "radiation"],
    "scale_envelope": {"dx_min": "5m", "dx_max": "250km"},
    "regime_expertise": ["maritime-conifer", "post-fire", "snow-dominated", "WUI"],
    "available_skill_records": 12847,
    "accepts_tasks": ["benchmark", "assemble", "score", "calibrate", "discover_data"]
  }
}
```

### Task Types

| Type            | Description                                 | Artifacts                 |
| --------------- | ------------------------------------------- | ------------------------- |
| `assemble`      | Propose representations for a family/regime | Process Graph IR fragment |
| `benchmark`     | Score against remote observations           | Skill records             |
| `calibrate`     | Parameter estimation with remote data       | Posterior distributions   |
| `score`         | Compare outputs against remote holdings     | Skill vector              |
| `discover_data` | Search remote catalogs                      | Product manifests         |
| `share_skill`   | Exchange accumulated skill records          | Anonymized batches        |
| `propose_rung`  | Suggest new representation for a gap        | Candidate manifest        |

### Federated Assembly

1. **Discover** — Query peer Agent Cards for capability map
2. **Delegate** — Route process families to best-qualified instance
3. **Compose** — Graft IR fragments; validate cross-boundary conservation
4. **Execute** — Run sub-models remotely or colocate; exchange boundary conditions
5. **Share** — Skill records flow back via A2A; federated learning without raw data

Cross-instance skill sharing: anonymized records (config hash + metrics only), trust-weighted Bayesian incorporation, differential privacy safeguards. Over time, the community builds a distributed posterior over model structures.

## Monitoring Dashboard

| View                  | Content                                                            | Update            |
| --------------------- | ------------------------------------------------------------------ | ----------------- |
| **Agent Workflows**   | Live task DAG + reasoning traces                                   | WebSocket         |
| **Optimization**      | Pareto frontier animation; convergence; posterior entropy          | Per cycle         |
| **Skill Evolution**   | Skill time-series per family × region × regime                     | Per benchmark     |
| **Data Ingestion**    | Discovered/ingested datasets; coverage; novelty                    | Per event         |
| **Process Discovery** | Residual analyses; hypotheses; learned representations             | Per cycle         |
| **Provenance**        | Full dependency graph for any configuration                        | On demand         |
| **Regime Map**        | Geographic regime boundaries + optimal rungs                       | Per regime cycle  |
| **Federation**        | A2A peers; shared skills; federated assembly                       | Per A2A event     |
| **Process Evolution** | Survival tier distribution; fitness over generations; ALife events | Per generation    |
| **Foundation Models** | Earth-2 ensemble status; model weights; bias correction            | Per inference     |
| **Observation Intel** | Satellite tasking; anomaly detections; coverage gaps               | Per observation   |
| **Geoengineering**    | Interventions; setpoint tracking; termination risk                 | Per control cycle |
| **Planetary Defense** | NEO catalog; impact probabilities; deflections                     | Per update        |

```text
Agent Swarm (Rust) ──► Event Bus (async, append-only) ──► Next.js (SSR + WebSocket)
                                    │
                                    ▼
                           Event Store (SQLite/Postgres) + Skill Store
```

Optional steering panel: adjust objectives, weights, budgets, allowlists. Changes take effect next cycle without restart.

## Data Plane

Governed, reproducible data pipeline:

| Agent                 | Function                                                        |
| --------------------- | --------------------------------------------------------------- |
| **Data Requirements** | Read compiled IR → acquisition plan                             |
| **Data Discovery**    | Search STAC catalogs; select best per region/resolution/license |
| **Data Acquisition**  | Download with rate limiting, retries, checksums                 |
| **Preprocessing**     | Reprojection, resampling, tiling, cloud masking → Zarr/COG      |
| **QA/QC**             | Completeness, outliers, uncertainty, fallback triggers          |
| **Provenance**        | Source IDs, transforms, timestamps → reproducible data BOM      |
| **Streaming**         | NRT subscriptions (active fire, weather); `DataUpdateEvent`     |

## Execution Model

### Multi-GPU

| Device    | Workload                                      |
| --------- | --------------------------------------------- |
| **GPU 0** | Radiation + energy balance + fuel moisture    |
| **GPU 1** | Hydrology kernels (soil column + routing)     |
| **GPU 2** | Event solvers (fire, embedded high-res hydro) |
| **CPU**   | Ecology/competition + BGC (branchy logic)     |

Embedded domains allocate scratch on GPU; transfer only boundaries and summarized outputs. Asynchronous streams with pinned buffer exchange.

### Spatial Representations

1. **Raster grid** — Main landscape fields
2. **River network graph** — Routing
3. **Embedded raster** — Fire / high-res hydro event solvers

## Project Structure

```text
core/
  registry/           # Variables, units, semantics
  graph_ir/           # Process graph IR
  compiler/           # Rung selection, closure, schedule generation
  runtime/            # Task scheduler, device manager, event bus
  scoring/            # Skill metrics, comparison protocols
  optimizer/          # Fitness-driven selection loop
  a2a/                # A2A protocol: agent cards, tasks, artifacts
knowledgebase/
  inference/          # Neural inference engine (graph transformer)
  index/              # Unified index over code, manifests, skills
  embeddings/         # Process + error + regime embeddings
  migrations/         # KB schema evolution
ontology/
  schema/             # Type + relation definitions
  processes/          # Process ontology instances
  datasets/           # Dataset ontology instances
  metrics/            # Metric ontology instances
  graph/              # Compiled graph index
process_registry/
  schema/             # Manifest schema
  hydrology/          # H0–H2 manifests
  fire/               # F0–F3 manifests
  ecology/            # E0–E2 manifests
  biogeochem/         # B0–B2 manifests
  radiation/          # R0–R2 manifests
  atmosphere/         # A0–A3 manifests
  ocean/              # O0–O2 manifests
  cryosphere/         # C0–C2 manifests
  human_systems/      # HS0–HS3 manifests
  trophic_dynamics/   # TD0–TD2 manifests
  evolution/          # EV0–EV2 manifests
modules/
  radiation/          # R0, R1
  hydrology/          # H0, H1, routing
  ecology/            # E0
  biogeochem/         # B0, B1
  fire/               # F1, F2
  atmosphere/         # A0, A1
  ocean/              # O0, O1
  cryosphere/         # C0, C1
  human_systems/      # HS0, HS1
  trophic_dynamics/   # TD0, TD1
  evolution/          # EV0, EV1
operators/
  remap/              # Conservative remapping
  projection/         # State transforms between rungs
  coupling/           # Cross-component operators
benchmark/
  observations/       # Observation registry manifests
  skill_store/        # Append-only skill database
  hypotheses/         # Configuration experiments
  experiments/        # Runners, emulators, BMA
  diagnostics/        # ILAMB, IOMB, E3SM Diags
scenarios/
  data_manifests/     # Forcing sources, parameter priors
  msd/                # MSD scenario definitions
agents/
  a2a_gateway/        # Peer discovery, task routing
  msd_coupling/       # Natural ↔ human coupling
  scenario_discovery/ # AI-driven scenario exploration
  eesm_diagnostics/   # Multi-component evaluation
  process_discovery/  # Residual analysis, ML learning
  geoengineering/     # Intervention optimization
  planetary_defense/  # NEO tracking, impact modeling
  trophic_dynamics/   # Food web assembly
  evolution/          # Eco-evolutionary dynamics
geoengineering/
  interventions/      # Intervention manifests
  strategies/         # Evaluated strategy records
  control/            # Control law implementations
  termination/        # Termination shock analysis
planetary_defense/
  neo_catalog/        # NEO data ingest
  extinctions/        # Mass extinction scenarios
  impact_models/      # Impact cascade pipeline
  deflection/         # Deflection strategy modeling
discovery/
  residual_analysis/  # Bias detection + attribution
  learners/           # Neural operators, symbolic regression
  validators/         # Conservation, stability, generalization
  manifest_gen/       # Auto-manifest generation
dashboard/
  app/                # Next.js pages
  components/         # React components
  lib/                # Event store client, WebSocket
reference/            # Existing ESMs for study
```

## Design Principles

1. **Agents are the system** — Primary actors at every lifecycle stage; intelligence in the swarm, not configuration
2. **Salient dynamics first** — Prioritize processes with the greatest effect on time evolution of system states; add detail incrementally as budget and accuracy demand
3. **Never idle** — Every simulation advances the posterior; always a next hypothesis to test
3. **Continual, not one-shot** — Model selection evolves with observations, regimes, and discoveries
4. **Declarative contracts** — Modules declare I/O, scale, conservation; compiler wires; agents reason over contracts
5. **Conservation enforced** — Mass/energy preserved in remapping, aggregation, rung transitions
6. **Information-loss tracked** — Every downscale/downgrade attaches uncertainty; drives upgrade decisions
7. **Hysteresis switching** — Rung transitions use hold timers, not single-timestep flipping
8. **Provenance by default** — Every model + data artifact: hashed, dependency-graphed, reproducible
9. **Bayesian structural learning** — Posterior over structures, not just parameters; BMA for predictions
10. **Knowledgebase-first extensibility** — New representation = manifest + code + provenance → deposit into knowledgebase → auto-benchmark + integrate
11. **Federated by design** — A2A collaboration without centralizing proprietary data
12. **Human–natural coupling** — Human systems are first-class process families, not boundary conditions
13. **AI processes are first-class** — Discovered representations carry full manifests, provenance, validation
14. **Discovery deposits into knowledgebase** — Residual analysis every cycle; learned representations deposited into central knowledgebase for subsequent integration
15. **Knowledgebase is the config** — No hard-coded resources; everything discoverable via ontology-indexed knowledgebase queries
16. **Monitoring never blocks** — Dashboard observe-only; steering takes effect next cycle
17. **Geoengineering = control** — Feedback loop with stability verification, not static scenarios
18. **Planetary defense = full coupling** — Extinctions through complete ESM; paleo-calibrated
19. **Trophic/evolution are planetary** — Food webs, eco-evolutionary dynamics are first-class
20. **All timescales** — Seconds (fire) through millions of years (extinction recovery)
21. **Neural inference over knowledgebase** — Process selection driven by a learned neural model reasoning over the knowledgebase; errors and uncertainties are the primary input signals
22. **Knowledgebase grows continuously** — Every discovered process, every skill record, every observation feeds back into the central store; the system's knowledge compounds over time
23. **Processes are alive** — Process models are living organisms with survival tiers, constitutional invariants, heartbeat monitoring, self-replication, and phylogenetic lineage; they earn existence through demonstrated skill

## Technology Stack

| Component         | Technology                                                                                 |
| ----------------- | ------------------------------------------------------------------------------------------ |
| Language          | Rust (traits for contracts, strong typing, memory safety)                                  |
| Manifests         | YAML/JSON + unit schemas                                                                   |
| Execution         | Pluggable backends: CPU solvers, GPU kernels, ML emulators                                 |
| Data processing   | GDAL, xarray, Zarr, COG                                                                    |
| Orchestration     | Ray or Dask                                                                                |
| Ontology          | In-memory property graph + serialized manifests; optional RDF/OWL                          |
| Neural inference  | Graph transformer (PyTorch Geometric / DGL); process + error embeddings                    |
| Collaboration     | A2A protocol                                                                               |
| Diagnostics       | ILAMB, IOMB, E3SM Diagnostics                                                              |
| ML learning       | PyTorch/JAX (FNO, DeepONet); PySR/gplearn; PINNs                                           |
| Dashboard         | Next.js (App Router, WebSocket, Recharts/D3)                                               |
| Event store       | SQLite (dev) / PostgreSQL (prod)                                                           |
| NEO data          | JPL Horizons/CNEOS/Sentry, MPC, USAF/USSF feeds                                            |
| Control systems   | MPC, PID, RL (PPO/SAC)                                                                     |
| Evolution         | Trait-based models, quantitative genetics; PBDB, TimeTree                                  |
| ALife / evolution | automaton-inspired process lifecycle; survival tiers, constitution, heartbeat, replication |
| GPU acceleration  | wgpu 24.x, cudarc, NCCL; multi-GPU kernel dispatch                                         |
| Neural operators  | NVIDIA Modulus (FNO, PINO, DeepONet, MeshGraphNet)                                         |
| Foundation models | NVIDIA Earth-2/earth2studio (FourCastNet, Pangu-Weather, GraphCast, GenCast, CorrDiff)     |
| Edge AI           | PhiSat-2-inspired autonomous observation; on-board anomaly detection                       |

## Reference Models

Available in `reference/` for study: CESM, E3SM, WRF-SFIRE, ParFlow, iLand, FATES, LPJ-GUESS, NoahMP, Landlab, Badlands, ORCHIDEE, VIC, WRF-Hydro, NOAA GFDL ESM4, PEcAn.

## License

TBD
