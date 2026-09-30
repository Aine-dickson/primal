# Divergence Register

This register records every point where Prismal departs from, amends, or has not yet settled a position in the exploration record (`docs/design/01` to `09.11`).

The exploration documents are frozen. They are not edited to reflect later decisions; this register is the authoritative record of what changed and why.

## How to use this register

- Each entry has a stable ID (`D-###`). IDs are never reused or renumbered.
- **Status values:**
  - `Open`: a question the owner must decide. No default is assumed.
  - `Proposed`: a recommended position awaiting owner review.
  - `Accepted`: the current position of the project. Supersedes the exploration record.
  - `Rejected`: considered and declined; the original position (or another entry) stands.
  - `Superseded`: replaced by a later entry (named in the entry).
- When a status changes, update the entry in place and append a line to its **History**. Do not delete entries.
- Every entry names its source (`Original position`) and its evidence (`Audit finding`), so the reasoning can be reconstructed later without the conversation that produced it.

## Summary

| ID | Topic | Status |
|---|---|---|
| D-017 | Product purpose and long-term scope | Accepted |
| D-001 | v1 scope: first slice is mechanics + math | Accepted |
| D-002 | Language form: kernel-first; authorship open, programming-literate authors expected first | Accepted |
| D-003 | Prior art as a design input | Accepted |
| D-004 | Hybrid dynamics semantics | Accepted |
| D-005 | Multiple contributions: type-supplied default combination | Accepted |
| D-006 | Syntax chosen after semantics, by comparison; early sketch allowed | Accepted |
| D-007 | Causal (one-way) equations in v1 | Accepted |
| D-008 | Algebraic cycles detected statically | Accepted |
| D-009 | Explanatory narrative as a first-class layer | Accepted |
| D-010 | Field as function and field as state | Accepted |
| D-011 | Three kernels instead of one | Accepted |
| D-012 | Validation by executable reference programs | Accepted |
| D-013 | Stochastic event-time sampling | Accepted |
| D-014 | Affine quantities, literals and dimension | Accepted |
| D-015 | Scoped determinism guarantees | Accepted |
| D-016 | Exploration record frozen, normative docs separate | Accepted |
| D-018 | Output targets: web player first; output form follows content and author intent | Accepted |
| D-019 | Relationship to Mava Studio | Accepted |
| D-020 | Constraints checked, not enforced, in v1 | Accepted |
| D-021 | Plane angle is dimensionless | Accepted |
| D-022 | Spaces have frames; conversions are explicit | Accepted |
| D-023 | Intervenable bindings are declared by the model | Accepted |
| D-024 | No event priorities; ordering by cascades | Accepted |
| D-025 | Learner exploration during a narrated lesson | Accepted |
| D-026 | Accessibility baseline in v1 | Accepted |
| D-027 | Requestable events | Accepted |
| D-028 | Syntax direction: candidate A amended with author conveniences from B and C | Accepted |
| D-029 | Function values that read model bindings | Accepted |
| D-030 | Literal `0` adopts zero vectors | Accepted |
| D-031 | Model variants in reference-program cases | Accepted |
| D-032 | Tuple literals as vectors: typing by expected type | Accepted |
| D-033 | Order of run-directing actions within a beat | Accepted |
| D-034 | Formula representation for expressions | Accepted |
| D-035 | Keywords and spelling of the working syntax | Accepted |
| D-036 | Comments and identities across text and visual editing | Accepted |
| D-037 | IR serialization: versioned JSON | Accepted |
| D-038 | Self-retriggering decided through flows, specialized on the handler's discrete values | Accepted |
| D-039 | Named mathematical constants kept by name in the IR | Accepted |
| D-040 | Reserved words and contextual keywords | Accepted |
| D-041 | `on(E)` follows an occurrence of `E` as well as an emission | Accepted |

---

## D-017: Product purpose and long-term scope

- **Status:** Accepted
- **Original position:** The exploration record generalized the project into a universal scientific modeling language (01 to 09.11).
- **Audit finding:** F-03 described this as scope drift away from education. The owner corrected that reading (see History).
- **Accepted position:** Prismal is a universal scientific modeling language **whose purpose is education**. The broad scientific scope is intended. Every language output serves one of three use modes:
  1. **Self-study:** a learner explores a concept on their own.
  2. **Deep study:** intensive investigation of a concept (experiments, parameters, multiple representations, comparison).
  3. **Syllabus-based content creation:** authors produce scientifically grounded educational content aligned to a curriculum (including narrated and exported material).
- **Consequences:**
  - The case catalogue (01) stands as the scope horizon; it is not cut back.
  - The scope question moves to sequencing: which part of the horizon comes first (D-001).
  - The three use modes become design drivers. A construct that serves none of them needs a justification.
  - Use mode 3 supports D-009 (explanatory narrative as a first-class layer).
- **History:**
  - 2026-09-29 accepted by the owner in response to audit finding F-03: "somewhere chatgpt got my intent, more so on universal scientific language; however this doesn't go away from the basis of education."

## D-001: v1 scope

- **Status:** Accepted
- **Original position:** No scope limit. "Don't limit ourselves before limits show up" (01, 02); final validation matrix covers math, physics, chemistry, biology, algorithms, GPU, video, sensors (09.11.7.4).
- **Audit finding:** F-03, as corrected by D-017.
- **Question:** The long-term scope is universal (D-017). Which slice of it is built and validated first? This is an ordering question, not a limit on the language's design.
- **Options considered:** mechanics + math; math only (no time evolution); chemistry kinetics (stochastic, many-body); one real syllabus unit.
- **Accepted position:** The first slice is **2D mechanics plus supporting mathematics**:
  - Kinematics and dynamics: projectile, pendulum, bouncing ball, spring-mass systems.
  - Mathematics: functions and their graphs, vectors, parameters.
  - Slice shape (as presented with the decision): thin but end to end through every layer: model, simulation, multiple synchronized views, interaction, and a narrated explanation timeline, so all three use modes of D-017 are exercised. The output medium for the slice is the web player; see D-018, which amended this line (video export was removed from the first slice).
- **Reasons:**
  - Forces the hardest core rules early: continuous evolution, events (collisions, bounces), and multiple contributions to one variable (D-004, D-005).
  - Expected results are known analytically (range, period, bounce times, energy), which makes executable validation straightforward (D-012).
  - Chemistry kinetics and stochastic processes come in a later slice; the core must not preclude them.
- **Consequences:** The first reference programs are drawn from this slice: projectile (with and without drag), pendulum, bouncing ball, spring-mass, function plot with a draggable parameter, vector addition.
- **History:**
  - 2026-09-29 opened by the design audit.
  - 2026-09-29 reframed from "scope cut" to "first slice of a universal scope" following D-017.
  - 2026-09-29 accepted by the owner: mechanics + math.
  - 2026-09-29 slice shape amended by D-018: web player only; video export later.

## D-002: Language form and primary author

- **Status:** Accepted
- **Original position:** Not addressed. The series assumes a standalone DSL implicitly.
- **Audit finding:** F-10.
- **Options considered:**
  1. Kernel-first: Rust library and API first, textual syntax designed later over proven semantics.
  2. Standalone language as the first deliverable (parser, type checker, diagnostics, editor support).
  3. Visual editor plus data format (possibly Mava Studio), with a text language later or never.
- **Accepted position:**
  - **Form: kernel-first.** The semantic kernel and runtime are built as a Rust library and API. The reference programs (D-012) are written against that API. The authoring surface (textual syntax, visual editor, or both) is designed afterward, over semantics already proven by execution.
  - **Authorship is open.** Anyone may author, including people and content the language makers did not target. No author profile is built into the language.
  - **Expected first authors: educators and content creators, most likely with programming knowledge.** The textual surface may assume programming literacy.
  - **Non-programmers are reached through Mava Studio** (see D-019), not by simplifying the language itself.
- **Reasons:**
  - Avoids repeating the exploration record's main failure: choosing syntax before semantics were defined (F-05).
  - Defers the fixed costs of a language (grammar, diagnostics, editor tooling) until the semantics justify them.
- **Consequences:**
  - The Rust API is an internal and developer-facing layer, not the main authoring surface. It may be verbose.
  - The expected main authoring surface is a textual language for programming-literate educators and creators. The syntax study (D-006) is evaluated against that audience: familiar to programmers, but reading close to the science and mathematics being taught. A visual editor is optional, not required.
  - Because authorship is open, the language must stay general: domain libraries, representations and interactions are open to any author, and nothing assumes a curriculum, subject or audience.
  - Output targets were split out as D-018.
