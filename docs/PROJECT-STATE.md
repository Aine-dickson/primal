# Prismal Project State

**Read this first.** This file is the entry point for anyone resuming work on Prismal, in a new session or years later. It records where the project stands, what is decided, and what comes next. Update it at the end of every working session.

- **Last updated:** 2026-09-30
- **Phase:** Core semantics spec v0. All three parts drafted (`docs/spec/01`, `02`, `03`); D-020 to D-027 accepted. First-slice reference programs written (`docs/spec/reference-programs/`). Syntax study done (`docs/syntax-study/`); D-028 accepted with amendment; working syntax written (`docs/syntax-study/working-syntax.md`); specification and reference programs converted to it; syntax study findings resolved (D-029 to D-034).

## What Prismal is

A modeling language and runtime for mathematics, science and interactive systems. One model is simulated, observed, and projected into many representations (equations, graphs, spatial scenes, animations, data).

**Origin:** a Manim-like tool for 2D interactive simulation scenes in math, physics and chemistry, built in Rust, where a concept becomes an interactive scene that can also be exported as video.

**Purpose (D-017, accepted):** a universal scientific language whose purpose is education, through three use modes:
self-study, deep study, and syllabus-based educational content creation.

**Tagline:** One model. Many representations.

## Document map

| Location | Role | Status |
|---|---|---|
| `docs/PROJECT-STATE.md` | Entry point and current state | Living |
| `docs/decisions/divergence-register.md` | Every departure from the exploration record, with reasons and status | Living, authoritative |
| `docs/spec/` | Core semantics specification (model kernel, runtime contract and presentation kernel v0 drafts) | Living, normative |
| `docs/syntax-study/` | Syntax study (D-006): candidates A, B, C with all reference programs, comparison, specification findings S-1 to S-12 | Study record, non-normative |
| `docs/decisions/carried-forward.md` | Every resolution from the exploration record that is kept (R-01 to R-70), and where it lands | Living |
| `docs/audit/2026-09-29-design-audit.md` | Audit of the exploration record: findings F-01 to F-15 | Fixed record |
| `docs/design/` (01 to 09.11) | Exploration record: case catalogue, models, semantics, DSL study | Frozen, non-normative |
| `docs/design/10-branding.md` | Name, taglines, descriptions | Current |

**Precedence:** divergence register, then the specification, then carried-forward record, then exploration record. Most of the exploration record's analysis (01 to 09.10) is carried forward; see the carried-forward record before assuming something must be redesigned.

## Current position

1. The exploration record (01 to 09.11) mapped the problem space well and established sound architectural separations.
2. The audit found that the DSL stages (09.x) did not produce a DSL: syntax was never chosen, dynamics semantics are informal, validation was asserted rather than tested, and no first slice of the (intentionally universal) scope is defined. See the audit summary and its addendum.
3. **No DSL has been adopted.** Nothing in 09.x is binding.

## Decided so far

