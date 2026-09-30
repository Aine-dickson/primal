# 5. Events and Modes

Flows describe smooth change. **Events** describe sudden change: a ball bounces, a pump switches off, a counter ticks. An event has a **trigger** (when it happens) and a **handler** (what it does). **Discrete state** holds values that change only at events, such as a mode (`pumping`) or a count.

## A pumped tank

A tank drains steadily. A pump fills it; a controller switches the pump off when the level reaches `high` and on again when it falls to `low`.

```text
model Tank {
  param {
    inflow: Quantity<L/T> = 0.02 m/s    // level rise from the pump
    drain:  Quantity<L/T> = 0.01 m/s    // level fall from the outlet
    high:   Length = 1 m
    low:    Length = 0.5 m
    h0:     Length = 0.2 m
  }
  state {
    level: Length = h0
  }
  discrete {
    pumping: Boolean = true
    alarms:  Real    = 0
  }

  flow {
    der(level) = (if pumping then inflow else 0) - drain
  }

  event full  on rising(level - high)  { set pumping = false }
  event empty on falling(level - low)  { set pumping = true }
  event alarm on full                  { set alarms = alarms + 1 }
}
```

- `discrete { pumping: Boolean = true }` is **discrete state**: constant between events, changed only by handlers. A Boolean discrete variable is a **mode**; flows can depend on it with `if ... then ... else`.
- `event full on rising(level - high) { ... }`: the trigger `rising(g)` happens when `g` crosses zero upward, here when `level` rises through `high`. `falling(g)` is a downward crossing, `crossing(g)` either way. The runtime locates the crossing instant precisely, not at the next step.
- The handler `{ set pumping = false }` is a list of **operations**. `set` replaces a value. All operations of a handler read the values from before the event, and commit together.
- `event alarm on full` is triggered by another event: it happens right after `full`, at the same time, in the next **microstep** (D-041). A handler can also signal an event explicitly with `emit E`. Events that cause events form a cascade at one instant; the runtime limits its length.
- A continuous guard is always a crossing. `level >= high` would be a level condition, which cannot trigger an event by itself (D-004): a level condition may only enable a trigger, as in `event full on rising(level - high) if pumping { ... }`.

With the default values the level rises at `0.01 m/s` from `0.2 m`, reaches `1 m` at 80 s, falls at `0.01 m/s` to `0.5 m` at 130 s, rises again to `1 m` at 180 s, and so on.

```text
presentation TankLab for Tank {
  view levels: plot(x: [0 s, 400 s], y: [0 m, 1.2 m]) {
    series_plot(level every 1 s)
  }
  view tank: plot(x: [-1, 1], y: [0 m, 1.2 m]) {
    marker(at: (0, level)) as surface
  }
  panel controls {
    slider(inflow, range: [0.011 m/s, 0.05 m/s])
    slider(high, range: [0.6 m, 1.2 m])
    label(alarms)
  }
  observe {
    fulls   = elapsed on full
    empties = elapsed on empty
    log     = event_log over [t0, t0 + 200 s]
    count   = alarms at t0 + 200 s
  }
}
```

- `fulls = elapsed on full` records `elapsed` at each occurrence of `full`: a series. `fulls[1]` in a test is its first value.
- `event_log over [t0, t0 + 200 s]` is the list of event occurrences in that interval, in order.

```cases
run cycles of Tank with TankLab {
  until t0 + 200 s
  expect {
    fulls[1]   == 80 s  within 1e-9 s
    empties[1] == 130 s within 1e-9 s
    fulls[2]   == 180 s within 1e-9 s
    log   == [full, alarm, empty, full, alarm]
    count == 2 exactly
  }
}

run strong_pump of Tank with TankLab {
  param { inflow = 0.05 m/s }
  until t0 + 200 s
  expect {
    fulls[1]   == 20 s within 1e-9 s      // 0.8 m at 0.04 m/s
    empties[1] == 70 s within 1e-9 s      // 0.5 m at 0.01 m/s
  }
}
```

## Time events

Events can also happen at given times:

