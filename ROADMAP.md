# ROADMAP

Implementation roadmap for MAESMA. Phases build toward a fully autonomous agent swarm that reasons over a central Process Knowledgebase to assemble, evaluate, optimize, and invent process models without human intervention. Tasks are ordered by dependency.

---

## Phase 0 — Foundations

### 0.1 Variable Registry & Unit System
- [x] Define canonical variable registry schema (name, units, dimensionality, semantics, bounds, uncertainty)
- [x] Implement unit-aware type system with compile-time checking
- [x] Define shared forcing variables: `P`, `Tair`, `RH/VPD`, `Wind`, `SWdown`, `LWdown`, `CO2`
- [x] Define shared slow state variables: `LAI`, `canopy_height`, `CBD/CBH`, fuel loads, soil moisture/temp profiles, `SWE`, soil C/N pools, `streamflow`, `water_table_proxy`
- [x] Define shared disturbance state: `burn_severity`, `mortality_fraction`, `soil_hydrophobicity`, `char_fraction`, `ash_nutrient_pulse`
- [x] Implement bounds checking, update-authority declarations, and discretization metadata

### 0.2 Process Module Manifest Schema
- [x] Design manifest format (YAML/JSON): process name, category, scale envelope, I/O signatures, assumptions, closures, conservation, numerical requirements, calibration parameters, data requirements, backends, skill model, ontology links
- [x] Write manifest examples for 2–3 process rungs
- [x] Build manifest parser, validator, and JSON/YAML schema

### 0.3 Process Knowledgebase
- [x] Define knowledgebase directory layout (family → process → rung: code + manifests + skill records)
- [x] Implement KB loader: discover, parse, validate, build in-memory index over code + manifests + ontology links + skill records
- [x] Implement KB query API: list representations, filter by scale/regime/budget/backend, rank by skill/cost, return compatibility edges
- [x] Implement `maesma kb validate` and `maesma kb check-closure` CLI commands
- [x] Implement KB deposit API: accept new representations (hand-coded or discovered) with manifest + provenance
- [x] Populate initial knowledgebase entries:
  - Hydrology: H0 bucket, Green-Ampt, H1 Richards 1D, kinematic wave routing
  - Fire: F0 stochastic, F1 Rothermel + CFS FBP cellular, severity/mortality effects
  - Ecology: E0 cohort mosaic, fuel strata derivation
  - Biogeochem: B0 big-leaf C, B1 multi-pool C/N
  - Radiation: R0 daily potential solar, R1 sub-daily energy balance
  - Atmosphere: A0 prescribed, A1 WRF-like downscaling
  - Ocean: O0 slab mixed-layer, O1 z-coordinate regional
  - Cryosphere: C0 degree-day melt, C1 energy-balance snowpack + sea-ice
  - Human Systems: HS0 exogenous scenarios, HS1 sectoral demand/supply
  - Trophic Dynamics: TD0 static food-web, TD1 dynamic Lotka-Volterra
  - Evolution: EV0 fixed traits, EV1 trait-mediated community assembly

### 0.4 Process Graph IR
- [x] Define IR types (Rust structs/traits): `StateVariable` nodes, `ProcessRepresentation` edges, constraints, schedule, embedding declarations
- [x] Implement IR serialization/deserialization
- [x] Implement graph traversal and query utilities

### 0.5 Canonical Spatial Representations
- [x] Implement raster grid, river network graph, and embedded raster subdomain representations
- [x] Define topology adapters (grid ↔ mesh ↔ network ↔ patch mosaic)

### 0.6 Observation Registry
- [x] Define observation dataset manifest schema: observable name, spatiotemporal coverage, uncertainty, compatibility constraints, scoring protocol
- [x] Register initial datasets:
  - USGS gauges, FLUXNET towers, MTBS/RAVG, FIA plots, SMAP, MODIS LAI/ET/NDVI, GEDI canopy height
  - ARGO + satellite altimetry, NSIDC sea-ice/GRACE, CERES TOA, ARM sites
  - EIA energy, GBIF/eBird, NEON surveys, FAO fisheries, TRY/BIEN traits
  - PBDB fossil record, TimeTree phylogenies, Quaternary pollen records
  - CNEOS Sentry NEO catalog, MPC orbital elements
- [x] Implement observation access adapters (point extraction, spatial averaging, temporal alignment)

### 0.7 Skill Metric Library
- [x] Implement: RMSE, MAE, bias, correlation (Pearson/Spearman), KGE + decomposition, CRPS, timing errors, conservation residuals
- [x] Implement multi-objective scoring aggregation (weighted + Pareto)
- [x] Define comparison protocols per observation type

---

## Phase 1 — Compiler & Validators

### 1.1 Closure & Consistency Checker
- [x] Unit consistency, state-space closure, double-counting detection
- [x] Conservation compatibility, boundary condition consistency, numerical stability (CFL, stiffness)

### 1.2 Model Compiler
- [x] Process set selection from knowledgebase via neural inference
- [x] Rung selection per family given scale/regime/budget
- [x] Discretization + coupling strategy selection (operator splitting vs monolithic)
- [x] Executable schedule generation (stepping order, coupling frequency, remapping operators)

### 1.3 Conservative Remapping & Projection
- [x] Grid↔grid conservative remapping (aggregation + disaggregation)
- [x] Grid↔network transfer, state projection (individual↔cohort), temporal coupling operators
- [x] Information-loss tracking for every projection/aggregation

---

## Phase 2 — Runtime & Scheduler

### 2.1 Task Scheduler
- [x] Task graph execution engine with coupling cadence management
- [x] Asynchronous subcycling for fast physics in subdomains
- [x] Device assignment (GPU 0/1/2 + CPU) with pinned buffer exchange

### 2.2 Event-Driven Embedding
- [x] Ignition → spawn F1/F2 fire solver in bounding box
- [x] Extreme rain → upgrade H0→H1/H2 in affected basins
- [x] Drought → upgrade R0→R1 locally
- [x] Management action → spawn disturbance operator
- [x] Hysteresis management: hold timers, graceful downshift
- [x] Embedded subdomain GPU allocation + boundary transfer

