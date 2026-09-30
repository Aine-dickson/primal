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
| `trace(P every Δ)` | the path of a point up to the instant shown | spatial |
| `function_graph(f)` | the graph of a function `Real -> Real` over the plot's `x` range | plot |
| `series_plot(e every Δ)` | a value against elapsed time | plot with a time `x` axis |
| `axes`, `grid` | coordinate reference | spatial |
| `label(e)` | a value as text | any |
| `formula(f [, live: true])` | the definition of a derived binding or function, typeset | any |
| `formula("R", expression [, live: true])` | a labeled expression, `R = ...`, typeset | any |
| `slider(p, range: [a, b] [, step: s])` | a control for a parameter | any |
| `number_input(p [, range: [a, b]])` | a typed value for a parameter | any |
| `toggle(p)` | a switch for a Boolean parameter | any |

`as name` after a representation names it (`marker(pos) as ball`), for timelines and for readers.

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
