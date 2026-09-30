# Candidate C: Nested Structure

Candidate notation for the syntax study (D-006). Non-binding.

## 1. Design

**Principle:** the source is shaped like the thing it describes. Declarations are written as in candidate A (a keyword on every line). What changes is grouping:

- **Modes.** Behavior that holds only in some discrete situations is written inside named modes, as in a hybrid automaton: each mode lists its flows and events, and `enter M` switches mode. Flows outside any mode hold always.
- **Processes.** A named group of behaviors (MK-14.1) such as `gravity` or `drag`, so that a presentation can refer to it and a reader sees what belongs together.
- **View trees.** Representations are written inside the view that shows them, and interactions inside the representation they act on.
- **Explicit sequencing in beats.** A beat's actions start together (PK-9.1); a sequence inside a beat is written `sequence { ... }`.

### 1.1 Additions to candidate A's model forms

| Form | Lowers to |
|---|---|
| `modes name { M1 { ... } M2 { ... } }` | a discrete state `name` of an enumeration type `{M1, M2}`, initial value the first mode |
| a flow `der(x) = e` inside mode `M` | a term of the single defining flow of `x`: `der(x) = if name == M then e else ...`; in modes that do not define `der(x)`, the rate is `0` (the state is held) |
| a contribution `der(x) += e` inside `M` | the contribution `der(x) += if name == M then e else 0` |
| an event inside `M` | the same event with the enabling condition `name == M` (MK-15.5) |
| `enter M` in a handler | `set name = M` |
| `process P { ... }` | a process (MK-14.1) with identity and name `P` whose behaviors are the enclosed flows and events; mode `continuous` if it contains flows |

Everything else in the model is candidate A: `param`, `state`, `discrete`, `derived`, `flow`, `event`, `equation`, `constraint`, `where`, `set`.

### 1.2 Presentation forms

| Form | Element |
|---|---|
| `view name: spatial(...) { reps }` | view with its representations |
| `rep(...) { on drag [part] as p { propose x = e } }` | representation with its declared inverse |
| `panel name { reps }` | a view without a coordinate system, for controls and formulas |
| `observe { name = expr schedule; ... }` | observations |
| `beat B { actions }`, `sequence { actions }` | concurrent actions; explicit sequence |

### 1.3 R-46 distinctions

As candidate A, with two differences:

| Distinction | Difference from A |
|---|---|
| Evolution | flows inside a mode are conditional without writing the condition; the condition is the mode block |
| Assignment | `enter M` is an assignment to the mode's discrete state; the state itself is not declared with `discrete` |
| Representation / interaction | an interaction is written inside its representation, so the target needs no name |

---

## RP-01 Projectile, no drag

```text
space Plane = euclidean(2)

model Projectile {
  param g:     Acceleration  = 9.81 m/s^2
  param k:     Quantity<1/L> = 0         where k >= 0              // quadratic drag coefficient
  param speed: Velocity      = 20 m/s    where speed > 0 m/s
  param angle: Angle         = 45 deg    where 0 deg < angle < 90 deg

  state pos: Point<Plane>            = Plane.origin
  state vel: Vector<Plane, Velocity> = speed * (cos(angle), sin(angle))

  modes motion {
    flying {
      flow der(pos) = vel
      process gravity { flow der(vel) += (0, -g) }
      process drag    { flow der(vel) += -k * |vel| * vel }

      event apex   on falling(vel.y)
      event landed on falling(pos.y) {
        set vel = (0, 0)
        enter grounded
      }
    }
    grounded { }                       // no flows: pos and vel are held
  }
}

presentation ProjectileChecks for Projectile {
  observe {
    t_apex  = elapsed   on apex
    h_apex  = pos.y     on apex
    t_land  = elapsed   on landed
    range   = pos.x     on landed
    y_land  = pos.y     on landed
    at_rest = pos       at t0 + 5 s
    events  = event_log over t0 .. t0 + 5 s
  }
}
```