- **History:**
  - 2026-09-29 opened by the design audit.
  - 2026-09-29 accepted by the owner: kernel-first; primary author educators and creators.
  - 2026-09-29 amended by the owner: "chances are high it will be people with programming knowledge being authors". Consequences updated accordingly.
  - 2026-09-29 amended by the owner: "Author can even be some person coming up with their own content that we as the makers of the language didn't target, so who an author is can't be entirely predictable." Authorship recorded as open; non-programmers reached via Mava Studio (D-019).

## D-003: Prior art as a design input

- **Status:** Accepted
- **Original position:** Design derived from first principles; no prior systems cited.
- **Audit finding:** F-04.
- **Accepted position:** Each section of the core semantics spec cites the prior art it adopts or departs from (Modelica, hybrid automata and superdense time, Gillespie SSA, NetLogo, GeoGebra/Desmos, Manim/Motion Canvas, FRP), with a sentence explaining the choice. Sections also cite the carried-forward resolutions (`R-##`) they restate, so both external prior art and the project's own earlier analysis are traceable.
- **Reason:** These systems document the failure modes the series missed (F-02, F-06, F-07, F-08).
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner ("Go to next natural item" in reply to the proposal to accept D-003 and move to D-004). Citation of `R-##` IDs added, following the owner's request that the earlier analysis be visibly reflected.

## D-004: Hybrid dynamics semantics

- **Status:** Accepted
- **Original position:** Informal hybrid loop (06 sections 22-23); guards written as level conditions (`y <= 0`); simultaneous events handled by ordering policy (06 section 17).
- **Audit finding:** F-02, as corrected in the audit addendum.
- **Builds on:** R-28 (event times are located), R-29 (deterministic same-time policy), R-57 (contributions), R-60 (event occurs when the guard becomes true).
- **Accepted position:** Dynamics follow a hybrid-automaton formulation:
  - **Flows:** continuous variables evolve by declared derivatives.
  - **Guards:** zero-crossing conditions with explicit direction (rising, falling, either). Level conditions are allowed only for discrete-time rules, never as continuous event guards.
  - **Resets:** event handlers assign new values; they are the only place continuous state is replaced.
  - **Event instants:** superdense time `(t, n)`; an event cascade at one instant advances `n`, with a declared maximum iteration count.
  - **Zeno policy:** required for any model where events can accumulate (detect and stop, or apply a declared threshold rule such as resting contact).
- **Reason:** Standard, well-understood semantics (hybrid automata; Modelica, Simulink and Ptolemy practice) with known implementation strategies. It closes the gaps left by R-60: crossing direction, same-instant cascades, and Zeno behavior.
- **Consequence:** The bouncing ball becomes the first reference program (D-012): its bounce times form a geometric series, so the result is exactly checkable, and it exercises the Zeno policy.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner.

## D-005: Multiple contributions to one variable

- **Status:** Accepted
- **Original position:** RUN-01 (09.10b) was resolved in 09.10c Part A section 2 (R-57): evolution distinguishes definition from contribution, and multiple contributions require explicitly defined combination semantics; 09.10c deliberately declined to make sum a default. 09.11 section 7 relisted the question as open.
- **Audit finding:** F-06 (corrected in the audit addendum).
- **Point of difference with R-57:** only whether a target's type may supply a default combination. Everything else in R-57 is adopted.
- **Options considered:** always explicit (R-57 as written); type-supplied default; per-model declaration.
- **Accepted position:**
  - Everything in R-57 stands: behaviors contribute to an evolution target; they do not each define it; combination semantics must be defined.
  - **Combination may be supplied by the target's type.** A quantity type can declare its natural combination; rates of change (derivatives such as acceleration, velocity, reaction rate) declare sum. A target can override its type's combination. A type without a declared combination requires every target of that type to state one explicitly.
  - Sum is therefore never assumed by the core; it is declared at the type level, once.
  - **Direct replacement** of a value happens only in event resets (D-004) and interventions. Two replacements of the same value at the same instant are a reported conflict, never a silent winner (R-26).
- **Reason:** Matches Modelica force balances and physics-engine force accumulation, removes repetitive declarations in the common case, and keeps R-57's safeguard that no combination is assumed by the core.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner: type-supplied default.

## D-006: Syntax chosen after semantics, by comparison

- **Status:** Accepted
- **Original position:** Syntax options enumerated (09.3), operators deferred (09.4 section 17), examples written in inconsistent notations and reported validated (09.10, 09.11).
- **Audit finding:** F-05.
- **Builds on:** R-03 (no syntax before meaning), R-44 (hybrid surface style, as direction), R-46 (distinctions the syntax must preserve), R-69 (09.x example syntax has no standing).
- **Accepted position:**
  1. **No syntax is adopted until the core semantics spec exists.**
  2. **A provisional sketch syntax is allowed early**, only so that spec examples and reference programs are readable. It is explicitly non-binding, is labeled as sketch wherever it appears, and gets no preference in the comparison.
  3. **Selection by comparison:** two or three candidate syntaxes in R-44's hybrid style are compared by writing every reference program (D-012) in each.
  4. **Criteria:** readability for programming-literate authors (D-002), closeness to the science being taught, clear visibility of the R-46 distinctions, and clean round-tripping with the serializable model representation edited by Mava Studio (D-019).
- **Reason:** Syntax chosen before semantics was the main source of inconsistency in the series. The sketch allowance keeps the spec readable without letting an early notation harden by habit.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner, with an early non-binding sketch syntax allowed.

## D-007: Causal (one-way) equations in v1

- **Status:** Accepted
- **Original position:** Equations are relationships whose role is set by context; `F = m * a` does not imply a direction (05c section 11, 09.4 section 18, 09.11 section 5). Implies acausal solving without stating it.
- **Audit finding:** F-07.
- **Options considered:** causal in v1 with acausal later; acausal from the start; defer the decision.
- **Accepted position:**
  - **v1 is causal.** Evolution is written as explicit derivatives and assignments (the author states which quantity is computed from which).
  - **Equations remain first-class values.** An equation such as `F = m a` can be displayed, labeled, bound to live model values, and checked at runtime (residual reported), but it does not drive the simulation.
  - **Acausal solving is a later, explicit extension.** The core must not preclude it: equation objects keep their symbolic form so a future solver can consume them.
- **Reasons:**
  - Acausal solving (causalization, algebraic-loop solving, DAE index reduction) is a major subsystem; Modelica took years to mature it.
  - The first slice (D-001, mechanics + math) needs only causal evolution.
  - The educational value of equations (display, linking symbols to live values, checking) is preserved.
- **History:**
  - 2026-09-29 opened by the design audit.
  - 2026-09-29 accepted by the owner: one-way in v1.

## D-008: Algebraic cycles detected statically

- **Status:** Accepted
- **Original position:** "Cycles are not automatically errors"; the runtime determines what kind of cycle it is (05c section 48, 09.11 section 5).
- **Audit finding:** F-08.
- **Builds on:** R-35 (cycles are not automatically errors), R-53 (composition invariant 7), D-007 (causal formulas).
- **Accepted position:** Dependency analysis runs before execution and classifies cycles. A cycle through state (integrator, delay, discrete step) is valid. A cycle among instantaneous derived values is an algebraic loop: a compile-time error unless an explicit solver is declared for it.
- **Reason:** Solvability is not decidable at runtime in general; the structural check is standard and cheap.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner.

## D-009: Explanatory narrative as a first-class layer

- **Status:** Accepted
- **Original position:** Author-driven animation is a presentation concern and a "pure animation" execution mode (05c section 63, 06 section 60).
- **Audit finding:** F-03.
- **Builds on:** R-22 (education above the core), R-23 (three clocks), R-32 (static models and pure animation), R-07 (simulation is not animation), D-017, D-018.
- **Accepted position:** The explanatory timeline (scenes, beats, reveals, camera direction, narration cues) is a peer of the model, with a precise synchronization contract to simulation time (for example: a beat can wait for a model event; a scene can pause, seek or branch the simulation). The model never depends on the timeline; the timeline observes and directs it. The same timeline drives the interactive player (where the learner may pause, intervene or branch) and later video export (linear playback).
- **Reason:** It is the capability that motivated the project (Manim-style explanation with interactive simulation) and the main difference from a general modeling tool.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner; the core spec gets a timeline section alongside dynamics.

