# Design Audit: Documents 01 to 09.11

- **Date:** 2026-09-29
- **Scope:** `docs/design/01` through `docs/design/09.11`, the full design series produced before the name Prismal was chosen.
- **Purpose:** Establish which parts of the series are sound, which are unsupported or wrong, and what must be reconsidered before committing to a DSL.
- **Status of the audited documents:** Exploration record. Not normative. See `docs/decisions/divergence-register.md` for the positions that replace or amend them.

## 1. Summary verdict

The series is a strong **problem-space exploration** and a weak **design**.

It succeeds at mapping the space (the case catalogue, the capability matrix) and at identifying the architectural separations a system like this needs: model vs representation, semantic vs computational vs presentation state, simulation vs presentation time, observation vs intervention. These separations are correct, and they match what mature systems in this area converged on.

It does not succeed at producing a DSL. From 09.1 onward the documents enumerate options, defer the choice ("we don't yet choose the exact operators"), and later stages report those unmade choices as validated. 09.11 declares that the design "survives the complete validation suite" while listing 35 open questions, including "What exactly is a Binding?", the concept the reduced kernel is built on.

**Recommendation:** Do not commit to the DSL as described in 09.x. Keep 01 to 08 as the conceptual foundation, with the amendments below. Restart the DSL work from a short normative core-semantics spec, validated by executable reference programs rather than by assertion.

## 2. How the audit was done

- Full reading of the load-bearing documents: 05c (Refined Core Model), 06 (Runtime Semantics), 09.11 (DSL Validation).
- Targeted reading of 09.4 (the meaning of `=`), 09.10a/b/c (example programs), and the section structure of every other document.
- Cross-document consistency checks by search: operators, event guards, notation for evolution, stochastic formulas, units.
- Prior-art check: searched all documents for the established systems and theory this design overlaps with.
- The original conversation was checked to recover the initial product intent.

## 3. What holds up

These positions are sound and should be carried forward largely as written:

| Position | Where | Note |
|---|---|---|
| Model is not scene, simulation is not animation, representation is not what it represents | 05c section 86 | Core architectural invariants. |
| Three clocks: simulation, presentation, wall-clock | 06 section 3 | Standard in serious simulation tools; essential for video export. |
| Semantic state is authoritative; computational and presentation state are derived | 06 sections 4-5 | Enables multiple renderers and replay. |
| Read, compute, validate, commit (no partial commits) | 06 sections 6, 28-29 | Prevents order-dependent bugs; basis for undo and branching. |
| Equation is not solver; solver choice is execution configuration | 05c section 70, 06 section 12 | Correct, with the caveat in F-07. |
| Observation vs intervention | 05c section 51 | Important for experiment semantics. |
| Interaction targets semantic bindings, not pixels | 08, 09.11 section 7 | Correct and distinctive. |
| Writable projections must be declared; non-invertible projection is not an error | 07, 09.8 section 16 | Correct. |
| Error categories: authoring, semantic, constraint, computational, runtime, presentation | 06 section 56, 09.8 | Useful, if more categories than needed early. |
| Domain concepts (Projectile, Atom, Graph) live in libraries, not the core | 05c sections 78-80, 09.9 | Correct direction. |
| The case catalogue itself | 01 | Valuable long-term asset and test source. |

## 4. Findings

Severity: **Critical** blocks committing to a DSL. **Major** must be resolved in the core spec. **Minor** should be corrected but does not change direction.

### F-01 (Critical): Validation is asserted, not demonstrated

- **Where:** 09.10a/b/c and 09.11 (about 100 occurrences of "PASS", 79 in 09.11 alone).
- **Problem:** No example program was executed, simulated by hand with expected results, or checked against a defined semantics. Verdicts are judgments that a construct "maps" to a concept. Every hard case is marked `PASS*` with the deferral note "runtime semantics still need formal specification". The final verdict contradicts the open-question list in the same document.
- **Evidence of consequence:** see F-02, where an example marked "PASS" and "Excellent" has a real defect.
- **Correction:** Validation must use reference programs with known expected behavior (analytic results, conserved quantities, known algorithm traces). See section 6.

### F-02 (Critical): Hybrid dynamics are not grounded in established semantics

