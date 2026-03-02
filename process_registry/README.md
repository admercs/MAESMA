# Process Registry

The Process Registry catalogs every process representation available to the MAESMA compiler. Each entry is a **machine-readable manifest** that declares the representation's contract (I/O, scale, assumptions, conservation, cost) plus links to its concrete implementation code. The registry is the single source of truth for what the system can build models from.

## Purpose

1. **Catalog** — Enumerate all available process representations with full metadata.
2. **Compare** — Enable side-by-side comparison of competing representations for the same process (same inputs, different fidelity/cost/assumptions).
3. **Select** — Provide the compiler and agents with structured information to choose the right rung per regime.
4. **Track** — Link each representation to its accumulated skill records, enabling the self-improvement loop to update preferences.
5. **Extend** — Make adding a new representation as simple as dropping a manifest + implementation into the registry.

## Registry Entry Schema

Every entry is a YAML manifest conforming to this schema. The registry enforces that all required fields are present and well-typed before a representation is available to the compiler.

```yaml
# === Identity ===
id: "hydrology.infiltration.richards_1d"       # Unique dot-path ID
version: "0.3.0"                                 # Semver
family: "Hydrology"                              # ProcessFamily reference
process: "Infiltration"                          # Process reference
rung: 1                                          # Rung index (0 = coarsest)
name: "1D Richards Equation Infiltration"
description: >
  Vertically-resolved soil water flow using the Richards equation
  solved implicitly per column. Supports heterogeneous soil layers.

# === Scale Envelope ===
scale:
  spatial:
    dx_min: 30.0       # meters
    dx_max: 300.0
    topology: "raster"  # raster | network | embedded_raster | point
  temporal:
    dt_min: 60.0        # seconds
    dt_max: 3600.0
  regimes:              # Where this representation is appropriate
    - "humid"
    - "semi-arid"
    - "post-fire"
    - "snow-dominated"
  regimes_excluded:     # Where it is known to fail or be inappropriate
    - "permafrost"      # Would need freeze-thaw coupling

# === Interface Contract ===
inputs:
  - name: "precipitation_rate"
    variable: "P"
    units: "kg m-2 s-1"
    discretization: "raster_cell"
    temporal: "instantaneous"
  - name: "evapotranspiration_demand"
    variable: "ET_pot"
    units: "kg m-2 s-1"
    discretization: "raster_cell"
    temporal: "instantaneous"
  - name: "soil_properties"
    variable: "soil_texture"
    units: "categorical"
    discretization: "raster_cell"
    temporal: "static"
    data_source: "SoilGrids250m"

outputs:
  - name: "soil_moisture_profile"
    variable: "soil_moisture"
    units: "m3 m-3"
    discretization: "raster_cell"
    vertical: "layers"
    layers: [0.05, 0.15, 0.30, 0.60, 1.00, 2.00]  # meters
    update_authority: "owns"
  - name: "infiltration_excess_runoff"
    variable: "runoff_surface"
    units: "kg m-2 s-1"
    discretization: "raster_cell"
    update_authority: "owns"
  - name: "drainage"
    variable: "drainage_bottom"
    units: "kg m-2 s-1"
    discretization: "raster_cell"
    update_authority: "owns"

# === Assumptions & Closures ===
assumptions:
  - id: "vertical_flow_only"
    description: "No lateral subsurface flow between columns"
  - id: "rigid_soil_matrix"
    description: "Soil porosity does not change with moisture"
closures:
  - id: "van_genuchten"
    description: "Van Genuchten-Mualem soil hydraulic model"

# === Conservation Properties ===
conservation:
  - property: "mass"
    type: "exact"
    description: "Water mass conserved to machine precision per column"
  - property: "positivity"
    type: "exact"
    description: "Soil moisture bounded in [residual, porosity]"

# === Numerical Form ===
numerics:
  time_integration: "implicit"
  solver: "Newton-Raphson with line search"
  stiffness: "high"
  cfl_constrained: false
  convergence_tolerance: 1.0e-6
  max_iterations: 50

# === Calibration Parameters ===
parameters:
  - name: "Ks"
    description: "Saturated hydraulic conductivity"
    units: "m s-1"
    range: [1.0e-7, 1.0e-3]
    prior: { distribution: "log_normal", mu: -10.0, sigma: 2.0 }
    identifiability: "high"
  - name: "alpha_vg"
    description: "Van Genuchten alpha parameter"
    units: "m-1"
    range: [0.5, 15.0]
    prior: { distribution: "log_normal", mu: 1.0, sigma: 1.0 }
    identifiability: "medium"
  - name: "n_vg"
    description: "Van Genuchten n parameter"
    units: "dimensionless"
    range: [1.1, 3.0]
    prior: { distribution: "normal", mu: 1.8, sigma: 0.4 }
    identifiability: "medium"

# === Data Requirements ===
data_requirements:
  forcing:
    - variable: "P"
      min_cadence: "hourly"
      sources: ["AORC", "NLDAS-2", "ERA5-Land"]
    - variable: "ET_pot"
      min_cadence: "hourly"
      sources: ["computed_from_energy_balance", "Penman-Monteith"]
  static:
    - variable: "soil_texture"
      sources: ["SoilGrids250m", "STATSGO", "SSURGO"]
    - variable: "DEM"
      sources: ["Copernicus30m", "SRTM"]
  calibration_targets:
    - observable: "soil_moisture"
      products: ["SMAP_L3", "in_situ_probes"]
    - observable: "streamflow"
      products: ["USGS_gauges"]
      note: "Indirect — requires coupling with routing"

# === Implementation ===
implementations:
  - id: "richards_1d_rust_cpu"
    language: "Rust"
    backend: "CPU"
    entrypoint: "modules/hydrology/richards_1d/src/lib.rs"
    gpu_support: false
    differentiable: false
    cost_model:
      flops_per_cell_per_step: 5000
      memory_per_cell_bytes: 480   # 6 layers × 10 vars × 8 bytes
      scaling: "linear"
  - id: "richards_1d_cuda"
    language: "CUDA"
    backend: "GPU"
    entrypoint: "modules/hydrology/richards_1d/src/cuda/kernel.cu"
    gpu_support: true
    differentiable: false
    cost_model:
      flops_per_cell_per_step: 5000
      memory_per_cell_bytes: 240   # float32
      scaling: "linear"
      min_cells_for_gpu_advantage: 100000

# === Skill Model (updatable) ===
skill_model:
  source: "expert_prior"           # expert_prior | empirical | hybrid
  last_updated: "2026-01-15"
  expected_skill:
    - regime: "humid"
      metric: "KGE"
      expected: 0.75
      confidence: "low"            # Will be updated by benchmarking
    - regime: "semi-arid"
      metric: "KGE"
      expected: 0.60
      confidence: "low"
    - regime: "post-fire"
      metric: "KGE"
      expected: 0.50
      confidence: "very_low"
      notes: "Hydrophobicity coupling untested"
  known_weaknesses:
    - "No lateral flow — underestimates hillslope redistribution in steep terrain"
    - "Frozen soil not handled — fails in permafrost regimes"
  superseded_by: "hydrology.integrated_surface_subsurface.parflow"  # Rung 2

# === Provenance ===
provenance:
  authors: ["MAESMA Team"]
  based_on: ["ParFlow Richards solver", "CLM soil hydrology"]
  references:
    - "Richards, L.A. (1931). Capillary conduction of liquids through porous mediums."
    - "van Genuchten, M.T. (1980). A closed-form equation for predicting the hydraulic conductivity..."
  license: "MIT"

# === Ontology Links ===
ontology:
  compatible_with:
    - "hydrology.routing.kinematic_wave"
    - "hydrology.routing.diffusive_wave"
    - "ecology.succession.cohort_mosaic"
    - "radiation.energy_balance.sub_daily"
  incompatible_with:
    - "hydrology.infiltration.bucket"          # Same process, different rung — mutually exclusive
    - "hydrology.infiltration.green_ampt"      # Same process, different rung
  requires_coupling_with:
    - representation: "radiation.energy_balance.*"
      reason: "Needs ET demand"
      coupling_frequency: "hourly"
    - representation: "hydrology.routing.*"
      reason: "Passes runoff to routing"
      coupling_frequency: "per_timestep"
```