### 2.3 Error & Uncertainty Monitoring
- [x] Per-process, per-region error monitors
- [x] Refinement triggers (event-driven, threshold-based, regime-based)
- [x] Information-loss accumulation tracking
- [x] Rung upgrade/downgrade decision engine

### 2.4 Skill Score Store
- [x] Append-only, versioned schema: config hash, rungs, coupled context, region, regime, season, metric vector, observation IDs, provenance
- [x] Backend (embedded DB or indexed Parquet/Arrow)
- [x] Query API: filter by rung/region/regime/context; aggregate across dimensions
- [x] Versioning: track skill evolution as modules, parameters, or data improve

### 2.5 Disturbance Operator Pipeline
- [x] Fire severity → canopy loss, dead fuel, hydrophobicity, C/N emissions + char
- [x] Disturbance footprint application to slow-tier ecology/BGC state
- [x] Post-disturbance spin-up and stabilization

### 2.6 Neural Inference Engine
- [x] Graph transformer architecture over Process Knowledgebase: process manifests, skill vectors, error fields, regime context as node/edge features
- [x] Embedding pipeline: encode process manifests, skill records, spatiotemporal error fields, regime tags, compute budgets
- [x] Inference API: given current error signals + regime + budget, propose process selections, assembly configurations, and representation gaps
- [x] Training loop: skill score deltas from accepted proposals as reward signal; incremental retraining as KB grows
- [x] Uncertainty-aware outputs: calibrated confidence per proposal; low-confidence routes to Active Learning
- [x] Integration with Knowledgebase Retrieval Agent and Autonomous Optimizer Agent

---

## Phase 3 — Process Modules

### 3.1 Radiation & Energy Balance
- [x] **R0**: Daily potential solar (slope/aspect + latitude) + empirical canopy attenuation
- [x] **R1**: Sub-daily SW partitioning + LW + canopy energy balance (hourly)
- [x] Fuel moisture drying model (coupled to R0/R1)

### 3.2 Hydrology
- [x] **H0**: Bucket + curve-number runoff + simple baseflow (km, daily)
- [x] **H1**: Multi-layer Richards + infiltration (30–300 m, 5–60 min)
- [x] **Routing**: Kinematic wave → diffusive wave
- [x] Post-fire runoff/erosion risk index

### 3.3 Ecology & Succession
- [x] **E0**: Cohort mosaic (species × age/biomass cohorts, 30–250 m, annual)
- [x] Fuel strata derivation from stand structure; crown fire threshold

### 3.4 Biogeochemistry
- [x] **B0**: Big-leaf carbon + simple soil pools
- [x] **B1**: Multi-pool C/N + litter + microbial decomposition (daily)
- [x] Fire coupling: combustion emissions, char/black carbon, post-fire mineralization

### 3.5 Fire Behavior & Effects
- [x] **F1**: Rothermel + CFS FBP cellular spread with wind/topo/fuels (10–100 m, 1–10 min); Rothermel for surface fire ROS, CFS FBP for crown fire initiation/spread
- [x] Disturbance output: burn perimeter, severity, fuel transitions, heat pulse, hydrophobicity, ash, canopy loss
- [ ] **F2**: Wind-aware spread with plume feedback (5–50 m, sec–min); physics-based models (Balbi, level-set) — *(stretch)*

### 3.6 Atmosphere
- [x] **A0**: Prescribed reanalysis (ERA5-Land, AORC, NLDAS-2); interpolation to model grid
- [x] **A1**: Regional downscaling (WRF-like, 5–25 km, min); bdy conditions from reanalysis/GCM
- [ ] **A2**: Non-hydrostatic convection-permitting (1–4 km, sec); cloud microphysics + aerosol — *(stretch)*
- [x] Coupling: atmosphere → radiation, hydrology, fire

### 3.7 Ocean
- [x] **O0**: Slab mixed-layer + prescribed SST/sea-ice (1°, daily)
- [x] **O1**: z-coordinate regional (0.25°, hourly); thermohaline, mixed-layer, coastal
- [ ] **O2**: Eddy-resolving MPAS-Ocean (1–10 km, min); mesoscale, ocean biogeochem — *(stretch)*
- [x] Coupling: ocean ↔ atmosphere, cryosphere

### 3.8 Cryosphere
- [x] **C0**: Degree-day melt (km, daily); snowpack SWE, simple glacier mass balance
- [x] **C1**: Energy-balance snow (multi-layer, 30–300 m, hourly) + sea-ice thermo; albedo feedbacks
- [ ] **C2**: Dynamic ice-sheet + sea-ice rheology (10 km, min) — *(stretch)*
- [x] Coupling: cryosphere → hydrology, ocean, atmosphere

### 3.9 Human Systems (MSD)
- [x] **HS0**: Exogenous scenarios (SSPs/RCPs, national, annual)
- [x] **HS1**: Sectoral demand/supply (energy, water, agriculture; regional, monthly; reads climate impacts)
- [ ] **HS2**: Agent-based infrastructure (power grid, water, transport; county, hourly; cascading failure) — *(stretch)*
- [ ] **HS3**: Fully coupled IAM (global economy + technology + emissions) — *(stretch)*
- [x] Coupling: human ↔ hydrology, ecology, atmosphere

### 3.10 Trophic Dynamics
- [x] **TD0**: Static food-web topology + fixed trophic efficiencies (biome, annual)
- [x] **TD1**: Dynamic multi-guild Lotka-Volterra (landscape, monthly); Holling II/III, allometric scaling, density-dependent mortality
- [ ] **TD2**: Individual-based predator-prey with body-size structure (patch, daily) — *(stretch)*
- [x] Coupling: trophic ↔ ecology, biogeochem, evolution, hydrology, ocean
- [x] Validation: GBIF, eBird, NEON, FAO fisheries, stable isotope studies, biomass pyramids

