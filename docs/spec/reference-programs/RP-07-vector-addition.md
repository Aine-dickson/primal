# RP-07 Vector addition

## Purpose

Two displacement vectors added tip to tail from a point. The program checks the point and vector rules (R-09, MK-4.4), spaces (D-022, MK-4.5), dimension and literal rules (MK section 3), and dragging a vector's head through a declared inverse (PK-5.6). Most of its expectations are static diagnostics.

## Model

**sketch (non-binding, D-006)**

```text
space Plane : Euclidean 2

object VectorDemo {
  param A : Point<Plane>            = origin + (1 m, 2 m)
  param u : Vector<Plane, Length>   = (3 m, 0 m)
  param w : Vector<Plane, Length>   = (0 m, 2 m)

  derived sum    : Vector<Plane, Length> = u + w
  derived length : Length                = |sum|
  derived B      : Point<Plane>          = A + sum
  derived mid    : Point<Plane>          = A + u              // where w starts
}
```

## Presentation

```text
presentation VectorPlot for VectorDemo {
  view scene : spatial(Plane) { scale 1 m -> 40 px; y up }
  show axes, grid in scene
  show marker(A) in scene
  show arrow(u,   at A)   in scene draggable head inverse (head) -> u = head - A
  show arrow(w,   at mid) in scene
  show arrow(sum, at A)   in scene
  show label(length) in scene
  observe state = (sum, length, B) live
}
```

## Learner script

| At | Input | Intended action |
|---|---|---|
| 1.0 s | drag the head of `u` to the model point `(5 m, 1 m)` and release | intervention `set u = (5 m, 1 m) - A` |

## Expected results

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-07.E1 | `sum` initially | (3 m, 2 m) | exact | analytic | fixed |
| RP-07.E2 | `length` initially | 3.60555127546399 m (`sqrt 13`) | one rounding | analytic | fixed |
| RP-07.E3 | `B` initially | the point (4 m, 4 m) | exact | analytic | fixed |
| RP-07.E4 | after the drag | `u = (4 m, -1 m)`, `sum = (4 m, 1 m)`, `length` = 4.12310562561766 m (`sqrt 17`), `B = (5 m, 3 m)` | exact, `length` to one rounding | analytic | fixed |
| RP-07.E5 | after the drag | the arrow for `w` starts at the new `mid = (5 m, 1 m)`; every view shows the new state in the same frame (PK-7.5) | exact | behavior | fixed |

## Diagnostic variants

Each variant adds one line to the model. Each MUST be rejected before execution.

| ID | Added line | Expected diagnostic |
|---|---|---|
| RP-07.D1 | `derived bad : Point<Plane> = A + B` | MK-E04 (point plus point) |
| RP-07.D2 | `derived bad : Point<Plane> = 2 * A` | MK-E04 (scaling a point) |
| RP-07.D3 | `derived bad = u + (3, 0)` | MK-E03 (bare non-zero literals with a dimensioned vector) |
| RP-07.D4 | `param vel : Vector<Plane, L/T> = (1 m/s, 0 m/s)` and `derived bad = u + vel` | MK-E01 (adding length and velocity) |
| RP-07.D5 | `space Board : Euclidean 2`, `param z : Vector<Board, Length> = (1 m, 0 m)` and `derived bad = u + z` | MK-E05 (values from different spaces) |
| RP-07.D6 | `derived ok = u + (0, 0)` | accepted: the literal `0` adopts the required dimension (MK-3.8) |

D6 is a positive control for the literal rule: it MUST be accepted.

## History

- 2026-09-29 written.