- **D-017:** universal scientific scope, purpose is education (self-study, deep study, syllabus-based content creation).
- **D-001:** first slice is 2D mechanics plus supporting math, end to end through all layers.
- **D-002:** kernel-first (Rust library and API before any syntax); authorship is open (anyone may author); expected first authors are programming-literate educators and creators; non-programmers come through Mava Studio (D-019).
- **D-007:** v1 formulas are causal (one-way); equations are displayed and checked, not solved. Acausal solving is a later extension.
- **D-019:** Prismal is a prerequisite for the Mava Studio redesign; Mava Studio will be the entry point for non-programmer authors. Prismal needs a complete, serializable semantic IR an editor can drive.
- **D-003:** every core-spec section cites the prior art it follows or departs from, and the `R-##` resolutions it restates.
- **D-004:** hybrid dynamics: flows, directional crossing guards, resets as the only state replacement, superdense time for same-instant event chains (with a limit), declared Zeno policy.
- **D-005:** behaviors contribute to a quantity; the quantity's type may supply the combination (sum for rates of change), overridable; direct replacement only in resets and interventions, conflicts reported.
- **D-006:** syntax chosen only after the semantics spec, by comparing 2-3 candidates on all reference programs; a labeled, non-binding sketch syntax may be used earlier for readable examples.
- **D-008:** cycles classified before running: loops through state allowed; loops between instantaneous values rejected unless a solver is declared.
- **D-009:** the explanation timeline (scenes, beats, reveals, camera, narration cues) is a peer of the model: it can wait on model events and direct the simulation (play, pause, seek, reset, branch); the model never depends on it; it is medium-independent.
- **D-010:** two field forms: formula fields are functions; evolving fields are core state whose discretization (grid, resolution, boundaries) is declared and visible to the author.
- **D-011:** the kernel concepts are organized as a model kernel, a runtime contract, and a presentation kernel (including the explanation timeline) with narrow interfaces; one model can carry several presentations.
- **D-012:** validation by executable reference programs with known results; first-slice set: projectile (with and without drag), bouncing ball, pendulum, spring-mass, draggable function plot, vector addition, narrated projectile lesson.
- **D-013:** two stochastic modes: per-step sampling and exact event-time sampling (exponential waiting times, Gillespie); exploration-record formulas are not normative.
- **D-014:** affine quantities (°C, time instants) follow point/vector rules; literal 0 adopts any unit, other bare numbers mixed with units are errors; spatial dimension is a type parameter in general interfaces.
- **D-015:** replay is bit-identical on the same build and platform, tolerance-equivalent across platforms (native vs browser); an optional deterministic-math mode may come later.
- **D-016:** exploration documents frozen as the historical record; marked correction notes allowed inside them; continuity via R-##, changes via D-###; new normative docs short and citing R-##.
- **D-020:** v1 constraints are checked, never enforced: each has a policy (reject, report, stop); constrained systems are written in coordinates that satisfy the constraint; enforcement comes later with acausal solving.
- **D-021:** plane angle is dimensionless (SI): `rad` = 1, `deg` = π/180, `rev` = 2π; display unit is binding metadata.
- **D-022:** points and vectors belong to a declared space with standard axes; spaces may declare extra frames; components read in a named frame; cross-space values need explicit conversion.
- **D-023:** parameters are intervenable by default; state only when the model declares it; constants, derived values and inputs never; ranges are `reject` constraints; interventions apply at an event instant.
- **D-024:** no event priorities; all events due at one microstep form one transition; ordering only by emitted cascades.
- **D-025:** learner model changes during a narrated lesson only in author-marked explore beats, on a branch; the lesson returns to its own run unless the beat is marked keep; outside explore beats the learner controls timeline and view only.
- **D-026:** accessibility baseline in v1: generated text alternatives from bindings (overridable), keyboard operation of every control and drag, captions for narration, no color-only encoding.
- **D-027:** requestable events: a model may declare events `on request`; a timeline, button or experiment triggers them through an intervention; the handler is ordinary model code, validated and logged.
- **D-028:** syntax direction: candidate A (keyword-led statements) as the base, amended with grouped declaration and flow blocks (one-line form kept), interval ranges, the timeline shorthand `run rate r until E`, C's view trees, nested interactions, explicit beat sequences and named processes, and B's default space. No type inference from literals, no modes in v1.
- **D-029:** derived bindings may hold function values that read model bindings (evaluated every instant); declared functions and stored function values stay closed (MK-10.3, MK-10.7, MK-E20).
- **D-030:** the literal `0` also stands for the zero vector of a required vector type (never a point or instant).
- **D-031:** reference programs may declare named model variants; cases may run them.
- **D-032:** tuple literals are vectors when the context expects one (declaration, argument, flow target, other operand, through scaling); otherwise MK-E21.
- **D-033:** run-directing actions in a beat apply in written order at its start, before presentation time passes.
- **D-034:** `formula` representation typesets expressions, definitions and labeled expressions from the IR; never strings.
- **D-018:** first target is an interactive web player (WASM). Output form and medium follow the nature of the content and the author's intent; no medium is the defining output.

## Waiting on the owner

Nothing blocking. Since 2026-09-30 the owner has delegated acceptance of recommended positions (see Working process).

## Next steps

**Next session starts here:** keyword and spelling pass on the working syntax (comparison section 8.2), then comments and identities in the IR (section 8.3), then the IR format and the Rust kernel prototype (step 4). The Rust kernel prototype (step 4) may start in parallel against the spec and the reference programs.

1. Core semantics spec v0: all three parts drafted; decisions D-020 to D-026 accepted.
2. Reference programs with expected results (D-012): first-slice suite written (RP-01 to RP-08); provisional tolerances to be confirmed by the prototype.
3. Syntax study: three candidates compared; D-028 accepted with amendment; working syntax written. Findings S-1 to S-12 resolved. Keyword pass to follow.
4. Rust kernel prototype running the reference programs.

