# RP-05 Spring-mass

## Purpose

A mass on an ideal spring, released from rest at 0.1 m from equilibrium. The trajectory is known in closed form, so the program checks the trajectory itself at a given time, the period, energy conservation, and the two equation roles: a `display` equation with a dimension check, and a `check` equation whose residual is reported (MK section 11).

## Model

**working syntax (D-028, non-binding)**

```text
model SpringMass {
  param {
    m:  Mass            = 1 kg      where m > 0 kg
    k:  Quantity<M/T^2> = 4 N/m     where k > 0 N/m
    x0: Length          = 0.1 m
  }
  state {
    x: Length   = x0                  // displacement from equilibrium
    v: Velocity = 0
  }
  derived {
    energy: Energy = 0.5 * m * v^2 + 0.5 * k * x^2
    T:      Time   = 2π * sqrt(m / k)
  }

  flow {
    der(x) = v
    der(v) = -(k / m) * x
  }

  equation period_law:   T == 2π * sqrt(m / k)
  equation conservation: energy == 0.5 * k * x0^2 checked within 2e-5 J

  event pass on rising(x)             // once per period
}
```

## Observations

```text
presentation SpringChecks for SpringMass {
  observe {
    x_10      = x                           at t0 + 10 s
    passes    = elapsed                     on pass
    energy    = energy                      every 0.01 s
    residuals = diagnostics of conservation over [t0, t_end]
  }
}
```

## Cases

| Case | Configuration | End |
|---|---|---|
| `A-dopri5` | defaults | `t_end = t0 + 100 s` |
| `B-rk4` | `solver = rk4`, `h = 0.01 s` | `t_end = t0 + 100 s` |

## Expected results

Exact solution: `x(t) = x0 cos(ω t)`, `ω = sqrt(k/m) = 2 /s`, period `T = π s`, energy `E0 = ½ k x0^2 = 0.02 J`. The first rising crossing of `x` is at `3T/4 = 2.35619449019234 s`.

Energy drift measured with plain floating-point implementations over 100 s (`tools/refvals.py`): `rk4` at `h = 0.01 s` at most `8.9e-9` relative; a simple `dopri5` at default tolerances at most `5.0e-5` relative.

### Case A-dopri5

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-05.E1 | `x_10` | 0.0408082061813392 m (`0.1 cos 20`) | abs 1e-5 m | analytic | fixed |
| RP-05.E2 | `passes[1]` | 2.35619449019234 s | rel 1e-5 | analytic | fixed |
| RP-05.E3 | `passes[k+1] - passes[k]`, every `k` | 3.14159265358979 s | rel 1e-5 | analytic | fixed |
| RP-05.E4 | number of passes | 32 (the last at 99.745 s) | exact | behavior | fixed |
| RP-05.E5 | `energy` | max relative deviation from 0.02 J below `1e-3` | - | bound | fixed |
| RP-05.E6 | `residuals` | none reported (every residual within `2e-5 J`) | - | behavior | fixed |

### Case B-rk4

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-05.E7 | `x_10` | 0.0408082061813392 m | abs 1e-7 m | analytic | fixed |
| RP-05.E8 | `energy` | max relative deviation below `1e-7` | - | bound | fixed |

### Both cases

| ID | Observation | Expected | Kind | Status |
|---|---|---|---|---|
| RP-05.E9 | `period_law` | accepted: both sides have dimension T (MK-11.3); no runtime effect | behavior | fixed |
| RP-05.E10 | derived `T` | 3.14159265358979 s, exact in binary64 up to one rounding | analytic | fixed |

## Diagnostic variants

| ID | Change | Expected diagnostic |
|---|---|---|
| RP-05.D1 | `equation period_law: T == 2π * sqrt(k / m)` | MK-E01 (sides have dimensions T and 1/T) |
| RP-05.D2 | `energy: Energy = 0.5 * m * v^2 + 0.5 * k * x` in the `derived` block | MK-E01 (adding energy and force) |
| RP-05.D3 | `flow der(v) = -(k / m) * x + 1` | MK-E03 (bare non-zero literal added to an acceleration) |

## History

- 2026-09-29 written.
- 2026-09-30 programs rewritten in the working syntax (D-028); corrections from the syntax study applied.
- 2026-09-30 provisional tolerances confirmed by the Rust prototype and made fixed. Measured: `dopri5` `x_10` abs error 2.1e-7, worst period rel error 5.2e-8, energy drift 4.9e-5; `rk4` 2.4e-9, 1.3e-9, 1.8e-8.
