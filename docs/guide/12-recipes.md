# 12. Recipes

Short answers to "how do I ...". Each recipe is a complete program, tested like the rest of the guide, with a pointer to the chapter that explains it. Copy one, run it, change it.

## Graph a function and change it with a slider

```text
model Line {
  param {
    a: Real = 2   in [-5, 5]
    b: Real = 1   in [-5, 5]
  }
  derived { f(x: Real): Real = a * x + b }
}

presentation LinePlot for Line {
  view graph: plot(x: [-5, 5], y: [-10, 10]) {
    axes
    function_graph(f)
  }
  panel controls {
    slider(a, range: [-5, 5], step: 0.5)
    slider(b, range: [-5, 5], step: 0.5)
    formula(f, live: true)
  }
  observe { at3 = f(3) live }
}
```

```cases
run line of Line with LinePlot {
  expect { at3 == 7 exactly }
}
```

`function_graph(f)` draws any function from `Real` to `Real` over the plot's `x` range; `formula(f, live: true)` typesets its definition with the current values. Chapter 1.

## Show a value changing over time

```text
model Decay {
  param {
    n0:  Real     = 100
    tau: Time = 2 s    where tau > 0 s
  }
  state { n: Real = n0 }
  flow { der(n) = -n / tau }
}

presentation DecayPlot for Decay {
  view curve: plot(x: [0 s, 10 s], y: [0, 100]) {
    axes
    series_plot(n every 0.05 s)
  }
  panel controls { slider(tau, range: [0.5 s, 5 s]) }
  observe { left = n at t0 + 5 s }
}
```

```cases
run decay of Decay with DecayPlot {
  until t0 + 10 s
  expect { left == 8.20849986238988 within 1e-4 }     // 100 exp(-5 / 2)
}
```

`series_plot(n every 0.05 s)` samples `n` and draws it against time in a plot whose `x` axis is a time range. A flow `der(n) = ...` is a differential equation; Prismal integrates it numerically, by default to a relative tolerance of about `1e-6`, so the expectation allows `1e-4` on a value that started at 100. Chapter 4.

## Let the learner set a value by dragging

```text
space Plane = euclidean(2)

model Square in Plane {
  param { side: Length = 2 m   in [0.5 m, 4 m] }
  derived { perimeter: Length = 4 * side }
}

presentation SquareLab for Square {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    polygon(origin, origin + (side, 0 m), origin + (side, side), origin + (0 m, side)) as square
    marker(origin + (side, side)) as corner {
      on drag as p { propose side = p.x }
    }
  }
  panel numbers { label(perimeter) }
  observe { around = perimeter live }
}
```

```cases
run square of Square with SquareLab {
  expect { around == 8 m within 1e-12 m }
}
```

`on drag as p { propose side = p.x }` declares what a drag means: the pointer's point `p` proposes a new `side`, which the model checks against its range; a drag outside it is shown as invalid. Arrow keys move a focused marker too. Chapters 3 and 7.

## Many similar objects

```text
space Plane = euclidean(2)

model Row in Plane {
  object Ball {
    param { m: Mass = 1 kg }
    state { pos: Point = origin }
  }
  parts { balls: Ball[5] { m = index * 1 kg; pos = origin + (index * 1 m, 0 m) } }
  derived { total: Mass = sum(b.m for b in balls) }
}

presentation RowView for Row {
  view scene: spatial(Plane, scale: 1 m -> 40 px, y: up) {
    for b in balls { marker(b.pos) as ball }
  }
  observe { total = total live }
}
```

```cases
run row of Row with RowView {
  expect { total == 15 kg within 1e-12 kg }     // 1 + 2 + 3 + 4 + 5
}
```

`object Ball { ... }` declares a type; `balls: Ball[5]` makes five, with `index` the number of each; `sum(... for b in balls)` adds over them; `for b in balls { ... }` draws one marker per ball. Members can also come and go during a run (`Drop[max 40]`, or `[max inf]` for no limit) and be joined by relations (springs, bonds). Chapter 9.

## More recipes in the chapters

| To ... | Use | Chapter |
|---|---|---|
| make something happen at an instant or when a value crosses zero | `event e on at t0 + 2 s { ... }`, `event e on falling(x) { ... }` | 5 |
| switch between modes (on and off, flying and resting) | `discrete { on: Boolean = true }` and `if on then ... else ...` in flows | 5 |
| check that a model keeps a law (energy, a bound) | `constraint ... policy report`, `equation E: a == b checked within tol` | 6 |
| put views side by side | `layout row(scene, column(plot, controls))` | 7 |
| color a shape or draw it dashed | `color: red`, `line: dashed` | 7 |
| add a button that makes the model do something | `event kick on request { ... }` and `button(kick)` | 7, 8 |
| tell a story with pauses for the learner | `timeline { scene s { beat b { ... } } }`, `explore ... fallback { ... }` | 8 |
| animate the camera or reveal a drawing | `camera scene to P zoom 2 for 1 s`, `reveal draw for 2 s { ... }` | 8 |
| let a learner add or remove objects | `on request(b in balls)`, `on click request remove(b)` | 9 |

## Run tests from the command line or in CI

Every `run` block is a test. The `prismal` command runs the cases of a file and prints `pass` or `FAIL` for each expectation; it exits with an error status when one fails, so it can gate a build:

```sh
cargo run -p prismal-web --bin prismal -- cases my-program.prismal
```

A Markdown file works too: its `text` and `cases` blocks form the program. That is how every program in this guide is tested.

## Export images and videos

```sh
cargo run --release -p prismal-media -- my-program.prismal Lesson frame.png --at 5
cargo run --release -p prismal-media -- my-program.prismal Lesson lesson.mp4 --fps 30
cargo run --release -p prismal-media -- my-program.prismal Lesson lesson.mp4 --speech system --captions both
cargo run --release -p prismal-media -- my-program.prismal Lab frames
```

A lesson plays as it would in a video, with each explore beat's fallback; a lab records its run from the start. Videos need ffmpeg; a name without a video extension writes the frames as PNG images instead, with the command that encodes them. Captions are drawn in, given as a subtitle track, or both, and always written beside the video as WebVTT, with a second WebVTT track describing the events shown. `--voice DIR` uses your own recordings, one per caption (`b3.wav`). Chapter 8.

## Embed Prismal in another program

Any program that can start a process and exchange lines of JSON can run Prismal: load a program, open a presentation, send the learner's input or its own values, and draw the frames Prismal describes. A Python example that drives a cart with its own thrust input:

```sh
cargo build --release -p prismal-stdio
python crates/prismal-stdio/client.py
```

The protocol and everything a host can do are in the [host interface](../spec/05-host-interface.md). Rust programs use the `prismal-host` crate directly; the web player is a host of the same interface, compiled to WebAssembly.