```cases
run A_45deg of Projectile with ProjectileChecks {
  until t0 + 5 s
  expect {
    t_land == 2.88320807823261 s within 1e-9 s
    range  == 40.7747196738022 m within 1e-8 m
    y_land in [-1e-8 m, 0 m]
    t_apex == 1.44160403911630 s within 1e-9 s
    h_apex == 10.1936799184506 m within 1e-8 m
    at_rest == (pos on landed) exactly
    events == [apex, landed]
  }
}

run B_60deg of Projectile with ProjectileChecks {
  param angle = 60 deg
  until t0 + 5 s
  expect {
    t_land == 3.53119430697019 s within 1e-9 s
    range  == 35.3119430697019 m within 1e-8 m
  }
}
```

The `drag` process is now active only in `flying`. In RP-01 this changes nothing (after landing `vel` is zero, so the drag term is zero), but it is a different model from the flat form, and C invites it.

## RP-02 Projectile with drag

The RP-01 model, unchanged.

```text
presentation DragChecks for Projectile {
  observe {
    t_apex  = elapsed on apex
    h_apex  = pos.y   on apex
    t_land  = elapsed on landed
    range   = pos.x   on landed
    v_land  = vel     on landed microstep 0                                // before the reset
    energy  = 0.5 * |vel|^2 + g * (pos - Plane.origin).y   every 0.01 s    // per unit mass
    balance = der(vel) - ((0, -g) - k * |vel| * vel)        every 0.01 s
  }
}
```

```cases
run A_default of Projectile with DragChecks {
  param k = 0.01 /m
  until t0 + 5 s
  expect {
    t_land == 2.67328857282834 s within rel 1e-5
    range  == 31.3229266146783 m within rel 1e-5
    v_land == (9.76677238322685 m/s, -12.3148244403059 m/s) within rel 1e-5
    (vel on landed) == (0, 0) exactly
  }
}

run B_tight of Projectile with DragChecks {
  param k = 0.01 /m
  config { rtol = 1e-10; atol = 1e-12 }
  until t0 + 5 s
  expect { t_land == 2.67328857282834 s within rel 1e-8 }
}
```

With the processes named, a presentation can also observe each contribution separately (`gravity.der(vel)`, `drag.der(vel)`), which the flat form cannot name.

## RP-03 Bouncing ball

```text
model BouncingBall {
  param g:  Acceleration = 9.81 m/s^2
  param h0: Length       = 1 m     where h0 > 0 m
  param e:  Real         = 0.8     where 0 <= e < 1          // restitution

  state y: Length   = h0
  state v: Velocity = 0

  modes motion {
    bouncing {
      flow der(y) = v
      flow der(v) = -g

      event bounce on falling(y) {
        set v = -e * v
      } zeno settle {
        set y = 0 m
        set v = 0 m/s
        enter resting
      }
      event top on falling(v)                                // apex of each flight
    }
    resting { }                                              // y and v held
  }
}

presentation BounceChecks for BouncingBall {
  observe {
    bounce_times = elapsed on bounce
    top_heights  = y       on top
    zeno         = event_log of bounce where zeno_applied
    final        = (y, v, motion) at t0 + 6 s
    late_events  = event_log over t0 + 4.1 s .. t0 + 6 s
  }
}
```

```cases
run A_settle of BouncingBall with BounceChecks {
  until t0 + 6 s
  expect {
    bounce_times[1] == 0.451523640985731 s within 1e-8 s
    top_heights[1]  == 0.64 m within 1e-8 m
    final == (0 m, 0 m/s, resting) exactly
    late_events == []
  }
}

run C_invalid_e of BouncingBall {
  param e = 1.2
  expect { initialization fails }
}
```

The observation `final` reads `motion` (an enumeration) where RP-03 reads `resting` (a Boolean). The expectation changes form, not meaning. Case `B-stop` is a model variant (comparison S-4).

## RP-04 Pendulum