## D-010: Field as function and field as state

- **Status:** Accepted
- **Original position:** Field reduced to a library specialization of Function (09.11 section G).
- **Audit finding:** F-09.
- **Builds on:** R-15 (fields first-class, meaning independent of storage); supersedes R-65.
- **Accepted position:** Two forms. Analytic fields are functions over a domain. Field state is a core state form over a domain, and its discretization (grid, resolution, boundary treatment) is declared execution configuration visible to the author.
- **Reason:** Discretization changes results, so it cannot be hidden as a runtime optimization.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner; formula fields are needed in the first slice, evolving fields in a later slice.

## D-011: Three kernels instead of one

- **Status:** Accepted
- **Original position:** One 21-concept semantic kernel covering model, presentation and interaction (09.11.7.1).
- **Audit finding:** F-11.
- **Builds on:** R-66 (the 21 kernel concepts, all carried forward), R-05 (layered separation), R-42 (08 section 97 layers), D-009, D-019.
- **Accepted position:** The same concepts, organized in three parts connected by narrow interfaces:
  - **Model kernel:** values, types, bindings, identity, objects, relations, collections, domains, expressions, functions, constraints, state, processes (flows), events, operations.
  - **Runtime contract:** clocks, steps, commit, event instants, randomness, snapshots, replay.
  - **Presentation kernel:** observation and data, projection, representation, view, explanation timeline (D-009), interaction.
  - A model runs without any presentation (headless testing, offline export). One model can carry several presentations. The serializable model representation (D-019) keeps model and presentation sections separate.
- **Reason:** Consistent with the layer separation the series itself argues for; each part can be validated independently.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner; the core spec is structured this way.

## D-012: Validation by executable reference programs

- **Status:** Accepted
- **Original position:** Validation by construct-to-concept mapping with PASS verdicts (09.10, 09.11).
- **Audit finding:** F-01.
- **Builds on:** Supersedes R-68 and R-70. 09.5b section 66 had called for a set of actual programs.
- **Accepted position:** A fixed set of reference programs with expected results serves as the acceptance suite for the semantics spec (expressible?), the syntax study (D-006), and the prototype (correct results?). The set grows with each slice; programs are never silently removed.
- **First-slice set (D-001):**

| Program | Checkable result |
|---|---|
| Projectile, no drag | Range and flight time match the analytic formulas. |
| Projectile with drag | Two contributions combine (D-005); matches a high-precision reference solution. |
| Bouncing ball | Bounce times form a geometric series; the Zeno policy triggers (D-004). |
| Pendulum | Small-angle period close to 2π√(L/g); energy drift within declared bounds per solver. |
| Spring-mass | Period 2π√(m/k); energy conserved within tolerance. |
| Function plot with draggable parameter | Dragging updates the model value; all views stay synchronized (R-37, R-40). |
| Vector addition | Point/vector rules enforced (R-09); invalid operations rejected with a diagnostic. |
| Narrated projectile lesson | The timeline waits on the landing event, then pauses and seeks the simulation (D-009). |
- **Reason:** Assertion-based validation passed a defective example.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner; first-slice program list adopted, to grow with later slices.

## D-013: Stochastic event-time sampling

- **Status:** Accepted
- **Original position:** Per-step probability sampling (06 section 19; 09.11 section 3 with an incorrect formula).
- **Audit finding:** F-12.
- **Builds on:** R-10 (distribution is not random source), R-30 (reproducible random streams), R-61 (stochastic concepts), D-004 (event machinery).
- **Accepted position:** Two stochastic modes: per-step sampling and exact event-time sampling (exponential waiting times, Gillespie SSA). Formulas in the exploration record are not normative.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner; specified in detail when the stochastic slice is planned.

## D-014: Affine quantities, literals and dimension

- **Status:** Accepted
- **Original position:** Units and Point/Vector distinction (05c, 09.5); examples use `°C`, unitless `0` in dimensioned vectors, and `Point<2>` in a general interface.
- **Audit finding:** F-13.
- **Builds on:** R-08 (units and dimensions), R-09 (point vs vector), R-11 (no assumed 2D).
- **Accepted position:** Affine quantities (temperature scales with an offset, time instants) follow the Point/Vector rules. A policy for dimensionless literals is defined: a literal `0` adopts the required dimension; any other bare number combined with a dimensioned quantity is an error (for example `5` written where `5 m` was meant). Spatial dimension is a type parameter in general interfaces.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner.

## D-015: Scoped determinism guarantees

- **Status:** Accepted
- **Original position:** Three determinism levels, including "strong deterministic" without platform scope (06 section 67).
- **Audit finding:** F-14.
- **Builds on:** R-34 (determinism levels), R-30 (reproducible random streams), R-25 (transactional commit).
- **Accepted position:** Bit-identical replay is guaranteed only for the same build and platform. Across platforms, results are equivalent within declared tolerances unless a deterministic math mode is selected. That mode (identical results everywhere at some speed cost, for example for graded assessment) is not built in the first slice, but the design must leave room for it.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner.

## D-016: Exploration record frozen, normative docs separate

