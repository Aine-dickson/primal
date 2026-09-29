# RP-06 Function plot with draggable parameter

## Purpose

The graph of `f(x) = a x^2`, where the learner changes `a` with a slider, by dragging a point on the curve, or from the keyboard. The model is static (MK-14.15). The program checks interventions on parameters (D-023, RC section 11), `reject` constraints during interaction (PK-10.6), the drag transaction (PK-10.5 to PK-10.8), synchronized views (PK-7.5), undo (PK-10.10), rejection of non-intervenable targets (RC-11.4), and the accessibility baseline (D-026).

## Model

**sketch (non-binding, D-006)**

```text
object QuadraticDemo {
  param a : Real = 1
  constraint -5 <= a and a <= 5 : reject
  derived f : Real -> Real = fn(x) => a * x^2
}
```

## Presentation

```text
presentation QuadraticPlot for QuadraticDemo {
  view plot : plot(x in [-3, 3], y in [-5, 20])
  show function_graph(f) in plot
  show slider(a, range -5 .. 5, step 0.1)
  show equation("y = a x^2", live values)
  show marker(point (1, f(1))) in plot
       draggable inverse (px, py) -> a = py / 1^2      // x fixed at 1
  observe a_now  = a live
  observe f2     = f(2) live
  observe log    = intervention log
}
```

## Learner script

All instants are presentation time.

| At | Input | Intended action |
|---|---|---|
| 1.0 s | slider set to `2` | intervention `set a = 2` |
| 2.0 s | slider set to `7` | intervention `set a = 7` |
| 3.0 s | pointer down on the marker | drag begins |
| 3.1 s to 3.5 s | pointer moves to `y = 2, 3, 4, 6, 9` (one per 0.1 s) | proposals `a = 2, 3, 4, 6, 9` |
| 3.6 s | pointer up | drag commits |
| 4.0 s | focus slider, press Right arrow twice | two interventions, `+0.1` each |
| 5.0 s | undo | |
| 5.5 s | redo | |
| 6.0 s | an intervention `set f = ...` submitted directly through the interaction path | |

## Expected results

| ID | Observation | Expected | Kind | Status |
|---|---|---|---|---|
| RP-06.E1 | `a_now`, `f2` before any input | `1`, `4` | analytic | fixed |
| RP-06.E2 | after 1.0 s | `a = 2`, `f2 = 8`; one log entry | behavior | fixed |
| RP-06.E3 | after 2.0 s | intervention rejected by the constraint; `a = 2` unchanged; rejection reported to the presentation (RC-10.5); no log entry | behavior | fixed |
| RP-06.E4 | during the drag, 3.1 s to 3.5 s | proposals `2, 3, 4` shown as valid previews, `6` and `9` shown as invalid; nothing committed; the log is unchanged | behavior | fixed |
| RP-06.E5 | after release, 3.6 s | `a = 4` (the last valid proposal, PK-10.8); `f2 = 16`; exactly one new log entry | behavior | fixed |
| RP-06.E6 | after the key presses, 4.0 s | `a = 4.2` within `1e-12`; two new log entries | behavior | fixed |
| RP-06.E7 | after undo, 5.0 s | `a = 4.1` within `1e-12`; the last entry removed from the log (PK-10.10) | behavior | fixed |
| RP-06.E8 | after redo, 5.5 s | `a = 4.2` within `1e-12`; the entry restored | behavior | fixed |
| RP-06.E9 | the attempt on `f`, 6.0 s | rejected before reaching the model: `f` is derived, not intervenable (RC-11.4, D-023); `a` unchanged | behavior | fixed |
| RP-06.E10 | every presented frame | the function graph, the slider, the equation's live value and the marker all show the same `a` (PK-7.5) | behavior | fixed |
| RP-06.E11 | slider text alternative at the end | names the binding `a` and states its value `4.2` (PK-11.1) | behavior | fixed |
| RP-06.E12 | marker text alternative at the end | states the point's coordinates, `x = 1` and `y = 4.2` (PK-11.1) | behavior | fixed |
| RP-06.E13 | keyboard drag | with the marker focused, arrow keys move it by the declared step along its inverse, producing interventions like the pointer drag (PK-11.2) | behavior | fixed |
| RP-06.E14 | model trajectory | the run has a single simulation instant; every commit is a new microstep at `t0` (RC-12.4) | behavior | fixed |

E4 and E5 fix the rule for a drag that goes past a constraint boundary: the committed value is the last proposal that passed validation (`4`), not the boundary value (`5`). A presentation that wants the boundary value declares an inverse that clamps (PK-5.7).

E6 to E8 use `within 1e-12` because `4 + 0.1 + 0.1` is not exactly `4.2` in binary64.

## History

- 2026-09-29 written.