```text
space Plane = euclidean(2)

model Pendulum {
  param g:     Acceleration = 9.81 m/s^2
  param L:     Length       = 1 m        where L > 0 m
  param m:     Mass         = 1 kg       where m > 0 kg
  param pivot: Point<Plane> = Plane.origin
  param θ0:    Angle        = 10 deg

  state θ: Angle         = θ0            // from the downward vertical
  state ω: Quantity<1/T> = 0

  process swing {
    flow der(θ) = ω
    flow der(ω) = -(g / L) * sin(θ)
    event upswing on rising(θ)           // once per period
  }

  derived bob:    Point<Plane> = pivot + L * (sin(θ), -cos(θ))
  derived energy: Energy       = 0.5 * m * (L * ω)^2 + m * g * L * (1 - cos(θ))

  constraint rod: |bob - pivot| == L within 1e-9 m policy report
}

presentation PendulumChecks for Pendulum {
  observe {
    theta_start = θ           at t0
    upswings    = elapsed     on upswing
    energy      = energy      every 0.01 s
    diagnostics = diagnostics over t0 .. t_end
  }
}
```

```cases
run A_dopri5 of Pendulum with PendulumChecks {
  until t0 + 100 s
  expect {
    upswings[1] == 1.50741947047395 s within rel 1e-5
    diagnostics == []
  }
}

run B_rk4 of Pendulum with PendulumChecks {
  config { solver = rk4; h = 0.01 s }
  until t0 + 100 s
}

run C_rk4_no_step of Pendulum {
  config { solver = rk4 }
  expect { configuration rejected }
}
```

## RP-05 Spring-mass

```text
model SpringMass {
  param m:  Mass            = 1 kg      where m > 0 kg
  param k:  Quantity<M/T^2> = 4 N/m     where k > 0 N/m
  param x0: Length          = 0.1 m

  state x: Length   = x0                // displacement from equilibrium
  state v: Velocity = 0

  process oscillation {
    flow der(x) = v
    flow der(v) = -(k / m) * x
    event pass on rising(x)             // once per period
  }

  derived energy: Energy = 0.5 * m * v^2 + 0.5 * k * x^2
  derived T:      Time   = 2π * sqrt(m / k)

  equation period_law:   T == 2π * sqrt(m / k)
  equation conservation: energy == 0.5 * k * x0^2 checked within 2e-5 J
}

presentation SpringChecks for SpringMass {
  observe {
    x_10      = x                            at t0 + 10 s
    passes    = elapsed                      on pass
    energy    = energy                       every 0.01 s
    residuals = diagnostics of conservation  over t0 .. t_end
  }
}
```

```cases
run A_dopri5 of SpringMass with SpringChecks {
  until t0 + 100 s
  expect {
    x_10 == 0.0408082061813392 m within 1e-5 m
    residuals == []
  }
}

run B_rk4 of SpringMass with SpringChecks {
  config { solver = rk4; h = 0.01 s }
  until t0 + 100 s
  expect { x_10 == 0.0408082061813392 m within 1e-7 m }
}
```

## RP-06 Function plot with draggable parameter

```text
model QuadraticDemo {
  param a: Real = 1   where -5 <= a <= 5
  derived f(x: Real): Real = a * x^2
}

presentation QuadraticPlot for QuadraticDemo {
  view plot: plot(x: -3 .. 3, y: -5 .. 20) {
    function_graph(f)
    marker(at: (1, f(1))) {
      on drag as p { propose a = p.y / 1^2 }                // x fixed at 1
    }
  }
  panel controls {
    slider(a, range: -5 .. 5, step: 0.1)
    formula(f, live: true)
  }
  observe {
    a_now = a    live
    f2    = f(2) live
    log   = intervention_log
  }
}
```

## RP-07 Vector addition

```text
space Plane = euclidean(2)

model VectorDemo {
  param A: Point<Plane>          = Plane.origin + (1 m, 2 m)
  param u: Vector<Plane, Length> = (3 m, 0 m)
  param w: Vector<Plane, Length> = (0 m, 2 m)

  derived sum:    Vector<Plane, Length> = u + w
  derived length: Length                = |sum|
  derived B:      Point<Plane>          = A + sum
  derived mid:    Point<Plane>          = A + u              // where w starts
}

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
  observe { state = (sum, length, B) live }
}
```

