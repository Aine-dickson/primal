# Rust Kernel Prototype

The first implementation of Prismal's core semantics, built kernel-first (D-002): a Rust library and API that runs the first-slice reference programs (D-012) against the specification (`docs/spec/`). It has no text parser yet; programs are built through the IR API, which the working syntax (D-028) will lower to.

- **Status:** v0 prototype, 2026-09-30. All model-side expectations of RP-01 to RP-07 pass; RP-08's model-side behavior passes.
- **Build and test:** `cargo test` at the repository root (Rust 1.94 or later). `cargo run --release -p prismal-runtime --example report` prints the measured errors quoted below.

## Structure

| Crate | Specification | Content |
|---|---|---|
| `crates/prismal-ir` | `04-ir.md` | IR types with JSON serialization (D-037), dimensions and units, a builder API |
| `crates/prismal-kernel` | `01-model-kernel.md` | Type and dimension checking with expected-type propagation (D-030, D-032), static diagnostics MK-E01 to MK-E22, dependency analysis, compilation, evaluation with statuses |
| `crates/prismal-runtime` | `02-runtime-contract.md` | Runs, `dopri5` and `rk4` with dense output, crossing detection and location, event iteration in superdense time, Zeno detection, constraints and equation checks, interventions and requests, time events, observation, an interactive session with undo and redo |

The reference programs are in `crates/prismal-runtime/tests/common/mod.rs`, each mirroring its working-syntax text in `docs/spec/reference-programs/`. Each program has its own test file.

## Coverage of the reference programs

| Program | Covered by the prototype | Not yet covered |
|---|---|---|
| RP-01 | E1 to E9, D1 to D5, IR JSON round trip | - |
| RP-02 | E1 to E12 | - |
| RP-03 | E1 to E15, D1 to D3, model variant V1 | - |
| RP-04 | E1 to E11, D1, D2 | - |
| RP-05 | E1 to E10, D1 to D3 | - |
| RP-06 | E1 to E9, E14 (interventions, rejection, drag previews and commit, undo, redo, non-intervenable target) | E10 to E13: frames, text alternatives, keyboard drag (presentation) |
| RP-07 | E1 to E5 (model values after the drag), D1 to D6 | Rendering of E5 (presentation) |
| RP-08 | Relaunch on the learner's branch (E5) and on the lesson run (E9); requestability (D-027) | Timeline timing, captions, explore beats, video export (presentation) |

Every run in the suite is also run twice and compared bit for bit (RC-14.5).

## Measured accuracy

Provisional tolerances in the reference programs were confirmed with these measurements and are now fixed (their histories record the values).

| Program | Quantity | Measured | Tolerance |
|---|---|---|---|
| RP-01 | landing time, range | 1.4e-12 s, 2.0e-11 m | 1e-9 s, 1e-8 m |
| RP-02 default | landing time, range, apex height (relative) | 1.7e-7, 8.9e-8, 4.5e-7 | 1e-5 |
| RP-02 tight | same | 2.9e-12, 2.7e-12, 1.1e-11 | 1e-8 |
| RP-03 | Zeno detection | bounce 63 at 4.06370922186394 s (predicted: bounce 63 at 4.06370922604679 s) | bounce 61 to 65, within 1e-4 s of the limit |
| RP-04 `dopri5` | worst period (relative), energy drift | 2.0e-7, 7.7e-5 | 1e-5, 1e-3 |
| RP-04 `rk4` | same | 7.9e-9, 1.35e-7 | 1e-6, 1e-6 |
| RP-05 `dopri5` | `x(10 s)`, energy drift | 2.1e-7 m, 4.9e-5 | 1e-5 m, 1e-3 |
| RP-05 `rk4` | same | 2.4e-9 m, 1.8e-8 | 1e-7 m, 1e-7 |

The energy drifts agree with the predictions of `tools/refvals.py` (RP-04: 7.8e-5 and 1.3e-7).

## Findings

- **D-038.** The specification's rule for self-retriggering events (MK-15.11) contradicted the reference programs: read literally it accepted the bouncing ball without a Zeno policy; read as RP-03 describes it, it rejected the valid projectile. The rule now follows flows, specialized on the discrete values the handler sets.
- **Diagnostics come in groups.** An invalid variant can produce a secondary diagnostic besides the named one (RP-01.D4 also gives MK-E16, because the invalid flow condition defeats the specialization of D-038). The suite requires the named diagnostic to be among those reported.
- **`emit` semantics.** A first version made the emitted event itself due; MK-15.10 makes the events triggered `on(E)` due. Fixed and covered by a unit test with the cascade limit (RC-8.3).

## Not implemented

| Item | Where specified |
|---|---|
| Text parser for the working syntax | D-028, `docs/syntax-study/working-syntax.md` |
| Presentation kernel: views, representations, frames, timeline engine, accessibility output | `03-presentation-kernel.md` |
| Inputs (`input` bindings, `on input`) | RC section 11.2 |
| Snapshots and backward seek within a dynamic run (undo recomputes from the start) | RC section 14.1 |
| Payloads of requested and emitted events | MK-15.1 |
| `contribute` operations on discrete state; collections and relations | MK sections 8, 16 |
| Failure policy `pause` (interactive) | RC-10.3 |
| Frames of spaces (D-022), affine temperature units | MK sections 3.4, 4 |

## History

- 2026-09-30 prototype written; RP-01 to RP-07 model-side expectations and RP-08 model behavior pass; D-038 raised.