### 3.11 Evolution
- [x] **EV0**: Fixed trait distributions (PFT level, static)
- [x] **EV1**: Trait-mediated assembly + adaptive shifts (population, decadal); SLA, wood density, seed mass, drought/fire tolerance, thermal optimum
- [ ] **EV2**: Genotype-phenotype + selection + speciation (individual, generational); quantitative genetics — *(stretch)*
- [x] Coupling: evolution ↔ ecology, trophic, disturbance, biogeochem
- [x] Validation: PBDB, TimeTree, TRY, BIEN, LTER, Quaternary pollen records

---

## Phase 4 — Unified Ontology

### 4.1 Ontology Schema
- [x] Process Domain: `ProcessFamily`, `Process`, `Representation`, `Module`, `StateVariable`, `Assumption`, `Constraint`, `ScaleEnvelope`, `NumericalForm`, `CouplingOperator`
- [x] Dataset Domain: `Observable`, `Product`, `CatalogSource`, `AccessSpec`, `TransformRecipe`, `QualitySpec`, `License`, `LatencyClass`
- [x] Metric Domain: `Metric`, `ScoringProtocol`, `FitnessFunction`, `SkillRecord`, `SkillModel`, `CostModel`
- [x] Cross-domain relations: `requires_forcing`, `calibrates_against`, `evaluated_by`, `measured_by`, `validates`, `uses_metric`, `scores`, `discovered_via`, `penalizes`
- [x] Data discovery relations: `potentially_measures`, `discovery_query`, `relevance_score`, `novelty_score`
- [x] In-memory property graph from YAML manifests; cross-domain query API
- [x] Schema files: `process_schema.yaml`, `dataset_schema.yaml`, `metric_schema.yaml`, `cross_domain_schema.yaml`, `geoengineering_schema.yaml`, `planetary_defense_schema.yaml`

### 4.2 Geoengineering Ontology Domain
- [x] Classes: `Intervention`, `ControlTarget`, `InterventionSchedule`, `SideEffectConstraint`, `TerminationScenario`, `StrategyRecord`, `InterventionCostModel`
- [x] Relations: `actuates_via → ProcessFamily`, `measured_by → Observable`, `evaluated_by → ScoringProtocol`, `monitors → Observable`, `tests → StrategyRecord`
- [x] Initial intervention manifests: SAI (SO₂, CaCO₃), MCB, OAE (olivine, lime), DAC, SRM, iron fertilization
- [x] Control targets: T_global, T_Arctic, pH_ocean, RF_total, precipitation_regional

### 4.3 Planetary Defense Ontology Domain
- [x] Classes: `NearEarthObject`, `ImpactScenario`, `ExtinctionEvent`, `DeflectionStrategy`, `ThreatAssessment`, `BiosphereImpact`, `RecoveryTrajectory`
- [x] Relations: `triggers → ProcessFamily`, `calibrates → ImpactScenario`, `assessed_by → ThreatAssessment`, `mitigates → NearEarthObject`, `modeled_by → (Trophic, Evolution, Ecology, Biogeochem)`, `validated_by → ExtinctionEvent`
- [x] Historical extinction records: K-Pg, P-T, Late Devonian, End-Triassic, End-Ordovician
- [x] NEO data sources: CNEOS Sentry, Scout, Horizons, MPC, ATLAS, CSS, Pan-STARRS, LSST/Rubin

### 4.4 Initial Population
- [x] Catalog 30–60 processes; define 2 rungs each for ~15 key processes
- [x] Register 10–20 observation products, 5–10 forcing products, catalog sources (NASA CMR, STAC, Copernicus, CKAN)
- [x] Register standard metrics + scoring protocols + default fitness functions
- [x] Encode cross-domain linkages: fire↔ecology, radiation↔fuel moisture, hydrology↔BGC, trophic↔ecology/ocean, evolution↔disturbance, geoengineering→process families, planetary defense→cascade processes

### 4.5 Skill & Cost Model Annotations
- [x] Expert-prior skill models per Representation: accuracy class per regime, known failure modes
- [x] Cost models: FLOPS/cell/timestep, memory, GPU/CPU preference, scaling behavior
- [x] `compatible_with`/`incompatible_with` edges
- [x] Mark all as updatable by the knowledgebase learning loop (distinguish expert priors from empirical posteriors)

---

## Phase 5 — AI Agent Swarm

### 5.1 Intent & Scope Agent
- [x] Parse user objectives → observable requirements + error bands + priority tiers

### 5.2 Knowledgebase Retrieval Agent
- [x] Query Process Knowledgebase via neural inference engine for candidates given errors, scale/regime/budget/data
- [x] Rank candidates by inferred error-reduction potential, cost/skill tradeoff

### 5.3 Model Assembly Agent
- [x] Build candidate SAPG from neural inference engine proposals; pick rungs, declare embeddings, propose coupling cadence

### 5.4 Closure & Consistency Agent
- [x] Automated validation: missing variables, double-counted physics, unit mismatches, conservation violations, CFL

### 5.5 Provenance & Audit Agent
- [x] Generate "why this model" reports: selections, rejections, sensitivity hotspots

### 5.6 Benchmarking Agent
- [x] Run/schedule simulations; extract outputs at observation locations; compute multi-metric skill; write to Store; compare against existing records

### 5.7 Model Selection Agent
- [x] Neural inference + posterior $p(M_k | \mathbf{y}) \propto p(\mathbf{y} | M_k)\,p(M_k)$; marginal likelihoods; online Bayesian updating; BMA ensemble weights
- [x] Inference engine proposes candidates; Bayesian framework validates and updates posteriors

### 5.8 Active Learning Agent
- [x] Identify high-uncertainty configurations, under-observed regimes, sensitivity frontiers
- [x] Propose transferability tests (cross-region validation)
- [x] Output prioritized experiment queue ranked by expected information gain

### 5.9 Skill Librarian Agent
- [x] Manage Store lifecycle (writes, queries, versioning, GC); aggregate across dimensions; detect skill drift; generate trend reports

