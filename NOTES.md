# Notes

Got it. What you’re describing is basically a **multi-scale “process graph compiler”**: given a target domain + scale + objectives, it **assembles** a consistent systems model from a library of scale-dependent process modules, with the right state variables, couplings, numerics, and data interfaces.

Here’s a concrete architecture that works in practice.

## 1) Core idea: a Scale-Aware Process Graph (SAPG)

Represent the model as a **typed, unit-aware, scale-aware directed hypergraph**:

* **Nodes = state variables/fields**

  * e.g., `T_air(x,y,z,t)`, `soil_moisture(layer,t)`, `canopy_carbon(pool,t)`, `fireline_intensity(s,t)`, `river_discharge(reach,t)`
* **Edges = process operators**

  * e.g., radiation, turbulence closure, stomatal conductance, infiltration, groundwater flow, crown fire transition, sediment transport
* **Hyperedges** because processes consume/produce *multiple* variables
* **Constraints**: units, bounds, conservation laws, closure requirements, stability conditions

Every process operator exists as **a family of representations across scales** (more below).

## 2) Define a “process module” schema

A module isn’t “code first”; it’s a **declarative contract** + one or more implementations.

**Module metadata (must be machine-readable):**

* **Process**: name + category (hydrology, fire, vegetation, atmo…)
* **Valid scale envelope**:

  * spatial: `Δx ∈ [1 m, 10 km]`
  * temporal: `Δt ∈ [0.1 s, 1 day]`
  * regime tags: `convective`, `mountainous`, `arid`, `snow`, `WUI`, etc.
* **Inputs/Outputs**: typed variables with units and discretization type

  * scalar, vector, field; grid vs mesh vs network vs agent set
* **Assumptions / closures**

  * hydrostatic/nonhydrostatic, equilibrium canopy, Richards vs bucket, etc.
* **Conservation properties**

  * exact mass conservation, energy conservation, monotonicity, positivity
* **Numerical requirements**

  * explicit/implicit, solver type, stiffness indicator, CFL constraints
* **Calibration parameters + priors**

  * parameter names, ranges, default priors, identifiability hints
* **Data requirements**

  * forcing sources, boundary conditions, required static fields

**Implementations:** one or more backends

* PDE solver kernel, reduced-order surrogate, agent model, empirical model, ML emulator…
* Each implementation advertises cost, accuracy, differentiability, GPU support, etc.

This lets you swap *process representation* without changing the graph semantics.

## 3) Multi-scale representation strategy: “Representation Ladder”

For each process, maintain a ladder of representations:

Example: **atmospheric transport**

* LES / nonhydrostatic Euler @ 10–100 m, seconds
* mesoscale WRF-like RANS closure @ 0.5–10 km, minutes
* bulk box/advection-diffusion @ 10–100 km, hours

Example: **soil hydrology**

* Richards equation (integrated surface-subsurface) @ 1–100 m
* multi-layer 1D Richards + lateral TOPMODEL-like @ 30 m–1 km
* bucket model @ km–10 km

Example: **fire**

* coupled fire-atmosphere (WRF-Fire/SFIRE style) @ meters–100s m
* semi-empirical spread (Rothermel / cellular spread) @ 10–100 m
* fire regime / area burned statistics @ km–50 km, seasonal

The key is: each rung **declares what it preserves**, what it approximates, and what state variables it expects.

## 4) Model construction: compile-time + run-time adaptation

### A) Compile-time (initial build)

Inputs:

* target **scale**: `(Δx, Δt, extent, horizon)`
* target **questions**: e.g., “runoff peaks?”, “carbon budgets?”, “fire spread hours?”
* **compute budget**: GPU/CPU, walltime, ensemble size
* **data availability** profile

Compiler steps:

1. **Select a process set** from ontology (what must be represented)
2. For each process, choose a **representation rung** that matches scale/regime/budget
3. Resolve **state-space closure**:

   * ensure every required variable is produced somewhere
   * introduce diagnostic variables if needed
4. Validate **consistency constraints**:

   * units, conservation compatibility, boundary condition consistency
5. Choose **discretization + coupling strategy**:

   * operator splitting vs monolithic
   * implicit/explicit partitioning (stiffness)
6. Produce an **executable schedule**:

   * time-stepping order, coupling frequency, remapping operators

Output:

* a runnable **coupled model graph**
* plus a calibration/inference spec (what parameters to tune, priors)

### B) Run-time (dynamic refinement/coarsening)

You want it to change scale and representation *during* execution.

Add:

* **Error/uncertainty monitors** (per process and per region)
* **Refinement triggers**

  * event-driven: ignition detected → switch fire module rung locally
  * hydrologic thresholds: rainfall intensity → refine routing
  * flow regime: Froude/Reynolds-based switching
* **Mesh/field transfer operators**

  * conservative remapping, aggregation, disaggregation
* **State projection operators**

  * map between different state representations (e.g., cohorts ↔ size-structured)
* **Asynchronous subcycling**

  * fast physics in a subdomain, coarse elsewhere

This looks like AMR + multi-physics “submodel embedding” rather than a single uniform-resolution run.

