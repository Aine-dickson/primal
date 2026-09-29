# Candidate B: Mathematical Sections

Candidate notation for the syntax study (D-006). Non-binding.

## 1. Design

**Principle:** a model reads like a worked problem statement. Roles are given by section headings (`parameters`, `state`, `dynamics`, ...) rather than by a keyword on each line. Within a section, lines use mathematical forms: `x' = v` for a rate of change, `:=` for a definition, `←` for a replacement, interval notation for parameter ranges, `falls through` for crossings. Indentation delimits blocks. Unicode spellings are the display form; every one has an ASCII spelling.

### 1.1 Model sections

| Section or form | Kernel element |
|---|---|
| `space Plane = Euclidean 2` | space (MK-4.1) |
| `model Name [in Space]` | root object type; `in Space` makes `Point`, `Vector D` and `origin` refer to that space |
| `constants` | lines `x = e` are bindings with role constant |
| `parameters` | lines `x [: T] = e [∈ interval]` are parameters; the interval lowers to a `reject` constraint |
| `inputs` | lines `x : T` are inputs |
| `state` | lines `x [: T] = e` are continuous state with initial definition `e` |
| `discrete` | lines `x [: T] = e` are discrete state |
| `definitions` | lines `x := e` are derived bindings; `f(x : A) := e` defines a function-valued derived binding |
| `dynamics` | lines `x' = e` define a flow; `x' += e` contribute (MK-14.7) |
| `events` | `name: when g falls through h` (lowers to `falling(g - h)`), `rises through`, `crosses`; `at τ`, `every Δ`, `after E`, `on start`, `on request`; handler lines `x ← e` indented below; `zeno stop` or `zeno settle` with its own indented lines |
| `equations` | lines `name: a = b [checked ± tol]` (MK-11) |
| `constraints` | lines `name: cond [± tol][, report \| stop]` (MK-12); default policy `report` |

Types may be omitted where the initial value fixes them: `g = 9.81 m/s²` declares an `Acceleration`, `θ0 = 10°` declares an `Angle` with display unit `deg`. A literal `0` fixes nothing, so `v : L/T = 0` needs its type. Dimension expressions are types (`k : 1/L`).

ASCII spellings: `'` is already ASCII; `←` is `<-`, `∈` is `in`, `↦` is `->`, `±` is `+-`, `≤` is `<=`, `−` is `-`, `²` is `^2`, `°` is `deg`, `t₀` is `t0`, `π` is `pi`.

### 1.2 Presentation sections

| Section or form | Kernel element |
|---|---|
| `presentation Name of Model` | presentation (PK-2) |
| `observe` | lines `name = expr <schedule>`; schedules `live`, `every Δ`, `at τ`, `at each E`, `just before E` (microstep 0), `during [τ1, τ2]` |
| `views` | `name: spatial Space, 1 m ↦ 40 px, y up` or `name: plot x ∈ [a, b], y ∈ [c, d]`, with the representations of that view indented below |
| `controls` | controls not placed in a view |
| `interaction` | `drag rep[.part] to p ⇒ x ← e` (declared inverse, PK-5.6) |
| `learner may` | permissions (PK-10.4) |
| `timeline` | `scene` lines, beats as `name: actions`, continuation lines indented |

### 1.3 Runs and expectations

`run Name: Model [with x = e, ...], observed by P, until τ [, solved with k = v, ...]`, followed by an indented `expect` section with lines `obs ≈ value ± tol` (absolute, or `± r %` relative) or `obs ∈ [a, b]`.

### 1.4 R-46 distinctions

| Distinction | Mark in B |
|---|---|
| Declaration | the section a name appears in, and `name : Type` |
| Definition | `f(x) := e` in `definitions` |
| Initialization | `x = e` in `parameters`, `state`, `discrete` |
| Assignment | `x ← e` (handlers, interventions, drags) |
| Derivation | `x := e` in `definitions` |
| Equation | `a = b` in `equations` |
| Constraint | interval `∈` on a parameter, or a line in `constraints` |
| Evolution | `x' = e`, `x' += e` in `dynamics` |
| Observation / projection / representation | `observe` / representation lines under a view / the representation kind |
| Input / interaction / operation | `inputs` / `interaction` / `←` |