### 5.10 Autonomous Optimizer Agent
- [x] Fitness function: $F(r,g,\ell) = \sum w_m S_m(r,g,\ell) - \lambda C(r)$
- [x] Per-region, per-regime loop: neural inference queries KB → compute fitness → Pareto frontier → swap if dominant → delegate to Active Learning if uncertain
- [x] Convergence detection, budget-aware mode, regime-aware mode; full provenance logging

### 5.11 Data Scout Agent
- [x] Gap analysis: identify observables/regions/periods with unreliable skill estimates; score by information gain
- [x] Catalog search: STAC, NASA CMR, Copernicus, CKAN with spatial/temporal/keyword filters
- [x] Relevance scoring: overlap, resolution compatibility, uncertainty adequacy, variable mapping
- [x] Novelty scoring: new coverage, regime coverage, independence from existing products
- [x] Ingest pipeline: auto-generate Product manifest + TransformRecipe → preprocess → deposit into KB → re-score
- [x] Governance: allowlisted sources, license check, rate limiting, optional first-ingest approval

### 5.12 A2A Gateway Agent
- [x] Agent Card schema + publication at `/.well-known/agent.json`
- [x] Peer discovery, capability caching, health status
- [x] Task lifecycle: `submitted → working → input-required → completed / failed / canceled`
- [x] Artifact exchange (IR fragments, skill records, manifests)
- [x] Auth (bearer tokens, mutual TLS), trust scoring, rate limiting, circuit breakers

### 5.13 MSD Coupling Agent
- [x] Bidirectional coupling: natural → human (climate impacts) and human → natural (emissions, land use, water extraction)
- [x] Coupling frequency negotiation; commodity/resource accounting
- [x] Cascading impact propagation: drought → water → energy → economic loss
- [x] MSD scenario parameter space (SSP × RCP × technology × policy)

### 5.14 Scenario Discovery Agent
- [x] AI-driven exploration: Latin hypercube / Sobol sampling, cluster outcomes, identify tipping points
- [x] Sensitivity analysis, scenario trees, Dynamical Digital Testbeds
- [x] Extreme event compounding analysis (drought + heatwave + infrastructure failure)

### 5.15 EESM Diagnostics Agent
- [x] ILAMB, IOMB, E3SM Diagnostics, PMP wrappers
- [x] Multi-component evaluation campaigns
- [x] RGMA thrust diagnostics: cloud, BGC, high-latitude, variability, extremes, water cycle
- [x] Standardized reports (HTML/PDF) with CMIP baselines

### 5.16 Process Discovery Agent
- [x] Structured residual analysis: systematic bias detection, attribution (variables, regions, seasons, regimes), persistence test, severity scoring
- [x] Hypothesis generation: classify residual → process type; query literature + A2A peers; hypothesis registry
- [x] ML learners: FNO/DeepONet, symbolic regression (PySR), hybrid physics+ML; physics-informed loss; train/val/test split by region
- [x] Validation gate: skill improvement, conservation (100-yr), stability when coupled, generalization, sensitivity
- [x] Auto-manifest generation: infer I/O, scale envelope, regime tags, cost/skill model; tag `origin: discovered`
- [x] Deposit into Process Knowledgebase with full manifest + provenance; immediately available for neural inference and selection
- [x] Lifecycle: `candidate → provisional → validated → production`; periodic re-validation, retraining, retirement
- [x] Discovery provenance: residual → hypothesis → training → validation → deployment chain

### 5.17 Geoengineering Strategy Agent
- [x] Control target registration (setpoint, tolerance, measurement source)
- [x] Forward simulation pipeline (50–500 yr through coupled ESM)
- [x] Multi-objective evaluation: cooling efficacy, side-effect penalties, economic cost, termination shock risk
- [x] Strategy optimization: MPC, RL (PPO/SAC), portfolio optimization
- [x] Termination shock: cessation simulation, rebound quantification, minimum safe ramp-down
- [x] Robustness: ECS range, SSP range, technology failure, tipping points
- [x] Pareto frontier: skill vs. cost vs. side effects vs. termination risk
- [x] Adaptive control laws (dynamic controllers, not static prescriptions)

### 5.18 Planetary Defense Agent
- [x] NEO ingest: CNEOS Sentry, Scout, Horizons, MPC, USAF SSN, USSF 18th SDS, ESA NEOCC, survey telescopes
- [x] Threat assessment: local catalog ranked by Palermo scale; configurable threshold; probability evolution; reports
- [x] Impact cascade: atmospheric injection, thermal pulse → firestorms, ocean impact → tsunami/acidification, nuclear winter → food web collapse, long-term recovery
- [x] Deflection: kinetic impactor, gravity tractor, ion beam, nuclear standoff; campaign optimization
- [x] Mass extinction calibration: K-Pg, P-T, Late Devonian, End-Triassic; compare modeled loss/recovery vs. PBDB
- [x] Observation campaign recommendations for orbit uncertainty reduction

### 5.19 Trophic Dynamics Agent
- [x] Food web assembly from diet matrices, stable isotopes, gut content; guild assignment; metabolic theory
- [x] Calibration: energy flow, biomass pyramids, functional response parameters
- [x] Coupling: trophic → ecology (herbivory, dispersal), → biogeochem (nutrient cycling), ↔ ocean, ↔ evolution, → human systems
- [x] Trophic cascade detection

### 5.20 Evolution Agent
- [x] Trait evolution: distributions (mean, variance, heritability) per population; selection, drift, gene flow
- [x] Speciation/extinction: reproductive isolation, fitness collapse, adaptive radiation, background extinction rate
- [x] Validation: diversification rates vs. TimeTree, trait trajectories vs. paleo record, post-extinction recovery timescales
- [x] Coupling: evolution → ecology, trophic, disturbance, climate, biogeochem

---

## Phase 6 — Data Plane

### 6.1 Data Ontology
- [x] Primitives: `Observable`, `Product`, `AccessMethod`, `License`, `LatencyClass`, `TransformRecipe`, `QualityFlags`
- [x] Link to process ontology: each Representation declares required observables + resolution + uncertainty tolerance