## 5) Glue: canonical interfaces and remapping

To make dynamic construction feasible, you need strict interfaces:

* **Canonical variable registry**

  * names, units, dimensionality, semantics, uncertainty
* **Topology adapters**

  * grid ↔ mesh ↔ river network ↔ patch mosaic
* **Conservative remapping library**

  * mass/energy conservation for aggregation/disaggregation
* **Temporal coupling**

  * interpolation/extrapolation with boundedness constraints

If you do only one “hard” thing first, do this registry + remapping layer.

## 6) How to combine “landscape ecology” with “event physics”

This is where most systems break.

A robust approach is **two-tier coupled modeling**:

* **Slow tier (days–years):** vegetation/landscape dynamics (iLand/LANDIS-II class)

  * produces fuels, structure, moisture proxies, management actions
* **Fast tier (seconds–days):** event physics (fire spread, plume, flood waves)

  * consumes fuels + met forcing; returns disturbance footprints + severity
* Coupling via **event assimilation**:

  * when a fast event occurs, it writes a “disturbance operator” into the slow tier

This avoids trying to run tree demography at 1-second timesteps.

## 7) Implementation blueprint (practical)

### Components

1. **Process Library**

   * modules stored as packages with declarative manifest + code
2. **Model IR (intermediate representation)**

   * graph + constraints + solver schedule (like an ML compiler IR)
3. **Compiler**

   * selection, closure, validation, schedule generation
4. **Runtime**

   * executes the schedule, monitors errors, triggers model morphing
5. **Data layer**

   * standardized forcing/parameter datasets, caching, provenance
6. **Calibration/Inference**

   * parameter estimation per rung with consistent priors

### Recommended tech pattern

* Declarative manifests: JSON/YAML + unit schemas
* Strong typing: Rust is a great fit (traits for module contracts)
* Execution: pluggable backends

  * CPU PDE solver, GPU kernels, ML emulators
* Provenance: every compiled model gets a hash + full dependency graph

## 8) The “gotchas” you should design around

* **State mismatch across rungs** (e.g., cohorts vs individual trees)

  * solve with explicit *state projection operators* + information loss tracking
* **Double counting** when two modules represent overlapping physics

  * enforce exclusivity constraints in the ontology
* **Numerical instability when switching**

  * smooth transitions, hysteresis, spin-up windows, conservative remap
* **Calibration drift**

  * rung-specific calibration + hierarchical priors; don’t reuse parameters blindly
* **Coupling frequency errors**

  * explicitly model coupling error; adapt coupling interval where needed

## 9) A concrete MVP that proves the concept (3–6 modules)

Pick a minimal but compelling set:

* Atmosphere forcing: coarse met driver (reanalysis/WRF output)
* Land surface water balance: bucket ↔ 1D Richards ladder
* River routing: kinematic wave
* Vegetation: cohort-based succession (LANDIS-ish) with simple fuels
* Fire: cellular spread ↔ coupled wind-aware spread ladder (no full WRF-Fire yet)
* Disturbance feedback: severity → vegetation mortality + fuel updates

Then demonstrate:

* compile model at **1 km / hourly** for regional planning
* dynamically embed **30 m / 1 min** fire module around an ignition
* roll up burn severity back into the regional model

That’s the “hello world” of dynamic multi-scale systems modeling.

---

If you tell me two things, I can turn this into a very specific spec (data schemas + module contracts + IR + scheduler rules):

1. Your first demo domain: **wildfire + hydrology + forest**? or just one?
2. Your intended runtime target: **single GPU workstation** or **HPC cluster**?

Cool—this is a strong “integrated landscape Earth system” stack, but on a **multi-GPU workstation** you’ll only win if you’re ruthless about (1) *separating fast vs slow tiers*, and (2) *embedding high-fidelity submodels only where/when needed*.

Below is a concrete design that gets you to a credible end-to-end demo and scales upward without rewriting everything.

---

## 0) The organizing principle: two-tier + embedded event solvers

You’re mixing processes with wildly different natural timesteps:

* **Fast tier (seconds → hours):** fire spread + plume/heat flux, canopy energy balance at sub-hourly, overland flow response to intense rain, shortwave shading/terrain solar.
* **Slow tier (days → centuries):** succession/competition, soil C/N pools, long-term ecohydrology, management.

So don’t try to “one-timestep them all.” Instead:

### Tier A: Landscape/Ecosystem (slow)

Runs on daily → monthly “macro steps” with internal substeps where needed.

### Tier B: Event Physics (fast, local)

Spins up on triggers (ignition, extreme rain, heatwave), runs minute/second time, only in a subset of the domain, then writes back **disturbance operators** (mortality, severity, soil hydrophobicity, ash/nutrient pulses, canopy loss, etc.).

The “dynamic construction” part is the runtime choosing **where** to spawn Tier B and **which rung** (fidelity) to use.

---

## 1) Your process library: the minimum set of “families” and ladders

Each family has 2–4 rungs (representations), with explicit scale envelopes.

### A) Fire behavior & effects

