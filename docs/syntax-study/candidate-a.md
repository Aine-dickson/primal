# Candidate A: Flat Keyword Statements

Candidate notation for the syntax study (D-006). Non-binding.

## 1. Design

**Principle:** every statement starts with a keyword that names what kind of element it is. A reader can classify any single line without looking at its surroundings. Behaviors (flows, events) are listed flat inside the model; there is no grouping construct. Braces delimit blocks; newlines or `;` separate statements.

### 1.1 Model statements

| Form | Kernel element |
|---|---|
| `space Plane = euclidean(2)` | space (MK-4.1) |
| `model Name { ... }` | root object type (MK-7.2); `object` for contained types |
| `const x: T = e` | binding, role constant |
| `param x: T = e [where cond]` | binding, role parameter; `where` attaches a `reject` constraint (MK-12.4) |
| `input x: T` | binding, role input |
| `state x: T = e` | binding, role continuous state; `e` is the initial definition |
| `discrete x: T = e` | binding, role discrete state |
| `derived x: T = e` | derived binding |
| `derived f(x: A): B = e` | derived binding whose value is a function (may read bindings, see comparison section 7) |
| `fn f(x: A): B = e` | pure function (MK-10.3): reads only its parameters and constants |
| `flow der(x) = e` / `flow der(x) += e` | defining flow / contribution (MK-14.7) |
| `event E on <trigger> [if cond] [{ ops }] [zeno stop \| zeno settle { ops }]` | event (MK-15) |
| `equation N: lhs == rhs [checked within tol]` | equation, role display, or check with tolerance (MK-11) |
| `constraint [N:] cond [within tol] [policy reject\|report\|stop]` | constraint (MK-12) |

Modifiers after a declaration: `intervenable` (state, D-023), `private` (MK-6.1), `symbol "v"`, `unit deg` (display metadata, MK-6.1).

Triggers: `rising(g)`, `falling(g)`, `crossing(g)`, `at τ`, `every Δ [from τ0]`, `on E`, `on start`, `on input(i)`, `on request [(payload: T)]`. There is no level-style trigger; a Boolean condition can only appear after `if` (enabling condition, MK-15.5).

Operations in handlers: `set x = e`, `contribute x += e`, `create`, `destroy`, `connect`, `disconnect`, `emit E(payload)`.

### 1.2 Presentation statements

| Form | Kernel element |
|---|---|
| `presentation Name for Model { ... }` | presentation (PK-2) |
| `observe name = expr <schedule>` | observation; schedules `live`, `every Δ`, `at τ`, `on E [microstep n]`, `over τ1 .. τ2` (PK-3.1) |
| `view name = spatial(Space, scale: 1 m -> 40 px, y: up)` / `plot(x: a .. b, y: c .. d)` | view (PK-7) |
| `show rep(...) [in view] [as name]` | representation with its projection (PK-5, PK-6) |
| `on drag name[.part] as p { propose x = e }` | interaction: declared inverse of a projection (PK-5.6, PK-10.5) |
| `permit learner ...` | interaction permissions (PK-10.4) |
| `timeline { scene S { beat B { actions } } }` | explanation timeline (PK-9) |

### 1.3 Runs and expectations

| Form | Element |
|---|---|
| `run Name of Model with Presentation { param x = e; config k = v; until τ; expect ... }` | run configuration (RC section 3, RC section 16) with expectations (PK-4) |
| `expect obs == value within tol` / `within rel r` / `expect obs in [a, b]` | expectation |

### 1.4 R-46 distinctions

| Distinction | Mark in A |
|---|---|
| Declaration | role keyword and `name: Type` |
| Definition | `fn f(x) = e`, `derived f(x) = e` |
| Initialization | `= e` in a `param`, `state` or `discrete` declaration |
| Assignment | `set x = e` (only in handlers, `intervene`, runs) |
| Derivation | `derived x = e` |
| Equation | `equation N: a == b` |
| Constraint | `constraint`, or `where` on a parameter |
| Evolution | `flow der(x) = e` and `flow der(x) += e` |
| Observation / projection / representation | `observe` / the `show` statement's arguments / the representation kind in `show` |
| Input / interaction / operation | `input` binding / `on drag ... propose` / `set` and the other handler operations |

