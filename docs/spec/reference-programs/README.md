# Reference Programs

The acceptance suite for Prismal (D-012). Each program is a model, optionally a presentation and a timeline, with named cases and expected results. The same suite checks three things:

1. **The specification:** every program can be expressed in the core semantics (`01` to `03`).
2. **The syntax study:** every candidate syntax writes every program (D-006).
3. **The prototype:** every program runs and meets its expectations.

Programs are never removed silently. A change to an expected value, a tolerance, or a program's content is recorded in the program's history with its reason.

## First-slice suite (D-001, D-012)

| ID | Program | File | Exercises |
|---|---|---|---|
| RP-01 | Projectile, no drag | `RP-01-projectile.md` | flows, crossing events, event location, observation at events |
| RP-02 | Projectile with drag | `RP-02-projectile-drag.md` | two contributions to one derivative (D-005), reference-solution comparison |
| RP-03 | Bouncing ball | `RP-03-bouncing-ball.md` | resets, modes, self-retriggering events, Zeno policy (D-004) |
| RP-04 | Pendulum | `RP-04-pendulum.md` | angles (D-021), constraint by construction (D-020), period, energy drift per solver |
| RP-05 | Spring-mass | `RP-05-spring-mass.md` | analytic trajectory, period, energy, display equation |
| RP-06 | Function plot with draggable parameter | `RP-06-function-plot.md` | static model, interventions (D-023), constraints with `reject`, synchronized views, accessibility (D-026) |
| RP-07 | Vector addition | `RP-07-vector-addition.md` | points and vectors (R-09), spaces (D-022), static diagnostics |
| RP-08 | Narrated projectile lesson | `RP-08-narrated-lesson.md` | explanation timeline (D-009), waits on events, seek, explore beats (D-025), video fallback |

## Format of a program

Each file has these parts:

- **Purpose:** what the program tests and which rules it exercises (by rule ID).
- **Model, presentation, timeline:** in the sketch notation. **sketch (non-binding, D-006)**. The meaning is fixed by the specification, not by the notation.
- **Cases:** named run configurations. A case lists parameter overrides and configuration settings that differ from the defaults (RC section 16). Where an expectation only holds under some parameter values (the analytic range holds only without drag), it belongs to the case with those values. This is how the suite expresses conditional expectations (PK section 15).
- **Expectations:** each has an ID (`RP-01.E3`), an observation (PK section 3), an expected value, a tolerance, and a kind:
  - `analytic`: the expected value is exact mathematics. The tolerance allows for event location (`ε_t`), solver error and rounding, and is justified in the file.
  - `reference`: the expected value comes from a high-precision computation outside Prismal (`tools/refvals.py`). The tolerance allows for the solver.
  - `bound`: a limit rather than a value (energy drift below a bound).
  - `diagnostic`: the program, or a stated variant of it, MUST be rejected with the named diagnostic (MK section 18).
  - `behavior`: a stated sequence of events, states or presented frames.
- **Status of each expectation:** `fixed` (follows from the specification and mathematics) or `provisional` (a tolerance or bound that depends on solver behavior and is to be confirmed or tightened when the prototype first runs the program; a change is recorded in the history).
- **History.**

## Conventions

- Values are written in coherent SI units unless a unit is shown.
- Numbers are given to 15 significant digits where they are exact or reference values. Longer values are in the output of `tools/refvals.py`.
- `g = 9.81 m/s^2` in every program.
- Default run configuration is RC section 16 (`dopri5`, `rtol = 1e-6`, `atol = 1e-9`, `ε_t = 1e-10 s`, `T_ref = 1 s`), unless a case says otherwise.
- **Learner scripts.** Programs with interaction or explore beats include a learner script: timed inputs (control changes, pointer drags, continue presses) at stated presentation instants. A learner script is a test harness input, not part of the language; the prototype feeds it through the same interaction path as a real learner (PK section 10).
- **Same-platform replay.** Every program is also run twice on the same build and platform; the two runs MUST be bit-identical (RC-14.5). This is an implicit expectation of every case and is not repeated in each file.

## Reference values

`tools/refvals.py` computes every analytic and reference value in the suite, using only the Python standard library:

- analytic values in 50-digit decimal arithmetic;
- the drag projectile with classical RK4 in 50-digit decimal arithmetic at two step sizes, combined by Richardson extrapolation (the two results agree to about 1e-15, so the values are accurate well beyond every tolerance used);
- energy drift of the pendulum and spring-mass with plain floating-point RK4 and Dormand-Prince 5(4), as a basis for the provisional bounds.

## History

- 2026-09-29 first-slice suite written (RP-01 to RP-08).