* **Rung F0 (regional):** stochastic/fire regime or semi-empirical area-burned + severity mapping (km / daily).
* **Rung F1 (landscape):** cellular spread (Rothermel-like) with wind/topography/fuels (10–100 m / 1–10 min).
* **Rung F2 (event local):** wind-aware spread with simplified plume feedback (coupled surface wind adjustment) (5–50 m / seconds–minutes).
* **Rung F3 (optional later):** full fire–atmosphere coupling (WRF-Fire class) for research-grade plumes (meters–100 m / seconds).

**Outputs (disturbance operator):**

* burn perimeter/time, severity (fraction canopy/duff/soil consumption)
* live/dead fuel transitions
* heat pulse to soil, hydrophobicity index, ash deposition, nutrient pulse
* canopy loss affecting radiation/ET for months-years

### B) Hydrology (surface + subsurface + routing)

* **Rung H0:** bucket + curve-number-ish runoff + simple baseflow (km / hourly–daily).
* **Rung H1:** multi-layer soil water + infiltration (Green-Ampt or 1D Richards per cell) (30–300 m / 5–60 min).
* **Rung H2:** integrated surface–subsurface (ParFlow-like) for embedded basins (10–100 m / minutes).
* **Routing ladder:** kinematic wave → diffusive wave → full 2D shallow water only for flooded reaches.

**Outputs back to ecology/BGC:**

* soil moisture/temperature profiles, water table proxy, streamflow/soil saturation frequency
* post-fire runoff/erosion risk indices (even if erosion is later)

### C) Ecology: succession + competition + demography

* **Rung E0:** cohort mosaic (LANDIS-ish): species × age/biomass cohorts per cell (30–250 m / annual).
* **Rung E1:** size-structured cohorts (diameter classes) (10–100 m / annual).
* **Rung E2 (optional later):** individual-based trees (iLand-ish) embedded in “plots” (10–50 m / annual with intra-annual carbon allocation).

**Outputs to fuels/fire:**

* fuel strata (1h/10h/100h/1000h), canopy bulk density, canopy base height, live fuel moisture proxies
* stand structure for crown fire thresholds

### D) Biogeochemistry (C/N, decomposition, soil pools)

* **Rung B0:** big-leaf carbon + simple soil pools (fast enough everywhere).
* **Rung B1:** multi-pool soil C/N + litter + microbial decomposition temperature/moisture response (daily).
* **Rung B2 (optional later):** vertically resolved soil biogeochem for embedded sites.

**Fire coupling:**

* combustion emissions (C, N), char/black carbon fraction, post-fire mineralization pulses

### E) Energetics + solar dynamics (radiation/energy balance)

* **Rung R0:** daily potential solar (slope/aspect + latitude) + empirical canopy attenuation.
* **Rung R1:** sub-daily shortwave partitioning + longwave + canopy energy balance (hourly).
* **Rung R2 (optional later):** 3D radiative transfer for tiny subdomains (rarely worth it early).

**This is the glue** because radiation drives ET, fuels drying, canopy stress, snow melt.

---

## 2) The model “compiler”: what it must decide

Given: extent/resolution/horizon, GPU budget, data availability.

It selects:

1. **Which rung per family** globally (default)
2. **Where to allow embeddings** (regions eligible for higher rungs)
3. **Coupling schedule** (how often modules exchange state)
4. **Transfer operators** between rungs and between grids/networks

### Coupling schedule that works on a workstation

* Radiation/energy balance: **hourly** (or 3-hourly) global
* Hydrology: **hourly** global; **minutes** only inside embedded basins
* Fire: off until trigger; then **minutes/seconds** in embedded subdomain
* Ecology + BGC: **daily** internal, with **annual** structural updates (succession/competition)

---

## 3) Runtime adaptation: how you “dynamically construct” during execution

### Triggers (examples)

* **Ignition trigger:** detected ignition + fuels + wind threshold → spawn F1/F2 in a bounding box + buffer
* **Extreme rain trigger:** rainfall intensity/antecedent saturation threshold → upgrade H0→H1/H2 in affected basins
* **Drought stress trigger:** sustained high VPD / low soil moisture → upgrade radiation + canopy energy rung locally (R0→R1)
* **Management action trigger:** harvest/thin/prescribed burn event → spawn disturbance operator without full fire physics

### Hysteresis

Don’t flip rungs every timestep. Use hysteresis windows:

* “stay upgraded” for N hours/days after event, then gracefully downshift.

### The *non-negotiable* piece: projection operators

You need explicit maps like:

* E2 (individual trees) → E0 (cohorts): aggregate basal area/LAI/height distributions
* E0 → fuels: derive fuel strata
* Fire severity → BGC pools: partition consumed/charred/transferred C/N
* High-res hydro → low-res hydro: conservative aggregation of storage/flux

Also track **information loss** (uncertainty increases on downscaling).

---

## 4) Data model: your canonical state registry (keep it small at first)

Start with a minimal canonical set that all modules can agree on:

### Shared forcings

* `P`, `Tair`, `RH/VPD`, `Wind`, `SWdown`, `LWdown` (or parameterize LW), optional `CO2`

### Shared slow state