- **Status:** Accepted
- **Original position:** Each document in the chain treated as the foundation for the next.
- **Audit finding:** F-15.
- **Builds on:** R-01 to R-70 (carried-forward record).
- **Accepted position:**
  - `docs/design/01` to `09.11` are frozen as the exploration record. Their original text is not rewritten.
  - **Correction notes are allowed** inside the frozen documents, next to the passage they correct, in exactly this form:
    `> **Correction note (YYYY-MM-DD; D-### or R-##; audit F-## if any):** what is wrong and where the current position lives.`
    Notes point to the register or carried-forward record; they do not restate decisions in full.
  - Mechanical formatting repairs (such as broken code fences) are allowed and are not content edits.
  - Continuity is recorded in the carried-forward record (R-##); changes in this register (D-###). Precedence: register, carried-forward record, exploration record.
  - New normative documents are short and decision-oriented, and cite R-## and D-### for their sources (D-003).
- **Reason:** Rewriting 750,000 characters of conversational material is costly and would destroy the historical record.
- **History:**
  - 2026-09-29 proposed by the design audit.
  - 2026-09-29 accepted by the owner with an amendment: clearly marked correction notes may be added inside the frozen documents.

## D-018: Output targets

- **Status:** Accepted
- **Original position:** Multiple renderers and video export assumed (05c, 06, 07); no target platforms chosen. Separately, the owner's early direction in the design conversation (preceding 01): "we don't have to force the output to be in one presentational form; based on the analysis, we can come up with different adaptable forms of representing the concepts". The resulting principle, one concept with several valid representations (for example radioactive decay as particles, an N(t) graph, a half-life timeline, or an interactive experiment), underlies 07.
- **Audit finding:** F-10 (split from D-002).
- **Options considered:** web player (WASM); video export; native desktop; Mava Studio as host.
- **Accepted position:**
  - **First slice target: an interactive web player** (Rust compiled to WebAssembly, running in the browser).
  - **Output is not fixed per project.** Both the presentational form (spatial scene, graph, timeline, equation, experiment panel, and so on) and the output medium (interactive player, video, still image, and others) are chosen according to the nature of the content and the author's intention. The architecture must treat every medium as one projection target among several, never as the defining output.
  - Video export, desktop, and Mava Studio integration are later additions that must fit this principle without redesign.
- **Reasons:**
  - The browser reaches learners without installation and can be embedded in learning platforms (self-study and deep study, D-017).
  - Tying the core to one medium would contradict the representation principle the owner set at the start.
- **Consequences:**
  - The presentation kernel (D-011) must separate "what representation" from "which medium" so the same scene can later render to video.
  - D-009 (narrative timeline) must be medium-independent: the same timeline drives the interactive player now and video later.
- **History:**
  - 2026-09-29 split from D-002.
  - 2026-09-29 accepted by the owner: web player (WASM) first, with the explicit instruction not to ignore the early principle that output depends on the nature of the content and the author's intention.

## D-019: Relationship to Mava Studio

- **Status:** Accepted
- **Original position:** Not addressed in the exploration record. Mava Studio (the owner's Tauri + Vue authoring tool) was mentioned only at the start of the design conversation.
- **Accepted position:**
  - Prismal is a **prerequisite for the redevelopment of Mava Studio**, which is due for a redesign.
  - The redesigned Mava Studio is expected to be the main entry point for **non-programmer authors**. Programming-literate authors can use Prismal's textual surface directly.
  - Mava Studio is built on Prismal, not the other way around: Prismal must not depend on Mava Studio.
- **Consequences:**
  - Prismal needs a **complete, serializable model representation** (the semantic IR that 09.4 section 34 sketched) that a visual editor can create, inspect and modify, not only a textual syntax.
  - Text and editor should be two views of the same model. Round-tripping between them is a design goal to evaluate in the syntax study (D-006).
  - Kernel APIs should expose what an editor needs: introspection of bindings, types, units, dependencies, diagnostics, and the representations available for a value.
  - The web player (D-018) should be embeddable in Mava Studio (Tauri renders web content).
- **History:** 2026-09-29 accepted by the owner: "Mava is going to face a redesign soon, and chances are that that's where most non programmers will be reconciled from. This initiative is one of the prerequisites of the redevelopment of Mava Studio."

## D-020: Constraints checked, not enforced, in v1

- **Status:** Accepted
- **Original position:** Constraints restrict valid states; "their enforcement strategy belongs to computation/runtime semantics" (05c section 37). Constrained direct manipulation is listed as supported (09.11.7.4, `PASS*`). No enforcement mechanism was specified.
- **Raised by:** core semantics spec v0, model kernel section 12.
- **Builds on:** R-25 (no partial commits), R-41 (proposed state, validated commit), R-50 (value restrictions are constraints), D-007 (causal v1).
- **Question:** When a constraint is violated, does the runtime move state to satisfy it (enforce), or only detect the violation (check)?
- **Options considered:**
  1. Check only: every constraint has a policy (`reject` the change, `report` a diagnostic, or `stop` the run). The runtime never changes state to satisfy it.
  2. Enforce in v1: the runtime projects state back onto the constraint (as physics-engine joints and GeoGebra's point-on-object do).
- **Accepted position:** option 1 for v1. Constrained systems are written in coordinates that satisfy the constraint by construction (a pendulum in its angle, not as a free bob held by a rod), and the constraint can be stated as a check. Enforcement is a later extension, together with acausal solving (D-007), because it needs the same solver machinery.
- **Consequences:** Constrained dragging (a point dragged along a curve) is not a model-kernel feature in v1; the presentation kernel may still map a drag onto a curve parameter. The first-slice programs do not need enforcement.
- **History:**
  - 2026-09-29 proposed in the model kernel spec v0.
  - 2026-09-29 accepted by the owner: check only.

## D-021: Plane angle is dimensionless

- **Status:** Accepted
- **Original position:** Not addressed. Units and dimensions are distinct and checked (05c sections 5-6, 09.5), with no statement on angles.
- **Raised by:** core semantics spec v0, model kernel section 3.5 (the pendulum reference program).
- **Builds on:** R-08 (units and dimensions), D-014.
- **Question:** Is plane angle its own dimension, or dimensionless as in the SI?
- **Options considered:**
  1. Dimensionless (SI 2019): `rad` is 1, `deg` is π/180, `rev` is 2π. Trigonometric functions take dimensionless arguments. A binding may carry `deg` as its display unit.
  2. Separate base dimension (as Boost.Units does): catches passing a non-angle to `sin`, but formulas such as `v = ω r` and `E = ½ I ω²` then need explicit `/rad` corrections everywhere.
- **Accepted position:** option 1. It matches the SI and the physics textbooks learners use, and keeps standard formulas unchanged. The degree/radian confusion that option 2 guards against is handled by units: `30 deg` is converted correctly wherever it is written.
- **Consequence:** angular velocity and frequency share the dimension 1/T; telling them apart is part of the deferred question of quantity kinds of equal dimension (model kernel section 21).
- **History:**
  - 2026-09-29 proposed in the model kernel spec v0.
  - 2026-09-29 accepted by the owner.

## D-022: Spaces have frames; conversions are explicit

- **Status:** Accepted
- **Original position:** Vectors are `Vector2`, `Vector3` or `Vector<Domain>` (05c section 7); Point and Vector are distinct (09.5 section 12). Coordinate transformations are listed as an open question (09.11.9, question 25).
- **Raised by:** core semantics spec v0, model kernel section 4.
- **Builds on:** R-09 (point vs vector), R-11 (no assumed 2D), D-014 (dimension as a type parameter).
- **Question:** How do points and vectors relate to coordinate systems?
- **Accepted position:**
  - A space is declared by the model with a dimension `n` and has a standard frame (origin and orthonormal axes).
  - Points and vectors belong to one space: `Point<S>`, `Vector<S, D>`. Values of different spaces never combine.
  - A space may declare further frames (for example axes along an inclined plane), each defined by a transform from another frame. Components are read in the standard frame by default (`v.x`) or in a named frame (`v.in(F).x`).
  - Moving a value between spaces needs an explicit function.
- **Reason:** a vector's meaning does not depend on the frame it is read in, but its components do. Keeping the frame explicit prevents mixing components from different axes, a common error in mechanics teaching (inclined planes, rotating frames), and follows the frame-tagged discipline of robotics libraries such as ROS tf2.
- **Consequence:** the first slice needs only one space with its standard frame; extra frames are needed from the inclined-plane and rotating-frame cases onward.
- **History:**
  - 2026-09-29 proposed in the model kernel spec v0.
  - 2026-09-29 accepted by the owner.

## D-023: Intervenable bindings are declared by the model

- **Status:** Accepted
- **Original position:** Interactions identify their target domain (R-33) and model-changing interactions name their semantic target (R-40). Which model values may be changed from outside was not specified.
- **Raised by:** core semantics spec v0, model kernel sections 6.3 and 17.
- **Builds on:** R-33, R-40, R-41, D-009 (the timeline directs the simulation), D-017 (use modes).
- **Question:** Can any model value be changed from outside the model (by a learner, a timeline or an experiment), or only values the model author allows?
- **Options considered:**
  1. Everything stored is intervenable.
  2. Parameters are intervenable by default; state bindings only when the model declares them intervenable; constants, inputs and derived values never. Allowed ranges are constraints with policy `reject`.
- **Accepted position:** option 2. The model author knows which changes keep the model meaningful (changing a ball's position mid-flight is meaningful; changing an internal counter is not). This follows FMI, which marks parameters as tunable or fixed and allows changes only at event instants.
- **Consequence:** the presentation kernel can list, for any model, exactly what a learner may manipulate; Mava Studio can show it in the editor (D-019).
- **History:**
  - 2026-09-29 proposed in the model kernel spec v0.
  - 2026-09-29 accepted by the owner.

## D-024: No event priorities; ordering by cascades

- **Status:** Accepted
- **Original position:** Same-time events are collected, ordered "by explicit semantics", evaluated and resolved (06 section 17). "Events may carry priority", which "must be semantic only when the model explicitly requires it" (06 section 18).
- **Raised by:** core semantics spec v0, runtime contract section 8.
- **Builds on:** R-25, R-26, R-29, D-004 (superdense time), D-005 (conflicts reported, never resolved by a winner).
- **Question:** When several events are due at the same instant, may the author give them priorities that decide which is handled first?
- **Options considered:**
  1. No priorities. All events due at one microstep form one transition; every handler reads the same state; overlapping writes are a conflict. An author who needs "A then B" writes A's handler to emit an event that performs B at the next microstep.
  2. Numeric priorities (as in many discrete-event simulators): higher-priority events are handled first, each seeing the previous one's result.
- **Accepted position:** option 1. Priorities make results depend on a number chosen far from the events it affects, and silently turn a conflict into a winner, which D-005 and R-26 rule out. Cascades express the same intent where the order is visible in the model, and superdense time (D-004) already provides the microsteps they need.
- **Consequence:** 06 section 18 (priority) is not carried forward in v0. Priorities can be added later if a reference program shows that cascades are too awkward.
- **History:**
  - 2026-09-29 proposed in the runtime contract spec v0.
  - 2026-09-29 accepted by the owner.

## D-025: Learner exploration during a narrated lesson

- **Status:** Accepted
- **Original position:** Not addressed. The exploration record treats guided interaction and experiments as educational layers (08 sections 80-83) without saying how learner changes interact with an authored sequence. D-009 made the timeline able to direct the simulation and noted that the learner may pause, intervene or branch.
- **Raised by:** core semantics spec v0, presentation kernel section 9.3.
- **Builds on:** D-009, D-017 (use modes), D-023 (intervenable bindings), RC section 12 (branching).
- **Question:** While a narrated lesson plays, what happens when the learner changes the model (moves the ball, changes gravity)? The narration was written for the author's run, so learner changes can make it wrong.
- **Options considered:**
  1. Learner changes allowed at any time and carried into the lesson. Maximum freedom; narration may stop matching what is on screen.
  2. Learner changes allowed only in author-marked `explore` beats, on a branch of the lesson run. When the learner continues, the lesson returns to its own run where it left off, unless the beat is marked `keep`, in which case the lesson continues with the learner's choices (and its narration is written for that).
  3. No learner model changes during lessons; exploration only in separate self-study presentations.
- **Accepted position:** option 2. Outside `explore` beats the learner still controls the timeline (pause, replay, narration speed) and the view (zoom, pan) where the presentation allows, but not the model.
- **Reason:** keeps narration truthful while supporting "try it yourself" moments inside a lesson (use mode 3 content with use mode 1 and 2 moments). Branching already exists in the runtime contract, so no new mechanism is needed.
- **Consequence:** in video export, `explore` beats use a declared fallback (a pause, a scripted demonstration, or omission).
- **History:**
  - 2026-09-29 proposed in the presentation kernel spec v0.
  - 2026-09-29 accepted by the owner.

## D-026: Accessibility baseline in v1

- **Status:** Accepted
- **Original position:** Accessibility and non-visual representations are named as representation concerns (07 sections 70-71) without requirements.
- **Raised by:** core semantics spec v0, presentation kernel section 11.
- **Builds on:** R-37, D-017 (education), D-018 (web player first).
- **Question:** Is accessibility a v1 requirement of the presentation kernel, or a later addition?
- **Options considered:**
  1. Baseline in v1: every representation that conveys model information has a text alternative generated from its bindings (label, value, unit), overridable by the author; every control and every drag works from the keyboard; narration always has captions; color is never the only encoding of a value.
  2. Later: build the web player first and add accessibility after.
- **Accepted position:** option 1.
- **Reason:** Prismal's representations are bound to named, typed, unit-carrying model values, so text alternatives can be generated rather than written by hand; that is cheap now and expensive to retrofit. Educational content is often required to meet accessibility rules (WCAG 2.2) in schools and universities. PhET has shown that accessible interactive simulations are practical.
- **Consequence:** the frame description (presentation kernel section 12) carries text alternatives and announcements from the start; the web player exposes them to assistive technology.
- **History:**
  - 2026-09-29 proposed in the presentation kernel spec v0.
  - 2026-09-29 accepted by the owner.

## D-027: Requestable events

- **Status:** Accepted
- **Original position:** Interactions produce operations on the model (08 sections 73-76); a button may trigger an action (08). The model kernel (MK-17.2) limits interventions to `set` on intervenable bindings and `create` or `destroy` on intervenable collections.
- **Raised by:** reference program RP-08 (narrated lesson). The lesson needs a "launch again" step, for the timeline and for a learner's button. With `set` alone this cannot be expressed: changing `angle` after the start does not relaunch the ball, because the launch velocity is an initial value; and a presentation that set `pos`, `vel` and `flying` itself would be doing the model's physics outside the model.
- **Builds on:** D-023 (the model declares what may be changed from outside), MK section 15 (events), RC section 11 (interventions).
- **Question:** How does the outside (a timeline, a learner's button, an experiment) ask the model to perform one of its own actions?
- **Options considered:**
  1. **Requestable events.** A model may declare an event with the trigger `on request`, optionally with a typed payload. An intervention may request it. The event's handler is ordinary model code: it runs with the model's own rules (ownership, conflicts, constraints), at the intervention's instant, and is logged like any intervention.
  2. Presentation-side scripts that `set` several state bindings. Puts model logic (how to relaunch) in the presentation, and requires every bound state to be intervenable.
  3. `reset` with new parameter values. Loses everything else in the run (the log, the lesson's history), and cannot express actions in the middle of a run (a "drop another ball" button).
- **Accepted position:** option 1. Only events the model declares `on request` can be requested; this extends D-023 from values to actions, and keeps the model author in control of what the outside can do.
- **Consequences:** MK-15.3 gains the trigger `on request`; MK-17.2 gains "request a requestable event"; the timeline gains the action `request(E)`; a `button` control can target a requestable event. The RP-01 model gains `relaunch on request` for RP-08.
- **History:**
  - 2026-09-29 proposed while writing reference program RP-08.
  - 2026-09-29 accepted by the owner.

## D-028: Syntax direction

- **Status:** Accepted (amended)
- **Original position:** 09.3 enumerates syntax options and names a hybrid direction (R-44) without choosing; 09.10 and 09.11 write examples in inconsistent notations (F-05, R-69).
- **Raised by:** the syntax study (`docs/syntax-study/`), carried out under D-006.
- **Builds on:** D-006 (selection by comparison on all reference programs), R-44 (hybrid style), R-46 (distinctions to preserve), R-47 and D-019 (serializable IR edited by Mava Studio), D-002 (programming-literate authors).
- **Question:** Which of the three candidate syntaxes, each used to write RP-01 to RP-08, becomes the base direction?
- **Options considered:**
  1. **A, flat keyword statements:** every statement starts with a keyword naming its kind; behaviors listed flat; braces. Every R-46 distinction is marked on the line; nearly one-to-one with the IR.
  2. **B, mathematical sections:** roles by section headings, mathematical operators (`x'`, `:=`, `←`, intervals), inferred types, indentation. Closest to the science and 22 % fewer tokens, but `=` has four meanings chosen by section, inferred types move errors from their cause, and many IR elements have two surface forms.
  3. **C, nested structure:** A's declarations, with behavior grouped in modes and processes, representations nested in views and interactions in representations. Removes repeated mode conditions, but modes hide a discrete state, add an implicit hold rule, and need a new IR construct or pattern recognition to print back.
- **Recommended position:** option 1, amended with C's presentation structure (view trees, interactions inside representations, explicit `sequence` in beats), C's optional named processes (which map to MK-14.1), and B's default space for a model (`model Name in Space`). Modes, sections, inferred types and Unicode-first spelling are not adopted in v1; each may return as its own proposal with a program that needs it. Scientific notation for learners is carried by typeset equations (PK-6.5), not by the source.
- **Reason:** A is strongest on local visibility of the R-46 distinctions and on round-tripping with the IR, the two criteria that are checkable properties rather than taste, and it suits the expected first authors (D-002). C's presentation structure is where C was clearly better and it round-trips one to one. The comparison, the diagnostic variants and the round-trip analysis are in `docs/syntax-study/comparison.md`.
- **Owner amendment:** A is the base, and forms from B and C that improve the author's experience are adopted where they still lower to the same IR and print canonically:
  1. **Grouped declaration blocks:** `const { }`, `param { }`, `input { }`, `state { }`, `discrete { }`, `derived { }`, each listing its declarations below the keyword. The one-line form (`param g: Acceleration = 9.81 m/s^2`) stays valid; the formatter prints blocks.
  2. **Grouped flows:** `flow { der(x) = e; der(v) += e }`, so the laws of motion sit together. The one-line `flow` form stays valid.
  3. **Interval ranges:** `angle: Angle = 45 deg in (0 deg, 90 deg)` as an alternative to `where`; both lower to one `reject` constraint. Intervals are also the one range notation in presentations and runs (plot axes, sliders, observation windows).
  4. **Timeline shorthand:** `run rate 1 until landed` for the two actions `run rate 1` and `wait until landed`.
  5. The recommended additions stand: C's view trees, interactions inside representations and explicit `sequence` in beats; C's optional named processes; B's default space (`model Name in Space`).
  - Not adopted: types inferred from unit literals (keeps errors at their declaration), C's modes, B's sections, indentation and Unicode-first spelling.
- **Accepted position:** the recommended position with the owner amendment. The working syntax is in `docs/syntax-study/working-syntax.md`.
- **Consequences:** the combined candidate is written for all eight programs and becomes the working syntax for the prototype parser, still non-binding until frozen; a keyword and spelling pass follows as a separate entry; the study's specification findings S-1 to S-12 are resolved in the specification and reference programs; comments and identities across visual edits are settled with the IR format.
- **History:**
  - 2026-09-29 proposed by the syntax study.
  - 2026-09-29 accepted with amendment by the owner: grouped declaration blocks (one-line form kept, formatter groups), grouped flows, interval ranges, timeline shorthand; type inference from unit literals declined.

## D-029: Function values that read model bindings

- **Status:** Accepted
- **Original position:** MK-10.3: functions are pure and cannot read bindings other than their parameters and constants; a function that needs model state takes it as an argument. RP-06 defines `derived f : Real -> Real = fn(x) => a * x^2`, which reads the parameter `a`. MK 19.5 calls this expressible without addressing the conflict.
- **Raised by:** syntax study, finding S-1 (`docs/syntax-study/comparison.md` section 7).
- **Builds on:** R-13 (derived values are definitions), R-17 (expressions are pure), D-008 (dependency analysis), MK-10.6 (expressions keep their symbolic form).
- **Question:** May a function value depend on the model's bindings, and if so, where?
- **Options considered:**
  1. **Derived function values.** A derived binding may have a function type, and its body may read model bindings (`derived { f(x: Real): Real = a * x^2 }`). It is re-evaluated at every instant like any derived binding, so `f(2)` always uses the current `a`. Its dependencies are the bindings its body reads, as for any derived binding. MK-10.3 keeps applying to declared functions (`fn`), which stay closed. Stored bindings (parameters, state, constants) of function type may hold only closed functions.
  2. **Strict MK-10.3.** RP-06 becomes `fn f(a: Real, x: Real): Real = a * x^2`, and the presentation passes `a` in (`function_graph(x => f(a, x))`). The model then has no binding for "the function being studied"; the displayed formula is `f(a, x)`, not `f(x) = a x^2`, and lambdas move into presentations.
  3. **Drop MK-10.3.** Any function may read bindings. Library functions would carry hidden dependencies on the model that calls them, and a function reused in two models would mean different things in each.
- **Accepted position:** option 1.
- **Reason:** "the function f with parameter a" is how the mathematics is taught, and the learner's slider acts on `a` while the graph and formula follow; that needs `f` to be a model element that depends on `a`. Keeping it a derived binding reuses existing rules: no stored value, no capture semantics (it is recomputed every instant), dependencies visible to the analysis of MK section 13. Restricting stored function values to closed functions avoids the question of which instant's `a` a stored function would remember.
- **Consequences:** MK-10.3 is reworded to apply to declared functions; MK section 6 or 10 gains the rule for derived function values and the restriction on stored ones; a new static diagnostic for a stored binding holding a function that reads bindings; MK 19.5 cites this entry. RP-06 is unchanged in meaning.
- **History:**
  - 2026-09-29 proposed from syntax study finding S-1.
  - 2026-09-29 accepted by the owner. Applied to the model kernel (MK-10.3, MK-10.7, MK-E20, MK 19.5).

## D-030: Literal `0` adopts zero vectors

- **Status:** Accepted
- **Original position:** MK-3.8 (D-014): the bare literal `0` adopts whatever dimension its context requires. It does not say whether `0` may stand for a vector.
- **Raised by:** syntax study, finding S-3. Every candidate had to write `(0, 0)` or `(0 m/s, 0 m/s)` for a zero velocity or rate.
- **Builds on:** D-014, MK-4.4.
- **Question:** May the literal `0` stand for the zero of a vector type?
- **Options considered:**
  1. `0` adopts the zero of any required `Quantity<D>` or `Vector<S, D>`. It never stands for a `Point` or an `Instant`, which have no zero (their counterparts are `origin` and `t0`).
  2. Keep `0` scalar-only; vectors are written `(0, 0)`.
- **Accepted position:** option 1.
- **Reason:** the zero vector is as unambiguous as the zero quantity: it is the additive identity whatever the space and dimension, which the expected type supplies. `if flying then vel else 0` reads as the physics does. Points and instants are excluded because a zero there would silently mean "the origin", which D-014 and D-022 require to be explicit.
- **Consequences:** MK-3.8 extended; the reference programs and working syntax write `0` for zero vectors.
- **History:**
  - 2026-09-30 proposed from syntax study finding S-3 and accepted under the owner's standing delegation of 2026-09-30.

## D-031: Model variants in reference-program cases

- **Status:** Accepted
- **Original position:** reference-program README: a case lists parameter overrides and configuration settings. RP-03 case `B-stop` changes the event's Zeno policy, which is neither.
- **Raised by:** syntax study, finding S-4.
- **Builds on:** D-012, R-53 (no implied inheritance), RC section 16.
- **Question:** How does a reference program test a model that differs from its main model in structure?
- **Options considered:**
  1. **Named model variants in the reference-program format.** A program may list model variants, each with an ID and a stated change of named elements (like its diagnostic variants). A case names the variant it runs. Nothing is added to the language.
  2. **A language construct for variants** (a model defined as another model with replacements). This is a form of inheritance, which R-53 limits, and no first-slice program needs it outside testing.
  3. **Make the Zeno policy run configuration.** It is model meaning (what happens at accumulation), and RC-16.1 keeps configuration to settings that change accuracy or execution, not behavior.
- **Accepted position:** option 1.
- **Reason:** the need is a testing need, and the diagnostic variants already establish the pattern. A language construct can be proposed later if authors need model variants for teaching (comparing two models side by side).
- **Consequences:** the reference-program README gains a "Model variants" part; RP-03 declares variant `V1` (Zeno policy `stop`) and case `B-stop` runs it.
- **History:**
  - 2026-09-30 proposed from syntax study finding S-4 and accepted under the owner's standing delegation of 2026-09-30.

## D-032: Tuple literals as vectors: typing by expected type

- **Status:** Accepted
- **Original position:** MK section 4 defines points and vectors but not how a vector is written. The sketches use tuples, typed by a declaration (`param u : Vector<Plane, Length> = (3 m, 0 m)`) or by nothing visible (`pivot + L * (sin(θ), -cos(θ))`).
- **Raised by:** syntax study, finding S-5.
- **Builds on:** D-014, D-022 (explicit spaces), MK-4.5.
- **Question:** How does a tuple literal get its space and dimension?
- **Options considered:**
  1. **Expected type.** A tuple literal is a `Tuple` unless its context expects a vector, in which case it is that vector. The expected type comes from a declaration, a parameter of a called function, a flow target, or the other operand of `+`, `-` or a comparison, and propagates through scaling (the tuple in `L * (sin(θ), -cos(θ))` added to a `Point<Plane>` is expected to be `Vector<Plane, 1>`). Combining a `Tuple` with a vector where no expected type made it a vector is a static error.
  2. **Explicit constructors** (`Plane.vec(1 m, 2 m)`). Unambiguous, but noisy in every formula.
  3. **The model's default space** (D-028, `model Name in Space`) for every tuple. Fails silently in models with two spaces.
- **Accepted position:** option 1, with an explicit constructor available where no context supplies the type.
- **Reason:** the expected type is present wherever a physicist writes a vector (a declaration, a flow, an addition to a point), so the literal can stay mathematical while D-022's rule against implicit cross-space values still holds: the space always comes from a typed operand, never from a default.
- **Consequences:** MK section 4 gains the rule; new static diagnostic MK-E21 (tuple used as a vector with no expected vector type); MK 19.3 cites it.
- **History:**
  - 2026-09-30 proposed from syntax study finding S-5 and accepted under the owner's standing delegation of 2026-09-30.

## D-033: Order of run-directing actions within a beat

- **Status:** Accepted
- **Original position:** PK-9.1: a beat's actions start together. RP-08 has `seek(t0)` with a live formula in one beat, and `request(relaunch)` with `run(rate 1)` in another; the order in which they take effect is not specified.
- **Raised by:** syntax study, finding S-8.
- **Builds on:** D-009, D-027, RC section 12, PK-8.7.
- **Question:** When a beat starts, in what order do its actions that act on the lesson run take effect, and what do the other actions see?
- **Options considered:**
  1. **Written order for run-directing actions.** At a beat's start, its run-directing actions (`seek`, `reset`, `branch`, `intervene`, `request`, `run`, `hold`) take effect in the order written, at the same presentation instant and before any presentation time passes. The beat's other actions (show, narrate, animate, wait) then start together and see the result. `sequence` is needed only to order actions that take time.
  2. **Explicit sequence required.** Two run-directing actions in one beat without `sequence` are a static error.
  3. **Unordered.** Left to implementations. Breaks deterministic frames (PK-8.7).
- **Accepted position:** option 1.
- **Reason:** authors read a beat top to bottom, and the instantaneous run actions have an obvious intended order ("go back to the start, then play"). Applying them before presentation time passes keeps "actions start together" true for everything the learner sees, and keeps frames deterministic.
- **Consequences:** PK-9.1 gains the rule; RP-08 needs no `sequence` in `b4` and `b8`.
- **History:**
  - 2026-09-30 proposed from syntax study finding S-8 and accepted under the owner's standing delegation of 2026-09-30.

## D-034: Formula representation for expressions

- **Status:** Accepted
- **Original position:** PK-6.3 lists an `equation` representation typeset from an equation or expression, with symbols linked to bindings (PK-6.5). RP-06 shows a string (`"y = a x^2"`); RP-08 shows `R = v^2 sin(2θ) / g`, where `R`, `v` and `θ` are not bindings.
- **Raised by:** syntax study, finding S-10.
- **Builds on:** MK-10.6 (symbolic expressions), MK-6.1 (display symbol metadata), D-026 (text alternatives), D-029.
- **Question:** How does a presentation show a formula that is not a model equation, and how do its symbols relate to bindings?
- **Options considered:**
  1. **A `formula` representation.** `formula(e)` typesets an expression from its symbolic IR; `formula(f)` of a derived binding or derived function value shows its definition (`f(x) = a x²`); `formula("R", e)` adds a left-hand label, which is presentation text, not a binding. Symbols use each binding's display symbol (MK-6.1). `live` substitutes current values. Strings are never parsed as mathematics. `equation` stays the representation of a model equation (MK section 11).
  2. **Require a model equation** for everything shown. Forces lesson-only formulas into the model.
  3. **Typeset strings.** Loses the link to bindings, live values and generated text alternatives.
- **Accepted position:** option 1.
- **Reason:** every displayed symbol stays linked to a binding, so highlighting, live values and generated text alternatives work (D-026), without making lesson formulas part of the model.
- **Consequences:** PK-6.3 gains `formula`; PK-6.5 covers both; RP-06 shows `formula(f, live)`; RP-01's `speed` and `angle` declare display symbols `v` and `θ`; RP-08 shows `formula("R", speed^2 * sin(2 * angle) / g, live)`.
- **History:**
  - 2026-09-30 proposed from syntax study finding S-10 and accepted under the owner's standing delegation of 2026-09-30.

## D-035: Keywords and spelling of the working syntax

- **Status:** Accepted
- **Original position:** D-028 fixed the structure of the working syntax and left keywords and spellings to a later pass (`docs/syntax-study/comparison.md` section 8.2).
- **Raised by:** syntax study, open item "keyword and spelling pass".
- **Builds on:** D-028, R-46, D-002, D-019.
- **Question:** Which keywords and spellings does the working syntax use?
- **Options considered (per item):**
  1. Derived bindings: `derived` (kept) or `let` (rejected: in most programming languages `let` computes once, while a derived binding is re-evaluated at every instant) or `def`.
  2. Discrete state: `discrete` (kept) or `mode` (rejected: "mode" is the concept, not the binding, MK-14.4).
  3. Crossing triggers: `falling(g)`, `rising(g)`, `crossing(g)` (kept: the kernel terms, short, and they state the direction) or phrases such as `g falls through 0` (rejected: longer, and a second spelling per trigger).
  4. Microsteps in observations: `on E microstep n` (kept: one general form) or a named form such as `before handlers` (rejected: covers one case only).
  5. Equality: `==` for equations, constraints and expectations; `=` only gives a value (kept, D-028).
  6. `in`: a range after a declaration (`in [0, 1)`) and a target view in a timeline (`in scene { }`) (kept: the two contexts never overlap).
  7. Operators: ASCII only (`^`, `-`, `<=`, `->`). Unicode letters are allowed in identifiers (`θ`, `ω`, `π`). The formatter never introduces Unicode.
  8. Comments: `//` line comments; `///` documentation comments attached to the next element, carried into the IR (D-036).
  9. Constraint policy: `policy reject | report | stop`; equation role: `checked within tol`; parameter range: `where cond` or `in I`.
- **Accepted position:** the kept choices above. The full reserved-word list and lexical rules are in `docs/syntax-study/working-syntax.md` section 1.5.
- **Reason:** each choice keeps one spelling per construct (canonical printing, D-019), uses the kernel's own term where one exists, and avoids words whose common programming meaning contradicts the semantics.
- **History:**
  - 2026-09-30 proposed by the keyword pass and accepted under the owner's standing delegation of 2026-09-30.
  - 2026-09-30 the reserved-word list split into reserved words and contextual keywords (D-040).

## D-036: Comments and identities across text and visual editing

- **Status:** Accepted
- **Original position:** R-47 and D-019: the IR is complete and serializable; declared elements have identities stable across edits (MK-7.6). How identities and comments survive when the same model is edited as text and in Mava Studio was left open (`docs/syntax-study/comparison.md` section 8.3).
- **Raised by:** syntax study, round-trip analysis.
- **Builds on:** R-47, D-019, MK-6.2, MK-7.6, PK-2.3.
- **Question:** Where do element identities and author comments live, so that a text edit and a visual edit of one model both preserve them?
- **Options considered:**
  1. **Identities inline in the text** (`param g @id(4f2a) ...`). Always recoverable, but every line carries noise that authors must not touch.
  2. **A sidecar identity file** maintained by the compiler, mapping declaration paths to identities. Survives plain text edits except renames, and adds a second file to every model.
  3. **Identities in the IR, matched by declaration path.** The compiler matches each declaration to the previous IR by its path (`Projectile.param.speed`) and keeps its identity. A rename made through a tool (language server, Mava Studio) is an identity-preserving rename operation. A rename made by plain text editing is a removal and an addition; references that break are reported (PK-2.3), never silently rebound.
- **Accepted position:** option 3 for identities. For comments: `///` documentation comments and a comment block directly before a declaration are **author notes** of that element in the IR, and print back before it; other comments (inside expressions, at end of line) are kept by the text formatter but may be lost by a visual edit of that element, which the editor reports before saving.
- **Reason:** keeps source text free of machine data (D-002 authors), keeps the IR authoritative (R-47), and turns the failure case (a plain-text rename) into a reported breakage instead of a silent change of meaning. Notes attached to elements are what a visual editor can show and keep.
- **Consequences:** the IR format (`docs/spec/04-ir.md`) gives every declared element an identity and a `notes` field; the compiler keeps the previous IR to match paths.
- **History:**
  - 2026-09-30 proposed and accepted under the owner's standing delegation of 2026-09-30.

## D-037: IR serialization

- **Status:** Accepted
- **Original position:** D-019 and R-47 require a complete, serializable IR; the format was left to a separate specification.
- **Raised by:** IR format specification (`docs/spec/04-ir.md`), before the Rust prototype.
- **Builds on:** D-019, D-036, R-47.
- **Question:** How is the IR serialized?
- **Options considered:**
  1. **JSON**, versioned, with opaque string identities and tagged expression trees. Readable in every language, native in the browser (D-018, the web player and Mava Studio), diffable in version control, supported by serde in Rust.
  2. **A binary format** (CBOR, Protocol Buffers). Smaller and faster, but not readable or diffable, and needs a schema toolchain in every consumer.
  3. **The text syntax itself as the stored form.** Rejected by D-036: text carries no identities or notes reliably, and a visual editor would need the parser.
- **Accepted position:** option 1. A binary encoding of the same structure MAY be added later for large models; JSON stays the reference form.
- **Reason:** the IR's first consumers are the web player and Mava Studio, both in the browser, and authors reviewing changes in version control. Size is not a first-slice concern.
- **History:**
  - 2026-09-30 proposed and accepted under the owner's standing delegation of 2026-09-30.

## D-038: Self-retriggering is decided through flows, specialized on the handler's discrete values

- **Status:** Accepted
- **Original position:** MK-15.11: an event is self-retriggering if its handler writes a binding its trigger depends on "directly or through derived bindings". MK 19.2 and RP-03 say the bouncing ball's `bounce` is self-retriggering because its guard `y` depends on `v` "through the flow".
- **Raised by:** the Rust kernel prototype. The two readings disagree on the reference programs: following derived bindings only, `bounce` is not self-retriggering and RP-03.D1 (expects MK-E16) fails; following flows as well, RP-01's `landed` (which writes `vel` and `flying`, both read by the flow of `pos`) is self-retriggering and the valid RP-01 model is rejected.
- **Builds on:** D-004 (declared Zeno policy), MK-13 (dependency graph), MK-14.12 (flow conditions depend only on discrete state).
- **Question:** When must a crossing event declare a Zeno policy?
- **Options considered:**
  1. **Derived bindings only** (the text of MK-15.11). Misses the bouncing ball, the canonical Zeno case.
  2. **Derived bindings and flows.** Catches the bouncing ball, but also flags every event that stops the motion it detects (a landing that switches the flight mode off), which cannot retrigger.
  3. **Derived bindings and flows, with flows specialized on the handler's constant discrete writes.** Where the handler sets discrete state to a constant (`set flying = false`), conditionals in flows that test that state are resolved before following the flow. The projectile's `der(pos)` becomes `0` after `landed`, so `pos.y` no longer depends on anything the handler writes; the bouncing ball's `der(y) = v` does not depend on discrete state, so `bounce` is still self-retriggering.
- **Accepted position:** option 3. The analysis stays static and conservative: when a condition cannot be resolved, both branches are followed.
- **Reason:** it gives the intended answer on every reference program and matches the physics: an event is self-retriggering only if the state after its handler can still carry the guard back across zero. Because flow conditions may read only discrete state, parameters and constants (MK-14.12), the specialization is always well defined.
- **Consequences:** MK-15.11 is reworded; the prototype implements it (`self_retriggering` in `crates/prismal-kernel/src/check.rs`); RP-01 and RP-03 are unchanged.
- **History:**
  - 2026-09-30 found by the prototype, proposed and accepted under the owner's standing delegation of 2026-09-30.

## D-039: Named mathematical constants in the IR

- **Status:** Accepted
- **Original position:** the working syntax predefines `π` and `pi` (section 1.5) and writes `2π` as a product (RP-05); the IR (`docs/spec/04-ir.md` section 6) has no form for a named constant, so lowering would store π as a decimal literal.
- **Raised by:** the text parser, lowering RP-05's `T: Time = 2π * sqrt(m / k)`.
- **Builds on:** MK-10.6 (expressions keep their symbolic structure), D-034 (the formula representation typesets from the IR), IR-1.1.
- **Question:** How does the IR store a named mathematical constant?
- **Options considered:**
  1. **As its value** (`{"num": 3.141592653589793}`). No IR change, but the formula for RP-05's period shows `6.28319 √(m/k)` and cannot be printed back as `2π`.
  2. **As a named constant** `{"const": "pi"}`: dimensionless, with MK-3.8 applying as to a bare literal. One more expression form; typesetting and printing keep π.
  3. **As a predefined binding** of every model. Mixes a mathematical constant with model state: it would enter dependency analysis and could be a target of interventions.
- **Accepted position:** option 2, with `pi` the only constant in v0. Others are added when a program needs them; `e` is not predefined because it is a common binding name (the restitution in RP-03).
- **Reason:** keeps the symbolic structure that MK-10.6 and D-034 require, at the cost of one small expression form. Evaluation is unchanged: `2 * π` in binary64 equals the folded value, so the measurements of RP-05 are identical.
- **Consequences:** 04-ir section 6 gains the form, MK-10.8 states the rule, the kernel checks it as a dimensionless literal. `inf` is not a constant: it is only an interval bound, and an unbounded end lowers to a one-sided comparison, so the IR never holds an infinity (which JSON cannot represent).
- **History:**
  - 2026-09-30 found by the text parser, proposed and accepted under the owner's standing delegation of 2026-09-30.

## D-040: Reserved words and contextual keywords

- **Status:** Accepted
- **Original position:** D-035 and working syntax section 1.5 reserve every keyword of the language, including those of presentations, timelines and runs (`drag`, `scene`, `rate`, `until`, `limit`, `release`, ...).
- **Raised by:** the text parser. The reference programs use reserved words as names: RP-01 declares `process drag`, RP-07 and RP-08 name a view `scene`, RP-07 names an observation `state`. Read literally, D-035 rejects all three.
- **Builds on:** D-035, D-028, D-002 (authors are scientists and educators, whose ordinary names include drag, scene, rate, limit and release).
- **Question:** Which keywords are reserved?
- **Options considered:**
  1. **Every keyword reserved**, and the elements of the programs renamed (`air_drag`, `stage`). One simple rule, but ordinary scientific words become unusable as names, and the list grows with every presentation feature, breaking existing models.
  2. **Reserve the words of the model language, expressions and top-level items; recognize presentation, timeline and run words only where such a word is expected** (contextual keywords, as in C# and Kotlin). No ambiguity arises: those words never occur where a model expression or the name of a model element can start, and the places that expect them (the actions of a beat, the schedule of an observation, the items of a run) never hold a name.
  3. **Any word may be a name wherever the grammar expects a name.** Loses the guarantee that a model reads unambiguously to a person (`param { in: Real = 1 }`).
- **Accepted position:** option 2. Reserved: `space model presentation run object const param input state discrete derived fn flow process event equation constraint on if then else and or not otherwise in where true false zeno stop settle set contribute create destroy connect disconnect emit enter checked within policy reject report intervenable private symbol unit rising falling crossing at every from start request`. Every other keyword is contextual (working syntax section 1.5). The observation `state` of RP-07 is renamed `values`, because `state` begins a model block.
- **Reason:** keeps the model language unambiguous for readers and the parser, and leaves natural scientific names to authors. The contextual list can grow with the presentation kernel without breaking existing models.
- **Consequences:** working syntax section 1.5 lists the two groups; RP-07 changed (recorded in its history); the parser implements the split (`crates/prismal-syntax/src/parser.rs`).
- **History:**
  - 2026-09-30 found by the text parser, proposed and accepted under the owner's standing delegation of 2026-09-30.

## D-041: `on(E)` follows an occurrence of `E` as well as an emission

- **Status:** Accepted
- **Original position:** MK-15.3 defines the trigger `on(E)` as due when "event `E` occurred at the previous microstep". MK-15.10 says `emit(E)` makes `on(E)` triggers due at the next microstep, and RC section 8.1 step 1 collects "`on(E)` for each `E` emitted at `n - 1`". The prototype implemented the runtime contract: `on(E)` followed only emissions.
- **Raised by:** the language guide (`docs/guide/05-events-and-modes.md`). The tank's `event alarm on full` never happened: `full` occurred through its own trigger but was not emitted. The texts contradict each other for any event that occurs without being emitted.
- **Builds on:** D-004 (cascades at one instant through superdense time), D-024 (ordering only by cascades), MK-15.10.
- **Question:** When is `on(E)` due?
- **Options considered:**
  1. **Only after `emit(E)`** (the runtime contract). Chaining one event after another requires every handler to emit its own name (`event full ... { set pumping = false; emit full }`), which is redundant and easy to forget; MK-15.3 would be reworded.
  2. **After `E` occurs or is emitted.** `on(E)` reads as written: after `E`. `emit(E)` remains the way to signal `E` from another handler without `E`'s own trigger. One occurrence and one emission at the same microstep make `on(E)` due once.
  3. **`emit(E)` makes `E` itself occur** (with its handler), and `on(E)` follows occurrences only. Changes the meaning of `emit` established by MK-15.10 and the prototype's cascade tests, and runs `E`'s handler when another event only meant to signal it.
- **Accepted position:** option 2.
- **Reason:** it is the reading of MK-15.3 an author expects, it keeps MK-15.10, and it makes D-024's rule (order is expressed by cascades) usable without boilerplate.
- **Consequences:** MK-15.10 and RC section 8.1 step 1 state both sources; the runtime collects `on(E)` for events handled or emitted at the previous microstep (`crates/prismal-runtime/src/run.rs`, test `on_follows_occurrence`). An event triggered on itself (`event a on a`) cascades until the cascade limit, as an endless chain of emissions already did.
- **History:**
  - 2026-09-30 found by the language guide, proposed and accepted under the owner's standing delegation of 2026-09-30.