`=` has four meanings depending on the section: initial value, default, equation, equality in a constraint. `:=`, `'` and `←` are unambiguous anywhere.

---

## RP-01 Projectile, no drag

```text
space Plane = Euclidean 2

model Projectile in Plane
  parameters
    g         = 9.81 m/s²
    k : 1/L   = 0            ∈ [0, ∞)            // quadratic drag coefficient
    speed     = 20 m/s       ∈ (0 m/s, ∞)
    angle     = 45°          ∈ (0°, 90°)
  state
    pos : Point        = origin
    vel : Vector L/T   = speed * (cos(angle), sin(angle))
  discrete
    flying = true
  dynamics
    pos' = if flying then vel else (0, 0)
    vel' += if flying then (0, −g) else (0, 0)    // gravity
    vel' += −k * |vel| * vel                      // drag
  events
    apex:   when vel.y falls through 0
    landed: when pos.y falls through 0
      flying ← false
      vel ← (0, 0)

presentation ProjectileChecks of Projectile
  observe
    t_apex  = elapsed     at each apex
    h_apex  = pos.y       at each apex
    t_land  = elapsed     at each landed
    range   = pos.x       at each landed
    y_land  = pos.y       at each landed
    at_rest = pos         at t₀ + 5 s
    events  = event log   during [t₀, t₀ + 5 s]
```

```cases
run A-45deg: Projectile, observed by ProjectileChecks, until t₀ + 5 s
  expect
    t_land  ≈ 2.88320807823261 s   ± 1e-9 s
    range   ≈ 40.7747196738022 m   ± 1e-8 m
    y_land  ∈ [−1e-8 m, 0 m]
    t_apex  ≈ 1.44160403911630 s   ± 1e-9 s
    h_apex  ≈ 10.1936799184506 m   ± 1e-8 m
    at_rest = pos at each landed
    events  = [apex, landed]

run B-60deg: Projectile with angle = 60°, observed by ProjectileChecks, until t₀ + 5 s
  expect
    t_land  ≈ 3.53119430697019 s   ± 1e-9 s
    range   ≈ 35.3119430697019 m   ± 1e-8 m
```

## RP-02 Projectile with drag

The RP-01 model, unchanged.

```text
presentation DragChecks of Projectile
  observe
    t_apex  = elapsed     at each apex
    h_apex  = pos.y       at each apex
    t_land  = elapsed     at each landed
    range   = pos.x       at each landed
    v_land  = vel         just before landed                           // before the reset
    energy  = 0.5 * |vel|² + g * (pos − origin).y        every 0.01 s  // per unit mass
    balance = vel' − ((0, −g) − k * |vel| * vel)         every 0.01 s
```

```cases
run A-default: Projectile with k = 0.01 /m, observed by DragChecks, until t₀ + 5 s
  expect
    t_land  ≈ 2.67328857282834 s   ± 0.001 %
    range   ≈ 31.3229266146783 m   ± 0.001 %
    v_land  ≈ (9.76677238322685 m/s, −12.3148244403059 m/s)   ± 0.001 %
    vel at each landed = (0, 0)

run B-tight: as A-default, solved with rtol = 1e-10, atol = 1e-12
  expect
    t_land  ≈ 2.67328857282834 s   ± 1e-6 %
```

## RP-03 Bouncing ball

```text
model BouncingBall
  parameters
    g  = 9.81 m/s²
    h0 = 1 m         ∈ (0 m, ∞)
    e  = 0.8         ∈ [0, 1)                 // restitution
  state
    y = h0
    v : L/T = 0
  discrete
    resting = false
  dynamics
    y' = v
    v' = if resting then 0 else −g
  events
    bounce: when y falls through 0
      v ← −e * v
      zeno settle
        y ← 0 m
        v ← 0 m/s
        resting ← true
    top: when v falls through 0               // apex of each flight

presentation BounceChecks of BouncingBall
  observe
    bounce_times = elapsed            at each bounce
    top_heights  = y                  at each top
    zeno         = event log of bounce, zeno applied
    final        = (y, v, resting)    at t₀ + 6 s
    late_events  = event log          during [t₀ + 4.1 s, t₀ + 6 s]
```

