# 11. Solutions to the exercises

Each chapter ends with exercises, and solves one of them in place. This chapter solves the others. Every program here is compiled and every case run by the guide's tests, like the programs of the chapters, so each solution is known to work.

A solution is one way to write the answer; others may be as good. Try the exercise first: a solution read before trying teaches much less.

## Chapter 1

**Exercises 1 and 2.** A vertical shift `c` with its slider, and a case with `k = 0.5`:

```text
model ShiftedWave {
  param {
    A: Real = 1     in [0, 3]
    k: Real = 1     in [0.5, 4]
    c: Real = 0     in [-2, 2]
  }
  derived {
    y(x: Real): Real = A * sin(k * x) + c
  }
}

presentation ShiftedPlot for ShiftedWave {
  view graph: plot(x: [-6, 6], y: [-5, 5]) {
    function_graph(y)
  }
  panel controls {
    slider(A, range: [0, 3], step: 0.1)
    slider(k, range: [0.5, 4], step: 0.1)
    slider(c, range: [-2, 2], step: 0.1)
    formula(y, live: true)
  }
  observe { y1 = y(1) live }
}
```

```cases
run shifted of ShiftedWave with ShiftedPlot {
  param { c = 1 }
  expect { y1 == 1.84147098480790 within 1e-12 }     // sin(1) + 1
}

run longer_wave of ShiftedWave with ShiftedPlot {
  param { k = 0.5 }
  expect { y1 == 0.479425538604203 within 1e-12 }    // sin(0.5)
}
```

A parameter's range (`in [-2, 2]`) is checked when the program starts: a case that sets `c = 3` fails to initialize, as `too_tall` does in chapter 1.

## Chapter 2

**Exercise 1.** Fuel for the trip. `Quantity<M/L>` is a mass per length; times a length it is a mass:

```text
model FuelTrip {
  param {
    distance:  Length        = 120 km    where distance > 0 m
    fuel_rate: Quantity<M/L> = 0.06 kg/km
  }
  derived {
    fuel: Mass = fuel_rate * distance
  }
}

presentation FuelChecks for FuelTrip {
  observe { used = fuel live }
}
```

```cases
run default_fuel of FuelTrip with FuelChecks {
  expect { used == 7.2 kg within 1e-9 kg }
}
```

**Exercise 2.** Without the square, `0.5 * mass * speed` is a momentum, `M L T^-1`, and an energy is `M L^2 T^-2`:

```error
// error: MK-E01
model Wrong {
  param {
    mass:  Mass     = 1200 kg
    speed: Velocity = 20 m/s
  }
  derived { kinetic: Energy = 0.5 * mass * speed }
}
```

The message names both dimensions: the declared type's and the expression's.

## Chapter 3

**Exercises 1 and 2.** The midpoint of `AB` with the median from `C`, and the height onto `AB`:

```text
space Plane = euclidean(2)

model Median in Plane {
  param {
    A: Point = origin
    B: Point = origin + (4 m, 0 m)
    C: Point = origin + (0 m, 3 m)
  }
  derived {
    M:    Point         = A + (B - A) / 2
    ab:   Length        = |B - A|
    area: Quantity<L^2> = abs((B - A).x * (C - A).y - (B - A).y * (C - A).x) / 2
    h:    Length        = 2 * area / ab
  }
}

presentation MedianLab for Median {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    grid; axes
    segment(A, B); segment(B, C); segment(C, A)
    segment(C, M, line: dashed) as median
    marker(A) as a { on drag as p { propose A = p } }
    marker(B) as b { on drag as p { propose B = p } }
    marker(C) as c { on drag as p { propose C = p } }
    marker(M) as midpoint
  }
  panel measures { label(h) }
  observe {
    mx     = M.x live
    my     = M.y live
    height = h   live
  }
}
```

```cases
run right_triangle of Median with MedianLab {
  expect {
    mx     == 2 m within 1e-12 m
    my     == 0 m within 1e-12 m
    height == 3 m within 1e-12 m
  }
}
```

`(B - A) / 2` halves a vector; adding it to the point `A` gives a point. `area / ab` divides an area by a length, which is a length, as `h` declares.

## Chapter 4