* `LAI`, `canopy_height`, `CBD/CBH` (canopy bulk density/base height)
* `fuel_[1h,10h,100h,1000h,live]`
* `soil_moisture[layer]`, `soil_temp[layer]`, `snow_water_equiv` (if needed)
* `soilC[pools]`, `soilN[pools]`, `litter[pools]`
* `streamflow` (network), `soil_storage`, `water_table_proxy`

### Shared disturbance state

* `burn_severity`, `mortality_fraction`, `soil_hydrophobicity`, `char_fraction`, `ash_nutrient_pulse`

Unit-aware, bounds-aware, and each variable declares:

* discretization (grid/mesh/network), vertical structure, and update authority (which module “owns” it vs diagnoses it)

---

## 5) Multi-GPU workstation execution plan (practical)

You want to minimize multi-GPU synchronization.

### Recommended partition

* **GPU 0:** radiation + energy balance + fuel moisture drying model (these are dense per-cell kernels)
* **GPU 1:** hydrology kernels (soil column physics + routing)
* **GPU 2:** event solvers (fire spread / embedded hydro upgrades)
* **CPU:** ecology/competition + BGC bookkeeping (often branchy; move to GPU later if needed)

Use a runtime that supports asynchronous streams:

* Step schedule produces tasks; tasks run on assigned device; exchange state via pinned buffers.

### Memory strategy

* Keep global grids resident on GPUs (float32 fields)
* Embedded domains allocate scratch on GPU2; transfer only boundary and summarized outputs back

---

## 6) MVP build that proves the concept (and is doable)

### Phase 1: “Coupled landscape baseline”

* Global grid: 100–250 m, hourly met
* Modules: R0/R1 + H0/H1 + E0 + B0/B1
* Validate basic seasonal patterns: soil moisture, ET proxy, biomass growth, streamflow response

### Phase 2: “Event embedding works”

* Add F1 (cellular spread) embedded at 30 m for ignition boxes
* Fire writes disturbance operator:

  * canopy loss, dead fuel addition, soil hydrophobicity index, C/N emissions + char
* Hydrology upgrades to H1/H2 *only* in burned watersheds during storms
* Show emergent behavior:

  * post-fire runoff peaks increase, recovery over years via succession

### Phase 3: “Dynamic rung switching”

* Demonstrate automatic rung selection based on:

  * compute budget + error proxies + triggers
* Show the same scenario run at:

  * coarse global only vs embedded events, with different outcomes and cost

This is the demo people immediately “get.”

---

## 7) What to implement first (order matters)

If you want dynamic assembly to actually work, implement in this order:

1. **Variable registry + unit/bounds checks**
2. **Process module manifest schema** (scale envelope, I/O, assumptions, numerics)
3. **Remapping/projection operators** (conservative aggregation + downscaling)
4. **Scheduler** (task graph + coupling cadence)
5. Then modules, one by one, with simple rungs first

Everything else is downstream.

---

## 8) Concrete file/module layout (so your team can build it)

* `core/`

  * `registry/` (variables, units, semantics)
  * `graph_ir/` (process graph, constraints)
  * `compiler/` (rung selection, closure, validation)
  * `runtime/` (task scheduler, device manager, remap)
* `modules/`

  * `radiation/` (R0/R1)
  * `hydrology/` (H0/H1 + routing)
  * `ecology/` (E0)
  * `biogeochem/` (B0/B1)
  * `fire/` (F1 embedded)
* `operators/`

  * `remap/` (grid↔grid, grid↔network, aggregation)
  * `projection/` (state transforms between rungs)
* `scenarios/`

  * data manifests + forcing sources + parameter priors

---

## 9) One design choice you should lock in now

Pick the **canonical spatial representations** you’ll support in v1:

1. **Raster grid** (main landscape fields)
2. **River network graph** (routing)
3. **Embedded raster subdomains** (for fire / high-res hydro)

If you add arbitrary meshes too early, you’ll burn time on remapping.

---

If you want, I can write you the **module manifest spec** (YAML/JSON), the **IR types**, and the **scheduler rules** (including how embeddings are spawned and how state projections are applied) in a way that’s directly implementable in Rust with multi-GPU task execution.

Alright—make **(1) a process ontology** and **(2) AI agents** first-class citizens, and you end up with a framework that can *discover*, *compose*, *validate*, and *adapt* models instead of hand-wiring them.

Here’s a concrete design that fits your multi-GPU workstation target and your “dynamic construction at any scale” goal.

---

## 1) The core loop: Ontology → Discovery → Compilation → Runtime Adaptation

### Inputs (from user / scenario)

* domain + extent + target resolution(s)
* objectives (fire spread fidelity? peak discharge? carbon balance?)
* compute budget (multi-GPU workstation)
* data availability (DEM/soils/fuels/met, etc.)

### Outputs

* a **compiled, executable coupled model**
* a **provenance bundle**: which processes/assumptions were selected and why
* a **live adaptation policy**: when/where to upgrade/downgrade process rungs

---

## 2) Ontology: what it must represent (minimum viable, but future-proof)

Think of the ontology as a **knowledge graph for processes and representations**.

### A) Concepts (classes)

