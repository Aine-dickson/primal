# 7. Presentations

A presentation turns a model into something a learner sees and handles. It never changes the model's meaning: it observes values, draws them, and offers the learner a controlled set of actions, each of which the model validates. One model may have several presentations.

## Structure

```prismal
presentation Name for Model {
  view name: spatial(Space, scale: 1 m -> 40 px, y: up) { representations }
  view name: plot(x: [a, b], y: [c, d]) { representations }
  panel name { representations }
  permit learner { zoom; pan; timeline_controls }
  observe { name = expression schedule }
  timeline { ... }                                  // chapter 8
}
```

| Part | Purpose |
|---|---|
| `view ...: spatial(...)` | a region showing a space, with a scale from lengths to pixels and an orientation (`y: up` or `y: down`) |
| `view ...: plot(...)` | a region with two axes; each range gives its axis a dimension (`[0 s, 5 s]` is a time axis) |
| `panel ...` | a region without coordinates, for controls, formulas and labels |
| `permit learner { ... }` | what the learner may do with the views and the timeline: `zoom`, `pan`, `timeline_controls` |
| `observe { ... }` | named values recorded for tests and for the Observations tab |

## Representations

| Representation | Shows | Where |
|---|---|---|
| `marker(P)`, `marker(at: (x, y))` | a point | spatial (a `Point`), plot (a pair in the axes' dimensions) |
| `arrow(v, from: P [, scale: 1 m/s -> 4 px])` | a vector drawn from a point | spatial |
| `segment(P, Q)` | a straight segment | spatial |
| `polyline(P, Q, ...)`, `polygon(P, Q, ...)` | an open or closed path through points | spatial |
| `circle(P, r)`, `ellipse(P, a, b [, rotate: θ])` | a circle of radius `r`, an ellipse with radii `a` and `b` turned by `θ`, around `P` | spatial |
| `arc(P, r, from: θ1, to: θ2)` | part of the circle of radius `r` around `P`, from angle `θ1` to `θ2` | spatial |
| `trace(P every Δ)` | the path of a point up to the instant shown | spatial |
| `function_graph(f)` | the graph of a function `Real -> Real` over the plot's `x` range | plot |
| `series_plot(e every Δ)` | a value against elapsed time | plot with a time `x` axis |
| `axes`, `grid` | coordinate reference | spatial |
| `label(e)` | a value as text | any |
| `formula(f [, live: true])` | the definition of a derived binding or function, typeset | any |
| `formula("R", expression [, live: true])` | a labeled expression, `R = ...`, typeset | any |
| `equation(name [, live: true])` | a model equation, typeset, with its symbols' current values | any |
| `table(e every Δ)` | rows of sampled values; a tuple `e` gives one column per component | any |
| `button(E [, label: "..."])` | requests the `on request` event `E` when pressed | any |
| `slider(p, range: [a, b] [, step: s])` | a control for a parameter | any |
| `number_input(p [, range: [a, b]])` | a typed value for a parameter | any |
| `toggle(p)` | a switch for a Boolean parameter | any |
| `group(at: P, rotate: θ, scale: k) { ... }` | its members, placed, turned and scaled together (see Groups) | spatial |

`as name` after a representation names it (`marker(pos) as ball`), for timelines and for readers.

A `marker` is a dot of fixed size on the screen. A `circle`, `ellipse` or `arc` has radii in the model's units, so it grows when the view is zoomed. Their angles are measured counterclockwise from the space's `x` axis: `arc(P, r, from: 0 deg, to: 90 deg)` is the quarter from the right of `P` to above it.

### Color and line

A drawn representation may say how it looks: `color:` names a color, and `line:` how a stroke is drawn (D-061).

```prismal
segment(origin, pos, color: blue, line: dashed) as rod
trace(pos every 0.1 s, color: green, line: dotted)
marker(pos, color: red) as ball
```

- Colors are named: `red`, `orange`, `yellow`, `green`, `teal`, `blue`, `purple`, `pink`, `gray`, `ink`. Each medium draws a name with a value that suits its theme, so a program looks right on a light page, a dark one, or in a video; `ink` is the color of text.
- Lines are `solid`, `dashed` or `dotted`.
- Markers take a color; arrows, segments, polylines, polygons, circles, ellipses, arcs, traces, graphs and series take a color and a line. Other kinds, and groups, take neither.
- Without them, each kind is drawn its own way (arrows take turns through a few colors, a trace is dashed).
- Color never carries meaning alone (D-026): what a color distinguishes is also named (`as rod`) or labeled, and the text alternative does not mention it.

Every representation has a **text alternative** generated from what it shows ("slider for θ = 45 deg, from 5 deg to 85 deg"), for screen readers and the player's Description tab (D-026).

## Controls and inverses

A control changes a binding; so can a drawn representation that declares an **inverse**:

```prismal
marker(at: (x, 0)) { on drag as p { propose x = p.x * 1 m } }
arrow(u, from: A)  { on drag head as h { propose u = h - A } }
```

- `on drag as p` gives the drag's position as `p`: a `Point` in a spatial view, a pair `(p.x, p.y)` in a plot view, in the axes' dimensions.
- `propose x = ...` says which binding the gesture changes and how. The target must be something the learner may change.
- Proposals go through the model's checks. While dragging, an invalid proposal (outside a range, violating a `reject` constraint) is shown as invalid with the reason; releasing commits the last valid one. Each commit is one intervention, which undo removes.

What the learner may change: **parameters**, by default; **state**, only when the model declares it `intervenable` (`state { x: Length = 0 m intervenable }`); never constants, derived bindings or inputs (D-023). A control on anything else is rejected when the presentation is checked (PK-E03).

## A program

A cannon on flat ground, with gravity switchable to the Moon's. The trajectory is drawn as a function of horizontal distance; dragging the landing point along the ground sets the launch speed.

```text
model Cannon {
  param {
    speed: Velocity = 20 m/s   where speed > 0 m/s   symbol "v"
    angle: Angle    = 45 deg   in [5 deg, 85 deg]    symbol "θ"  unit deg
    moon:  Boolean  = false
  }
  derived {
    g:     Acceleration = if moon then 1.62 m/s^2 else 9.81 m/s^2
    reach: Length       = speed^2 * sin(2 * angle) / g
    top:   Length       = (speed * sin(angle))^2 / (2 * g)
    // Heights in metres against distances in metres, as plain numbers for the plot.
    path(x: Real): Real = x * tan(angle) - (g / (1 m/s^2)) * x^2 / (2 * (speed / (1 m/s))^2 * cos(angle)^2)
  }
}
```

- `symbol "v"` is a **display symbol**: formulas show `v` for `speed`.
- A `function_graph` draws functions from `Real` to `Real`. Dividing a quantity by a unit gives a plain number: `speed / (1 m/s)` is `20` for `20 m/s`.

```text
presentation CannonLab for Cannon {
  view flight: plot(x: [0, 60], y: [0, 25]) {
    function_graph(path)
    marker(at: (reach / (1 m), 0)) as landing {
      on drag as p { propose speed = sqrt(p.x * 1 m * g / sin(2 * angle)) }
    }
  }
  panel controls {
    number_input(speed, range: [1 m/s, 30 m/s])
    slider(angle, range: [5 deg, 85 deg])
    toggle(moon)
    formula("R", speed^2 * sin(2 * angle) / g, live: true)
    label(top)
  }
  observe {
    R   = reach live
    H   = top   live
    log = intervention_log
  }
}
```

- The landing marker sits at `(reach / (1 m), 0)`: the plot's axes are plain numbers, so the reach is converted to metres as a number.
- Dragging it to `p.x` proposes the speed whose reach is `p.x` metres: `R = v² sin 2θ / g` solved for `v`. Dragging to a distance that needs more than the parameter allows is shown as invalid.
- `formula("R", ..., live: true)` typesets `R = v² sin(2θ) / g` with the current values of `v`, `θ` and `g`. `label(top)` shows the apex height.
- `intervention_log` observes every committed intervention: the learner's actions, in order, with their instants.

```cases
run default_cannon of Cannon with CannonLab {
  expect {
    R == 40.7747196738022 m within 1e-9 m
    H == 10.1936799184506 m within 1e-9 m
    log == []
  }
}

run on_the_moon of Cannon with CannonLab {
  param { moon = true }
  expect { R == 246.913580246914 m within 1e-9 m }
}

run flatter of Cannon with CannonLab {
  param { angle = 30 deg }
  expect { R == 35.3119430697019 m within 1e-9 m }
}
```

## Groups

A **group** draws several representations with one shared placement: its members are written in the group's own frame, then moved to `at`, turned by `rotate` and scaled by `scale`. A rigid body is drawn this way: its shape once, in its own coordinates, and its position and angle from the model.

```text
space Plane = euclidean(2)

model Wheel in Plane {
  param {
    r: Length   = 0.5 m
    v: Velocity = 1 m/s
  }
  state {
    x: Length = 0 m
    φ: Angle  = 0
  }
  discrete {
    rolling: Boolean = true
  }
  derived {
    // The point on the rim that starts on the right, in the plane.
    rim: Point = origin + (x + r * cos(φ), r + r * sin(φ))
  }
  flow {
    der(x) = if rolling then v else 0
    der(φ) = if rolling then -v / r else 0      // rolling without slipping, clockwise
  }
  event road_end on rising(x - 4 m) {
    set rolling = false
  }
}
```

```text
presentation WheelView for Wheel {
  view road: spatial(Plane, scale: 1 m -> 100 px, y: up) {
    axes
    group(at: origin + (x, r), rotate: φ) as wheel {
      circle(origin, r) as tyre
      segment(origin, origin + (r, 0 m), color: gray)
      marker(origin + (r, 0 m), color: red) as valve
    }
    trace(rim every 0.02 s)
  }
  observe {
    lowest = rim.y at t0 + 0.785398163397448 s
  }
}
```

- The wheel is drawn around its own centre, `origin`, as a circle with one spoke; the group places that centre at `(x, r)` and turns it by `φ`. The marker `valve` is therefore always at the model's `rim`, and the trace of `rim` draws a cycloid behind it.
- `at` is a point (the group's `origin` goes there; default the view's origin), `rotate` an angle (default 0), `scale` a number (default 1). Points of members are moved, turned and scaled; vectors of arrows are turned and scaled.
- A group holds markers, arrows, segments, polylines, polygons, circles, ellipses, arcs and other groups; a group inside a group is placed in its parent's frame. Groups belong in spatial views.
- A group and each named member can be the target of `highlight`, `hide` and `reveal` in a timeline. The text alternative of a group lists those of its members.
- A trace is sampled over time, so it is drawn outside the group, from a model point (`rim`).
- The wheel stops at the end of a 4 m road (`road_end`), so the whole path fits the view.

After a quarter turn, at `t = π r / (2 v)`, the rim point touches the road:

```cases
run quarter_turn of Wheel with WheelView {
  until t0 + 2 s
  expect { lowest == 0 m within 1e-9 m }
}
```

## Checks on a presentation

A presentation is checked against its model before anything is shown. Some mistakes:

```error
// error: PK-E03
model Cannon {
  param { speed: Velocity = 20 m/s }
  derived { twice: Velocity = 2 * speed }
}
presentation Bad for Cannon {
  panel p { slider(twice, range: [0 m/s, 50 m/s]) }
}
```

A derived value cannot be set; the slider must target a parameter (`speed`).

```error
// error: PK-E04
space Plane = euclidean(2)
model Moving in Plane {
  param { vel: Vector<Velocity> = (3 m/s, 4 m/s) }
}
presentation Bad for Moving {
  view scene: spatial(Plane, scale: 1 m -> 40 px, y: up) {
    arrow(vel, from: origin)
  }
}
```

A velocity arrow in a view that maps lengths needs its own scale: `arrow(vel, from: origin, scale: 1 m/s -> 10 px)` (PK-5.5).

## Exercises

1. Add a second function graph to `CannonLab` for the same launch without the Moon toggle (always `9.81 m/s^2`), to compare the two trajectories.
2. Add a marker at the apex, `(reach / (2 m), top / (1 m))`, with an inverse that sets the angle from the dragged height.
3. For the `Triangle` of chapter 3, add `number_input` controls for the coordinates of `A`. (Hint: a control targets a whole binding; which bindings would the model need?)

<details>
<summary>A solution to exercise 1</summary>

The second graph needs its own derived function in the model, since a presentation shows the model's values and does not compute new ones:

```text
model Cannon2 {
  param {
    speed: Velocity = 20 m/s   where speed > 0 m/s
    angle: Angle    = 45 deg   in [5 deg, 85 deg]   unit deg
    moon:  Boolean  = false
  }
  derived {
    g: Acceleration = if moon then 1.62 m/s^2 else 9.81 m/s^2
    path(x: Real): Real  = x * tan(angle) - (g / (1 m/s^2)) * x^2 / (2 * (speed / (1 m/s))^2 * cos(angle)^2)
    earth(x: Real): Real = x * tan(angle) - 9.81 * x^2 / (2 * (speed / (1 m/s))^2 * cos(angle)^2)
  }
}

presentation Compare for Cannon2 {
  view flight: plot(x: [0, 60], y: [0, 25]) {
    function_graph(path)
    function_graph(earth)
  }
  panel controls { toggle(moon); slider(angle, range: [5 deg, 85 deg]) }
  observe { e10 = earth(10) live }
}
```

```cases
run compare of Cannon2 with Compare {
  expect { e10 == 7.5475 within 1e-12 }     // 10 - 9.81 * 100 / (2 * 400 * 0.5)
}
```

</details>