## RP-08 Narrated projectile lesson

The RP-01 model with one addition (D-027), outside the modes block, and display symbols:

```text
  param speed: Velocity = 20 m/s  where speed > 0 m/s             symbol "v"
  param angle: Angle    = 45 deg  where 0 deg < angle < 90 deg    symbol "θ"

  event relaunch on request {
    set pos = Plane.origin
    set vel = speed * (cos(angle), sin(angle))
    enter flying
  }
```

```text
presentation ProjectileLesson for Projectile {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    axes
  }
  permit learner { timeline_controls; zoom; pan }
  observe {
    landings = (elapsed, pos.x) on landed
    log      = event_log
  }

  timeline {
    scene launch {
      beat b1 {
        in scene { marker(pos) as ball; arrow(vel, from: pos) }
        narrate("A ball is launched at 45 degrees.", 4 s)
      }
      beat b2 { run(rate: 1); wait_until(landed) }
      beat b3 {
        hold
        highlight(ball)
        narrate("It lands here. Why this distance?", 3 s)
      }
      beat b4 {
        sequence {
          seek(t0)
          show formula("R", speed^2 * sin(2 * angle) / g, live: true)
        }
        narrate("Watch the horizontal speed.", 3 s)
      }
      beat b5 {
        run(rate: 0.5)
        in scene { arrow((vel.x, 0), from: pos) }
        wait_until(landed)
      }
    }
    scene try_it {
      beat b6 { hold; narrate("Choose your own angle.", 3 s) }
      beat b7 {
        explore(limit: 60 s, keep: angle) {
          slider(angle, range: 10 deg .. 80 deg)
        } fallback {
          sequence { intervene { set angle = 60 deg }; wait(3 s) }
        }
      }
      beat b8 {
        sequence { request(relaunch); run(rate: 1) }
        wait_until(landed)
      }
      beat b9 { narrate("Compare the distance with the first launch.", 3 s) }
    }
  }
}
```

```cases
run A_keep of Projectile with ProjectileLesson {
  learner {
    at 23.0 s: set slider angle = 60 deg
    at 25.0 s: continue
  }
  expect {
    start of b3 == 6.88320807823261 s within 1e-8 s
    end of b8   == 28.5311943069702 s within 1e-8 s
  }
}
```

---

## 2. Observations from writing

- **Modes remove the repeated conditions.** RP-03 reads `flow der(v) = -g` inside `bouncing`, instead of `if resting then 0 else -g`. The bouncing ball and the projectile read as the physical story: a flight phase and a resting phase.
- **Modes change what "defined once" means.** In A, two `flow der(v) = ...` lines violate MK-14.7 (a target is defined by exactly one flow; see comparison S-7). In C, one definition per mode is normal and they merge into one kernel flow. The rule "a state without a flow in the active mode is held" is implicit; forgetting a flow in one mode does not produce an error, it freezes the state.
- **The mode variable disappears.** `motion` is discrete state, but it is never declared with `discrete`, and observations and expectations must use it (`final` in RP-03). Only one mode variable is natural; two independent mode variables (a ball that is both `flying`/`grounded` and `charged`/`neutral`) need two `modes` blocks whose flows combine, and the reader must work out which flows are active in which combination.
- **Moving drag into `flying` changed the model.** The equivalence in RP-01 is an accident of `vel` being reset to zero. C makes it easy to put behavior inside a mode without asking whether it belongs there.
- **Processes pay off when named in presentations** (observing the drag contribution alone) and when a law has several parts. For single flows (`gravity`, `drag`) they add a line of structure each.
- **View trees are clear** and remove most representation names (RP-07 needs none; RP-08 still needs `ball` for `highlight`). Interactions inside their representation make the target obvious.
- **Explicit `sequence`** made the timeline's concurrency visible: in `b4` the formula must appear after the seek, which A and B leave to the reader's assumption (comparison S-8).
- **Deeper nesting.** RP-08 reaches seven levels of braces in the fallback of `b7` (presentation, timeline, scene, beat, fallback, sequence, intervene); A reaches five.
