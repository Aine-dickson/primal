# 6. Checks and Tests

A model can state facts that must hold (constraints) and laws it expects to satisfy (equations). A program can state what a run must produce (cases with expectations). This chapter covers all three. Tests are how a program is shown to be right: a case with an independently computed expected value is a claim that anyone can rerun.

## Constraints

```prismal
constraint rod: |bob - pivot| == L within 1e-9 m policy report
```

A **constraint** is a condition on the model's values, checked by the runtime; it is never enforced by moving values (D-020). Its **policy** says what a violation does:

| Policy | A violation |
|---|---|
| `reject` | refuses the change: a run whose initial state violates it fails to initialize; a control, drag or lesson action that would violate it is not applied |
| `report` | is recorded as a diagnostic; the run continues |
| `stop` | ends the run with a diagnostic |

`within tol` gives an equality a tolerance; it applies to `==` only. A parameter's range, `in [0, 1)` or `where x > 0`, is a `reject` constraint on that parameter.

## Equations

```prismal
equation period_law: T == 2π * sqrt(m / k)
equation conservation: energy == 0.5 * k * x0^2 checked within 2e-5 J
```

An **equation** states a relation between the model's values. It does not compute anything (models are causal in v1, D-007): it is shown in presentations and, with `checked within tol`, checked by the runtime at every step, with a diagnostic when it fails. `der(x)` may appear in an equation, to check a law of motion against the flows.

## A program

A skydiver falling with air resistance proportional to the square of the speed. The speed approaches the terminal speed `vt = sqrt(m g / c)`; the closed-form solution is `v(t) = vt tanh(g t / vt)` and the distance fallen `d(t) = (vt² / g) ln(cosh(g t / vt))`.

```text
model Skydiver {
  param {
    m:  Mass          = 80 kg       where m > 0 kg
    g:  Acceleration  = 9.81 m/s^2
    c:  Quantity<M/L> = 0.25 kg/m   where c > 0 kg/m
    v0: Velocity      = 0
  }
  state {
    fallen: Length   = 0 m
    v:      Velocity = v0           // downward speed
  }
  derived {
    vt: Velocity = sqrt(m * g / c)
  }
  flow {
    der(fallen) = v
    der(v)      = g - (c / m) * v^2
  }

  event nearly_terminal on rising(v - 0.9 * vt)

  equation motion: der(v) == g - (c / m) * v^2 checked within 1e-9 m/s^2
  constraint subterminal: v <= vt policy reject
}

presentation SkydiverChecks for Skydiver {
  observe {
    v5          = v       at t0 + 5 s
    v10         = v       at t0 + 10 s
    d10         = fallen  at t0 + 10 s
    ninety      = elapsed on nearly_terminal
    speed_there = v       on nearly_terminal
    events      = event_log over [t0, t0 + 20 s]
    problems    = diagnostics over [t0, t_end]
  }
}
```

- `event nearly_terminal on rising(v - 0.9 * vt)` has no handler: it exists to be observed.
- `diagnostics over [t0, t_end]` observes every diagnostic of the run (failed equation checks, reported constraints); `t_end` is the run's end.

## Cases and expectations

```cases
run fall of Skydiver with SkydiverChecks {
  until t0 + 20 s
  expect {
    v5  == 39.4514324565817 m/s within rel 1e-5
    v10 == 52.7496070171611 m/s within rel 1e-5
    d10 == 347.981946430556 m within rel 1e-5
    ninety[1] == 8.40839389456238 s within rel 1e-5
    speed_there in [50.4 m/s, 50.5 m/s]
    (v on nearly_terminal) == 50.4257077292922 m/s within 1e-6 m/s
    events   == [nearly_terminal]
    problems == []
  }
}

run tight of Skydiver with SkydiverChecks {
  config { rtol = 1e-10; atol = 1e-12 }
  until t0 + 10 s
  expect { d10 == 347.981946430556 m within rel 1e-9 }
}

run too_fast of Skydiver {
  param { v0 = 60 m/s }
  expect { initialization fails }
}

run heavier of Skydiver with SkydiverChecks {
  param { m = 100 kg }
  until t0 + 20 s
  expect { ninety[1] == 9.40087016491784 s within rel 1e-5 }
}
```

The forms of an expectation:

| Form | Passes when |
|---|---|
| `a == b within tol` | the difference is at most `tol` (same dimension as `a`) |
| `a == b within rel r` | the relative difference is at most `r` |
| `a == b exactly` | the values are identical |
| `a in [lo, hi]` | the value lies in the interval |
| `obs[k] == ...` | the `k`-th value of a series observation (from 1) |
| `(e on E) == ...` | `e` at the first occurrence of `E`; `(e on E microstep 0)` just before the event's operations |
| `events == [a, b]` | an event log is exactly these occurrences, in order; `== []` for none |
| `start of b3 == ...`, `end of b3 == ...` | a beat's start or end in presentation time (chapter 8) |
| `initialization fails` | the run cannot start (for example a `reject` constraint or a range is violated) |
| `configuration rejected` | the run configuration is invalid (for example `rk4` without a step) |

A tolerance should reflect what is being checked. `within rel 1e-5` matches the default solver; a tight configuration earns a tight tolerance. An event's instant is located exactly on the computed solution, so it is as accurate as the solution: here the 90 % instant is off by about `6e-6 s` with the default solver, and `within 1e-6 s` would fail. An expected value is written with enough digits for its tolerance, and comes from a closed form, a published value, or an independent computation, never from the program's own output.

## Reading a failure

`cargo run -p prismal-present --example cases -- skydiver.prismal` prints each case and expectation:

```sh
run fall
  expect 1: pass
  ...
  expect 4: FAIL: got 8.408388316452704e0, expected 8.408393894562380e0 (difference 5.5781096754969894e-6)
```

Expectations are numbered in the order written. This failure is the one described above: an event time checked with `within 1e-6 s`.

The player's Cases tab shows the same report.

## Exercises

1. Add a parachute: a discrete `open: Boolean = false`, an event `deploy on at t0 + 12 s` that sets it, and a drag coefficient `(if open then c_open else c)`. What happens to the constraint `subterminal`? Change its policy to `report` and observe `problems`.
2. For the `Oscillator` of chapter 4, add `equation conservation: energy == 0.5 * k * x0^2 checked within 1e-5 J` and a case with `c = 0 N*s/m` expecting `problems == []`. Then a case with the default damping: what does the diagnostic say?
3. Write a case for the `Tank` of chapter 5 that checks `(level on full) == 1 m within 1e-9 m`.

<details>
<summary>A solution to exercise 3</summary>

```text
model Tank {
  param {
    inflow: Quantity<L/T> = 0.02 m/s
    drain:  Quantity<L/T> = 0.01 m/s
    high:   Length = 1 m
    low:    Length = 0.5 m
  }
  state { level: Length = 0.2 m }
  discrete { pumping: Boolean = true }
  flow { der(level) = (if pumping then inflow else 0) - drain }
  event full  on rising(level - high) { set pumping = false }
  event empty on falling(level - low) { set pumping = true }
}

presentation TankChecks for Tank {
  observe { at_full = level on full }
}
```

```cases
run level_at_full of Tank with TankChecks {
  until t0 + 100 s
  expect {
    (level on full) == 1 m within 1e-9 m
    at_full[1] == 1 m within 1e-9 m
  }
}
```

</details>
