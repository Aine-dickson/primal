# RP-03 Bouncing ball

## Purpose

A ball dropped from 1 m bounces with restitution `e = 0.8`. Bounce times form a geometric series that converges at a finite time, so the event accumulates (Zeno behavior). The program checks resets (MK-15.9), modes (MK-14.12), self-retriggering detection (MK-15.11), the Zeno policy (MK-15.12, RC section 9), sign references after resets (RC-8.4), and the guard rule at a zero start (RC-7.2). It is the program in which the exploration record's example was defective (audit F-02).

## Model

**sketch (non-binding, D-006)**

```text
object BouncingBall {
  param g  : Acceleration = 9.81 m/s^2
  param h0 : Length       = 1 m
  param e  : Real         = 0.8               // restitution
  constraint 0 <= e and e < 1 : reject
  constraint h0 > 0 m : reject

  state y       : Length        = h0
  state v       : Quantity<L/T> = 0
  state resting : Boolean       = false      // discrete state: the mode

  flow der(y) = v
  flow der(v) = -g if not resting else 0 m/s^2

  event bounce on falling(y) {
    set v = -e * v
  } zeno settle { set y = 0 m; set v = 0 m/s; set resting = true }

  event top on falling(v) { }                  // apex of each flight, for observation
}
```

Notes:

- `bounce` writes `v`, and its guard `y` depends on `v` through the flow, so it is self-retriggering and must declare a Zeno policy.
- At `t0`, `v = 0` and then becomes negative. The guard of `top` has no non-zero sign before that, so no crossing occurs at the start (RC-7.2).
- After a bounce, the handler has flipped `v`, and sign references are re-taken from the post-event state (RC-8.4), so the flip itself is not a crossing of `top`'s guard.

## Observations

```text
presentation BounceChecks for BouncingBall {
  observe bounce_times = elapsed on(bounce)
  observe top_heights  = y       on(top)
  observe zeno         = event log entries of bounce with zeno applied
  observe final        = (y, v, resting) at(t0 + 6 s)
  observe late_events  = event log over(t0 + 4.1 s, t0 + 6 s)
}
```

## Cases

| Case | Overrides | End |
|---|---|---|
| `A-settle` | none | `t_end = t0 + 6 s` |
| `B-stop` | `bounce` declared with `zeno stop` instead of `settle`; failure policy `stop` | `t_end = t0 + 6 s` |
| `C-invalid-e` | `e = 1.2` | - |

## Expected results

Exact values, with `t1 = sqrt(2 h0 / g)` (first impact) and `v1 = sqrt(2 g h0)` (first impact speed):

- bounce `n` occurs at `t_n = t1 + (2 v1 / g) · e (1 - e^(n-1)) / (1 - e)`;
- the flight after bounce `n` reaches height `e^(2n) · h0`;
- the bounce times converge to `t_∞ = t1 (1 + e) / (1 - e) = 4.06371276887158 s`;
- the interval before bounce `n` is `2 e^(n-1) v1 / g`. It first falls below the default `ε_zeno = 1e-6 s` (RC-9.2) at bounce 63, at `t = 4.06370922604679 s`, `3.5e-6 s` before `t_∞`.

**Tolerances.** Free flight is a polynomial of degree 2, integrated exactly by `dopri5` apart from rounding. Each located bounce is at most `ε_t` late; the ball then restarts slightly below zero, which lengthens the next flight by at most about `ε_t / e`. After `n` bounces the accumulated error is below about `2.3 n ε_t`: `2.3e-9 s` at bounce 10. The time tolerance `1e-8 s` covers the first ten bounces.

### Case A-settle

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-03.E1 | `bounce_times[1]` | 0.451523640985731 s | abs 1e-8 s | analytic | fixed |
| RP-03.E2 | `bounce_times[2]` | 1.17396146656290 s | abs 1e-8 s | analytic | fixed |
| RP-03.E3 | `bounce_times[3]` | 1.75191172702464 s | abs 1e-8 s | analytic | fixed |
| RP-03.E4 | `bounce_times[4]` | 2.21427193539402 s | abs 1e-8 s | analytic | fixed |
| RP-03.E5 | `bounce_times[5]` | 2.58416010208954 s | abs 1e-8 s | analytic | fixed |
| RP-03.E6 | `bounce_times[10]` | 3.57889295102044 s | abs 1e-8 s | analytic | fixed |
| RP-03.E7 | `top_heights[1..3]` | 0.64 m, 0.4096 m, 0.262144 m | abs 1e-8 m | analytic | fixed |
| RP-03.E8 | no `top` at `t0` | the first `top` is after `bounce_times[1]` | exact | behavior | fixed |
| RP-03.E9 | `zeno` | exactly one entry, at bounce number 61 to 65 | - | behavior | provisional |
| RP-03.E10 | time of the `zeno` entry | before `t_∞` and within `1e-4 s` of it | - | bound | provisional |
| RP-03.E11 | `final` | `(0 m, 0 m/s, true)` exactly | exact | behavior | fixed |
| RP-03.E12 | `late_events` | empty: no event after the settle | exact | behavior | fixed |

Bounce times 6 to 9 are in the output of `tools/refvals.py` for implementations that check more of the series.

E9 and E10 are provisional because detection depends on the location error of the last few bounces relative to their shrinking intervals. The exact prediction is bounce 63 at `4.06370922604679 s`; the bounds allow for accumulated location error.

### Case B-stop

| ID | Observation | Expected | Kind | Status |
|---|---|---|---|---|
| RP-03.E13 | run status | `stopped`, with a Zeno diagnostic naming `bounce` (RC-9.3, RC-10.3) | behavior | fixed |
| RP-03.E14 | last committed state | the state before the detected occurrence; no partial commit (RC-10.1) | behavior | fixed |

### Case C-invalid-e

| ID | Observation | Expected | Kind | Status |
|---|---|---|---|---|
| RP-03.E15 | initialization | fails; the run does not start; the diagnostic names the constraint `0 <= e and e < 1` and the value `1.2` (RC-5.2) | behavior | fixed |

## Diagnostic variants

| ID | Change | Expected diagnostic |
|---|---|---|
| RP-03.D1 | remove the `zeno settle { ... }` clause | MK-E16 (self-retriggering crossing event without a Zeno policy) |
| RP-03.D2 | `flow der(v) = -g if y > 0 m else 0 m/s^2` | MK-E12 (flow condition on continuous state) |
| RP-03.D3 | handler `bounce` does `set v = -e * v; set v = 0 m/s` | MK-E19 (two sets on one target in one handler) |

## History

- 2026-09-29 written.