- **Where:** 06 sections 16-23, 09.10b (Bouncing Ball), 09.10c section 5, 09.11 sections 5 and 09.11.5.
- **Problem:** Mixing continuous evolution with discrete events is the hardest part of this system, and it is a well-studied problem (hybrid automata, zero-crossing detection in Modelica and Simulink, superdense time in Ptolemy). The documents describe it informally and miss its known pitfalls:
  - **Level vs edge triggering is never defined.** 09.4 writes `when x crosses 0` (edge); 09.10a, 09.10b, 09.10c and 09.11 write `when ball.position.y <= 0` (level).
  - **The Bouncing Ball example is defective.** The guard is `y <= 0` and the reset sets `y = 0`, so the guard is still true immediately after the event. Under level semantics the event retriggers indefinitely. The document marks it "PASS".
  - **Zeno behavior is not mentioned.** A bouncing ball with restitution below 1 has infinitely many bounces in finite time. Every hybrid simulator needs a policy for this.
  - **Event cascades at one instant are not modeled.** An event whose reset triggers another event at the same simulation time needs superdense time or an equivalent iteration rule. 06 section 17 covers only events that are simultaneous by coincidence.
  - **Crossing direction is not expressible** (rising vs falling).
- **Correction:** Adopt a hybrid-automaton formulation as the dynamics core: flows (derivatives), guards with explicit crossing direction, resets, superdense time for same-instant cascades, and a declared Zeno policy. See D-004.

### F-03 (Critical): Scope drifted from the product that motivated the project

- **Where:** whole series; compare with the original request (concept to interactive 2D scene, optionally exported as video, like Manim with a Rust physics engine, for math, physics and chemistry).
- **Problem:** The target became a universal scientific modeling language: GPU execution, microcontroller and sensor input, branching experiments, multi-solver co-simulation, PDE fields, biology, algorithms, education layer. No first version is defined. Meanwhile the capability that makes Manim valuable (choreographed explanation: scenes, beats, camera direction, narration timing) was reduced to a "pure animation" bypass mode (06 section 60).
- **Consequence:** A design with no scope cut cannot be validated or built incrementally, and the product's distinguishing feature is under-designed.
- **Correction:** Keep the catalogue as the long-term horizon. Define a v1 slice, and treat explanatory narrative as a peer of simulation, not a side mode. See D-001 and D-009. Requires an owner decision.

### F-04 (Critical): No engagement with prior art

- **Where:** whole series. None of the following appear in any document: Modelica, ModelingToolkit (Julia), Simulink, FMI, hybrid automata, DEVS, superdense time, Gillespie/SSA, SBML, NetLogo, GeoGebra, Desmos, Elm/FRP. Manim is mentioned only as a contrast.
- **Problem:** The series re-derives from scratch results these systems settled long ago, and misses their documented failure modes (F-02, F-06, F-07, F-08).

| Prior art | What it settles for Prismal |
|---|---|
| Modelica, ModelingToolkit | Acausal equations, algebraic loops, index reduction, event handling, connector semantics for summed contributions. |
| Hybrid automata, Ptolemy (superdense time) | Formal semantics for flows, guards, resets, simultaneous and cascading events. |
| Gillespie SSA, next-reaction method, SBML | Exact stochastic event timing for chemistry and decay. |
| NetLogo | Agent/population models and their authoring ergonomics. |
| GeoGebra, Desmos | Interactive math and constructions; dependency-driven redraw; what educators can author. |
| Manim, Motion Canvas | Explanatory timelines, scene choreography, video output. |
| Elm, FRP, incremental computation | Reactive projection and view update semantics. |
| Rust ecosystem (`uom`, Rapier, Vello, Bevy ECS) | Units, physics, rendering, and entity storage that need not be reinvented. |

- **Correction:** Every core-spec section should cite the prior art it adopts or departs from. See D-003.

### F-05 (Major): The DSL syntax was never chosen, yet was reported validated

- **Where:** 09.3, 09.4 section 17 ("We don't yet choose the exact operators"), 09.10, 09.11.
- **Inconsistencies found:**
  - 09.11 section 2 defines `:=` as "definition/derivation", then uses `:=` for mutation inside events (`ball.velocity.y := -ball.velocity.y`, `state := "boiling"`, `velocity := -velocity`).
  - 09.10b uses `set ... =` for the same mutation.
  - Four notations for evolution: `d(ball.position) / dt = ...` (09.10b), `dx/dt = v` (09.11), `temperature' = ...` (09.11 section 2), and `acceleration = (0, -g)` inside a `process` block (09.11 section 5), which by 09.11's own table would be initialization.
- **Correction:** Syntax is chosen after core semantics, by writing the reference programs in two or three candidate syntaxes and comparing. See D-006.

### F-06 (Major): Multiple contributions to one state variable (RUN-01) left open, though solutions are known

- **Where:** 06 section 10, 09.7 sections 14-15, 09.10b (wind + gravity), 09.11 section 7.
- **Problem:** Declared as "one of the most important findings", then left unresolved. The difficulty is created by letting each process write a full equation `d(v)/dt = X`. Known solutions: Modelica flow variables sum at connectors; physics engines accumulate forces; ECS systems use accumulator components.
- **Correction:** A derivative is defined as the combination of named contributions, with the combining operation declared by the target's type (default: sum for derivatives). Direct replacement of state happens only in event resets and interventions, which are exclusive per instant. See D-005.