**Exercise 1.** A driving force at the natural frequency. Undamped, the spring alone would swing at `sqrt(k / m) = 2 /s`; driven there, the swing grows until the damping takes as much energy per cycle as the force gives, at an amplitude of `F0 / (c w) = 0.125 m`:

```text
model Driven {
  param {
    m:  Mass            = 1 kg        where m > 0 kg
    k:  Quantity<M/T^2> = 4 N/m       where k > 0 N/m
    c:  Quantity<M/T>   = 0.4 N*s/m   where c >= 0 N*s/m
    x0: Length          = 0.1 m
    F0: Force           = 0.1 N
    w:  Frequency       = 2 /s
  }
  state {
    x: Length   = x0
    v: Velocity = 0
  }
  flow { der(x) = v }
  process spring  { flow der(v) += -(k / m) * x }
  process damping { flow der(v) += -(c / m) * v }
  process drive   { flow der(v) += (F0 / m) * cos(w * (t - t0)) }
}

presentation DrivenLab for Driven {
  view position: plot(x: [0 s, 40 s], y: [-0.15 m, 0.15 m]) {
    series_plot(x every 0.02 s)
  }
  panel controls { slider(w, range: [0.5 /s, 4 /s]) }
  observe { x40 = x at t0 + 40 s }
}
```

```cases
run resonance of Driven with DrivenLab {
  until t0 + 40 s
  expect { x40 == -0.124218697747 m within 1e-6 m }
}
```

`t` is an instant, not a duration: `w * t` is an error (MK-E04, scaling an instant), and `t - t0`, the time since the start, is what the cosine takes. The reference value comes from an independent fine-step integration. Move the slider away from `2 /s` and the swing stays small: that peak is resonance.

**Exercise 3.** A projectile in the plane. Gravity is a process, so it can be switched off or replaced later:

```text
space Plane = euclidean(2)

model Throw in Plane {
  param {
    g:     Acceleration = 9.81 m/s^2
    speed: Velocity     = 20 m/s
    angle: Angle        = 45 deg     unit deg
  }
  state {
    pos: Point            = origin
    vel: Vector<Velocity> = (speed * cos(angle), speed * sin(angle))
  }
  flow { der(pos) = vel }
  process gravity { flow der(vel) += (0 m/s^2, -g) }
}

presentation ThrowView for Throw {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    axes
    marker(pos) as ball
    trace(pos every 0.05 s)
  }
  observe {
    x1 = pos.x at t0 + 1 s
    y1 = pos.y at t0 + 1 s
  }
}
```

```cases
run one_second of Throw with ThrowView {
  until t0 + 2 s
  expect {
    x1 == 14.1421356237310 m within 1e-6 m     // 20 cos 45 deg
    y1 == 9.23713562373095 m within 1e-6 m     // 20 sin 45 deg - g / 2
  }
}
```

Without an event the ball flies through the ground; RP-01 adds the landing.

## Chapter 5

**Exercise 1.** An overflow valve at 1.1 m. The event as the exercise writes it is refused:

```error
// error: MK-E16
model Spill {
  param {
    inflow: Quantity<L/T> = 0.02 m/s
    drain:  Quantity<L/T> = 0.01 m/s
  }
  state { level: Length = 0.2 m }
  flow { der(level) = inflow - drain }
  event spill on rising(level - 1.1 m) { set level = 1.1 m }
}
```

`spill` sets the level its own guard reads, and the level keeps rising after it, so the event would happen again at once, forever. The compiler asks for a Zeno policy (chapter 5), but the better fix is the physics: while the tank overflows, what comes in goes out over the rim, and the level stays at 1.1 m. That is a mode:

```text
model Overflow {
  param {
    inflow: Quantity<L/T> = 0.02 m/s
    drain:  Quantity<L/T> = 0.01 m/s
    high:   Length = 1 m
    low:    Length = 0.5 m
  }
  state { level: Length = 0.2 m }
  discrete {
    pumping:     Boolean = true
    overflowing: Boolean = false
  }
  flow {
    der(level) = if overflowing then 0 m/s else (if pumping then inflow else 0) - drain
  }
  event full  on rising(level - high)  { set pumping = false }
  event empty on falling(level - low)  { set pumping = true }
  event spill on rising(level - 1.1 m) { set overflowing = true }
}

presentation OverflowChecks for Overflow {
  observe {
    spills = elapsed on spill
    fulls  = elapsed on full
    later  = level at t0 + 200 s
  }
}
```