* **Process** (e.g., Infiltration, CrownFireSpread, SoilRespiration, CanopyRadiationTransfer)
* **Representation** (a specific model form for a process at a scale/regime)
* **StateVariable** (typed + units + topology)
* **Assumption / Closure** (hydrostatic, equilibrium canopy, well-mixed, etc.)
* **Regime** (convective, snow, WUI, steep terrain, water-limited, etc.)
* **NumericalForm** (explicit/implicit, stiffness, solver family, CFL constraints)
* **DataRequirement** (forcing fields, parameter maps, calibration datasets)
* **Constraint** (conservation, positivity, boundedness, energy closure)
* **CouplingOperator** (remap, aggregation, disaggregation, state projection)

### B) Key relations (edges)

* `Process has_representation Representation`
* `Representation requires StateVariable`
* `Representation produces StateVariable`
* `Representation valid_over ScaleEnvelope`
* `Representation assumes Assumption`
* `Representation conserves Constraint`
* `Representation needs DataRequirement`
* `Representation compatible_with Representation` (or incompatible)
* `Representation composed_with Representation via CouplingOperator`
* `Representation has_cost_model` (GPU/CPU, memory, scaling)
* `Representation has_skill_model` (where it’s accurate / known weaknesses)

### C) Scale envelope needs to be explicit

Store both *spatial* and *temporal* validity, plus topology:

* `Δx`, `Δt`, extent, vertical structure, grid/network
* regime tags (“crown fire”, “post-fire hydrophobicity”, “snowmelt”)

This is what enables automatic rung selection and switching.

---

## 3) “Process Discovery” as an agentic workflow

You want agents to do *real work*:

* interpret intent
* propose candidate process sets
* select representations
* check closure/compatibility
* generate coupling schedules
* produce calibration plans
* monitor runtime and adapt

### Agent roster (minimal set that works)

1. **Intent & Scope Agent**

* turns “fire + hydro + ecology + BGC + energy + solar” into an explicit objective spec:

  * outputs list of required observables + acceptable error bands
  * assigns priority tiers (must-have vs nice-to-have)

2. **Ontology Retrieval Agent**

* queries the process graph for candidate processes and representations given scale/regime/budget/data.

3. **Model Assembly Agent**

* builds a candidate **Process Graph IR**:

  * picks default rungs globally
  * declares allowed embeddings (local upgrades)
  * proposes coupling cadence

4. **Closure & Consistency Agent**

* ensures the assembled model is well-posed:

  * every required variable is produced
  * no double-counted physics
  * units consistent
  * conservation claims valid
  * numerical stability constraints satisfied (CFL, stiffness partition)

5. **Data & Calibration Agent**

* determines what datasets are needed and how parameters will be inferred:

  * priors per rung
  * identifiability warnings
  * suggested calibration targets (stream gauges, burn severity maps, biomass plots)

6. **Runtime Sentinel Agent**

* monitors execution metrics and error proxies:

  * triggers embedding/upgrades (fire ignition, extreme rain)
  * triggers downgrades (hysteresis)
  * manages information-loss accounting when downscaling

7. **Provenance & Audit Agent**

* produces a human-readable “why this model” report:

  * selected representations + assumptions
  * alternatives rejected + reasons
  * sensitivity hotspots

That’s enough to get a useful “model compiler” product.

---

## 4) The Process Graph IR (what the agents actually build)

Your IR should be dead simple and enforceable:

* **Nodes**: `StateVariable` on specific topology (grid/network) and resolution tier
* **Edges**: `ProcessRepresentation` instances
* **Constraints** attached to nodes/edges
* **Schedule**: coupling order + frequency + subcycling rules
* **Embedding declarations**:

  * `RegionSelector` (predicate)
  * `UpgradePolicy` (trigger + hysteresis)
  * `ProjectionOperators` (up/down mappings)

Agents operate on IR + ontology, not raw code.

---

## 5) How “dynamic construction at any scale” actually happens

### A) Compile-time rung selection

Agents choose a **global default rung** per process family, then allow local upgrades.

Example defaults for a workstation:

* radiation/energy: R1 (hourly)
* hydrology: H1 (hourly global) + routing
* ecology/BGC: E0 + B1 (daily/annual)
* fire: dormant until event; F1 embedded when triggered

### B) Runtime embeddings

When ignition happens:

* spawn an embedded domain (higher res, smaller dt)
* promote rungs (fire F1/F2, optionally hydro H2 in that basin)
* after event ends, write back a **disturbance operator** to slow tier:

  * canopy loss, mortality, fuel changes, soil hydrophobicity, C/N fluxes

This is “constructing a systems model at any scale” without running everything at the finest scale.

---

## 6) Making ontology + agents *trustworthy*, not vibes-based

You need two guardrails:

### Guardrail 1: machine-checkable contracts

Every representation must provide:

* I/O variable signatures + units
* conservation claims
* scale envelope
* numerical requirements
* cost model

Agents can only assemble what passes checks.

### Guardrail 2: explicit uncertainty + info-loss accounting

Whenever you:

* aggregate (fine → coarse)
* project (individual trees → cohorts)
* downshift rungs

