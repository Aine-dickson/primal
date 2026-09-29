# RP-02 Projectile with drag

## Purpose

The RP-01 model with quadratic air resistance switched on. Two behaviors contribute to the same derivative `der(vel)` (gravity and drag), combined by the default sum (MK-14.8, MK-14.9, D-005). There is no closed-form solution, so results are compared with a high-precision reference solution. The program also reads a microstep inside an event instant (RC-4.3): the velocity just before the landing reset.

## Model

The RP-01 model, unchanged. Drag is enabled by the parameter `k`.

## Observations

```text
presentation DragChecks for Projectile {
  observe t_apex  = elapsed on(apex)
  observe h_apex  = pos.y   on(apex)
  observe t_land  = elapsed on(landed)
  observe range   = pos.x   on(landed)
  observe v_land  = vel     on(landed, microstep 0)   // before the reset
  observe energy  = 0.5 * |vel|^2 + g * (pos - origin).y   every(0.01 s)   // per unit mass
  observe balance = der(vel) - ((0 m/s^2, -g) - k * |vel| * vel)  every(0.01 s)
}
```

`balance` uses MK-11.5: `der(vel)` in an expression evaluates to the combined flow. Its value is the difference between the sum the runtime formed and the sum written out by hand.

## Cases

| Case | Overrides | End |
|---|---|---|
| `A-default` | `k = 0.01 /m` | `t_end = t0 + 5 s` |
| `B-tight` | `k = 0.01 /m`; `rtol = 1e-10`, `atol = 1e-12` | `t_end = t0 + 5 s` |

`k = 0.01 /m` corresponds roughly to a baseball-sized ball in air. Other parameters are the RP-01 defaults (`speed = 20 m/s`, `angle = 45 deg`).

## Reference solution

From `tools/refvals.py`: classical RK4 in 50-digit decimal arithmetic at steps `1e-3 s` and `5e-4 s`, with the landing and apex located by secant iteration on the last step, combined by Richardson extrapolation. The two step sizes agree to about `1e-15` in every value below.

| Quantity | Value |
|---|---|
| landing time | 2.67328857282834 s |
| range | 31.3229266146783 m |
| landing velocity | (9.76677238322685, -12.3148244403059) m/s |
| apex time | 1.29589657291982 s |
| apex height | 8.78272286661045 m |

## Expected results

**Tolerances.** Unlike RP-01, `dopri5` is not exact here; its error is governed by `rtol`. With `rtol = 1e-6` over under three seconds, a relative error of `1e-5` leaves a margin of ten. With `rtol = 1e-10`, `1e-8` does the same. These are provisional until the prototype runs the program.

### Case A-default

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-02.E1 | `t_land` | 2.67328857282834 s | rel 1e-5 | reference | provisional |
| RP-02.E2 | `range` | 31.3229266146783 m | rel 1e-5 | reference | provisional |
| RP-02.E3 | `v_land` | (9.76677238322685, -12.3148244403059) m/s | rel 1e-5 per component | reference | provisional |
| RP-02.E4 | `t_apex` | 1.29589657291982 s | rel 1e-5 | reference | provisional |
| RP-02.E5 | `h_apex` | 8.78272286661045 m | rel 1e-5 | reference | provisional |
| RP-02.E6 | `energy` during flight | strictly decreasing from sample to sample | - | behavior | fixed |
| RP-02.E7 | `balance` during flight | every component within `1e-12 m/s^2` of zero | - | bound | fixed |
| RP-02.E8 | `range` compared with RP-01.E2 | strictly less than 40.7747196738022 m | - | behavior | fixed |
| RP-02.E9 | `vel on(landed)` (final microstep) | `(0 m/s, 0 m/s)` exactly | exact | behavior | fixed |

E7 checks D-005: the two contributions are summed, not replaced by one another. Its tolerance is rounding only, since both sides evaluate the same terms (the order of summation is fixed by MK-14.10).

E6 is robust to solver error: drag removes energy at the rate `k |vel|^3`, which is at least about `0.1 J/kg` per `0.01 s` sample (the slowest point is the apex, at a little over `11 m/s`), while `rtol = 1e-6` perturbs the energy by well under `1e-3 J/kg`.

E3 and E9 together check RC-4.3: microstep 0 of the landing instant holds the pre-reset velocity, and the final microstep holds the reset value.

### Case B-tight

| ID | Observation | Expected | Tolerance | Kind | Status |
|---|---|---|---|---|---|
| RP-02.E10 | `t_land` | 2.67328857282834 s | rel 1e-8 | reference | provisional |
| RP-02.E11 | `range` | 31.3229266146783 m | rel 1e-8 | reference | provisional |
| RP-02.E12 | `h_apex` | 8.78272286661045 m | rel 1e-8 | reference | provisional |

## History

- 2026-09-29 written.