```cases
run default_high of Overflow with OverflowChecks {
  until t0 + 200 s
  expect { spills == [] }
}

run high_set_above of Overflow with OverflowChecks {
  param { high = 1.2 m }
  until t0 + 200 s
  expect {
    spills[1] == 90 s within 1e-9 s      // 0.9 m at 0.01 m/s
    fulls     == []
    later     == 1.1 m within 1e-9 m
  }
}
```

With the default `high = 1 m` the pump stops first, so the tank never overflows. With `high` above the rim, the level reaches 1.1 m at 90 s and stays there: the pump never reaches `high`, and the tank overflows for good. (This model never leaves the overflowing mode; a pump that stops would need an event that ends it.)

**Exercise 2.** A leak from 150 s. At 150 s the pump is on (it restarted at 130 s) and the level is 0.7 m; from then the doubled drain, 0.02 m/s, equals the inflow, so the level stays at 0.7 m and never reaches `low` again:

```text
model Leaky {
  param {
    inflow: Quantity<L/T> = 0.02 m/s
    drain:  Quantity<L/T> = 0.01 m/s
    high:   Length = 1 m
    low:    Length = 0.5 m
  }
  state { level: Length = 0.2 m }
  discrete {
    pumping: Boolean = true
    leaking: Boolean = false
  }
  flow {
    der(level) = (if pumping then inflow else 0) - (if leaking then 2 * drain else drain)
  }
  event full  on rising(level - high) { set pumping = false }
  event empty on falling(level - low) { set pumping = true }
  event leak  on at t0 + 150 s        { set leaking = true }
}

presentation LeakyChecks for Leaky {
  observe {
    empties = elapsed on empty
    later   = level at t0 + 300 s
  }
}
```

```cases
run leak of Leaky with LeakyChecks {
  until t0 + 300 s
  expect {
    empties[1] == 130 s within 1e-9 s
    later      == 0.7 m within 1e-9 m
  }
}
```

With the leak at 100 s instead, while the pump is off at 0.8 m, the level would fall at 0.02 m/s and reach `low` at 115 s.

## Chapter 6

**Exercise 1.** A parachute that opens at 12 s. When it opens, the terminal speed `vt` drops from 56 m/s to 6.3 m/s while the diver still falls at 54 m/s, so `subterminal` is violated. With `reject`, the transition made by `deploy` is refused and the run fails at 12 s (RC section 10). With `report`, the run goes on: the diver slows towards the new terminal speed, and every step until then records a diagnostic:

```text
model Chute {
  param {
    m:      Mass          = 80 kg
    g:      Acceleration  = 9.81 m/s^2
    c:      Quantity<M/L> = 0.25 kg/m
    c_open: Quantity<M/L> = 20 kg/m
  }
  state {
    fallen: Length   = 0 m
    v:      Velocity = 0
  }
  discrete { open: Boolean = false }
  derived {
    drag: Quantity<M/L> = if open then c_open else c
    vt:   Velocity      = sqrt(m * g / drag)
  }
  flow {
    der(fallen) = v
    der(v)      = g - (drag / m) * v^2
  }
  event deploy on at t0 + 12 s { set open = true }
  constraint subterminal: v <= vt policy report
}

presentation ChuteChecks for Chute {
  observe {
    v12      = v at t0 + 12 s
    v20      = v at t0 + 20 s
    problems = diagnostics over [t0, t0 + 11 s]
  }
}
```

```cases
run chute of Chute with ChuteChecks {
  until t0 + 20 s
  expect {
    v12      == 54.3765192220466 m/s within 1e-5 m/s    // vt tanh(g t / vt)
    v20      == 6.26418390547676 m/s within 1e-5 m/s
    problems == []                                     // none before the chute opens
  }
}
```

After opening, `v = vt coth(g (t - 12 s) / vt + arcoth(v12 / vt))`, which is within a millionth of `vt` by 20 s. A constraint states what must hold for the model to be valid; "never faster than terminal speed" is true of a free fall but not of a fall whose drag changes, so the model's constraint was too strong.

