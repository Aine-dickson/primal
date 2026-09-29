# RP-01 Projectile, no drag

## Purpose

A ball launched from the ground with no air resistance. Its flight time, range and apex are known exactly, so the program checks flows (MK section 14), crossing events and their direction (MK-15.4), event location (RC section 7), the mode switch at landing (MK-14.12), and observations at event instants (PK-3.4).

The model is shared with RP-02 (drag) and RP-08 (narrated lesson).

## Model

**sketch (non-binding, D-006)**

```text
space Plane : Euclidean 2

object Projectile {
  param g      : Acceleration  = 9.81 m/s^2
  param k      : Quantity<1/L> = 0              // quadratic drag coefficient
  param speed  : Quantity<L/T> = 20 m/s
  param angle  : Real          = 45 deg          // dimensionless (D-021)
  constraint speed > 0 m/s : reject
  constraint 0 deg < angle and angle < 90 deg : reject
  constraint k >= 0 : reject

  state pos    : Point<Plane>       = origin
  state vel    : Vector<Plane, L/T> = speed * (cos(angle), sin(angle))
  state flying : Boolean            = true       // discrete state: the mode

  flow der(pos)  = vel if flying else (0 m/s, 0 m/s)
  flow der(vel) += (0 m/s^2, -g) if flying else (0 m/s^2, 0 m/s^2)   // gravity
  flow der(vel) += -k * |vel| * vel                                   // drag

  event apex   on falling(vel.y) { }
  event landed on falling(pos.y) { set flying = false; set vel = (0 m/s, 0 m/s) }
}
```

Notes:

- `speed` and `angle` are parameters, so a lesson or learner can change the launch (D-023); the initial velocity is computed from them (MK-6.7).
- `apex` has no operations; it exists so that the apex can be observed (PK-3.1 `on(E)`).
- At launch `pos.y = 0` and rising. That is not a falling crossing (RC-7.3), so `landed` does not occur at `t0`.
- After landing, `flying = false`: both flows are zero and the ball stays at the landing point. The drag contribution is zero because `vel` was reset to zero.

## Observations

```text
presentation ProjectileChecks for Projectile {
  observe t_apex  = elapsed on(apex)
  observe h_apex  = pos.y   on(apex)
  observe t_land  = elapsed on(landed)
  observe range   = pos.x   on(landed)
  observe y_land  = pos.y   on(landed)
  observe at_rest = pos     at(t0 + 5 s)
  observe events  = event log over(t0, t0 + 5 s)
}
```

## Cases

| Case | Overrides | End |
|---|---|---|
| `A-45deg` | none (`speed = 20 m/s`, `angle = 45 deg`, `k = 0`) | `t_end = t0 + 5 s` |
| `B-60deg` | `angle = 60 deg` | `t_end = t0 + 5 s` |

## Expected results

Exact values: flight time `T = 2 v sin(θ) / g`, range `R = v^2 sin(2θ) / g`, apex time `T / 2`, apex height `H = (v sin θ)^2 / (2g)`.

**Tolerances.** With constant acceleration the exact solution is a polynomial of degree 2, which `dopri5` (order 5) reproduces exactly apart from rounding. The remaining error is event location: the event instant lies at most `ε_t = 1e-10 s` beyond the root (RC-7.6). Time tolerances are therefore `1e-9 s` (10 `ε_t`, allowing rounding), and position tolerances are the speed times that (under `1e-8 m`).

### Case A-45deg

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-01.E1 | `t_land` | 2.88320807823261 s | abs 1e-9 s | analytic | fixed |
| RP-01.E2 | `range` | 40.7747196738022 m | abs 1e-8 m | analytic | fixed |
| RP-01.E3 | `y_land` | in `[-1e-8 m, 0 m]` | - | bound | fixed |
| RP-01.E4 | `t_apex` | 1.44160403911630 s | abs 1e-9 s | analytic | fixed |
| RP-01.E5 | `h_apex` | 10.1936799184506 m | abs 1e-8 m | analytic | fixed |
| RP-01.E6 | `at_rest` | equal to `pos on(landed)`, bit for bit | exact | behavior | fixed |
| RP-01.E7 | `events` | exactly `apex` then `landed`, one each; nothing at `t0` | exact | behavior | fixed |

E3 checks RC-7.6: the landing state is committed at the far end of the location bracket, so `pos.y` is at or just below zero (at most `|vel.y| · ε_t`, about `1.4e-9 m`).

### Case B-60deg

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-01.E8 | `t_land` | 3.53119430697019 s | abs 1e-9 s | analytic | fixed |
| RP-01.E9 | `range` | 35.3119430697019 m | abs 1e-8 m | analytic | fixed |

## Diagnostic variants

Each variant changes one line of the model. Each MUST be rejected before execution.

| ID | Change | Expected diagnostic |
|---|---|---|
| RP-01.D1 | `event landed on pos.y <= 0 m { ... }` written as a continuous trigger and stored as a level condition (no lowering) | MK-E13 (level condition as continuous trigger) |
| RP-01.D2 | `flow der(vel) += (1, -g)` | MK-E03 (bare non-zero literal with a dimensioned quantity) |
| RP-01.D3 | add `flow der(vel) = (0 m/s^2, -g)` alongside the contributions | MK-E10 (target both defined and contributed to) |
| RP-01.D4 | `flow der(pos) = vel if pos.y > 0 m else (0 m/s, 0 m/s)` | MK-E12 (flow condition depends on continuous state) |
| RP-01.D5 | handler `landed` also does `set g = 0 m/s^2` | MK-E08 (handler sets a parameter) |

D1 applies to the IR: a surface syntax may lower `y <= 0` to `falling(y)` (MK-15.6), in which case the variant is written directly in IR form.

## History

- 2026-09-29 written.