### F-07 (Major): Acausal equations are implied without acknowledging their cost

- **Where:** 05c section 11, 09.4 section 18, 09.11 section 5 ("`F = m * a` does not automatically mean `a := F / m`").
- **Problem:** Treating equations as relationships the runtime can solve in any direction is the Modelica model. It requires symbolic manipulation, causalization, algebraic-loop solving and index reduction for DAEs. It is a major subsystem, not a detail. The documents neither commit to it nor rule it out.
- **Correction:** v1 is causal. Evolution is written as explicit derivatives and assignments. Equations are first-class values for display and for constraints checked (not solved) at runtime. Acausal solving is a later, explicit extension. See D-007. Requires an owner decision.

### F-08 (Major): Algebraic cycles are claimed to be a runtime judgment

- **Where:** 05c section 48, 06 section 11, 09.11 section 5 ("The runtime determines whether the cycle is algebraic, discrete, continuous, delayed, unstable, impossible to solve").
- **Problem:** Solvability cannot be decided at runtime in general. The standard distinction is structural and static: a cycle that passes through state (an integrator, a delay, a discrete step) is well-defined; a cycle among instantaneous derived values is an algebraic loop and must be rejected or handed to an explicitly declared solver. The particle-field example in 05c and 06 is a cycle through state and is not an algebraic loop, so the documents conflate the two cases.
- **Correction:** The dependency analysis classifies cycles statically. Instantaneous cycles are compile-time errors unless a solver is declared. See D-008.

### F-09 (Major): Reducing Field to Function loses field state

- **Where:** 09.11 section G ("Field can be a library-level specialization of Function").
- **Problem:** Two different things share the name field. An analytic field (`E(x, y) = kq / r²`) is a function. An evolving field (temperature under diffusion) is state over a domain that must be discretized, and the discretization (grid, resolution, boundary treatment) changes results. 05c section 29 recognized this; 09.11 dropped it.
- **Correction:** Keep both: analytic fields are functions; field state is a core state form whose discretization is part of the declared execution configuration and is visible to the author.

### F-10 (Major): Language form, host and targets are undecided

- **Where:** absent from all documents.
- **Unanswered:** Standalone language with its own parser, type checker, diagnostics and editor support, or an embedded DSL in Rust, or a data format driven by a library? Who writes Prismal: developers, educators, content creators? Which targets: desktop, web (WASM), video file? How does it relate to Mava Studio (the Tauri + Vue authoring tool)?
- **Why it matters:** A standalone language carries large fixed costs (grammar, error messages, language server, package management) before the first simulation runs. The answer changes what "DSL design" even means.
- **Correction:** Decide explicitly. See D-002. Requires an owner decision.

### F-11 (Major): The "minimal" kernel is not minimal and mixes layers

- **Where:** 09.11.7.1 (21 kernel concepts).
- **Problem:** The kernel combines modeling (Object, Relation, State, Process, Event), presentation (Projection, Representation, View) and interaction (Interaction) in one list, which contradicts the layer separation the series argues for. Several pairs remain unresolved (Process vs Event vs Operation; Data vs Collection).
- **Correction:** Three small kernels with narrow interfaces: model kernel, runtime contract, presentation kernel. Each can then be validated on its own.

### F-12 (Minor): Stochastic formulas are inconsistent and step-bound

- **Where:** 06 section 19 (`P(decay during dt) = 1 - e^(-λdt)`, per nucleus, correct), 09.11 section 3 (`P(decay during dt) = λN dt`, called a probability but actually the expected count for the population; can exceed 1).
- **Problem:** Both treat randomness as sampled per time step. Exact methods sample event times directly (exponential waiting times, Gillespie), which is more accurate, step-size independent, and fits event-driven execution better.
- **Correction:** Support event-time sampling as a first-class stochastic mode alongside per-step sampling.

### F-13 (Minor): Units and space gaps

- **Where:** 09.10b (Bouncing Ball), 09.11 section 6.
- **Problems:**
  - Affine quantities are not addressed. Temperature in °C and time instants behave like points (difference is meaningful, sum is not), the same distinction the documents draw for Point vs Vector. `100 °C` appears in examples without this.
  - `(0, -g)` mixes a dimensionless literal with an acceleration; the literal-zero policy is undefined.
  - `interface Positioned { position : Point<2> }` fixes 2D, contradicting 05c section 16 ("The core should not assume Euclidean 2D space").
- **Correction:** Handle affine quantities the same way as points; define a policy for dimensionless literals; make dimension a type parameter.

### F-14 (Minor): Determinism claims need platform scope