…you attach an **uncertainty increment** / information-loss token to affected variables. The Runtime Sentinel uses this to decide when upgrades are warranted.

---

## 7) The MVP “Ontology-first” deliverable set

If you want this to come alive fast, build these artifacts first:

1. **Process ontology v0**

* 30–60 processes across your domains
* 2 rungs each for ~15 of them (enough for a demo)
* stored as a graph (RDF/OWL if you want, or a pragmatic property graph)

2. **Representation manifests**

* one manifest per rung implementation (declarative + link to code)

3. **Compiler IR + validators**

* closure checker, unit checker, incompatibility checker, schedule checker

4. **Agents v0**

* retrieval + assembly + closure/consistency
* runtime sentinel later (after you have a running sim loop)

---

## 8) Suggested ontology skeleton for your specific domain stack

Top-level process groups:

* **Radiation & solar geometry** (terrain + canopy attenuation)
* **Surface energy balance** (latent/sensible heat, canopy temp, fuel drying)
* **Hydrology** (infiltration, soil water, runoff generation, routing, groundwater proxy)
* **Fire behavior** (spread, spotting proxy, crown transition, heat flux)
* **Fire effects** (mortality, consumption, hydrophobicity, emissions, nutrient pulses)
* **Ecology** (succession, competition, regeneration)
* **Biogeochemistry** (decomposition, soil respiration, plant allocation, C/N pools)

Then define crosslinks:

* fire consumes fuels produced by ecology
* radiation/energy sets fuel moisture and plant stress
* hydrology sets soil moisture → plant stress and decomposition
* fire effects alter hydrology (hydrophobicity) and BGC (char/ash pulses)

---

## 9) What I’d implement first on the workstation

Do this in order:

1. **Ontology + manifest schema**
2. **IR + validators**
3. **Assembly agent** that can output a runnable schedule (even if the modules are stubs)
4. Fill modules incrementally (radiation → hydrology → ecology/BGC → fire embedding)

That gives you a framework that’s already “model-discovering” before the physics is complete.

---

If you want the most immediately actionable next step, tell me:

* preferred graph tech: **RDF/OWL** (semantic web) vs **property graph** (Neo4j-ish) vs **pure Rust in-memory + serialized manifests**

Either way, I can give you:

* a concrete **manifest schema** (fields, types, examples for each rung),
* the **IR types** (Rust structs/traits),
* and the **agent orchestration** (who calls what, and what they are allowed to mutate).

You can absolutely make “data collection + processing” a **first-class agent capability** in the framework—just do it as a **governed, reproducible data plane** (not ad-hoc web scraping), with explicit contracts about *what data is needed, where it comes from, how it’s transformed, and how provenance is tracked*.

Here’s a concrete way to enable agents to do this safely and reliably, including **raw remote sensing streams**.

---

## 1) Make “Data” a core graph alongside “Process”

Add a second ontology layer: **Data Ontology**, linked to your Process Ontology.

### Data Ontology primitives

* **Observable**: what you want (e.g., canopy height, burn severity, ET, soil moisture, snow, LAI, DEM)
* **Product**: a dataset that provides it (e.g., Sentinel-2 L2A, Landsat L2, VIIRS active fire, GEDI, SRTM/Copernicus DEM)
* **Access method**: STAC, OPeNDAP, S3 object store, API, FTP, Pub/Sub stream
* **License/constraints**: usage rights, attribution, redistribution rules
* **Latency class**: archival / daily / hourly / sub-hourly / NRT (near-real-time)
* **Transform recipe**: the exact processing chain to produce *model-ready* fields
* **Quality flags**: cloud mask, QA bits, uncertainty estimates

### The key linkage

Each **ProcessRepresentation** declares:

* required observables + spatial/temporal resolution + uncertainty tolerance
* acceptable substitutes (fallbacks) and degradation modes

This is what allows agents to say: “To run H1 hydrology at 30–100 m, I need soil texture + DEM + precip forcing; if soil grids unavailable, use SoilGrids + uncertainty penalty.”

---

## 2) Agent roles for a full “data plane”

### A) Data Requirements Agent

* reads the compiled model IR
* outputs a **data acquisition plan**:

  * datasets needed
  * coverage window
  * latency requirements
  * expected volume and compute cost

### B) Data Discovery Agent

* searches catalogs (ideally via **STAC** and official registries)
* selects best sources given region/latency/resolution/license
* outputs dataset pointers + access credentials needed (if any)

### C) Data Acquisition Agent

* performs downloads / stream subscriptions
* enforces rate limits, retries, and checksum validation
* stores raw data in a **content-addressed cache** (hash-based)

### D) Preprocessing Agent

* does all geospatial processing into *model-ready* forms:

  * reprojection, resampling, tiling
  * cloud masking, BRDF-ish corrections if needed
  * deriving products (NDVI/LAI proxies, fuel moisture proxies, burn severity)
  * time aggregation/disaggregation (hourly ↔ daily)
* writes outputs to chunked formats optimized for simulation (Zarr/Cloud-Optimized GeoTIFF)

### E) QA/QC Agent

* checks completeness, QA flags, outliers, temporal gaps
* attaches uncertainty estimates and quality masks
* decides whether to trigger fallback sources

