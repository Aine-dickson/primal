# 9. Systems of objects

Many systems are made of several things of one kind: balls in a box, planets around a star, masses on a chain of springs. This chapter declares a kind of thing once, as an **object type**, and puts objects of that type in a model: one at a time, or as a **collection** of members (D-055).

## Object types

An object type is declared inside a model with `object`, and has the body of a model: parameters, inputs, state, derived values, flows, events.

```prismal
object Ball {
  param { r: Length = 0.1 m }
  input { g: Acceleration = 9.81 m/s^2 }
  state {
    pos: Point            = origin + (0 m, 1 m)
    vel: Vector<Velocity> = 0
  }
  flow {
    der(pos) = vel
    der(vel) = (0 m/s^2, -g)
  }
}
```

- An object reads nothing of the model around it. What it needs from outside is an **input** (`g` here), which the model connects; an input nobody connects keeps its default.
- An object type is written once and used by as many objects as the model declares. It lives in the model's space: `Point` is a point of the model's plane.

## Parts

The model declares its objects in a `parts` block:

| Part | Declares |
|---|---|
| `ball: Ball` | one object of type `Ball` |
| `ball: Ball { g = g }` | one object, with a value for one of its bindings |
| `row: Ball[3] { pos = origin + (index * 1 m, 2 m) }` | a collection of three members, numbered 1 to 3 |

- A value in braces is an **override**: a starting value for a member's state or parameter, or a **connection** for an input. It is written in the model's scope: `g = g` connects the member's input `g` to the model's parameter `g`, which it then follows.
- In the overrides of a collection, `index` is the member's number, so each member can start in its own place.
- Members are numbered in order; `row[2]` is the second.

## Reading members

| Expression | Reads |
|---|---|
| `ball.pos` | a binding of an object |
| `row[2].pos` | a binding of one member; the number is a constant |
| `sum(b.m for b in row)` | a sum over the members, each named `b` |
| `min(...)`, `max(...)`, `any(...)`, `all(...)` | the smallest, the largest, whether one or every member satisfies a condition |
| `count(row)`, `count(b for b in row if c)` | how many members, or how many satisfy `c` |
| `sum(... for o in row if o != b)` | a filter; `o != b` compares members, not values |

`sum` over no member is `0`, `any` is false and `all` true. `min` and `max` take only filters that compare members.

## A row of balls

Three balls dropped from different heights bounce on a floor at their own radius; a fourth ball bounces on the Moon.

```text
space Plane = euclidean(2)

model Drops in Plane {
  object Ball {
    param { r: Length = 0.1 m }
    input { g: Acceleration = 9.81 m/s^2 }
    state {
      pos: Point            = origin + (0 m, 1 m)
      vel: Vector<Velocity> = 0
    }
    discrete { resting: Boolean = false }
    flow {
      der(pos) = vel
      der(vel) = if resting then 0 else (0 m/s^2, -g)
    }
    event bounce on falling(pos.y - r) {
      set vel = (vel.x, -0.8 * vel.y)
    } zeno settle {
      set vel = 0
      set pos = origin + (pos.x, r)
      set resting = true
    }
  }
  param { g: Acceleration = 9.81 m/s^2 }
  parts {
    row: Ball[3] {
      pos = origin + (index * 1 m, index * 1 m)
      g = g
    }
    moon: Ball { g = 1.62 m/s^2 }
  }
  derived {
    highest: Length = max(b.pos.y for b in row)
    landed:  Real   = count(b for b in row if b.resting)
  }
}
```

- `row` has three balls, the first 1 m to the right and 1 m up, the next 2 m and 2 m, the last 3 m and 3 m. Each follows the model's `g`.
- `moon` is one ball, its input connected to a constant.
- Each ball has its own `bounce` event: the balls bounce independently, and each settles when its bounces accumulate (chapter 5).
- `highest` follows the highest ball; `landed` counts the balls at rest.

A `for` in a view draws one representation per member:

```text
presentation DropsView for Drops {
  view scene: spatial(Plane, scale: 1 m -> 60 px, y: up) {
    axes
    for b in row { marker(b.pos) as ball }
    marker(moon.pos) as moon_ball
  }
  panel numbers {
    label(highest)
    label(landed)
  }
  observe {
    second = row[2].pos.y live
    top    = highest live
    rested = landed live
    moon_y = moon.pos.y live
  }
}
```