- **Where:** 06 sections 20, 67.
- **Problem:** "Strong deterministic" (bit-identical) results across machines are not achievable with floating point, different CPUs, fused multiply-add, parallel reductions, or WASM vs native without deliberate engineering.
- **Correction:** State the guarantee per scope: bit-identical on the same build and platform; tolerance-equivalent across platforms unless a deterministic math mode is chosen.

### F-15 (Minor): Document hygiene

- **Numbering drift:** 04 plans "09 Interaction API, 10 Implementation"; 05b and 06 name the next step "07 DSL Design"; the final chain places DSL Design at 09. Cross-references inside documents are therefore unreliable.
- **Conversational residue:** progress trees, "we are now here" markers, and closing paragraphs pointing to the next step are embedded in the documents.
- **Broken code fences:** 466 fences had content on the opening line, which hides that line when rendered. Fixed during this audit.
- **Volume:** roughly 750,000 characters, heavy repetition, many 45+ section documents of enumeration. Hard to use as a reference.
- **Correction:** Freeze 01 to 09.11 as the exploration record. Write new normative documents short and decision-oriented.

## 5. Items worth rethinking before any DSL commitment

These are not errors but directions to reconsider deliberately:

1. **Language-first vs kernel-first.** Building the runtime kernel and reference programs against a Rust API first, then designing syntax around proven semantics, removes the main risk the series ran into (designing syntax for undefined semantics).
2. **Explanatory narrative as a first-class layer.** For the educational use case, the author's story (what appears, when, what the narration says, where attention goes) matters as much as the model. Model-driven views and author-driven timelines need a common, precise synchronization model.
3. **Authoring audience.** If educators and content creators are the primary authors, a visual editor (possibly Mava Studio) over a textual format may matter more than surface syntax elegance.
4. **Reuse over rebuild.** Units (`uom`), rigid-body physics (Rapier), 2D rendering (Vello), and video encoding exist in Rust. The core value of Prismal is the semantic layer and the authoring model, not reimplementing these.

## 6. Recommended path

1. **Freeze** 01 to 09.11 as the exploration record (done in this audit; marked non-normative).
2. **Owner decisions:** D-001 (v1 scope), D-002 (language form and host), D-007 (causal v1).
3. **Core semantics spec v0:** short and normative, grounded in cited prior art. Covers bindings, state, flows and contributions, guards and resets, event instants, discrete steps, stochastic modes, observation, projection, interaction operations.
4. **Reference programs as acceptance tests,** each with an expected result:
   - Projectile with and without drag (analytic range for the no-drag case).
   - Bouncing ball (bounce times form a geometric series; Zeno policy exercised).
   - Pendulum (small-angle period; energy drift bound per solver).
   - Radioactive decay (half-life statistics; exact vs per-step sampling agree within tolerance).
   - A + B to C reaction, stochastic (Gillespie trajectory statistics vs rate equation).
   - BFS on a fixed graph (exact visit order).
   - Game of Life glider (exact state after N generations).
   - Function plot with a draggable parameter (projection and writable binding).
5. **Syntax study:** write the reference programs in two or three candidate syntaxes; choose based on the result.
6. **Prototype** the kernel in Rust and run the reference programs before freezing syntax.

## Addendum: owner responses

This section records owner responses to findings. The findings above are left as written.

- **2026-09-29, F-03 (scope):** Partly corrected. The universal scientific scope is intended, not drift. Its purpose is education through three use modes: self-study, deep study, and syllabus-based content creation. The valid part of F-03 is sequencing (no first slice defined) and the under-weighting of explanatory narrative. Recorded as D-017 (accepted) and D-001 (reframed).
- **2026-09-29, traceability review:** A section-by-section reading of the resolutions in 01 to 09.10 (recorded in `docs/decisions/carried-forward.md`, R-01 to R-70) corrects three findings. The findings above are left as written; these corrections take precedence.
  - **F-01:** The statement that Binding was undefined overstates the gap. 09.6 sections 69-71 and 09.10c Part A section 3 give a working definition (the binding chain and binding roles, R-52 and R-58). 09.11 listed it as open because no formal definition exists; the finding that 09.11's validation verdicts are unsupported stands.
  - **F-02:** 09.10c Part A section 5 states edge semantics in prose ("event occurs when guard becomes true", R-60), which avoids the retriggering defect if applied. The findings that remain: example syntax cannot express crossing direction, same-instant cascades and Zeno behavior are not addressed, and the example was passed without executing it.
  - **F-06:** Incorrect as written ("left unresolved"). 09.10c Part A section 2 resolved multiple contributions with an explicit contribution mechanism and declared combination semantics (R-57). 09.11 relisted it as an open question. The remaining open point is narrower: whether a type may supply a default combination (D-005).
  - **General:** Section 3 of this audit ("What holds up") understated the amount of sound analysis. Of 70 recorded resolutions, 58 are adopted as written and 7 adopted with amendments.
