# 1. A First Program

This chapter writes a complete program: a model of a wave `y(x) = A sin(k x)`, a presentation that plots it with sliders for `A` and `k`, and two test cases.

## The model

```text
model Wave {
  param {
    A: Real = 1     in [0, 3]       // amplitude
    k: Real = 1     in [0.5, 4]     // wave number
  }
  derived {
    y(x: Real): Real = A * sin(k * x)
    top: Real = A
  }
}
```

Reading it line by line:

- `model Wave { ... }` declares a model named `Wave`. Names of models, presentations and cases start with a capital letter by convention; the language does not require it.
- `param { ... }` is a block of **parameters**: values fixed for a run unless the learner or a lesson changes them. Each declaration is `name: Type = value`.
- `Real` is the type of plain numbers. Chapter 2 introduces quantities with units.
- `in [0, 3]` is the parameter's **range**: an interval. A value outside it is rejected, whether it comes from a test case, a slider or a lesson. Intervals use `[` and `]` for closed ends and `(` and `)` for open ends: `[0, 1)` means `0 <= x < 1`.
- `derived { ... }` is a block of **derived bindings**: values computed from other bindings, always up to date. `y(x: Real): Real = ...` is a derived function: it has a parameter `x` and reads the model's `A` and `k`.
- `//` starts a comment to the end of the line.

A block with one declaration can be written on one line: `param A: Real = 1 in [0, 3]` means the same as the block.

## The presentation

```text
presentation WavePlot for Wave {
  view graph: plot(x: [-6, 6], y: [-3, 3]) {
    function_graph(y)
  }
  panel controls {
    slider(A, range: [0, 3], step: 0.1)
    slider(k, range: [0.5, 4], step: 0.1)
    formula(y, live: true)
    label(top)
  }
  observe {
    y1   = y(1) live
    peak = top  live
  }
}
```

- `presentation WavePlot for Wave` declares how `Wave` is shown. The model never refers to its presentations.
- `view graph: plot(x: [-6, 6], y: [-3, 3])` is a **plot view** named `graph`, with fixed axis ranges.
- `function_graph(y)` draws the graph of the function `y` over the plot's `x` range.
- `panel controls { ... }` is a region without coordinates, for controls and text.
- `slider(A, range: [0, 3], step: 0.1)` is a **control**: moving it changes the parameter `A`. A control can only target something the model allows to change (parameters, by default).
- `formula(y, live: true)` typesets the definition of `y`, `y(x) = A sin(k x)`, with the current values of `A` and `k` beside it.
- `label(top)` shows a value as text.
- `observe { ... }` declares **observations**: named values a presentation records, which tests can check. `y1 = y(1) live` observes `y(1)` as it currently is; `peak = top live` observes the derived value `top`.

Every representation of the same binding shows the same value in the same frame: when the slider moves `A`, the graph, the formula and the label change together.

## Test cases

```cases
run defaults of Wave with WavePlot {
  expect { y1 == 0.841470984807897 within 1e-12 }
}

run steeper of Wave with WavePlot {
  param { A = 2; k = 2 }
  expect {
    y1 == 1.81859485365136 within 1e-12    // 2 sin(2)
    peak == 2 exactly
  }
}
```

- `run defaults of Wave with WavePlot { ... }` is a **case**: a run of `Wave`, observed through `WavePlot`.
- `param { A = 2; k = 2 }` overrides parameters for this case. Several statements on one line are separated by `;`.
- `expect { ... }` lists **expectations**. `y1 == 0.841470984807897 within 1e-12` passes when the observation `y1` is within `1e-12` of the value; `exactly` requires equality. An expectation is about an observation of the case's presentation (`y1`, `peak`); a case checks what its presentation observes.

The expected values come from outside the program: `sin(1) = 0.841470984807897`. A test that copies the implementation's output tests nothing.

## Running it

Paste the three blocks into the player's Source tab and compile. The graph appears with the two sliders; the Cases tab runs the two cases. From the command line, save the blocks in `wave.prismal` and run `cargo run -p prismal-web --bin prismal -- cases wave.prismal`.

## What can go wrong

A value outside a parameter's range is an error when the run starts:

```cases
run too_tall of Wave {
  param { A = 5 }
  expect { initialization fails }
}
```

`initialization fails` is itself an expectation: this case passes because the run is refused. It documents that the range is enforced.

A name that does not exist is reported by the compiler, with its line and column:

```error
// error: SX-E03
model Wave {
  param A: Real = 1
  derived y(x: Real): Real = B * x
}
```

## Exercises

Solutions to the exercises not solved here are in [chapter 11](11-solutions.md).

1. Add a parameter `c: Real = 0 in [-2, 2]` that shifts the wave vertically, `y(x) = A sin(k x) + c`, with a slider.
2. Add a case that sets `k = 0.5` and expects `y1 == sin(0.5)` (compute the number).
3. Write a model `Parabola` with parameters `a`, `b`, `c` and a derived function `p(x: Real): Real = a * x^2 + b * x + c`, plotted over `[-5, 5]`.

<details>
<summary>A solution to exercise 3</summary>

```text
model Parabola {
  param {
    a: Real = 1   in [-3, 3]
    b: Real = 0   in [-5, 5]
    c: Real = -2  in [-5, 5]
  }
  derived {
    p(x: Real): Real = a * x^2 + b * x + c
  }
}

presentation ParabolaPlot for Parabola {
  view graph: plot(x: [-5, 5], y: [-10, 20]) {
    function_graph(p)
  }
  panel controls {
    slider(a, range: [-3, 3], step: 0.1)
    slider(b, range: [-5, 5], step: 0.1)
    slider(c, range: [-5, 5], step: 0.1)
    formula(p, live: true)
  }
  observe { at2 = p(2) live }
}
```

```cases
run parabola of Parabola with ParabolaPlot {
  expect { at2 == 2 exactly }       // 1 * 4 + 0 * 2 - 2
}
```

</details>