### 6.2 Data Contract Schema
- [x] `DataSpec`: variable, topology, extent, time range, cadence, latency, quality, provenance
- [x] `DataArtifact`: URI, format (COG/Zarr/NetCDF), chunking, QA mask, uncertainty, checksum, provenance

### 6.3 Data Agents
- [x] **Data Requirements** — IR → acquisition plan
- [x] **Data Discovery** — STAC/registry search; best sources per region/resolution/license
- [x] **Data Acquisition** — Download/stream with rate limiting, retries, checksums, content-addressed cache
- [x] **Preprocessing** — Reproject, resample, tile, cloud mask → Zarr/COG
- [x] **QA/QC** — Completeness, outliers, temporal gaps, uncertainty, fallback triggering

### 6.4 Data Governance
- [x] Allowlisted providers, credential vault, rate limiting, checksum + content hashing
- [x] License enforcement, deterministic processing (versioned recipes)

### 6.5 Initial Data Products
- [x] DEM (Copernicus/SRTM), Sentinel-2 L2A, VIIRS active fire (NRT), precipitation forcing, LAI/NDVI

---

## Phase 7 — Continual Optimization Loop

### 7.1 Combinatorial Hypothesis Engine
- [x] Structured enumeration (vary one family's rung); factorial experiments (interaction effects)
- [x] Budget-aware scheduling; configuration space pruning via consistency checker/posterior

### 7.2 Emulator-Accelerated Screening
- [x] Train surrogates from completed runs; pre-screen unpromising configurations
- [x] Track emulator fidelity; retrain as archive grows; uncertainty-aware elimination

### 7.3 Continuous Calibration
- [x] Hierarchical Bayesian: global → region → regime priors
- [x] Online parameter updating; identifiability diagnostics

### 7.4 Knowledgebase Feedback Loop
- [x] Update skill models (replace expert priors with posteriors)
- [x] Refine cost models with actual walltime/memory
- [x] Tighten/relax compatibility constraints from observed interactions
- [x] Shift default rung preferences per regime
- [x] Discover new regime tags, detect regime boundaries and drift
- [x] Sharpen parameter priors hierarchically; auto-update manifests

### 7.5 Experiment Orchestrator
- [x] Consume experiment queue from Active Learning; schedule across compute
- [x] Lifecycle: compile → run → score → store → update posteriors
- [x] Budget limits and diminishing-returns stopping criteria; full provenance

### 7.6 Autonomous Data Discovery Loop
- [x] Periodic gap analysis; integrate Data Scout with experiment orchestrator
- [x] New product → re-score affected representations → update active learning priorities
- [x] Autonomous regime discovery: cluster skill records, detect non-default outperformance, update ontology

---

## Phase 8 — A2A Federation

### 8.1 Protocol Infrastructure
- [x] A2A server (HTTP/SSE); `/.well-known/agent.json` serving
- [x] Peer registry with capability cache and health status
- [x] Mutual TLS + bearer tokens; A2A client for remote task dispatch

### 8.2 Federated Model Assembly
- [x] Capability-based delegation to best-qualified remote instance
- [x] IR fragment exchange; cross-boundary conservation validation
- [x] Remote execution (boundary conditions via streaming) and colocated modes

### 8.3 Cross-Instance Skill Sharing
- [x] Anonymized skill record format (config hash + metrics only)
- [x] Export/import with provenance; trust-weighted Bayesian incorporation
- [x] Differential privacy safeguards; community posterior dashboard

### 8.4 A2A-Enabled Data Discovery
- [x] `discover_data`, `share_skill`, `propose_rung` task types
- [x] A2A-mediated calibration (remote scoring against unique observations)

---

## Phase 9 — MultiSector Dynamics

### 9.1 Human System Coupling
- [x] HS0 one-way coupling (exogenous scenarios)
- [x] HS1 bidirectional via MSD Coupling Agent (climate impacts ↔ demand/extraction)
- [x] HS2 agent-based infrastructure: power grid, water systems, cascading failure — *(stretch)*

### 9.2 Water–Energy–Land Nexus
- [x] Coupled water-energy accounting; land-use change feedbacks; resource competition resolution

### 9.3 Dynamical Digital Testbeds
- [x] Testbed compiler: region + stressors → coupled model; evaluation against historical outcomes
- [x] DOE templates: coastal, western U.S. (fire+water+energy), Arctic

### 9.4 Scenario Space Exploration
- [x] Ensemble generation (SSP × RCP × technology × policy); AI-driven tipping point discovery
- [x] Scenario comparison dashboards; uncertainty propagation (climate → impact → decision)

---

## Phase 10 — EESM Diagnostics (RGMA)

### 10.1 Diagnostic Framework
- [x] Pluggable diagnostic packages (manifest + scoring + visualization)
- [x] CMIP-aligned output formatting (CMOR); automated campaigns after multi-component runs
- [x] Diagnostic scoring → Skill Score Store integration

### 10.2 RGMA Thrust Diagnostics
- [x] Cloud & Aerosol: cloud fraction, AOD, CRE, SW cloud feedback
- [x] Biogeochemical: global C budget, CO₂ flux, N₂O/CH₄, nutrient limitation
- [x] High-Latitude: permafrost, active layer, sea-ice, ice-sheet mass balance, Arctic amplification
- [x] Variability: ENSO, AMO, PDO, NAO indices, subseasonal-to-decadal
- [x] Extreme Events: GEV return periods, compound frequency, attribution
- [x] Water Cycle: P-ET-R closure, streamflow, soil moisture, groundwater, SWE

### 10.3 Model Hierarchy Analysis
- [x] Representation ladders as model hierarchy; complexity→skill tradeoff; process attribution
- [x] Hierarchy-guided uncertainty decomposition (structural vs. parametric vs. data)

---

## Phase 11 — Process Discovery

### 11.1 Residual Analysis Framework
- [x] Multi-metric residuals from best configuration; structured bias detection (Moran's I, spectral analysis, regime-conditional)
- [x] Attribution via partial correlation + causal inference (Granger, transfer entropy)
- [x] Persistence test (across calibrations, rung swaps, ensemble members, independent products)
- [x] Severity scoring: magnitude × extent × persistence × relevance

### 11.2 Hypothesis Generation
- [x] Classify residual → process type: diurnal→energy balance, spatial gradient→lateral transport, post-event→recovery coupling, seasonal phase→phenological feedback
- [x] Literature + A2A query for known matching processes; hypothesis registry with status tracking

### 11.3 ML Process Learning Pipeline
- [x] Training data extraction: residual–input pairing, QA/QC filters, region-based cross-validation
- [x] Neural operators (FNO, DeepONet) + physics-informed loss
- [x] Symbolic regression (PySR) with dimensional analysis + complexity–accuracy Pareto
- [x] Hybrid physics+ML (conservation by construction; graceful degradation)

### 11.4 Validation & Knowledgebase Deposit
- [x] Multi-criteria gate: skill improvement, conservation (100-yr), stability, generalization, sensitivity
- [x] Auto-manifest with provenance: I/O, scale envelope, regime tags, cost, skill, `origin: discovered`
- [x] Deposit into Process Knowledgebase with full manifest + provenance; immediately available for neural inference
- [x] Promotion: `candidate → provisional → validated → production`
- [x] Lifecycle: periodic re-validation, retraining on expanded data, retirement

### 11.5 Integration With Core Loop
- [x] Residual analysis on every benchmarking cycle; errors feed neural inference engine
- [x] Connect to Active Learning, Hypothesis Engine, A2A (`propose_rung`), Process Knowledgebase
- [x] Discovery budget per cycle; discovery metrics dashboard

---

## Phase 12 — Geoengineering Feedback Control

### 12.1 Control Infrastructure
- [x] Control targets (setpoint + tolerance + measurement source); error computation
- [x] Actuator interface per intervention type (control variables)
- [x] Control loop: observe → error → predict → intervene → simulate → verify → update
- [x] Integration with autonomous optimization loop

### 12.2 Strategy Discovery & Optimization
- [x] Intervention schedule representation (time-varying control law, 50–500 yr horizon)
- [x] MPC: rolling-horizon with ESM forward model; constraint satisfaction; emulator screening
- [x] RL: PPO/SAC with domain randomization (ECS, SSP uncertainty)
- [x] Portfolio optimization: synergies (SAI+DAC), antagonisms (iron fert+OAE); budget-constrained allocation

### 12.3 Termination Shock & Stability
- [x] Cessation simulation at multiple points; rebound rate, overshoot, tipping proximity
- [x] Minimum safe ramp-down discovery
- [x] Robustness: ECS 2–5°C, SSP1–5, actuator failure, tipping points (AMOC, WAIS, Amazon, permafrost)
- [x] Long-term stability verification (500+ yr)

### 12.4 Governance & Safety
- [x] Side-effect constraint checking (ozone, precipitation, pH, ecosystem impact)
- [x] Distributional equity analysis (regional winners/losers)
- [x] Reversibility classification per intervention; strategy provenance
- [x] Pareto frontier dashboard: cooling vs. cost vs. side effects vs. termination risk

---

## Phase 13 — Planetary Defense & Extinction Modeling

### 13.1 NEO Data Pipeline
- [x] CNEOS Sentry, Scout, Horizons, MPC, USAF SSN, USSF 18th SDS, ESA NEOCC, survey feeds
- [x] Automated threat ranking (Palermo scale); alerts on catalog changes

### 13.2 Impact Cascade Modeling
- [x] Parametric impact model: size/velocity/angle → energy partition
- [x] Atmospheric injection → AOD → radiative forcing (→ Atmosphere A1/A2)
- [x] Thermal pulse → firestorm probability (→ Fire F2/F3)
- [x] Ocean impact → tsunami + acidification + anoxia (→ Ocean O1/O2)
- [x] Nuclear winter → photosynthesis collapse → trophic cascade (→ TD0-TD2)
- [x] Long-term recovery → ecosystem rebuilding + adaptive radiation (→ Ecology + Evolution)

### 13.3 Mass Extinction Calibration
- [x] K-Pg: 10 km impactor; validate impact winter, temperature drop, acid rain, foram extinction, δ¹³C excursion
- [x] P-T: Siberian Traps; validate warming, anoxia extent, >90% marine loss, 5 Myr delayed recovery
- [x] Late Devonian, End-Triassic scenarios
- [x] Compare modeled loss/recovery vs. PBDB stratigraphic range data

### 13.4 Deflection Strategy Assessment
- [x] Kinetic impactor (DART-class), gravity tractor, ion beam, nuclear standoff
- [x] Campaign optimization: NEO orbit + lead time → optimal strategy + launch window
- [x] Post-deflection orbit verification

---

## Phase 14 — Runtime Sentinel & Streaming

### 14.1 Runtime Sentinel Agent
- [x] Monitor execution metrics per process/region; trigger embedding/upgrades and hysteresis-based downgrades; information-loss accounting

### 14.2 Data & Calibration Agent
- [x] Datasets per compiled model; parameter priors per rung; identifiability warnings; calibration targets

### 14.3 Streaming / NRT
- [x] Event bus: `DataUpdateEvent(variable, tiles, time_range, quality_delta)`
- [x] Active fire → ignition trigger; precip radar → hydro subcycling; burn severity → disturbance application
- [x] Tile-first processing pipeline

---

## Phase 15 — Dashboard & Event Store

### 15.1 Event Bus & Store
- [x] Schema: `AgentEvent { agent_id, event_type, timestamp, payload, cycle_id, correlation_id }`
- [x] Types: `task_started/completed/failed`, `skill_recorded`, `data_discovered/ingested`, `process_learned`, `rung_swapped`, `pareto_updated`, `regime_detected`, `a2a_exchange`, `objective_changed`
- [x] Async bus (Rust channels → append-only log); SQLite (dev) / PostgreSQL (prod)
- [x] Retention policy (hot 30d, warm 1yr, cold archive); correlation ID propagation

### 15.2 Dashboard Backend
- [x] REST + WebSocket API: `/events/stream`, `/events/query`, `/skill/timeseries`, `/pareto/frontier`, `/agents/status`, `/data/ingestions`, `/discovery/log`, `/regimes/map`
- [x] Auth (API key / OAuth); rate limiting + pagination

### 15.3 Dashboard Frontend (Next.js)
- [x] App Router + React Server Components
- [x] Views: Agent Workflows (live DAG), Optimization (Pareto animation), Skill Evolution (sparklines), Data Ingestion (feed), Process Discovery (residuals/hypotheses), Provenance Audit (dependency graph), Regime Map (Mapbox/Deck.gl), Federation Status (A2A peers)
- [x] WebSocket subscriptions; responsive layout

### 15.4 Optional Steering Panel
- [x] Adjust objectives, weights, budgets, allowlists, process inclusion, domain extent
- [x] Changes take effect next cycle; logged as `objective_changed` events

---

## Phase 16 — Demonstrations

### 16.1 Coupled Landscape Baseline
- [x] 100–250 m grid, hourly forcing; R0/R1 + H0/H1 + E0 + B0/B1; validate soil moisture, ET, biomass, streamflow

### 16.2 Event Embedding
- [x] F1 at 30 m for ignitions; disturbance → canopy loss, fuel changes, hydrophobicity; H1/H2 in burned watersheds during storms
- [x] Show: post-fire runoff peaks, multi-year recovery via succession

### 16.3 Dynamic Rung Switching
- [x] Auto rung selection (budget + error + triggers); coarse-only vs. embedded-events comparison

### 16.4 Knowledgebase-Driven Assembly
- [x] Expert-prior KB → neural inference proposals → combinatorial experiments → posterior convergence → deposit learned preferences into KB → recompile → transfer test

### 16.5 Continuous Learning
- [x] Multi-year hindcast + streaming ingest; show skill improvement, ontology divergence from priors, Active Learning queries

### 16.6 Autonomous Data Discovery
- [x] Start with gauges only; Data Scout discovers SMAP + MODIS via STAC; re-score all hydrology + ecology; regime detection

### 16.7 Fully Autonomous Operation
- [x] Input: domain + budget + objectives; system discovers data, assembles, benchmarks, converges, reports — no human intervention

### 16.8 A2A Federated Assembly
- [x] Two instances (land/fire + ocean/atmos); IR fragment exchange; cross-boundary validation; coupled run; show federated ≥ monolithic skill

### 16.9 Cross-Instance Skill Sharing
- [ ] Two instances benchmark hydrology on different regions; exchange anonymized records; show faster posterior convergence

### 16.10 MultiSector Dynamics
- [ ] Western U.S.: H1+E0+R1+F1+HS1; drought → water → energy → agriculture cascade; compare vs. HS0 decoupled

### 16.11 Full EESM Coverage
- [ ] Global + all 3 program areas; autonomous assembly + RGMA diagnostics + MSD coupling + comparison vs. E3SM/CESM baselines

### 16.12 AI-Driven Process Discovery
- [ ] Systematic streamflow timing bias → Process Discovery diagnoses lateral subsurface flow → symbolic regression closure → validate → deposit into KB → deploy → skill improvement

### 16.13 Human-Out-of-Loop Dashboard
- [ ] 72+ hr autonomous run; dashboard shows live DAG, evolving Pareto, data discovery, skill curves, regime map, provenance; optional steering after review

### 16.14 Geoengineering Strategy Discovery
- [ ] Targets: T≤1.5°C, ΔP<5%, pH>8.0; enumerate portfolios → 200-yr simulations → multi-objective Pareto → termination shock analysis → show SAI+DAC dominates SAI-only

### 16.15 Planetary Defense Impact
- [ ] Chicxulub-class scenario: energy partition → atmospheric injection → firestorms → tsunami → nuclear winter → trophic collapse → 10⁵-yr recovery
- [ ] Validate vs. K-Pg paleo record (iridium anomaly, fern spike, foram extinction, δ¹³C)

### 16.16 Trophic Cascade
- [ ] Temperate forest food web (producers, herbivores, mesopredators, apex, decomposers); wolf reintroduction → trophic cascade → ecosystem response; validate vs. Yellowstone data

### 16.17 Evolutionary Response
- [ ] 500-yr warming (SSP3-7.0): trait shifts, composition turnover, biodiversity balance; compare vs. LTER
- [ ] K-Pg mass extinction + recovery: adaptive radiation over 10⁵–10⁶ yr

---

## Phase 17 — Extensions

- [ ] **F2/F3 fire**: Wind-aware + plume feedback; fire–atmosphere coupling (WRF-Fire class)
- [ ] **H2 hydrology**: Integrated surface–subsurface (ParFlow-like) embedded domains
- [ ] **E1/E2 ecology**: Size-structured cohorts; individual-based trees (iLand-like)
- [ ] **B2 biogeochem**: Vertically resolved soil biogeochemistry
- [ ] **R2 radiation**: 3D radiative transfer for subdomains
- [ ] **A3 atmosphere**: Variable-resolution global (E3SM-Omega class)
- [ ] **O2 ocean**: Eddy-resolving MPAS-Ocean with ocean biogeochemistry
- [ ] **C2 cryosphere**: Dynamic ice-sheet (MALI) + sea-ice rheology + ice-shelf coupling
- [ ] **HS3 human systems**: Fully coupled IAM (global economy + endogenous emissions)
- [ ] **TD2 trophic dynamics**: Individual-based predator-prey with body-size scaling + optimal foraging
- [ ] **EV2 evolution**: Genotype-phenotype + quantitative genetics + speciation
- [x] Erosion/sediment transport module family
- [ ] ML emulator backends for fast surrogate execution
- [x] Arbitrary mesh support + general remapping
- [ ] HPC cluster execution backend
- [x] RDF/OWL semantic web ontology export
- [ ] Multi-instance A2A mesh (10+ peers, auto-negotiated federation)
- [ ] CMIP participation (auto-generate compliant simulation suites)
- [ ] Equation discovery at scale (symbolic regression across all families simultaneously)
- [ ] Deep-time Earth system (10⁶–10⁹ yr planetary evolution)
- [ ] Astrobiology extension (exoplanet habitability, biosignatures)
- [ ] Geoengineering governance AI (multi-stakeholder game-theoretic analysis)
- [ ] AI copilot (NL interface for EESM researchers)

---

## Phase 18 — GPU Acceleration & Neural Operators

### 18.1 Multi-GPU Execution Backend
- [x] wgpu 24.x + cudarc GPU backends; device enumeration and kernel dispatch
- [x] NCCL multi-GPU communication; pinned buffer exchange; async streams
- [x] Device assignment traits: GPU workload allocation per process family

### 18.2 NVIDIA Modulus Neural Operators
- [x] FNO (Fourier Neural Operator) integration for emulator rungs
- [x] PINO (Physics-Informed Neural Operator) with PDE residual loss
- [x] DeepONet for operator learning; MeshGraphNet for unstructured meshes
- [x] Modulus model registry: operator manifests, training provenance, skill records
- [x] Physics-informed loss construction from process constraints

---

## Phase 19 — Foundation Models & Autonomous Observation

### 19.1 NVIDIA Earth-2 / earth2studio Integration
- [x] `FoundationModelRunner` trait: load, run, bias-correct, score
- [x] Stub implementations for FourCastNet, Pangu-Weather, GraphCast, GenCast, CorrDiff
- [x] `EnsembleOrchestrator`: dispatch to multiple models, fuse predictions, uncertainty quantification
- [x] Foundation Model Agent: orchestrate inference, evaluate ensemble spread, route to neural operators when resolution insufficient
- [x] Bias correction pipeline: learned station-level corrections; retraining on expanding observations

### 19.2 PhiSat-2-Inspired Autonomous Observation
- [x] `ObservationPipeline` with on-board anomaly detection (spectral, thermal, structural)
- [x] Adaptive tasking: dynamically re-prioritize satellite passes based on simulation uncertainty
- [x] Observation relevance scoring: information gain estimation per observation opportunity
- [x] Autonomous Observation Agent: monitor uncertainty fields, request/schedule observations, ingest detections
- [x] Integration with Data Scout for observation-driven discovery cascades

---

## Phase 20 — Process Evolution

### 20.1 Evolutionary Population Management
- [x] `EvolutionCandidate` with fitness, generation, lineage tracking
- [x] `Population` with Pareto front computation, tournament selection, stagnation pruning
- [x] `EvolutionConfig`: population size, tournament size, mutation/crossover rates, speciation interval/threshold, elitism, stagnation limit
- [x] `ProcessLineage` with lineage ID, parent IDs, operator, generation metadata

### 20.2 Evolution Agent
- [x] Rewrite from stub to full ALife-aware agent (~238 lines)
- [x] `run_generation()`: Think→Act→Observe→Repeat loop
- [x] Generation reports with tier counts, Pareto front size, skill deltas

---

## Phase 21 — ALife Process Lifecycle

### 21.1 Process Automaton
- [x] `ProcessAutomaton` composite: wraps process with soul, constitution, heartbeat config, replication history
- [x] `SurvivalTier` enum (Normal/LowCompute/Critical/Archived) with budget/cadence multipliers, promote/demote
- [x] `Constitution` with 3 hierarchical immutable laws (Conserve, Earn existence, Maintain provenance)
- [x] `ProcessSoul`: identity, strengths, weaknesses, dominant niches, modification count, tier history
- [x] `HeartbeatConfig`: revalidation cadence, stagnation window, health check cadence, conservation demotion threshold
- [x] Conservation checking: `check_conservation(residual, tolerance)` → constitutional violation on breach
- [x] Stagnation detection: no fitness improvement within configurable window

### 21.2 Self-Replication & Lineage
- [x] `ReplicationEvent` + `ReplicationMethod` (Mutation/Crossover/Immigration/Speciation)
- [x] `ModificationRecord` + `ModificationType` (7 variants: parameter edit, structural change, hybridization, pruning, augmentation, rung swap, regime adaptation)
- [x] Phylogenetic lineage tracking: parent IDs, method, timestamp per replication

### 21.3 Heartbeat Daemon
- [x] `HeartbeatDaemon` in maesma-runtime with `tick()` loop
- [x] `HeartbeatOutcome` struct: process ID, previous/current tier, tier change, constitution violation, stagnation, skill-cost ratio
- [x] Emit ALife events: `HeartbeatCheck`, `SurvivalTierChange`, `ConstitutionViolation`, `StagnationDetected`, `ProcessArchived`

### 21.4 ALife Event System
- [x] 8 new event kinds in EventKind enum: SurvivalTierChange, SelfModification, Replication, ConstitutionViolation, HeartbeatCheck, StagnationDetected, ProcessArchived, Immigration
- [x] Event bus integration for real-time dashboard updates

---

## Phase 22 — Advanced Dashboard & Documentation

### 22.1 Dashboard Pages
- [x] Process Evolution page: survival tier donut, fitness over generations chart, constitutional invariant status, ALife events feed
- [x] Foundation Models page: ensemble status, model registry, inference logs
- [x] Observation Intelligence page: satellite tasking overview, anomaly detections, coverage maps
- [x] Sidebar updated to 13 navigation items

### 22.2 Animation
- [x] ALife Evolution scene: survival tier transitions, constitutional laws, heartbeat pulse, phylogenetic tree branching

### 22.3 Documentation Sync
- [x] Paper updated with ALife throughout: abstract, keywords, contributions, Process Evolution section (6 subsections), design principles (23), tech stack, discussion, conclusion
- [x] README updated with ALife Process Lifecycle section, 25 agents, 23 design principles, Earth-2/PhiSat-2/GPU tech stack
- [x] ROADMAP updated with Phases 18-22
