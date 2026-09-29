# Prismal Project State

**Read this first.** This file is the entry point for anyone resuming work on Prismal, in a new session or years later. It records where the project stands, what is decided, and what comes next. Update it at the end of every working session.

- **Last updated:** 2026-09-29
- **Phase:** Decision review complete (D-001 to D-019 decided). Next: core semantics spec v0.

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
| `docs/decisions/carried-forward.md` | Every resolution from the exploration record that is kept (R-01 to R-70), and where it lands | Living |
| `docs/audit/2026-09-29-design-audit.md` | Audit of the exploration record: findings F-01 to F-15 | Fixed record |
| `docs/design/` (01 to 09.11) | Exploration record: case catalogue, models, semantics, DSL study | Frozen, non-normative |
| `docs/design/10-branding.md` | Name, taglines, descriptions | Current |

**Precedence:** divergence register, then carried-forward record, then exploration record. Most of the exploration record's analysis (01 to 09.10) is carried forward; see the carried-forward record before assuming something must be redesigned.

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
- **D-018:** first target is an interactive web player (WASM). Output form and medium follow the nature of the content and the author's intent; no medium is the defining output.

## Waiting on the owner

Nothing blocking. New questions will be added to the divergence register as `Open` entries as the spec work raises them.

## Next steps

**Next session starts here:** core semantics spec v0, beginning with the model kernel (D-011). Write it in `docs/spec/`, short and normative, citing `R-##`, `D-###` and prior art (D-003). Examples may use a labeled sketch syntax (D-006). Check each section against the first-slice reference programs (D-012).

1. Core semantics spec v0: short, normative, citing prior art (D-003, D-004, D-005, D-008, D-010, D-011).
2. Reference programs with expected results, used as the acceptance suite (D-012).
3. Syntax study: reference programs written in two or three candidate syntaxes (D-006).
4. Rust kernel prototype running the reference programs.

## Session log

| Date | Summary |
|---|---|
| 2026-09-29 | Design series imported into `docs/design/`. Name Prismal adopted. Design audit completed; divergence register created with D-001 to D-016. Owner clarified purpose: D-017 accepted. D-001 accepted (first slice: 2D mechanics + math). D-002 accepted (kernel-first; educators and creators author). D-018 opened. D-007 accepted (causal v1). D-018 accepted (web player first; output follows content and intent). D-002 amended twice (authors likely programmers; authorship open). D-019 accepted (Mava Studio built on Prismal). Carried-forward record created (R-01 to R-70); audit corrected for F-01, F-02, F-06. D-003 to D-016 accepted (D-016 with correction notes allowed; notes added to 09.10b and 09.11). Agreed to review register entries one at a time. |

## Working process

Register entries are reviewed **one at a time**, each presented with enough context to decide without
rereading the audit. The decision and its reasoning are recorded in the register immediately.

All entries D-001 to D-019 have been reviewed (2026-09-29). New entries follow the same one-at-a-time process.

## Conventions

- Decisions are recorded in the divergence register, never by editing the frozen exploration documents.
- Register IDs (`D-###`) and audit finding IDs (`F-##`) are permanent; refer to them by ID.
- Writing style for all project text: plain punctuation (no em dashes, straight quotes), no decorative emoji, neutral documentation voice.