```cases
run A-settle: BouncingBall, observed by BounceChecks, until t₀ + 6 s
  expect
    bounce_times[1] ≈ 0.451523640985731 s   ± 1e-8 s
    top_heights[1]  ≈ 0.64 m                ± 1e-8 m
    final = (0 m, 0 m/s, true)
    late_events = []

run C-invalid-e: BouncingBall with e = 1.2
  expect
    initialization fails
```

Case `B-stop` is a model variant, written as a copy of the model with `zeno stop` (comparison S-4).

## RP-04 Pendulum

```text
space Plane = Euclidean 2

model Pendulum in Plane
  parameters
    g     = 9.81 m/s²
    L     = 1 m          ∈ (0 m, ∞)
    m     = 1 kg         ∈ (0 kg, ∞)
    pivot = origin
    θ0    = 10°
  state
    θ = θ0                                   // from the downward vertical
    ω : 1/T = 0
  definitions
    bob    := pivot + L * (sin(θ), −cos(θ))
    energy := 0.5 * m * (L * ω)² + m * g * L * (1 − cos(θ))
  dynamics
    θ' = ω
    ω' = −(g / L) * sin(θ)
  constraints
    rod: |bob − pivot| = L  ± 1e-9 m
  events
    upswing: when θ rises through 0          // once per period

presentation PendulumChecks of Pendulum
  observe
    theta_start = θ             at t₀
    upswings    = elapsed       at each upswing
    energy      = energy        every 0.01 s
    diagnostics = diagnostics   during [t₀, t_end]
```

```cases
run A-dopri5: Pendulum, observed by PendulumChecks, until t₀ + 100 s
  expect
    upswings[1] ≈ 1.50741947047395 s   ± 0.001 %
    diagnostics = []

run B-rk4: Pendulum, observed by PendulumChecks, until t₀ + 100 s, solved with rk4, h = 0.01 s

run C-rk4-no-step: Pendulum, solved with rk4
  expect
    configuration rejected
```

## RP-05 Spring-mass

```text
model SpringMass
  parameters
    m  = 1 kg        ∈ (0 kg, ∞)
    k  = 4 N/m       ∈ (0 N/m, ∞)
    x0 = 0.1 m
  state
    x = x0                                   // displacement from equilibrium
    v : L/T = 0
  definitions
    energy := 0.5 * m * v² + 0.5 * k * x²
    T      := 2π * sqrt(m / k)
  dynamics
    x' = v
    v' = −(k / m) * x
  equations
    period_law:   T = 2π * sqrt(m / k)
    conservation: energy = 0.5 * k * x0²     checked ± 2e-5 J
  events
    pass: when x rises through 0             // once per period

presentation SpringChecks of SpringMass
  observe
    x_10      = x                              at t₀ + 10 s
    passes    = elapsed                        at each pass
    energy    = energy                         every 0.01 s
    residuals = diagnostics of conservation    during [t₀, t_end]
```

```cases
run A-dopri5: SpringMass, observed by SpringChecks, until t₀ + 100 s
  expect
    x_10 ≈ 0.0408082061813392 m   ± 1e-5 m
    residuals = []

run B-rk4: SpringMass, observed by SpringChecks, until t₀ + 100 s, solved with rk4, h = 0.01 s
  expect
    x_10 ≈ 0.0408082061813392 m   ± 1e-7 m
```

## RP-06 Function plot with draggable parameter

```text
model QuadraticDemo
  parameters
    a = 1            ∈ [−5, 5]
  definitions
    f(x : Real) := a * x²

presentation QuadraticPlot of QuadraticDemo
  views
    plot: plot x ∈ [−3, 3], y ∈ [−5, 20]
      graph of f
      marker handle at (1, f(1))
  controls
    slider a ∈ [−5, 5] step 0.1
    formula f with live values
  interaction
    drag handle to p  ⇒  a ← p.y / 1²          // x fixed at 1
  observe
    a_now = a       live
    f2    = f(2)    live
    log   = intervention log
```

