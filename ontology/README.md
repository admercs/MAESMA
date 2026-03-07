# MAESMA Unified Ontology

This directory contains the unified ontology that governs all knowledge in MAESMA. Three interconnected knowledge domains — **Processes**, **Datasets**, and **Metrics** — are linked into a single queryable graph that agents use for model assembly, data acquisition, benchmarking, and autonomous self-improvement.

## Ontology Architecture

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         UNIFIED ONTOLOGY GRAPH                          │
│                                                                         │
│   ┌─────────────┐     requires      ┌─────────────┐     scores_with    │
│   │   PROCESS    │────────────────►  │   DATASET    │──────────────────►│
│   │   DOMAIN     │                   │   DOMAIN     │                   │
│   │             ◄├── validates ──────┤              │   ┌───────────┐   │
│   │  Process     │                   │  Observable  │   │  METRIC   │   │
│   │  Family      │   calibrates_with │  Product     │   │  DOMAIN   │   │
│   │  Represent.  │──────────────────►│  AccessSpec  │   │           │   │
│   │  Rung        │                   │  Transform   │   │  Metric   │   │
│   │  Module      │   evaluated_by    │  License     │   │  Protocol │   │
│   │  Assumption  │──────────────────►│  QualitySpec │   │  Score    │   │
│   │  Constraint  │                   │              │   │  Fitness  │   │
│   └──────┬──────┘                   └──────┬──────┘   └─────┬─────┘   │
│          │                                  │                 │         │
│          │         ┌────────────────────────┘                 │         │
│          │         │                                          │         │
│          ▼         ▼                                          ▼         │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │                    CROSS-DOMAIN RELATIONS                       │   │
│   │                                                                 │   │
│   │  Representation --requires_forcing--> Product                   │   │
│   │  Representation --calibrates_against--> Observable              │   │
│   │  Representation --evaluated_by--> ScoringProtocol               │   │
│   │  Observable --measured_by--> Product                            │   │
│   │  ScoringProtocol --uses_metric--> Metric                       │   │
│   │  SkillRecord --scores--> (Representation, Observable, Metric)   │   │
│   │  Product --discovered_via--> CatalogSource                      │   │
│   │  Metric --penalizes--> Constraint (conservation violation)      │   │
│   └─────────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────────┘
```

## Domain 1: Process Ontology

Concepts describing **what the model can do**.

### Concept Classes

| Class              | Description                                                                                   |
| ------------------ | --------------------------------------------------------------------------------------------- |
| `ProcessFamily`    | Top-level domain grouping (Fire, Hydrology, Ecology, Biogeochemistry, Radiation)              |
| `Process`          | A named physical/ecological process (e.g., Infiltration, CrownFireSpread, SoilRespiration)    |
| `Representation`   | A specific model form for a process at a scale/regime (a "rung" on the ladder)                |
| `Module`           | A concrete implementation of a representation (code, solver, emulator)                        |
| `StateVariable`    | Typed + unit-aware variable the representation consumes or produces                           |
| `Assumption`       | Closure or simplification the representation relies on (hydrostatic, well-mixed, equilibrium) |
| `Constraint`       | Conservation law, positivity, boundedness, or stability requirement                           |
| `ScaleEnvelope`    | Valid spatial/temporal resolution range + regime tags                                         |
| `NumericalForm`    | Solver characteristics (explicit/implicit, stiffness, CFL)                                    |
| `CouplingOperator` | Remap, aggregation, disaggregation, or state projection between representations               |

### Key Relations

| Relation             | From → To                                          | Meaning                                           |
| -------------------- | -------------------------------------------------- | ------------------------------------------------- |
| `has_process`        | Family → Process                                   | Domain contains this process                      |
| `has_representation` | Process → Representation                           | Process can be modeled by this representation     |
| `has_module`         | Representation → Module                            | Representation is implemented by this code/solver |
| `requires`           | Representation → StateVariable                     | Input dependency                                  |
| `produces`           | Representation → StateVariable                     | Output production                                 |
| `valid_over`         | Representation → ScaleEnvelope                     | Where this representation is applicable           |
| `assumes`            | Representation → Assumption                        | What simplifications are made                     |
| `conserves`          | Representation → Constraint                        | What conservation properties are guaranteed       |
| `compatible_with`    | Representation → Representation                    | Can be coupled without conflict                   |
| `incompatible_with`  | Representation → Representation                    | Mutual exclusion (double-counting)                |
| `composed_with`      | Representation → CouplingOperator → Representation | How two representations exchange state            |
| `has_cost_model`     | Module → CostModel                                 | Computational expense profile                     |
| `has_skill_model`    | Representation → SkillModel                        | Expected/measured accuracy per regime             |
| `supersedes`         | Representation → Representation                    | Strictly more capable (same physics, finer scale) |

## Domain 2: Dataset Ontology

Concepts describing **what data exists and how to access it**.

### Concept Classes

| Class                    | Description                                                                                    |
| ------------------------ | ---------------------------------------------------------------------------------------------- |
| `Observable`             | A measurable quantity in the real world, mapped to canonical variable registry                 |
| `Product`                | A specific dataset that provides one or more observables (e.g., "SMAP L3 Daily Soil Moisture") |
| `CatalogSource`          | A searchable catalog where products are discovered (STAC, CMR, CKAN, OpenDAP)                  |
| `AccessSpec`             | How to retrieve the product (protocol, endpoint, auth, rate limits)                            |
| `TransformRecipe`        | Versioned processing chain from raw product to model-ready field                               |
| `QualitySpec`            | Uncertainty, QA flags, spatial/temporal gaps, representativeness                               |
| `License`                | Usage rights, attribution, redistribution constraints                                          |
| `LatencyClass`           | Archival / daily / hourly / NRT / real-time                                                    |
| `SpatiotemporalCoverage` | Extent, resolution, time range, cadence                                                        |

### Key Relations

| Relation          | From → To                        | Meaning                                                   |
| ----------------- | -------------------------------- | --------------------------------------------------------- |
| `measures`        | Product → Observable             | This product provides data for this observable            |
| `discovered_via`  | Product → CatalogSource          | Where to find this product                                |
| `accessed_via`    | Product → AccessSpec             | How to download/stream                                    |
| `transformed_by`  | Product → TransformRecipe        | How raw data becomes model-ready                          |
| `has_quality`     | Product → QualitySpec            | Uncertainty and coverage info                             |
| `licensed_as`     | Product → License                | Usage constraints                                         |
| `has_latency`     | Product → LatencyClass           | How fresh the data is                                     |
| `covers`          | Product → SpatiotemporalCoverage | Where and when                                            |
| `substitutes_for` | Product → Product                | Fallback when primary is unavailable                      |
| `derived_from`    | Product → Product                | Lineage (e.g., dNBR derived from Sentinel-2)              |
| `validates`       | Observable → StateVariable       | This real-world quantity can validate this model variable |

### Data Discovery Relations (Autonomous)

| Relation               | From → To                   | Meaning                                                                         |
| ---------------------- | --------------------------- | ------------------------------------------------------------------------------- |
| `potentially_measures` | CatalogSource → Observable  | This catalog *may* contain products for this observable (search target)         |
| `discovery_query`      | Observable → SearchTemplate | Parameterized query to find products in catalogs                                |
| `relevance_score`      | Product → Observable        | How well this product actually constrains this observable (0–1)                 |
| `novelty_score`        | Product → SkillStore        | How much new information this product adds beyond existing observation coverage |

## Domain 3: Metric Ontology

Concepts describing **how model–observation comparisons are performed and scored**.

### Concept Classes

| Class             | Description                                                                                                                      |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `Metric`          | A named scoring function (RMSE, KGE, CRPS, bias, timing error, conservation residual, etc.)                                      |
| `ScoringProtocol` | A complete prescription for comparing model output to observations (includes spatial/temporal alignment, masking, normalization) |
| `FitnessFunction` | A multi-objective aggregation of metrics into a scalar or Pareto-comparable fitness value                                        |
| `SkillRecord`     | A single evaluation result: (configuration, region, regime, observation, metric vector, timestamp)                               |
| `SkillModel`      | A statistical model of expected skill as a function of region/regime/context — updatable with evidence                           |
| `CostModel`       | Computational cost as a function of grid size, timestep, and hardware — updatable with measurements                              |

### Key Relations

| Relation           | From → To                                                                     | Meaning                                              |
| ------------------ | ----------------------------------------------------------------------------- | ---------------------------------------------------- |
| `uses_metric`      | ScoringProtocol → Metric                                                      | Which metrics are computed                           |
| `compares`         | ScoringProtocol → (StateVariable, Observable)                                 | What model variable is compared to what observation  |
| `alignment_method` | ScoringProtocol → AlignmentSpec                                               | How model and obs are spatiotemporally matched       |
| `aggregates_via`   | FitnessFunction → Metric[]                                                    | How multiple metrics become fitness                  |
| `penalizes`        | Metric → Constraint                                                           | Conservation-violation metrics linked to constraints |
| `records`          | SkillRecord → (Representation, Region, Regime, ScoringProtocol, MetricVector) | Full evaluation provenance                           |
| `updates`          | SkillRecord → SkillModel                                                      | Evidence updates expected-skill models               |
| `measured_cost`    | SkillRecord → CostModel                                                       | Actual runtime measurements update cost predictions  |

## Cross-Domain Joins

The power of the unified ontology is in the **cross-domain queries** it enables:

| Query Pattern                                                                   | Used By                                     |
| ------------------------------------------------------------------------------- | ------------------------------------------- |
| "Which representations have the best skill for this observable in this regime?" | Model Assembly Agent                        |
| "Which observations are we missing for this process family in this region?"     | Active Learning Agent, Data Discovery Agent |
| "Which untested rung combinations would most reduce posterior uncertainty?"     | Active Learning Agent                       |
| "What data products exist in STAC that could validate this state variable?"     | Autonomous Data Discovery Agent             |
| "Which representations are cheapest while still exceeding skill threshold X?"   | Compiler (budget-constrained)               |
| "Has skill for representation R degraded in recent evaluation periods?"         | Skill Librarian (drift detection)           |
| "Which process interactions produce the largest skill differences?"             | Hypothesis Engine (factorial design)        |
| "What new remote sensing products have appeared that overlap our domain?"       | Data Scout Agent (autonomous)               |

## Serialization

The ontology is stored as:

- **Manifests** — YAML files per concept instance (one file per representation, product, protocol, etc.)
- **Graph index** — In-memory property graph built at startup from manifests; supports fast traversal and SPARQL-like queries
- **Versioned** — All manifests are version-controlled; the graph index is deterministically reproducible from manifests
- **Future** — Optional RDF/OWL export for interoperability with semantic web tooling

## Directory Layout

```
ontology/
  schema/                    # Ontology schema definitions (types, relations, constraints)
    process_schema.yaml
    dataset_schema.yaml
    metric_schema.yaml
    cross_domain_schema.yaml
  processes/                 # Process ontology instances
    families/                # ProcessFamily manifests
    representations/         # Representation manifests (one per rung)
    assumptions/             # Assumption catalog
    constraints/             # Conservation/stability constraint catalog
  datasets/                  # Dataset ontology instances
    observables/             # Observable → StateVariable mappings
    products/                # Product manifests (one per dataset)
    catalogs/                # CatalogSource definitions (STAC endpoints, CMR, etc.)
    transforms/              # TransformRecipe definitions
    licenses/                # License catalog
  metrics/                   # Metric ontology instances
    metrics/                 # Metric definitions
    protocols/               # ScoringProtocol definitions
    fitness/                 # FitnessFunction definitions
  graph/                     # Compiled graph index (generated, not hand-edited)
```
