# RP-07 Vector addition

## Purpose

Two displacement vectors added tip to tail from a point. The program checks the point and vector rules (R-09, MK-4.4), spaces (D-022, MK-4.5), dimension and literal rules (MK section 3), and dragging a vector's head through a declared inverse (PK-5.6). Most of its expectations are static diagnostics.

## Model

**working syntax (D-028, non-binding)**

```text
space Plane = euclidean(2)

model VectorDemo in Plane {
  param {
    A: Point          = origin + (1 m, 2 m)
    u: Vector<Length> = (3 m, 0 m)
    w: Vector<Length> = (0 m, 2 m)
  }
  derived {
    sum:    Vector<Length> = u + w
    length: Length         = |sum|
    B:      Point          = A + sum
    mid:    Point          = A + u            // where w starts
  }
}
```

## Presentation

```text
presentation VectorPlot for VectorDemo {
  view scene: spatial(Plane, scale: 1 m -> 40 px, y: up) {
    axes; grid
    marker(A)
    arrow(u, from: A) {
      on drag head as h { propose u = h - A }
    }
    arrow(w, from: mid)
    arrow(sum, from: A)
    label(length)
  }
  observe { values = (sum, length, B) live }
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

Each variant adds one line to the model (in the `param` or `derived` block). Each MUST be rejected before execution.

| ID | Added line | Expected diagnostic |
|---|---|---|
| RP-07.D1 | `bad: Point = A + B` | MK-E04 (point plus point) |
| RP-07.D2 | `bad: Point = 2 * A` | MK-E04 (scaling a point) |
| RP-07.D3 | `bad: Vector<Length> = u + (3, 0)` | MK-E03 (bare non-zero literals with a dimensioned vector) |
| RP-07.D4 | `vel: Vector<Velocity> = (1 m/s, 0 m/s)` and `bad: Vector<Length> = u + vel` | MK-E01 (adding length and velocity) |
| RP-07.D5 | `space Board = euclidean(2)`, `z: Vector<Board, Length> = (1 m, 0 m)` and `bad: Vector<Length> = u + z` | MK-E05 (values from different spaces) |
| RP-07.D6 | `ok: Vector<Length> = u + (0, 0)`, and `ok2: Vector<Length> = u + 0` | accepted: the literal `0` adopts the required dimension, and stands for the zero vector (MK-3.8, D-030) |

D6 is a positive control for the literal rule: it MUST be accepted.

## History

- 2026-09-29 written.
- 2026-09-30 programs rewritten in the working syntax (D-028); corrections from the syntax study applied; D6 extended with the zero-vector literal (D-030).
- 2026-09-30 observation `state` renamed `values`: `state` is a reserved word of the model language (D-040), found by the text parser.
- 2026-09-30 E1 to E5 confirmed through the presentation prototype.
