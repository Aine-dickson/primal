# Resolutions Carried Forward

This record lists the resolutions reached in the exploration record (`docs/design/01` to `09.11`) and states what happens to each one in the project's normative work. It answers the question: **which of the earlier analysis is kept, and where does it live now?**

It complements the divergence register. The register records departures and new decisions; this record records continuity. Most of the earlier analysis is carried forward.

## Status values

- **Adopted:** carried forward as a normative position. The core semantics spec restates it in final form and cites the `R-##` ID.
- **Adopted, amended:** carried forward with a change recorded in the divergence register (named).
- **Direction only:** kept as guidance for a later stage, not as a decision (for example syntax style before the syntax study).
- **Superseded:** replaced by a register entry (named). Kept here so the history is traceable.

IDs (`R-##`) are permanent. New resolutions found in a later reading are appended, not inserted.

## Summary

| Status | Count |
|---|---|
| Adopted | 57 |
| Adopted, amended | 8 |
| Direction only | 2 |
| Superseded | 3 |
| **Total** | **70** |

## 1. Method and founding principles (01 to 05b)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-01 | The case catalogue is a test space, not a feature roadmap. For every case ask "what does this case require an author to express?", then find the smallest set of general abstractions. | 01 Guiding Principle | Adopted. The catalogue remains the scope horizon (D-017). |
| R-02 | Organize by general capabilities, not by scientific subjects; domains emerge from combinations of capabilities. | 03 section 1 | Adopted |
| R-03 | Do not design syntax until the meaning of the language is known. | 05b section 109, 09.1 section 32 | Adopted. The 09.x series departed from its own rule; D-006 reinstates it. |
| R-04 | The language expresses meaning, not implementation (no solver, buffer, cache or hit-testing detail in author code). | 09.1 section 32 | Adopted |
| R-05 | Layered separation: what exists, what is true, how it changes, how it is calculated, what can be observed, how it is shown, how the user acts. | 05b section 108 | Adopted |
| R-06 | Domain knowledge plugs into the architecture rather than defining it; physics engines, solvers, renderers stay outside the semantic core. | 05b sections 107-108, 05c sections 80-81 | Adopted |

## 2. Core model (05c)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-07 | The ten core invariants: Model is not Scene; Model is not Simulation; Simulation is not Animation; Model Space is not Render Space; Semantic State is not Computational State; Semantic State is not Presentation State; Equation is not Solver; Domain Concept is not Core Primitive; Observation is not Intervention; Representation is not what it represents. | 05c section 86 | Adopted |
| R-08 | Quantities carry magnitude and unit; unit and dimension are distinct; dimensional arithmetic is checked. | 05c sections 5-6, 09.5 sections 7-9 | Adopted, amended by D-014 (accepted; affine quantities, dimensionless literals). |
| R-09 | Point and Vector are distinct (point + vector gives point; point - point gives vector; point + point is invalid). | 09.5 section 12, 09.11 type section 3 | Adopted, amended by D-014 (accepted; same rules for affine quantities). |
| R-10 | Distribution is distinct from random source; the seed is controllable for reproducibility. | 05c sections 13-14 | Adopted. D-013 (accepted) adds event-time sampling. |
| R-11 | Domain is first-class; space and time are domains; the core does not assume Euclidean 2D. | 05c sections 15-17 | Adopted. The 2D first slice (D-001) is sequencing only. D-014 (accepted) removes the `Point<2>` leak from general interfaces. |
| R-12 | Identity is semantic, not visual, and stays stable when collections reorder. | 05c section 19 | Adopted |
| R-13 | Stored state and derived state are distinct; derived values need not be stored. | 05c section 23 | Adopted |
| R-14 | Relations are first-class, may carry properties, and their topology can change at runtime. | 05c sections 26-27 | Adopted |
| R-15 | Fields are first-class; their semantic meaning is independent of computational storage. | 05c sections 28-29 | Adopted. Restored by D-010 (accepted) after 09.11 reduced fields to functions. |
| R-16 | Validity categories: valid, invalid, unknown, unavailable, pending. Failures are not ordinary values. | 05c section 31 | Adopted |
| R-17 | Behavior and evolution are distinct (a function computes without changing state). | 05c section 38 | Adopted |
| R-18 | An equation needs an explicit role (definition, constraint, relationship, evolution law, displayed statement) and never implies a solver. | 05c sections 11, 70 | Adopted, amended by D-007 (v1 roles: display and runtime check; evolution is written causally). |
| R-19 | Observation is distinct from intervention; measurements may carry unit, timestamp, uncertainty, resolution and instrument model. | 05c sections 50-51 | Adopted |
| R-20 | Three state spaces: semantic, computational, presentation. | 05c section 68 | Adopted |
| R-21 | Experiment, snapshot and branching are higher-level constructs composed from core concepts. | 05c sections 74-76 | Adopted |
| R-22 | Educational activity (lesson, question, feedback) sits above the scientific core. | 05c section 92 | Adopted. Consistent with D-017: education is the purpose, and its constructs are a layer over the model. |

