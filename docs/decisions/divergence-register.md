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