## Registry Directory Layout

```
process_registry/
  schema/
    manifest_schema.yaml       # JSON Schema / YAML schema for validation
    manifest_schema.json       # JSON Schema version for tooling
  hydrology/
    infiltration/
      bucket.yaml              # H0: Bucket model
      green_ampt.yaml          # H0.5: Green-Ampt
      richards_1d.yaml         # H1: 1D Richards (example above)
      parflow_integrated.yaml  # H2: Full ParFlow
    routing/
      kinematic_wave.yaml      # Default routing
      diffusive_wave.yaml      # Upgrade routing
    groundwater/
      water_table_proxy.yaml   # Simple GW
  fire/
    spread/
      stochastic_regime.yaml   # F0: Statistical fire regime
      rothermel_cellular.yaml  # F1: Rothermel surface + CFS FBP crown
      wind_aware_spread.yaml   # F2: Plume-coupled spread (Balbi/level-set)
      wrf_fire_coupled.yaml    # F3: Full atmosphere coupling
    effects/
      severity_mortality.yaml  # Fire effects → disturbance operator
  ecology/
    succession/
      cohort_mosaic.yaml       # E0: LANDIS-like
      size_structured.yaml     # E1: Diameter classes
      individual_based.yaml    # E2: iLand-like
    fuels/
      fuel_strata_derivation.yaml
  biogeochem/
    carbon/
      big_leaf.yaml            # B0
      multi_pool.yaml          # B1
    nitrogen/
      simple_n.yaml
      cn_coupled.yaml
  radiation/
    solar/
      daily_potential.yaml     # R0
      sub_daily_partitioned.yaml  # R1
    energy_balance/
      canopy_energy.yaml
    fuel_moisture/
      equilibrium_drying.yaml
```