### F) Provenance Agent

* records:

  * source dataset IDs
  * query parameters
  * exact transforms (container hash or code hash)
  * timestamps + checksums
* produces a reproducible “data bill of materials” for each sim run

### G) Streaming Update Agent (NRT)

* maintains subscriptions for active fire / weather / smoke / lightning streams
* posts “data update events” into the runtime so the model can re-assimilate

---

## 3) The “Data Contract”: how agents interact with the simulator

Treat data like processes: everything has a contract.

### DataSpec (per variable)

* `variable`: canonical registry name (units + semantics)
* `topology`: grid/network/points + resolution + vertical layers
* `extent`: bbox + CRS
* `time`: range + cadence + allowable latency
* `quality`: required QA bits + max cloud cover + uncertainty tolerance
* `lineage`: required provenance fields

### DataArtifact

* `uri` (local cache path / object storage path)
* `format` (COG, Zarr, NetCDF)
* `chunking/tiling`
* `qa_mask_uri`
* `uncertainty_uri`
* `checksum`
* `provenance_record_id`

Your runtime should refuse to run a module unless its DataSpecs are satisfied (or it explicitly opts into a degraded fallback path).

---

## 4) Remote sensing streams you’ll likely want (by domain)

### Fire (NRT + archival)

* **Active fire detections**: VIIRS/MODIS (NRT), GOES for high cadence (regional)
* **Burned area / severity**: Sentinel-2/Landsat derived (dNBR, etc.)
* **Fuel / vegetation state**: Sentinel-2 optical, Sentinel-1 SAR (cloud-resistant), land cover maps
* **Smoke / aerosols** (optional): satellite AOD products; useful if you later couple radiation impacts

### Hydrology

* **Precip forcing**: radar + gauge blends (regional), reanalysis downscales
* **Soil moisture**: SMAP (coarser, but assimilation constraints)
* **Snow**: MODIS snow cover + SWE products (regional)
* **DEM**: Copernicus/SRTM; hydro-conditioned flow direction products

### Ecology / BGC / Energetics

* **LAI/FPAR/NDVI**: MODIS/VIIRS for time series, Sentinel-2 for higher res snapshots
* **Canopy height/structure**: GEDI (where available), airborne LiDAR if you have it
* **Surface temperature**: thermal products (Landsat; coarser daily from other sources)
* **Albedo**: MODIS albedo products (energy balance)

(You don’t need all of these on day one—agents should pick the minimum required set per representation rung.)

---

## 5) Processing pipeline that actually works on a multi-GPU workstation

### Storage/layout choices

* **Raw archive**: keep original granules/scenes as-is (immutable)
* **Working cache**: COGs and chunked Zarr stores tiled to your sim grid
* **Feature store**: derived model-ready fields (fuels, LAI proxy, burn severity, etc.)

### Compute orchestration

* Use a task engine that can do parallel IO + CPU geoprocessing:

  * **Ray** or **Dask** for scheduling
* Use standard geospatial toolchain for correctness:

  * GDAL/rasterio, xarray, rioxarray
* GPU is most useful for:

  * heavy raster math over big tiles (feature extraction)
  * ML inference if you later add learned products (fuel classification, canopy segmentation)

### “Tile-first” processing

Everything should be:

1. query scenes overlapping tile
2. process into tile-local chunks
3. write tile chunks with consistent metadata

This makes streaming updates cheap (only update affected tiles).

---

## 6) Streaming design: event-driven data assimilation

Add a message bus inside your runtime:

* `DataUpdateEvent(variable, tiles, time_range, quality_delta)`
* Runtime Sentinel decides:

  * re-run assimilation for affected modules
  * trigger embedding/upgrades if thresholds crossed (e.g., new ignition → spawn fire domain)

Example:

* NRT active fire hit → spawn embedded fire solver around that tile cluster
* New precip radar scan → run hydrology subcycling for impacted basin tiles
* New burn severity map → apply disturbance operator to ecology/BGC tier

---

## 7) Governance and safety (so agents don’t become a mess)

Agents should **not** “download anything from anywhere” without rules.

Implement:

* **Allowlist of data providers** (NASA/NOAA/USGS/ESA/etc.)
* **Credential vault interface** (agents request token by dataset name; runtime supplies)
* **Rate limiting + backoff** per provider
* **Checksum + content hashing** (every raw file becomes immutable)
* **License enforcement**: provenance must include license, attribution, redistribution flags
* **Deterministic processing**: each transform has a versioned recipe; outputs are reproducible

This is what makes your system defensible and automatable.

---

## 8) What to build first (MVP “data plane”)

If you want to prove this quickly:

1. **STAC-based discovery + download** for 2–3 products (e.g., Sentinel-2 L2A, VIIRS active fire, a DEM)
2. **Tiling + reprojection pipeline** into your sim grid
3. One derived product:

   * burn severity proxy OR LAI proxy OR fuel classification
4. Provenance records that can fully reproduce outputs
5. Hook into runtime via `DataUpdateEvent`

That gives you end-to-end “agents can fetch + process remote sensing into model-ready fields.”

---