- `event e on at t0 + 5 s { ... }` happens once, at that instant.
- `event e on every 0.5 s { ... }` happens at `t0`, `t0 + 0.5 s`, `t0 + 1 s`, ... (each instant computed as `t0 + k Δ`, so no rounding accumulates). `on every 0.5 s from t0 + 1 s` starts later.
- `event e on start { ... }` happens once when the run starts.

Every trigger follows the word `on`: `on rising(g)`, `on every 0.5 s`, `on full`.

```text
model Metronome {
  param { period: Time = 0.5 s  where period > 0 s }
  state { phase: Real = 0 }
  discrete { beats: Real = 0 }
  flow { der(phase) = 1 / period }
  event tick on every 0.5 s { set beats = beats + 1 }
}

presentation MetronomeChecks for Metronome {
  observe {
    after2 = beats at t0 + 2.25 s
    ticks  = elapsed on tick
  }
}
```

```cases
run two_seconds of Metronome with MetronomeChecks {
  until t0 + 2.25 s
  expect {
    after2   == 5 exactly               // at 0, 0.5, 1, 1.5 and 2 s
    ticks[3] == 1 s within 1e-12 s
  }
}
```

## Repeating events and Zeno behavior

An event whose handler changes what its own trigger depends on can happen again and again. A bouncing ball whose bounces lose energy bounces infinitely often in a finite time (a **Zeno** behavior). Such an event must say what happens at the limit:

```text
model Ball {
  param {
    g: Acceleration = 9.81 m/s^2
    e: Real = 0.8  in [0, 1)            // restitution
  }
  state {
    y: Length   = 1 m
    v: Velocity = 0
  }
  discrete { resting: Boolean = false }
  flow {
    der(y) = v
    der(v) = if resting then 0 else -g
  }
  event bounce on falling(y) {
    set v = -e * v
  } zeno settle {
    set y = 0 m
    set v = 0 m/s
    set resting = true
  }
}

presentation BallChecks for Ball {
  observe {
    first = elapsed on bounce
    final = (y, v, resting) at t0 + 6 s
  }
}
```

```cases
run settles of Ball with BallChecks {
  until t0 + 6 s
  expect {
    first[1] == 0.451523640985731 s within 1e-8 s    // sqrt(2 * 1 m / g)
    final == (0 m, 0 m/s, true) exactly
  }
}
```

- `zeno settle { ... }` gives the operations applied when the bounces accumulate: the ball comes to rest. `zeno stop` ends the run with a diagnostic instead.
- The compiler decides which events can repeat this way. Leaving out the policy is an error:

```error
// error: MK-E16
model Ball {
  param { e: Real = 0.8 }
  state { y: Length = 1 m; v: Velocity = 0 }
  flow { der(y) = v; der(v) = -9.81 m/s^2 }
  event bounce on falling(y) { set v = -e * v }
}
```

The tank's events are not repeating in this sense: after `full` switches the pump off the level falls, away from `high`. The compiler follows the flows with `pumping` set to `false` to see this (D-038).

RP-03 in the reference programs is the complete bouncing ball, with its expected bounce times and the instant of the Zeno limit.

## Exercises

1. Add an overflow valve to the tank: an event `spill on rising(level - 1.1 m)` that sets the level back to `1.1 m`. When can it happen?
2. Give the tank a leak that starts at 150 s: a discrete `leaking: Boolean = false`, an event `at t0 + 150 s` that sets it, and a flow that drains twice as fast while it is set. Compute when the level next reaches `low`, and test it.
3. Write a model of a light that switches on for 2 s every 5 s, using two time events and a mode. (`on` is a reserved word, so the mode needs another name.)

<details>
<summary>A solution to exercise 3</summary>

```text
model Blinker {
  state { clock: Real = 0 }
  discrete { lit: Boolean = false }
  flow { der(clock) = 1 / 1 s }
  event light_on  on every 5 s                  { set lit = true }
  event light_off on every 5 s from t0 + 2 s    { set lit = false }
}

presentation BlinkerChecks for Blinker {
  observe {
    at1 = lit at t0 + 1 s
    at3 = lit at t0 + 3 s
    at6 = lit at t0 + 6 s
  }
}
```

```cases
run blinks of Blinker with BlinkerChecks {
  until t0 + 7 s
  expect {
    at1 == true exactly
    at3 == false exactly
    at6 == true exactly
  }
}
```

</details>