## RP-07 Vector addition

```text
space Plane = Euclidean 2

model VectorDemo in Plane
  parameters
    A            = origin + (1 m, 2 m)
    u : Vector L = (3 m, 0 m)
    w : Vector L = (0 m, 2 m)
  definitions
    sum    := u + w
    length := |sum|
    B      := A + sum
    mid    := A + u                           // where w starts

presentation VectorPlot of VectorDemo
  views
    scene: spatial Plane, 1 m ↦ 40 px, y up
      axes, grid
      marker at A
      arrow u_arrow: u from A
      arrow w from mid
      arrow sum from A
      label length
  interaction
    drag u_arrow.head to h  ⇒  u ← h − A
  observe
    state = (sum, length, B)   live
```

## RP-08 Narrated projectile lesson

The RP-01 model with one addition (D-027):

```text
  events
    relaunch: on request
      pos ← origin
      vel ← speed * (cos(angle), sin(angle))
      flying ← true
```

`speed` and `angle` show in formulas as `v` and `θ` through a `symbols` section of the model: `symbols: speed as v, angle as θ`.

```text
presentation ProjectileLesson of Projectile
  views
    scene: spatial Plane, 1 m ↦ 10 px, y up
      axes
  learner may
    control the timeline; zoom, pan
  observe
    landings = (elapsed, pos.x)   at each landed
    log      = event log

  timeline
    scene launch
      b1: show marker ball at pos, arrow vel from pos in scene
          narrate "A ball is launched at 45 degrees." (4 s)
      b2: run at rate 1 until landed
      b3: hold; highlight ball
          narrate "It lands here. Why this distance?" (3 s)
      b4: seek t₀
          show formula R = speed² * sin(2 * angle) / g with live values
          narrate "Watch the horizontal speed." (3 s)
      b5: show arrow (vel.x, 0) from pos in scene
          run at rate 0.5 until landed
    scene try_it
      b6: hold; narrate "Choose your own angle." (3 s)
      b7: explore for at most 60 s, keeping angle
            slider angle ∈ [10°, 80°]
          fallback: angle ← 60°; wait 3 s
      b8: request relaunch; run at rate 1 until landed
      b9: narrate "Compare the distance with the first launch." (3 s)
```

```cases
run A-keep: Projectile, presented by ProjectileLesson
  learner
    at 23.0 s: slider angle ← 60°
    at 25.0 s: continue
  expect
    start of b3 ≈ 6.88320807823261 s   ± 1e-8 s
    end of b8   ≈ 28.5311943069702 s   ± 1e-8 s
```

---

## 2. Observations from writing

- **Closest to a blackboard.** `θ' = ω`, `ω' = −(g / L) * sin(θ)`, `e = 0.8 ∈ [0, 1)` and `when y falls through 0` are what a physics teacher writes. Omitted types remove most of the declaration noise.
- **Role depends on the section.** A single line such as `x = x0` means initial value in `state`, default in `parameters`, and would be an equation in `equations`. In a diff, a code review or an error message quoting one line, the section is not visible. This is the main cost of B.
- **Inferred types move errors.** RP-04.D1 (`state θ : Length = 10 cm`) becomes `θ = 10 cm` with an inferred `Length`; the error is then reported at `sin(θ)` in `dynamics`, far from the line that caused it. Writing the type restores the local error.
- **Several surface forms per IR element.** `y falls through h` and `y − h falls through 0` lower to the same crossing; an interval and a written constraint lower to the same `reject` constraint; `run at rate 1 until landed` is two timeline actions. The printer needs a canonical choice for each (comparison section 5).
- **The same mode noise as A** (`if flying then ... else (0, 0)`): sections do not help with modes.
- **Typing and pasting.** Unicode forms need an editor that offers them; the ASCII spellings are always accepted but then files mix both unless a formatter normalizes them. Indentation is lost when code is pasted into many web forms, slides and chat tools that educators use.
- **The timeline reads as prose** (`run at rate 1 until landed`, `explore for at most 60 s, keeping angle`), which suits a lesson author.
