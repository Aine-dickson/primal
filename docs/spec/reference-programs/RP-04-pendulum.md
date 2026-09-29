# RP-04 Pendulum

## Purpose

A simple pendulum of length 1 m released from 10 degrees. It checks angles as dimensionless quantities with `deg` units (D-021), a constraint that holds by construction and is only checked (D-020, MK-12.2), period measurement from events, the difference between the exact period and the small-angle formula, and energy drift under the two required solvers (RC-6.3).

## Model

**working syntax (D-028, non-binding)**

```text
space Plane = euclidean(2)

model Pendulum in Plane {
  param {
    g:     Acceleration = 9.81 m/s^2
    L:     Length       = 1 m      where L > 0 m
    m:     Mass         = 1 kg     where m > 0 kg
    pivot: Point        = origin
    θ0:    Angle        = 10 deg
  }
  state {
    θ: Angle         = θ0                 // angle from the downward vertical
    ω: Quantity<1/T> = 0
  }
  derived {
    bob:    Point  = pivot + L * (sin(θ), -cos(θ))
    energy: Energy = 0.5 * m * (L * ω)^2 + m * g * L * (1 - cos(θ))
  }

  flow {
    der(θ) = ω
    der(ω) = -(g / L) * sin(θ)
  }

  constraint rod: |bob - pivot| == L within 1e-9 m policy report

  event upswing on rising(θ)              // once per period, moving toward positive θ
}
```

## Observations

```text
presentation PendulumChecks for Pendulum {
  observe {
    theta_start = θ           at t0
    upswings    = elapsed     on upswing
    energy      = energy      every 0.01 s
    diagnostics = diagnostics over [t0, t_end]
  }
}
```

## Cases

| Case | Configuration | End |
|---|---|---|
| `A-dopri5` | defaults (`dopri5`, `rtol = 1e-6`, `atol = 1e-9`) | `t_end = t0 + 100 s` |
| `B-rk4` | `solver = rk4`, `h = 0.01 s` | `t_end = t0 + 100 s` |
| `C-rk4-no-step` | `solver = rk4`, no `h` | - |

## Expected results

- Exact period for amplitude `θ0`: `T = 4 sqrt(L/g) K(k)`, `k = sin(θ0/2)`, where `K` is the complete elliptic integral of the first kind (computed by the arithmetic-geometric mean in `tools/refvals.py`): `T = 2.00989262729860 s`.
- Small-angle period `T0 = 2π sqrt(L/g) = 2.00606668071065 s`. `T / T0 - 1 = 1.907e-3`: at 10 degrees the small-angle formula is 0.19 % short. D-012 phrases this program's result as "close to `2π√(L/g)`"; the suite checks the exact period, and checks that the difference from `T0` is the expected 0.19 %, which is itself a teaching point.
- The pendulum starts at `+θ0` moving toward negative `θ`, so the first upswing (rising zero crossing) is at `3T/4 = 1.50741947047395 s`.
- Energy is conserved exactly by the model. Drift is numerical. Measured with plain floating-point implementations over 100 s (`tools/refvals.py`): `rk4` with `h = 0.01 s` drifts by at most `1.3e-7` relative; a simple `dopri5` at default tolerances by at most `7.8e-5` relative. The bounds below leave a margin of about ten.

### Case A-dopri5

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-04.E1 | `theta_start` | 0.174532925199433 (= 10 π/180) | exact in binary64 | analytic | fixed |
| RP-04.E2 | `upswings[1]` | 1.50741947047395 s | rel 1e-5 | analytic | fixed |
| RP-04.E3 | `upswings[k+1] - upswings[k]`, every `k` in the run | 2.00989262729860 s | rel 1e-5 | analytic | fixed |
| RP-04.E4 | measured period against `T0` | `period / T0 - 1` in `[1.85e-3, 1.96e-3]` | - | bound | fixed |
| RP-04.E5 | `energy` | max relative deviation from its initial value below `1e-3` | - | bound | fixed |
| RP-04.E6 | `diagnostics` | none: the rod constraint never reports | exact | behavior | fixed |
| RP-04.E7 | number of upswings | 50 (`floor((100 s - 3T/4) / T) + 1`; the last at 99.992 s) | exact | behavior | fixed |

### Case B-rk4

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-04.E8 | `upswings[k+1] - upswings[k]` | 2.00989262729860 s | rel 1e-6 | analytic | fixed |
| RP-04.E9 | `energy` | max relative deviation below `1e-6` | - | bound | fixed |
| RP-04.E10 | `energy` compared with case A | case B's max deviation is smaller than case A's | - | behavior | fixed |

E10 records the teaching point of the per-solver drift in D-012: the choice of solver visibly changes how well energy is conserved, and the author can see it (MK-1.2, RC-16.1).

### Case C-rk4-no-step

| ID | Observation | Expected | Kind | Status |
|---|---|---|---|---|
| RP-04.E11 | run configuration | rejected before the run starts: a fixed-step solver requires `h` (RC section 16) | behavior | fixed |

## Diagnostic variants

| ID | Change | Expected diagnostic |
|---|---|---|
| RP-04.D1 | `θ: Length = 10 cm` in the `state` block | MK-E02 (dimensioned argument to `sin`) |
| RP-04.D2 | `bob: Point = pivot + (sin(θ), -cos(θ))` (no `L`) | MK-E01 (the tuple is expected to be a displacement, D-032, but its components are dimensionless) |

## History

- 2026-09-29 written.
- 2026-09-30 programs rewritten in the working syntax (D-028); corrections from the syntax study applied (`Plane` declared; `Plane` is the default space). RP-04.D2 now expects MK-E01 instead of MK-E04: under D-032 the tuple takes the expected type `Vector<Plane, L>`, so the error is found in its components.
- 2026-09-30 provisional tolerances confirmed by the Rust prototype and made fixed. Measured: `dopri5` first upswing rel error 5.4e-8, worst period 2.0e-7, energy drift 7.7e-5 (1645 steps); `rk4` 8.0e-9, 7.9e-9, 1.35e-7.