**Exercise 2.** Energy is conserved without damping and lost with it:

```text
model Conserving {
  param {
    m:  Mass            = 1 kg        where m > 0 kg
    k:  Quantity<M/T^2> = 4 N/m       where k > 0 N/m
    c:  Quantity<M/T>   = 0.4 N*s/m   where c >= 0 N*s/m
    x0: Length          = 0.1 m
  }
  state {
    x: Length   = x0
    v: Velocity = 0
  }
  derived { energy: Energy = 0.5 * m * v^2 + 0.5 * k * x^2 }
  flow {
    der(x) = v
    der(v) = -(k / m) * x - (c / m) * v
  }
  equation conservation: energy == 0.5 * k * x0^2 checked within 1e-5 J
}

presentation ConservingChecks for Conserving {
  observe { problems = diagnostics over [t0, t_end] }
}
```

```cases
run undamped of Conserving with ConservingChecks {
  param { c = 0 N*s/m }
  until t0 + 10 s
  expect { problems == [] }
}
```

With the default damping the case fails, and its first diagnostic reads: check equation `conservation` at t = 0.1007 s, residual 2.098e-5 exceeds 1e-5. The residual is the energy lost so far; it passes the tolerance a tenth of a second in, and a diagnostic follows at every step after. The equation is not a law of the damped model, and the check says so.

## Chapter 7

**Exercise 2.** A marker at the apex whose drag sets the angle. The apex height is `(v sin θ)^2 / (2 g)`, so a dragged height `y` gives `sin θ = sqrt(2 g y) / v`. The language has `atan2` but no `asin`; `atan2(s, sqrt(1 - s^2))` is the angle whose sine is `s`:

```text
model Apex {
  param {
    speed: Velocity = 20 m/s   where speed > 0 m/s
    angle: Angle    = 45 deg   in [5 deg, 85 deg]   unit deg
  }
  derived {
    g:     Acceleration = 9.81 m/s^2
    reach: Length       = speed^2 * sin(2 * angle) / g
    top:   Length       = (speed * sin(angle))^2 / (2 * g)
    path(x: Real): Real = x * tan(angle) - (g / (1 m/s^2)) * x^2 / (2 * (speed / (1 m/s))^2 * cos(angle)^2)
  }
}

presentation ApexLab for Apex {
  view flight: plot(x: [0, 60], y: [0, 25]) {
    function_graph(path)
    marker(at: (reach / (2 m), top / (1 m))) as apex {
      on drag as p {
        propose angle = atan2(sqrt(2 * g * p.y * 1 m) / speed, sqrt(1 - 2 * g * p.y * 1 m / speed^2))
      }
    }
  }
  panel controls { slider(angle, range: [5 deg, 85 deg]) }
  observe {
    ax = reach / (2 m) live
    ay = top / (1 m)   live
  }
}
```

```cases
run apex of Apex with ApexLab {
  expect {
    ax == 20.3873598369011 within 1e-9
    ay == 10.1936799184506 within 1e-9
  }
}

run steeper of Apex with ApexLab {
  param { angle = 60 deg }
  expect { ay == 15.2905198776758 within 1e-9 }     // (20 sin 60)^2 / (2 g)
}
```

A height above `v^2 / (2 g)`, about 20.4 m, has no angle: the square root of a negative number is a status, and the drag's preview shows the proposal as not valid (PK-10.6).

**Exercise 3.** Controls target whole bindings, and a point is one binding: a `number_input` edits a number or a quantity, not a coordinate of a point. The model therefore declares the coordinates as parameters and builds the point from them:

```text
space Plane = euclidean(2)

model Corner in Plane {
  param {
    Ax: Length = 0 m   in [-5 m, 5 m]
    Ay: Length = 0 m   in [-5 m, 5 m]
    B:  Point  = origin + (4 m, 0 m)
    C:  Point  = origin + (0 m, 3 m)
  }
  derived {
    A: Point = origin + (Ax, Ay)
    G: Point = A + ((B - A) + (C - A)) / 3
  }
}

presentation CornerLab for Corner {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    segment(A, B); segment(B, C); segment(C, A)
    marker(G) as centroid
  }
  panel inputs {
    number_input(Ax, range: [-5 m, 5 m])
    number_input(Ay, range: [-5 m, 5 m])
  }
  observe { gx = G.x live }
}
```

