# 9. Reference

Every form of the working syntax on one page, with the chapter that introduces it. The normative description is `docs/syntax-study/working-syntax.md`; meanings are in `docs/spec/`.

## Top level

| Form | Meaning | Chapter |
|---|---|---|
| `space Plane = euclidean(2)` | a Euclidean space with axes `x`, `y` | 3 |
| `model Name [in Space] { ... }` | a model; `in Space` makes `Point`, `Vector<D>`, `origin` refer to that space | 1, 3 |
| `presentation Name for Model { ... }` | a presentation of a model | 1, 7 |
| `run Name of Model [with Presentation] { ... }` | a test case | 1, 6 |

## Model

| Form | Meaning | Chapter |
|---|---|---|
| `const { x: T = e }` | constants: never changed | 2 |
| `param { x: T = e [in I \| where c] [modifiers] }` | parameters: changed only by cases, controls and lessons, within their range | 1 |
| `state { x: T = e [intervenable] }` | continuous state, with its initial value | 4 |
| `discrete { x: T = e }` | discrete state: changed only by events | 5 |
| `derived { x: T = e; f(x: A): B = e }` | derived values and functions, always current | 1 |
| `flow { der(x) = e; der(y) += e }` | rate of change: one definition, or summed contributions | 4 |
| `process Name { flow ...; event ... }` | a named group of flows and events | 4 |
| `event E on trigger [if c] [{ ops }] [zeno settle { ops } \| zeno stop]` | an event | 5 |
| `equation N: a == b [checked within tol]` | a stated relation, shown and optionally checked | 6 |
| `constraint N: c [within tol] [policy reject \| report \| stop]` | a checked condition (`within` for `==` only) | 6 |

Each block keyword also has a one-line form: `param g: Acceleration = 9.81 m/s^2`, `flow der(x) = v`.

**Modifiers:** `symbol "θ"` (display symbol), `unit deg` (display unit of an angle), `intervenable` (state the learner may change), `private`.

**Triggers:** `rising(g)`, `falling(g)`, `crossing(g)` (zero crossings of `g`), `at τ`, `every Δ [from τ0]`, `start`, `request`, `E` (after event `E` occurs or is emitted), `input(i)`.

**Operations:** `set x = e`, `emit E`, `contribute x += e` (contributions to discrete state are not implemented by the prototype, MK-E11).

## Types and values

| Form | Meaning | Chapter |
|---|---|---|
| `Real`, `Boolean`, `Angle` | numbers, truth values, angles (dimensionless) | 1, 2 |
| `Length`, `Mass`, `Time`, `Velocity`, `Acceleration`, `Force`, `Energy`, `Power`, `Pressure`, `Frequency`, `Momentum`, `Area`, `Volume`, `Current`, `Amount` | named dimensions | 2 |
| `Quantity<M/T^2>` | any dimension from `L M T I Θ N J` | 2 |
| `Point`, `Vector<Length>`, `Vector<Velocity>` | geometry in the model's space | 3 |
| `9.81 m/s^2`, `45 deg`, `2π`, `0` | literals; `0` has any dimension and is the zero vector | 2, 3 |
| `origin + (1 m, 2 m)` | a point | 3 |
| `v.x`, `\|v\|` | component, norm | 3 |
| `[a, b]`, `(a, b)`, `[a, b)`, `(a, inf)` | intervals | 1 |
| `if c then a else b` | conditional | 4 |
| `t`, `t0`, `elapsed` | current time, start, time since start | 4 |
| `sin cos tan sqrt exp log abs min max atan2` | functions | 2 |

Operators: `+ - * / ^`, `== != < <= > >=`, `and or not`. Units: `m cm mm km s ms min h kg g N J W Pa Hz A K mol cd rad deg rev`.

## Presentation

| Form | Meaning | Chapter |
|---|---|---|
| `view n: spatial(Space, scale: 1 m -> 40 px, y: up) { reps }` | spatial view | 3 |
| `view n: plot(x: [a, b], y: [c, d]) { reps }` | plot view; ranges give the axes their dimensions | 1, 4 |
| `panel n { reps }` | region for controls and text | 1 |
| `permit learner { zoom; pan; timeline_controls }` | what the learner may do with views and time | 7, 8 |
| `observe { n = e schedule }` | observations | 1, 6 |
| `timeline { scene s { beat b { actions } } }` | a lesson | 8 |