## 3. Runtime semantics (06)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-23 | Three clocks: simulation time, presentation time, wall-clock time. | 06 section 3 | Adopted |
| R-24 | Semantic state is authoritative; computational and presentation state are derived. | 06 section 5 | Adopted |
| R-25 | Transitions read, compute, resolve, validate and commit; failed transitions do not partially commit. | 06 sections 6-7, 28-29, 66 | Adopted |
| R-26 | Write categories: independent, compatible (combined), exclusive, constraint-resolved, explicit conflict. The runtime never silently picks a winner. | 06 section 10 | Adopted. Refined by R-57 and D-005. |
| R-27 | Variable timesteps are allowed; a discrete step is not a time interval. | 06 sections 14-15 | Adopted |
| R-28 | Events defined by conditions are located by detecting the crossing, not by sampling at fixed steps. | 06 section 23 | Adopted. Formalized by D-004. |
| R-29 | Same-time events follow a deterministic, semantically defined policy, never machine execution order. | 06 sections 17-18 | Adopted. Extended by D-004 (superdense time for cascades). |
| R-30 | Reproducible parallel randomness uses deterministic streams (entity-indexed, event-indexed, counter-based). | 06 section 21 | Adopted |
| R-31 | Simulation frequency is independent of render frequency; interpolation between states is a presentation mechanism. | 06 sections 35-37 | Adopted |
| R-32 | Static models and pure animation are supported without a simulation clock. | 06 sections 59-60 | Adopted. D-009 (accepted) adds the explanation timeline as a first-class layer alongside it. |
| R-33 | Interactions identify their target domain: model, parameter, execution, view. | 06 section 42 | Adopted |
| R-34 | Determinism levels: strong, numerical, non-deterministic. | 06 section 67 | Adopted, amended by D-015 (accepted; platform scope). |
| R-35 | Cycles are not automatically errors. | 06 section 11, 09.7 composition invariant 7 | Adopted, amended by D-008 (accepted: cycles are classified statically; instantaneous cycles need a declared solver). |
| R-36 | The fifteen runtime invariants. | 06 section 74 | Adopted |

## 4. Projection and representation (07)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-37 | The twenty representation invariants (model is not representation; representation is not pixels; projections may be lossy and are not automatically invertible; visibility does not imply existence; and so on). | 07 section 130 | Adopted |
| R-38 | Projection takes source, context and configuration: `Projection(source, context, configuration) -> representation`. | 07 section 131 | Adopted |
| R-39 | One concept, many valid representations, chosen by content and intent (the radioactive decay example: particles, N(t) graph, half-life timeline, experiment panel). | Design conversation before 01; 07 | Adopted. Extended to output media by D-018. |

## 5. Interaction (08)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-40 | The twenty interaction invariants (input is not interaction; interaction is not operation; model-changing interactions name their semantic target; temporary interaction state is not committed state; and so on). | 08 section 92 | Adopted |
| R-41 | Direct manipulation proposes state; commit follows validation; interactions are transactional and support undo. | 08 sections 25-28 | Adopted |
| R-42 | The language need not mirror the internal layers syntactically. | 08 section 97 | Adopted |

## 6. DSL analysis (09.1 to 09.9)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-43 | Language requirements inventory (model, mathematics, behavior, context, computation, observation, projection, view, interaction, animation, experiment, composition, extension). | 09.1 section 31 | Adopted as the completeness checklist. The first slice (D-001) covers a subset. |
| R-44 | Hybrid surface style: mathematics looks mathematical, models look declarative, behavior looks rule and process oriented, views look declarative, interaction looks event oriented. | 09.3 section 23, 09.11 section 9 | Direction only. Input to the syntax study (D-006), evaluated against programming-literate authors (D-002). |
| R-45 | Surface constructs need not correspond one-to-one with kernel primitives; convenience constructs lower into the kernel. | 09.3 section 25, 09.11.7.1 | Adopted |
| R-46 | Distinctions the language must preserve: declaration, definition, initialization, assignment, derivation, equation, constraint, evolution; observation, projection, representation; input, interaction, operation. | 09.4 section 33 | Adopted. Operators are chosen in the syntax study (D-006). |
| R-47 | The syntax tree records what the author wrote; the semantic IR records what the author meant. Design the IR before the parser. | 09.4 sections 34-35 | Adopted. Strengthened by D-019: the IR must be complete and serializable so Mava Studio can edit it. |
| R-48 | Type invariants: type is not value, identity, role, state, or machine representation; unit is not dimension; expression is not evaluated value; type validity is not state validity; capability is not concrete type; interface is not inheritance; constraint is not type; alias is not a new nominal type. | 09.5 section 59, 09.5b section 64 | Adopted |
| R-49 | Type model: nominal semantic identity, explicit capabilities (interfaces), generic constrained behavior, composition-based specialization. "A domain type establishes semantic identity; a capability establishes reusable semantic compatibility." | 09.5b sections 62-66 | Adopted |
| R-50 | Value restrictions are constraints (`mass > 0 kg`), not proliferating refined types (`PositiveMass`). | 09.11 type section 7 | Adopted |
| R-51 | Absent, unknown, unavailable, pending and invalid are distinct and never collapse to `null`. | 09.11 type section 9 | Adopted |
| R-52 | Binding chain: source name, scope, binding, semantic identity, type/role/capabilities, value or model element. Invariants: a source name identifies a binding and semantic identity is independent of the name; lexical scope decides visibility while semantic ownership decides what a name refers to. | 09.6 sections 69-71 | Adopted. Formal definition in the core spec. |
| R-53 | Composition invariants: no implied inheritance; explicit connections; components own their mutable state; no silent namespace or state merging; definition is distinct from instance; subsystems nest; composition decides neither execution order nor representation. Inheritance limited. | 09.7 sections 57, 62-63 | Adopted |
| R-54 | Error model: diagnostic, status and operation result are distinct; the fourteen error principles (errors belong to layers; rejected operations are not necessarily errors; failures keep causal provenance and propagate along dependencies; the core describes what happened, policy decides what to do). | 09.8 sections 37, 40 | Adopted |
| R-55 | Extension architecture and the sixteen extension invariants (extensions add vocabulary, never redefine core semantics; DSL-defined and native extensions share one semantic boundary; user extensions are first-class; first-party libraries use the same mechanism; new core primitives need justification against the full case space). | 09.9 sections 51, 53 | Adopted. Consistent with open authorship (D-002). |