```cases
run moved of Corner with CornerLab {
  param { Ax = 1 m }
  expect { gx == 1.66666666666667 m within 1e-12 m }    // (1 + 4 + 0) / 3
}
```

`A` is derived now, so the marker of `A` could no longer be dragged by proposing `A`; a drag would propose `Ax` and `Ay` instead: `on drag as p { propose Ax = p.x; propose Ay = p.y }`.

## Chapter 8

**Exercise 1.** A beat that shows the velocity and replays the fall. The arrow is shown by the beat (`in scene { ... }`) and stays for the rest of the lesson:

```text
space Plane = euclidean(2)

model Dropped in Plane {
  param {
    g: Acceleration = 9.81 m/s^2
    h: Length       = 10 m     in [1 m, 50 m]
  }
  state {
    pos: Point            = origin + (0 m, h)
    vel: Vector<Velocity> = 0
  }
  discrete { airborne: Boolean = true }
  flow {
    der(pos) = if airborne then vel else 0
    der(vel) = if airborne then (0, -g) else 0
  }
  event landed on falling(pos.y) {
    set airborne = false
    set vel = 0
  }
}

presentation ArrowLesson for Dropped {
  view scene: spatial(Plane, scale: 1 m -> 12 px, y: up) {
    axes
    marker(pos) as ball
  }
  timeline {
    scene first_drop {
      beat fall   { run rate 1 until landed }
      beat slow   { seek t0; run rate 0.25 until landed }
      beat arrows {
        seek t0
        in scene { arrow(vel, from: pos, scale: 1 m/s -> 5 px) as speed }
        run rate 1 until landed
      }
    }
  }
}
```

```cases
run with_arrow of Dropped with ArrowLesson {
  expect {
    end of fall   == 1.42784312292706 s within 1e-8 s     // T = sqrt(2 h / g)
    end of slow   == 7.13921561463530 s within 1e-8 s     // T + 4 T
    end of arrows == 8.56705873756236 s within 1e-8 s     // T + 4 T + T
  }
}
```

The run-directing actions, `seek` then `run`, apply first, in order (D-033); the arrow appears as the replay starts.

**Exercise 3.** RP-08 with a learner who sets 30 degrees. Until the explore beat nothing changes. The learner continues at 25 s, so `b7` ends there; `b8` relaunches at 30 degrees and runs until the landing, which takes `T30 = 2 v sin(30°) / g = 20 / 9.81 = 2.03873598 s`. So `b8` ends at `27.0387359836901 s`. The range is `v^2 sin(60°) / g = 35.3119 m`, the same as at 60 degrees: complementary angles reach equally far, and the narration's "compare the distance" has a surprise in it.

```text
space Plane = euclidean(2)

model Launch in Plane {
  param {
    g:     Acceleration = 9.81 m/s^2
    speed: Velocity     = 20 m/s   where speed > 0 m/s
    angle: Angle        = 45 deg   in (0 deg, 90 deg)   unit deg
  }
  state {
    pos: Point            = origin
    vel: Vector<Velocity> = speed * (cos(angle), sin(angle))
  }
  discrete { flying: Boolean = true }
  flow {
    der(pos) = if flying then vel else 0
    der(vel) = if flying then (0, -g) else 0
  }
  event landed on falling(pos.y) { set flying = false; set vel = 0 }
  event relaunch on request {
    set pos    = origin
    set vel    = speed * (cos(angle), sin(angle))
    set flying = true
  }
}

presentation Relaunch for Launch {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    axes
    marker(pos) as ball
  }
  observe {
    times  = elapsed on landed
    ranges = pos.x on landed
  }
  timeline {
    scene launch {
      beat b1 { narrate "A ball is launched at 45 degrees." for 4 s }
      beat b2 { run rate 1 until landed }
      beat b3 { hold; narrate "It lands here. Why this distance?" for 3 s }
      beat b4 { seek t0; narrate "Watch the horizontal speed." for 3 s }
      beat b5 { run rate 0.5 until landed }
    }
    scene try_it {
      beat b6 { hold; narrate "Choose your own angle." for 3 s }
      beat b7 {
        explore limit 60 s keep angle {
          slider(angle, range: [10 deg, 80 deg])
        } fallback {
          sequence { intervene { set angle = 60 deg }; wait 3 s }
        }
      }
      beat b8 { request relaunch; run rate 1 until landed }
      beat b9 { narrate "Compare the distance with the first launch." for 3 s }
    }
  }
}
```