`=` always introduces a value for something being declared or set; `==` always asserts equality (equations, constraints, expectations, Boolean tests).

---

## RP-01 Projectile, no drag

```text
space Plane = euclidean(2)

model Projectile {
  param g:     Acceleration  = 9.81 m/s^2
  param k:     Quantity<1/L> = 0         where k >= 0              // quadratic drag coefficient
  param speed: Velocity      = 20 m/s    where speed > 0 m/s
  param angle: Angle         = 45 deg    where 0 deg < angle < 90 deg

  state    pos:    Point<Plane>            = Plane.origin
  state    vel:    Vector<Plane, Velocity> = speed * (cos(angle), sin(angle))
  discrete flying: Boolean                 = true

  flow der(pos)  = if flying then vel else (0, 0)
  flow der(vel) += if flying then (0, -g) else (0, 0)       // gravity
  flow der(vel) += -k * |vel| * vel                         // drag

  event apex   on falling(vel.y)
  event landed on falling(pos.y) {
    set flying = false
    set vel = (0, 0)
  }
}

presentation ProjectileChecks for Projectile {
  observe t_apex  = elapsed   on apex
  observe h_apex  = pos.y     on apex
  observe t_land  = elapsed   on landed
  observe range   = pos.x     on landed
  observe y_land  = pos.y     on landed
  observe at_rest = pos       at t0 + 5 s
  observe events  = event_log over t0 .. t0 + 5 s
}
```

```cases
run A_45deg of Projectile with ProjectileChecks {
  until t0 + 5 s
  expect t_land == 2.88320807823261 s within 1e-9 s
  expect range  == 40.7747196738022 m within 1e-8 m
  expect y_land in [-1e-8 m, 0 m]
  expect t_apex == 1.44160403911630 s within 1e-9 s
  expect h_apex == 10.1936799184506 m within 1e-8 m
  expect at_rest == (pos on landed) exactly
  expect events == [apex, landed]
}

run B_60deg of Projectile with ProjectileChecks {
  param angle = 60 deg
  until t0 + 5 s
  expect t_land == 3.53119430697019 s within 1e-9 s
  expect range  == 35.3119430697019 m within 1e-8 m
}
```

## RP-02 Projectile with drag

The RP-01 model, unchanged.

```text
presentation DragChecks for Projectile {
  observe t_apex  = elapsed on apex
  observe h_apex  = pos.y   on apex
  observe t_land  = elapsed on landed
  observe range   = pos.x   on landed
  observe v_land  = vel     on landed microstep 0                               // before the reset
  observe energy  = 0.5 * |vel|^2 + g * (pos - Plane.origin).y  every 0.01 s    // per unit mass
  observe balance = der(vel) - ((0, -g) - k * |vel| * vel)       every 0.01 s
}
```

```cases
run A_default of Projectile with DragChecks {
  param k = 0.01 /m
  until t0 + 5 s
  expect t_land == 2.67328857282834 s within rel 1e-5
  expect range  == 31.3229266146783 m within rel 1e-5
  expect v_land == (9.76677238322685 m/s, -12.3148244403059 m/s) within rel 1e-5
  expect (vel on landed) == (0, 0) exactly
}

run B_tight of Projectile with DragChecks {
  param k = 0.01 /m
  config rtol = 1e-10, atol = 1e-12
  until t0 + 5 s
  expect t_land == 2.67328857282834 s within rel 1e-8
}
```

## RP-03 Bouncing ball