**Representations:** `marker`, `arrow`, `segment`, `trace(P every Δ)`, `function_graph`, `series_plot(e every Δ)`, `axes`, `grid`, `label`, `formula`, `slider`, `number_input`, `toggle` (chapter 7). `as name` names one; `{ on drag [head] as p { propose x = e } }` declares its inverse.

**Observation schedules:** `live`, `every Δ`, `at τ`, `on E [microstep n]`, `over [a, b]`. **Sources:** any expression, `event_log [of E] [where zeno_applied]`, `diagnostics [of element]`, `intervention_log`.

**Timeline actions:** `narrate "..." [for d]`, `run rate r [until E]`, `hold`, `seek τ`, `reset`, `branch`, `highlight name`, `in view { reps }`, `show rep`, `intervene { set p = e }`, `request E`, `wait d`, `sequence { ... }`, `explore [limit L] [keep p, ...] { controls } [fallback { actions }]` (chapter 8).

## Runs

```prismal
run Name of Model with Presentation {
  param  { p = value; ... }
  config { solver = rk4; h = 0.01 s }        // or rtol = ...; atol = ...
  until t0 + 10 s
  learner { at 15 s: set slider h = 20 m; at 16 s: continue }
  expect { ... }
}
```

**Expectations** (chapter 6): `a == b within tol`, `a == b within rel r`, `a == b exactly`, `a in [lo, hi]`, `obs[k]`, `(e on E [microstep 0])`, `log == [E1, E2]`, `start of b`, `end of b`, `initialization fails`, `configuration rejected`.

## Words

**Reserved** (never names): `space model presentation run object const param input state discrete derived fn flow process event equation constraint on if then else and or not otherwise in where true false zeno stop settle set contribute create destroy connect disconnect emit enter checked within policy reject report intervenable private symbol unit rising falling crossing at every from start request`.

**Contextual** (keywords only where expected, names elsewhere): `for view panel observe live over microstep show as drag propose permit timeline scene beat sequence rate until hold seek reset branch intervene wait explore limit keep fallback narrate highlight animate camera bind release config expect exactly rel of with learner continue`.

## Diagnostics

**Syntax** (`SX`): E01 lexical error, E02 syntax error, E03 unknown name, E04 unknown type or dimension, E05 unknown unit, E06 not in the v0 IR, E07 reserved word used as a name, E08 form not allowed here, E09 duplicate declaration.

**Model** (`MK`):

| Code | Meaning |
|---|---|
| MK-E01 | dimension or type mismatch |
| MK-E02 | dimensioned argument to a dimensionless-only function |
| MK-E03 | bare number where a quantity is required |
| MK-E04 | invalid point operation (point + point) |
| MK-E05 | values from different spaces combined |
| MK-E06 | operation or flow targets something that cannot change that way |
| MK-E07 | continuous state set outside a handler or intervention |
| MK-E08 | handler sets a parameter |
| MK-E10 | target both defined and contributed to |
| MK-E11 | contribution to a target with no combination |
| MK-E12 | flow condition depends on continuous state or `t` |
| MK-E13 | level condition used as a continuous trigger |
| MK-E14 | algebraic loop |
| MK-E15 | cycle among initial definitions |
| MK-E16 | repeating crossing event without a Zeno policy |
| MK-E19 | two `set` operations on one target in one handler |
| MK-E20 | stored function value that reads bindings |
| MK-E21 | tuple used as a vector where no vector is expected |
| MK-E22 | target defined twice |

**Presentation** (`PK`): E01 broken reference, E02 expression does not check (the kernel's message is quoted), E03 control or action targets something the learner may not change, E04 missing or wrong scale or axis dimension, E05 representation unsuited to its sources or view, E06 representation kind not implemented.

**Run diagnostics** have categories rather than codes: configuration, initialization, constraint, equation, conflict, cascade, Zeno, computational, intervention (`docs/spec/02-runtime-contract.md` section 10).