```cases
run thirty of Launch with Relaunch {
  learner {
    at 23 s: set slider angle = 30 deg
    at 25 s: continue
  }
  expect {
    end of b8 == 27.0387359836901 s within 1e-8 s
    times[2]  == 4.92194406192272 s within 1e-8 s     // T45 + T30
    ranges[2] == 35.3119430697019 m within 1e-6 m     // as at 60 degrees
  }
}
```

## Chapter 9

**Exercise 1.** A fourth ball: `row: Ball[4]`. The new ball starts at `(4 m, 4 m)`, so it is the highest: `top` changes at 0.2 s (from 2.8038 m to 3.8038 m), and `rested` at 10 s becomes 4. A ball from 4 m bounces for about 8 s in all (`sqrt(2 × 3.9 m / g) × (1 + 0.8) / (1 - 0.8)`), so it rests before 10 s. `second` does not change:

```text
space Plane = euclidean(2)

model Drops4 in Plane {
  object Ball {
    param { r: Length = 0.1 m }
    state {
      pos: Point            = origin + (0 m, 1 m)
      vel: Vector<Velocity> = 0
    }
    discrete { resting: Boolean = false }
    flow {
      der(pos) = vel
      der(vel) = if resting then 0 else (0 m/s^2, -9.81 m/s^2)
    }
    event bounce on falling(pos.y - r) {
      set vel = (vel.x, -0.8 * vel.y)
    } zeno settle {
      set vel = 0
      set pos = origin + (pos.x, r)
      set resting = true
    }
  }
  parts {
    row: Ball[4] { pos = origin + (index * 1 m, index * 1 m) }
  }
  derived {
    highest: Length = max(b.pos.y for b in row)
    landed:  Real   = count(b for b in row if b.resting)
  }
}

presentation Drops4View for Drops4 {
  view scene: spatial(Plane, scale: 1 m -> 60 px, y: up) {
    for b in row { marker(b.pos) as ball }
  }
  observe {
    second = row[2].pos.y live
    top    = highest live
    rested = landed live
  }
}
```

```cases
run first_moments of Drops4 with Drops4View {
  until t0 + 0.2 s
  expect {
    second == 1.8038 m within 1e-9 m
    top    == 3.8038 m within 1e-9 m
  }
}

run at_rest of Drops4 with Drops4View {
  until t0 + 10 s
  expect { rested == 4 within 1e-12 }
}
```

**Exercise 4.** Two drops per spray. Each `create` makes the next drop, so a spray at `0.5 k` s makes drops `2k + 1` and `2k + 2`. At 1.2 s the pair from 0 s has landed (at 1.0194 s), and the pairs from 0.5 s and 1 s fly: four drops. The capacity now lasts half as long, so it is doubled:

```text
space Plane = euclidean(2)

model Fountain2 in Plane {
  object Drop {
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow {
      der(pos) = vel
      der(vel) = (0 m/s^2, -9.81 m/s^2)
    }
  }
  param { speed: Velocity = 5 m/s }
  parts { drops: Drop[max 80] }
  event spray on every 0.5 s {
    create drops { vel = (1 m/s, speed) }
    create drops { vel = (-1 m/s, speed) }
  }
  for d in drops {
    event land on falling(d.pos.y) { destroy d }
  }
  derived { flying: Real = count(drops) }
}

presentation Spray2 for Fountain2 {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    for d in drops { marker(d.pos) as drop }
  }
  observe {
    n     = flying live
    right = drops[3].pos.x live
    left  = drops[4].pos.x live
  }
}
```

```cases
run pairs of Fountain2 with Spray2 {
  until t0 + 1.2 s
  expect {
    n     == 4 within 1e-12
    right == 0.7 m within 1e-9 m      // thrown at 0.5 s, 1 m/s for 0.7 s
    left  == -0.7 m within 1e-9 m
  }
}
```