```text
model BouncingBall {
  param g:  Acceleration = 9.81 m/s^2
  param h0: Length       = 1 m     where h0 > 0 m
  param e:  Real         = 0.8     where 0 <= e < 1          // restitution

  state    y:       Length   = h0
  state    v:       Velocity = 0
  discrete resting: Boolean  = false

  flow der(y) = v
  flow der(v) = if resting then 0 else -g

  event bounce on falling(y) {
    set v = -e * v
  } zeno settle {
    set y = 0 m
    set v = 0 m/s
    set resting = true
  }

  event top on falling(v)                                    // apex of each flight
}

presentation BounceChecks for BouncingBall {
  observe bounce_times = elapsed on bounce
  observe top_heights  = y       on top
  observe zeno         = event_log of bounce where zeno_applied
  observe final        = (y, v, resting) at t0 + 6 s
  observe late_events  = event_log over t0 + 4.1 s .. t0 + 6 s
}
```

```cases
run A_settle of BouncingBall with BounceChecks {
  until t0 + 6 s
  expect bounce_times[1] == 0.451523640985731 s within 1e-8 s
  expect top_heights[1]  == 0.64 m within 1e-8 m
  expect final == (0 m, 0 m/s, true) exactly
  expect late_events == []
}

run C_invalid_e of BouncingBall {
  param e = 1.2
  expect initialization fails
}
```

Case `B-stop` changes the model (the Zeno policy), not the run configuration. A writes it as a copy of the model with `zeno stop` in place of the `settle` clause (see comparison section 7, S-4).

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

  flow der(θ) = ω
  flow der(ω) = -(g / L) * sin(θ)

  derived bob:    Point<Plane> = pivot + L * (sin(θ), -cos(θ))
  derived energy: Energy       = 0.5 * m * (L * ω)^2 + m * g * L * (1 - cos(θ))

  constraint rod: |bob - pivot| == L within 1e-9 m policy report

  event upswing on rising(θ)             // once per period
}

presentation PendulumChecks for Pendulum {
  observe theta_start = θ           at t0
  observe upswings    = elapsed     on upswing
  observe energy      = energy      every 0.01 s
  observe diagnostics = diagnostics over t0 .. t_end
}
```

```cases
run A_dopri5 of Pendulum with PendulumChecks {
  until t0 + 100 s
  expect upswings[1] == 1.50741947047395 s within rel 1e-5
  expect diagnostics == []
}

run B_rk4 of Pendulum with PendulumChecks {
  config solver = rk4, h = 0.01 s
  until t0 + 100 s
}

run C_rk4_no_step of Pendulum {
  config solver = rk4
  expect configuration rejected
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

  flow der(x) = v
  flow der(v) = -(k / m) * x

  derived energy: Energy = 0.5 * m * v^2 + 0.5 * k * x^2
  derived T:      Time   = 2π * sqrt(m / k)

  equation period_law:   T == 2π * sqrt(m / k)
  equation conservation: energy == 0.5 * k * x0^2 checked within 2e-5 J

  event pass on rising(x)               // once per period
}

presentation SpringChecks for SpringMass {
  observe x_10      = x                            at t0 + 10 s
  observe passes    = elapsed                      on pass
  observe energy    = energy                       every 0.01 s
  observe residuals = diagnostics of conservation  over t0 .. t_end
}
```

```cases
run A_dopri5 of SpringMass with SpringChecks {
  until t0 + 100 s
  expect x_10 == 0.0408082061813392 m within 1e-5 m
  expect residuals == []
}

run B_rk4 of SpringMass with SpringChecks {
  config solver = rk4, h = 0.01 s
  until t0 + 100 s
  expect x_10 == 0.0408082061813392 m within 1e-7 m
}
```

## RP-06 Function plot with draggable parameter

```text
model QuadraticDemo {
  param a: Real = 1   where -5 <= a <= 5
  derived f(x: Real): Real = a * x^2
}