## Session log

| Date | Summary |
|---|---|
| 2026-09-29 | Design series imported into `docs/design/`. Name Prismal adopted. Design audit completed; divergence register created with D-001 to D-016. Owner clarified purpose: D-017 accepted. D-001 accepted (first slice: 2D mechanics + math). D-002 accepted (kernel-first; educators and creators author). D-018 opened. D-007 accepted (causal v1). D-018 accepted (web player first; output follows content and intent). D-002 amended twice (authors likely programmers; authorship open). D-019 accepted (Mava Studio built on Prismal). Carried-forward record created (R-01 to R-70); audit corrected for F-01, F-02, F-06. D-003 to D-016 accepted (D-016 with correction notes allowed; notes added to 09.10b and 09.11). Agreed to review register entries one at a time. |
| 2026-09-29 | Core semantics spec started in `docs/spec/`: index and model kernel v0 (values, units, spaces, status, bindings, objects, collections, relations, domains, expressions, equations, constraints, dependency analysis, flows, events, operations, interface, static diagnostics). All first-slice reference programs checked expressible; one gap found (checks at event instants). D-020 to D-023 proposed. Carried-forward record gained spec pointers. D-020 accepted (check only). D-021 accepted (angles dimensionless). D-022 accepted (spaces and frames). D-023 accepted (declared intervenability). Runtime contract v0 drafted (clocks, runs, superdense trajectory, solvers, event location, event iteration, Zeno, failures, interventions, execution control, randomness, snapshots and replay, observer output); D-024 proposed and accepted. |
| 2026-09-29 | Session work committed (dc42472). Presentation kernel v0 drafted: presentations, observation and data (including `on(E)` observations, closing the model kernel's gap), expectations, projection, representation set, views, presentation time and animation, explanation timeline, interaction, accessibility, output and media. D-025 and D-026 proposed. D-025 accepted (explore beats on a branch). D-026 accepted (accessibility baseline in v1). |
| 2026-09-29 | Presentation kernel committed and pushed (a710ba4). First-slice reference programs written in `docs/spec/reference-programs/` (RP-01 to RP-08) with cases, expectations (analytic, reference, bound, behavior, diagnostic), learner scripts, and `tools/refvals.py` for every reference value. Conditional expectations resolved as cases. RP-08 exposed a gap (no way to request a model action from outside): D-027 proposed and accepted. |
| 2026-09-29 | Syntax study (D-006) in `docs/syntax-study/`: candidates A (flat keyword statements), B (mathematical sections) and C (nested structure) each write RP-01 to RP-08; comparison on the four D-006 criteria, diagnostic variants and IR round-tripping; `tools/metrics.py` for line and token counts. Twelve specification findings recorded (S-1 to S-12). D-028 proposed (A as base with C's presentation structure and processes, B's default space), then accepted with amendment (grouped declaration and flow blocks, interval ranges, timeline shorthand; no type inference). Working syntax written for all eight programs. S-1 presented as D-029 and accepted; model kernel updated (MK-10.3 reworded, MK-10.7, MK-E20). |
| 2026-09-30 | Owner delegated acceptance of recommended positions. Syntax study findings resolved: D-030 (zero vectors), D-031 (model variants in cases), D-032 (tuple literals typed by context), D-033 (order of run actions in a beat), D-034 (formula representation) accepted; corrections S-2, S-6, S-7 (MK-E22), S-9, S-11 (process kind), S-12 (representation names) applied. Specification examples and all reference programs converted to the working syntax; RP-04.D2 now expects MK-E01. |

## Working process

Since 2026-09-30 the owner has delegated acceptance: recommended positions are recorded directly as Accepted, noting the delegation, with full options and reasons, and summarized to the owner afterwards. Choices of product direction or owner taste, and hard-to-reverse choices, are still put to the owner. (Until then, entries were reviewed one at a time; D-001 to D-029 were reviewed that way.)

## Conventions

- Decisions are recorded in the divergence register, never by editing the frozen exploration documents.
- Register IDs (`D-###`) and audit finding IDs (`F-##`) are permanent; refer to them by ID.
- Writing style for all project text: plain punctuation (no em dashes, straight quotes), no decorative emoji, neutral documentation voice.