**Exercise 3.** Three beads on a line joined by springs of rest length `L`. Each bead is given its number, `n = index`; the force on bead `b` sums over the other beads whose number differs by one. The spring to a neighbour `o` pulls with `k` times its stretch, `(o.x - b.x) - (o.n - b.n) L`:

```text
model Chain {
  object Bead {
    param { n: Real = 0 }
    state {
      x: Length   = 0 m
      v: Velocity = 0
    }
    flow { der(x) = v }
  }
  param {
    k: Quantity<M/T^2> = 4 N/m
    m: Mass            = 1 kg
    L: Length          = 1 m
    a: Length          = 0.1 m
  }
  parts {
    beads: Bead[3] {
      n = index
      x = (index - 1) * L + (if index == 1 then -a else if index == 3 then a else 0 m)
    }
  }
  flow {
    for b in beads {
      der(b.v) = sum(k * ((o.x - b.x) - (o.n - b.n) * L) for o in beads if abs(o.n - b.n) == 1) / m
    }
  }
}

presentation ChainChecks for Chain {
  observe {
    middle = beads[2].x at t0 + 1 s
    last   = beads[3].x at t0 + 1 s
  }
}
```

```cases
run stretched of Chain with ChainChecks {
  until t0 + 2 s
  expect {
    middle == 1 m within 1e-9 m
    last   == 1.95838531634529 m within 1e-6 m     // 2 m + a cos(2 t)
  }
}
```

With both end beads pulled out by `a`, the middle bead feels equal and opposite pulls and stays put. Each end bead then swings on one spring against a fixed point, at `sqrt(k / m) = 2 /s`. A relation set (the chapter's springs) says the same with the springs as values of their own; the filter is the way when neighbours follow from numbers.

**Exercise 5.** A second marker above each planet requests `kick` for it. A lesson requests the same event to check it; with `mu = 0` there is no sun, so a kicked planet moves in a straight line:

```text
space Plane = euclidean(2)

model Kicks in Plane {
  object Planet {
    state {
      pos: Point            = origin + (1 m, 0 m) intervenable
      vel: Vector<Velocity> = (0 m/s, 1 m/s)
    }
    flow { der(pos) = vel }
  }
  param { mu: Quantity<L^3/T^2> = 1 m^3/s^2 }
  parts {
    planets: Planet[2, max 6] {
      pos = origin + (index * 1 m, 0 m)
      vel = (0 m/s, sqrt(mu / (index * 1 m)))
    }
  }
  flow {
    for p in planets {
      der(p.vel) = -mu * (p.pos - origin) / |p.pos - origin|^3
    }
  }
  event kick on request(p in planets, j: Vector<Momentum>) { set p.vel = p.vel + j / 1 kg }
}

presentation KickSky for Kicks {
  view sky: spatial(Plane, scale: 1 m -> 80 px, y: up) {
    marker(origin) as sun
    for p in planets {
      marker(p.pos) as planet { on drag as q { propose p.pos = q } }
      marker(p.pos + (0 m, 0.2 m), color: orange) as kicker {
        on click request kick(p, (0 kg*m/s, 0.5 kg*m/s))
      }
    }
  }
}

presentation KickTour for Kicks {
  view sky: spatial(Plane, scale: 1 m -> 80 px, y: up) {
    for p in planets { marker(p.pos) as planet }
  }
  observe {
    x1 = planets[1].pos.x at t0 + 1 s
    y2 = planets[2].pos.y at t0 + 1 s
  }
  timeline {
    scene only {
      beat kick_one { request kick(planets[1], (0.5 kg*m/s, 0 kg*m/s)); run rate 1; wait 1 s }
    }
  }
}
```

```cases
run straight of Kicks with KickTour {
  param { mu = 0 m^3/s^2 }
  expect {
    x1 == 1.5 m within 1e-9 m     // from 1 m at 0.5 m/s
    y2 == 0 m within 1e-9 m       // not kicked, and at rest without a sun
  }
}
```

The kicker is a separate marker because a representation has one click; the planet keeps its drag. The orange color tells the two apart, and the names `planet` and `kicker` say so in the text alternatives (D-061).