presentation QuadraticPlot for QuadraticDemo {
  view plot = plot(x: -3 .. 3, y: -5 .. 20)
  show function_graph(f)      in plot
  show marker(at: (1, f(1)))  in plot as handle
  show slider(a, range: -5 .. 5, step: 0.1)
  show formula(f, live: true)

  on drag handle as p { propose a = p.y / 1^2 }             // x fixed at 1

  observe a_now = a    live
  observe f2    = f(2) live
  observe log   = intervention_log
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
  view scene = spatial(Plane, scale: 1 m -> 40 px, y: up)
  show axes, grid          in scene
  show marker(A)           in scene
  show arrow(u, from: A)   in scene as u_arrow
  show arrow(w, from: mid) in scene
  show arrow(sum, from: A) in scene
  show label(length)       in scene

  on drag u_arrow.head as h { propose u = h - A }

  observe state = (sum, length, B) live
}
```

## RP-08 Narrated projectile lesson

The RP-01 model with one addition (D-027), and display symbols for the formula:

```text
  param speed: Velocity = 20 m/s  where speed > 0 m/s             symbol "v"
  param angle: Angle    = 45 deg  where 0 deg < angle < 90 deg    symbol "θ"

  event relaunch on request {
    set pos    = Plane.origin
    set vel    = speed * (cos(angle), sin(angle))
    set flying = true
  }
```

```text
presentation ProjectileLesson for Projectile {
  view scene = spatial(Plane, scale: 1 m -> 10 px, y: up)
  show axes in scene
  permit learner timeline_controls, zoom, pan

  observe landings = (elapsed, pos.x) on landed
  observe log      = event_log

  timeline {
    scene launch {
      beat b1 {
        show marker(pos) in scene as ball
        show arrow(vel, from: pos) in scene
        narrate "A ball is launched at 45 degrees." for 4 s
      }
      beat b2 { run rate 1; wait until landed }
      beat b3 {
        hold
        highlight ball
        narrate "It lands here. Why this distance?" for 3 s
      }
      beat b4 {
        seek t0
        show formula("R", speed^2 * sin(2 * angle) / g, live: true)
        narrate "Watch the horizontal speed." for 3 s
      }
      beat b5 {
        run rate 0.5
        show arrow((vel.x, 0), from: pos) in scene
        wait until landed
      }
    }
    scene try_it {
      beat b6 { hold; narrate "Choose your own angle." for 3 s }
      beat b7 {
        explore limit 60 s keep angle {
          show slider(angle, range: 10 deg .. 80 deg)
        } fallback {
          intervene set angle = 60 deg
          wait 3 s
        }
      }
      beat b8 { request relaunch; run rate 1; wait until landed }
      beat b9 { narrate "Compare the distance with the first launch." for 3 s }
    }
  }
}
```

```cases
run A_keep of Projectile with ProjectileLesson {
  learner at 23.0 s set slider angle = 60 deg
  learner at 25.0 s continue
  expect start of b3 == 6.88320807823261 s within 1e-8 s
  expect end of b8   == 28.5311943069702 s within 1e-8 s
}
```

---

## 2. Observations from writing

- **Line-level clarity holds throughout.** Every line in the eight programs is classifiable by its first word. The distinction between `state` and `discrete` is visible where the sketch notation hid it behind a comment (comparison S-2).
- **Mode noise.** Conditional flows repeat the mode test (`if flying then ... else (0, 0)`) in every flow of a moded model. In RP-01 two of three flows carry it; in a model with three modes it would dominate.
- **`where` reads well** for parameter ranges and lowers to exactly one kernel constraint. The constraint's identity is attached to the parameter, which the printer can recover from the IR without guessing (comparison section 5).
- **Beat actions start together** (PK-9.1), but a brace block of statements reads as a sequence to a programmer. `run rate 1; wait until landed` means the same either way; a beat such as `seek t0` followed by `show` would not always (comparison S-8).
- **Names for representations** (`as ball`, `as handle`, `as u_arrow`) are needed wherever a timeline or interaction refers to a representation. A flat list makes these names necessary; a nested form (candidate C) needs them less.
- **Run blocks mix three kinds of setting** (parameter overrides, solver configuration, learner script). The `param` and `config` keywords keep them apart; without them a parameter named `h` would collide with the solver step `h`.