If you tell me your **target geography** (e.g., “Pacific Northwest, WA”) and your **base grid** (say 100 m or 250 m), I’ll give you a concrete initial DataSpec set (exact variables), and a minimal provider allowlist + processing recipes for the first integrated demo.

You can remove *most* of the human politics from the loop, but you can’t remove humans entirely without replacing them with **hard constraints, automated verification, and governance**—otherwise you just get “model drift” and quiet failure.

The good news: your framework (ontology + agents + dynamic compilation) is exactly the right shape to do this. The trick is to turn “humans out of the loop” into:

* **humans set objectives + guardrails once**
* agents do discovery/assembly/data/verification automatically
* the system only ships changes that pass objective gates

## What “humans out” can mean, concretely

### Human inputs (minimal, upfront)

* region/extent + required outputs
* acceptable error budgets per observable (and lead times)
* compute budget + latency targets
* hard constraints: conservation, safety bounds, data licensing allowlists

Everything else becomes automated.

---

## The architecture: a fully automated model factory

### 1) Ontology-driven search space

Agents maintain a library of process representations (“rungs”), each with:

* I/O variables + units
* assumptions/closures
* valid scale envelope
* known failure regimes
* cost model (GPU time/memory)
* *mandatory* benchmark coverage

### 2) Compiler produces candidates

Given a scenario spec, the compiler generates *many* candidate coupled systems (not one), by varying:

* representation choices per process
* coupling cadence
* embedding policies (where to run fine-scale fire/hydro)
* solver partitions (implicit/explicit)

### 3) Automated evaluation harness (the real key)

Each candidate gets:

* replayed against a standard benchmark suite
* scored with proper metrics (not cherry-picked)
* stress-tested in known failure regimes
* checked for conservation/positivity/stability

Only then does it get promoted.

### 4) Continuous assimilation + retraining + revalidation

Agents also:

* ingest remote sensing + NRT streams
* update fuel/hydrology/ecology states via assimilation operators
* re-run evaluation on rolling windows
* detect drift and regressions

---

## The “no humans” fix only works if you add these gates

### Gate A: Physics invariants

Hard fail if violated:

* mass/energy conservation bounds (or explicitly declared non-conservative approximations)
* positivity (no negative soil moisture, negative biomass)
* bounded fluxes, stable timestepping (CFL)

### Gate B: Skill and reliability

Hard fail if:

* it doesn’t beat baseline(s) on held-out periods
* it’s miscalibrated beyond threshold (if probabilistic outputs exist)
* it improves one metric by breaking another (multi-objective Pareto checks)

### Gate C: Monoculture/correlation risk

Fail or penalize if:

* too many modules share the same assumption fingerprint
* performance gains come only from tuned targets
* improvement disappears under regime stratification

### Gate D: Reproducibility

Hard fail if:

* data lineage isn’t complete
* transforms aren’t deterministic/versioned
* results can’t be rebuilt from hashes

This is how you replace “expert judgment” with enforceable machine rules.

---

## How agents “replace humans” without becoming blind

### A) Use *adversarial* agents, not just helpful ones

You want internal competition, but between agents:

* **Builder Agent**: assembles best candidate
* **Breaker Agent**: tries to find regimes/cases where it fails
* **Auditor Agent**: checks invariants, provenance, licensing, reproducibility
* **Red Team Agent**: tries to exploit coupling loopholes (double counting, hidden leakage)

This gives you the benefits of skepticism without human politics.

### B) Automatic counterfactuals

For every proposed change:

* swap one module at a time (ablation)
* re-run subset of benchmarks
* attribute gains/losses
  This catches “fixes” that just move error around.

---

## Where you *should* keep a minimal human role

Even with full automation, humans still define:

* what “good” means (mission objective)
* what failures are unacceptable (safety constraints)
* when the environment changes (new sensors, new regions, new requirements)

But humans should not be choosing parameterizations, approving papers, or arguing about whose scheme wins. They only edit the **objective function and guardrails**.

---

## A realistic MVP that demonstrates “humans out”

For your stack (fire + hydro + ecology/BGC + energy/solar), the clean MVP is:

1. **Automated data plane**

* pulls DEM + landcover + Sentinel-2 + VIIRS active fire + precip forcing
* produces tiles: fuels, LAI proxy, burn severity updates, soil moisture proxy

2. **Two-tier model**

* slow tier: cohort ecology + soil C/N + daily energy balance
* event tier: embedded fire spread + embedded runoff response

3. **Automated benchmark suite**

* choose 20–50 historical events in one region (e.g., PNW)
* score: perimeter timing, burn severity classes, post-fire runoff peaks, recovery trajectory proxies

4. **Autonomous model selection**

* agents compile and test multiple candidate graphs
* system promotes the best candidate automatically if it passes gates

That’s “humans out of the loop” in a way that actually works.

---

If you want, I can write the *exact* “guardrails spec” as machine-readable policies (invariants + benchmark thresholds + allowed datasets) and the agent roles/messages so your orchestrator can run it like a CI pipeline for models.

## References

Existing Earth systems models (ESMs), regional models, and other process models can be found in the `reference` directory.