- The markers are named `ball[1]`, `ball[2]` and `ball[3]`; each has its own text alternative. `highlight ball` in a timeline highlights all three.

After 0.2 s every ball has fallen `g t² / 2 = 0.1962 m`, the Moon's `0.0324 m`; after 10 s the three balls rest on the floor:

```cases
run first_moments of Drops with DropsView {
  until t0 + 0.2 s
  expect {
    second == 1.8038 m within 1e-9 m
    top    == 2.8038 m within 1e-9 m
    moon_y == 0.9676 m within 1e-9 m
  }
}

run at_rest of Drops with DropsView {
  until t0 + 10 s
  expect {
    second == 0.1 m within 1e-9 m
    rested == 3 within 1e-12
  }
}
```

## Forces between members

An object cannot see the others: a ball does not know the row it is in. Forces between members are written by the model, which contains them all, with a `for` in its `flow` block:

```text
space Plane = euclidean(2)

model Stars in Plane {
  object Body {
    param { m: Mass = 1e24 kg }
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow { der(pos) = vel }
  }
  const { G: Quantity<L^3/M/T^2> = 6.674e-11 m^3/kg/s^2 }
  parts {
    bodies: Body[3] {
      pos = origin + (index * 1e7 m, 0 m)
      vel = (0 m/s, index * 100 m/s)
    }
  }
  flow {
    for b in bodies {
      der(b.vel) = sum(G * o.m * (o.pos - b.pos) / |o.pos - b.pos|^3 for o in bodies if o != b)
    }
  }
  derived {
    momentum: Vector<Momentum> = sum(b.m * b.vel for b in bodies)
  }
}
```

- `for b in bodies { der(b.vel) = ... }` gives each body its acceleration: the sum of the others' pulls, Newton's law of gravitation. `o != b` leaves the body itself out.
- The body's own flow moves it (`der(pos) = vel`); the model's flow sets how its velocity changes. A model may write the state of the objects it contains; an object never writes another's.
- `momentum` adds the bodies' momenta. The pulls between two bodies are equal and opposite, so the total never changes: the bodies start with momentum `(1 + 2 + 3) × 100 m/s × 1e24 kg = 6e26 kg m/s` upwards, and keep it.

```text
presentation Sky for Stars {
  view sky: spatial(Plane, scale: 1e6 m -> 10 px, y: up) {
    for b in bodies {
      marker(b.pos) as star
      trace(b.pos every 100 s)
    }
  }
  observe {
    px = momentum.x live
    py = momentum.y live
  }
}
```

```cases
run conserved of Stars with Sky {
  until t0 + 5000 s
  expect {
    px == 0 kg*m/s within 1e15 kg*m/s
    py == 6e26 kg*m/s within 1e15 kg*m/s
  }
}
```

The tolerance is a relative `2e-12`: the sums are computed in floating point, and the bodies' speeds grow to kilometres per second.

## Members that come and go

The collections so far keep their members for the whole run. A fountain does not: it throws a drop every half second, and each drop is gone when it falls back. A collection whose membership changes declares the most members it makes in one run, with `max` (D-057):

```text
space Plane = euclidean(2)

model Fountain in Plane {
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
  parts { drops: Drop[max 40] }
  event spray on every 0.5 s {
    create drops { vel = (1 m/s, speed) }
  }
  for d in drops {
    event land on falling(d.pos.y) { destroy d }
  }
  derived {
    flying:  Real   = count(drops)
    highest: Length = max(d.pos.y for d in drops) otherwise 0 m
  }
}
```

- `drops: Drop[max 40]` starts empty and makes at most 40 drops in a run. `Drop[3, max 40]` would start with three.
- `create drops { vel = (1 m/s, speed) }` in a handler makes the next drop, with a starting value for its velocity; its position starts where `Drop` says, at the origin. The values are read when the event happens, so they may use the model's state or the event's payload.
- `for d in drops { event land ... }` gives every drop its own `land` event, which reads the drop as `d`. When a drop falls back through zero height, `destroy d` removes it.
- A drop that is not made yet, or already destroyed, does nothing: it does not move, its events do not happen, and aggregates leave it out. `count(drops)` is the number of drops in the air.
- `max` over no drop has no value (at the start, before the first spray, and between drops if the fountain stops), so `highest` says what to use then with `otherwise`.