## How the Registry Is Used

### By the Compiler
1. Query registry for all representations matching target process families + scale envelope.
2. Filter by regime tags, data availability, and compute budget.
3. Check `compatible_with` / `incompatible_with` to prune invalid combinations.
4. Read I/O contracts to verify state-space closure.
5. Read cost models to estimate total compute requirement.

### By the Benchmarking Agent
1. Select two or more representations for the same process.
2. Compile identical models differing only in the target representation.
3. Run both against the same observation period.
4. Score with the same `ScoringProtocol` and write `SkillRecords`.
5. Update the `skill_model` section of each manifest with empirical results.

### By the Autonomous Optimizer
1. Read current skill models and posteriors from the Skill Score Store.
2. Identify representations with high uncertainty or under-tested regimes.
3. Generate candidate configurations (hypothesis engine).
4. Schedule experiments, collect scores, update ontology.
5. Compiler's default preferences shift automatically.

### By the Data Scout (Autonomous Data Discovery)
1. Read `data_requirements.calibration_targets` from each representation.
2. Identify observables with sparse or missing coverage.
3. Query `CatalogSource` entries (STAC, CMR) for new products.
4. If found, register new `Product` manifest, run preprocessing, add to observation registry.
5. Re-score representations with expanded observation coverage.

## Adding a New Representation

1. Create a YAML manifest in the appropriate subdirectory.
2. Fill all required fields (the validator will reject incomplete manifests).
3. Implement the code and set the `entrypoint` path.
4. Run `maesma registry validate <path>` to check schema compliance.
5. Run `maesma registry check-closure <path>` to verify I/O compatibility with the existing graph.
6. The representation is now available to the compiler and benchmarking agents.

## Fitness-Driven Process Selection

The registry enables **autonomous optimization of process selection**:

```
                    ┌──────────────────────────────┐
                    │    AUTONOMOUS OPTIMIZER       │
                    │                              │
                    │  1. Query skill models for   │
                    │     all representations of   │
                    │     target process            │
                    │                              │
                    │  2. Identify Pareto frontier  │
                    │     (skill vs cost)           │
                    │                              │
                    │  3. Select representation     │
  ┌────────────┐   │     maximizing fitness given  │   ┌──────────────┐
  │  PROCESS   │──►│     budget + regime           │──►│  COMPILED    │
  │  REGISTRY  │   │                              │   │  MODEL       │
  └────────────┘   │  4. If uncertainty high:      │   └──────┬───────┘
                    │     schedule benchmark        │          │
                    │     experiment first          │          ▼
                    │                              │   ┌──────────────┐
                    │  5. If new data found:        │   │  SKILL       │
                    │     re-score with expanded    │   │  RECORDS     │
                    │     observation coverage      │   └──────┬───────┘
                    │                              │          │
                    └──────────────┬───────────────┘          │
                                   │                          │
                                   └──────── feedback ◄───────┘
```

### Fitness Function

For each representation $r$ in regime $g$ and region $\ell$, the fitness is:

$$F(r, g, \ell) = \sum_{m \in \text{metrics}} w_m \cdot S_m(r, g, \ell) - \lambda \cdot C(r)$$

where:
- $S_m$ is the skill score for metric $m$ (from Skill Score Store)
- $w_m$ are metric weights (from the user's objective specification or defaults)
- $C(r)$ is the normalized computational cost
- $\lambda$ is the cost penalty (from compute budget)

When skill data is sparse, the optimizer falls back to the expert prior in the `skill_model` and flags the regime for active experimentation.