## 7. Refinements from the example programs (09.10)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-56 | Process is an umbrella with explicit execution modes: continuous, discrete, algorithmic, stochastic. The IR carries the mode. | 09.10c Part A section 1, SEM-03, RUN-04 | Adopted |
| R-57 | Evolution distinguishes definition from contribution. Several behaviors may contribute to one target only through explicitly defined combination semantics (`contribute acceleration to ball.velocity`). | 09.10c Part A section 2, RUN-01, SEM-04 | Adopted, amended by D-005 (a quantity type may supply the default combination, such as sum for derivatives; targets may override). |
| R-58 | Binding is the deep abstraction (name, type, value, role, ownership, mutability, metadata); parameter, variable, constant and derived value are roles; object properties keep distinct semantics from standalone state bindings. | 09.10c Part A section 3, SEM-02 | Adopted |
| R-59 | Relations are distinct from derived geometry; a line on screen is not a semantic relation. | 09.10c Part A section 4, SEM-01 | Adopted |
| R-60 | An event occurs when its guard becomes true; locating that instant is a runtime concern. | 09.10c Part A section 5, RUN-02 | Adopted, amended by D-004 (crossing direction, same-instant cascades, Zeno policy). |
| R-61 | Stochastic concepts: Distribution, RandomSource, StochasticProcess, RandomEvent; decay laws are library-defined stochastic processes. | 09.10c Part A section 6, RUN-03 | Adopted. D-013 (accepted) adds event-time sampling. |
| R-62 | Requirements RUN-01 to RUN-06 and SEM-01 to SEM-08. | 09.10c sections 42-43 | Adopted as core-spec requirements. |

## 8. Final kernel and validation (09.11)

| ID | Resolution | Source | Status |
|---|---|---|---|
| R-63 | Property, parameter, variable, constant, derived value, predicate and rule are roles or convenience constructs, not kernel primitives. | 09.11 sections B-F | Adopted |
| R-64 | Equation stays a first-class language construct even if its lowest-level form reduces to expressions and relations. | 09.11 section Q | Adopted. Consistent with D-007. |
| R-65 | Field reduced to a library specialization of Function. | 09.11 section G | Superseded by D-010 (accepted). |
| R-66 | The 21-concept semantic kernel as a single list. | 09.11.7.1 | Adopted, amended by D-011 (accepted; same concepts organized into model kernel, runtime contract and presentation kernel). |
| R-67 | The critical boundaries list (model is not simulation; representation is not view; type validity is not state validity; randomness is not distribution; and so on). | 09.11.8 | Adopted |
| R-68 | Validation verdicts (PASS) for 09.10 and 09.11. | 09.10, 09.11 | Superseded by D-012 (accepted; validation by executable reference programs). |
| R-69 | Example syntax used in 09.3 to 09.11 (`:=`, `set`, `d(x)/dt`, `x'`, `when ...`). | 09.3 to 09.11 | Direction only. None of it is adopted syntax; see D-006. The early sketch syntax allowed by D-006 may borrow from it, without standing. |
| R-70 | "Validation verdict: the current DSL design survives the complete validation suite." | 09.11.10 | Superseded by the design audit (F-01) and D-012 (accepted). |

## Maintenance

- When the core semantics spec restates a resolution, it cites the `R-##` ID, and this record gains a pointer to the spec section.
- If a later reading of the exploration record finds a resolution missing here, append it with the next free ID.