| Form | Does |
|---|---|
| `c: T[max m]`, `c: T[n, max m]` | a collection that starts with none or `n` members and makes at most `m` in a run |
| `create c { x = e ... }` | makes the next member, with starting values; several `create` in one handler make several members |
| `destroy b` | removes the member `b`: a loop variable, a contained member, or `c[k]` |
| `for b in c { event ... }` | an event of the model for each member |
| `set b.x = e` | in such an event, writes the member's binding (a model may write its members' state) |

The `k`-th drop made is `drops[k]` for the whole run: `drops[1]` is the first drop, which lands after `2 × 5 / 9.81 = 1.0194 s`. Its number is never given to another drop, so its name, its marker and its trace belong to one drop. Read by number, a drop holds its starting values before it is made and its last values after it is destroyed.

```text
presentation Spray for Fountain {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    axes
    for d in drops { marker(d.pos) as drop }
  }
  panel numbers {
    label(flying)
    label(highest)
  }
  observe {
    n     = flying live
    top   = highest live
    first = drops[1].pos.y live
  }
}
```

A marker is drawn for each drop in the air, and only while it is there. At 1.2 s the first drop has landed; the second, thrown at 0.5 s, is at `5 × 0.7 - 4.905 × 0.7² = 1.09655 m`, and the third, thrown at 1 s, is lower:

```cases
run early of Fountain with Spray {
  until t0 + 1.2 s
  expect {
    n     == 2 within 1e-12
    top   == 1.09655 m within 1e-9 m
    first == 0 m within 1e-9 m
  }
}
```

The capacity is part of the model: it says how long the fountain can run. 40 drops, one every half second, last until 19.5 s; a run past 20 s stops there with the diagnostic that `drops.capacity` is exceeded. Choose a capacity that covers the longest run the presentation shows.

## Names in results

Each member's bindings and events have names built from the member: `row[2].pos`, `moon.bounce`. They appear in diagnostics, event logs and text alternatives. A mistake in the object type is reported once for each member, and located at the line in the object type.

Relations between objects (a spring between two chosen balls, a link that can be made and broken) come later.

## Mistakes

```error
// error: MK-E26
space Plane = euclidean(2)
model M in Plane {
  object Ball {
    state { pos: Point = origin }
  }
  parts { row: Ball[3] }
  derived { y4: Length = row[4].pos.y }
}
```

`row` has members 1 to 3; there is no `row[4]`.

```error
// error: SX-E08
space Plane = euclidean(2)
model M in Plane {
  object Ball {
    state { pos: Point = origin }
  }
  parts { row: Ball[3] }
  derived { y: Length = row.pos.y }
}
```

A collection has no `pos`; one of its members has. Write `row[1].pos.y`, or an aggregate: `max(b.pos.y for b in row)`.

```error
// error: SX-E03
space Plane = euclidean(2)
model M in Plane {
  object Ball {
    state { pos: Point = origin }
  }
  parts { ball: Ball { g = 1.62 m/s^2 } }
}
```

`Ball` has no `g` to give a value to: an override names a binding of the object.

```error
// error: MK-E26
space Plane = euclidean(2)
model M in Plane {
  object Ball {
    state { pos: Point = origin }
  }
  parts { row: Ball[3] }
  event more on every 1 s { create row }
}
```

`row` has a fixed membership. To make members during a run, declare how many it can make: `row: Ball[3, max 10]`.

```error
// error: SX-E08
space Plane = euclidean(2)
model M in Plane {
  param { speed: Velocity = 1 m/s }
  event halt on every 1 s { destroy speed }
}
```

`destroy` removes a member of a collection; `speed` is a binding.

## Exercises

1. Add a fourth ball to `row`. Which expectations change?
2. Give each ball in `row` a different radius with an override (`r = index * 0.05 m`), and show the radius as a `circle(b.pos, b.r)` in the view.
3. Chain three masses with springs: each mass is pulled towards its neighbours. Write the force on member `b` as a sum over the others with a filter that keeps only neighbours. (Hint: give each member its number as a parameter, `n = index`, and compare numbers.)
4. Make the fountain throw two drops at each spray, one to each side (`vel = (1 m/s, speed)` and `vel = (-1 m/s, speed)`). How many drops are in the air at 1.2 s?

<details>
<summary>A solution to exercise 2</summary>

In `parts`, add `r = index * 0.05 m` to the overrides of `row`; in the view, draw each ball as a circle of its own radius:

```prismal
for b in row { circle(b.pos, b.r) as ball }
```

A ball's `bounce` compares `pos.y` with its own `r`, so each ball now rests on the floor at its own radius.

</details>
