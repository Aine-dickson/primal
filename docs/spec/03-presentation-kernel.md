# 03 Presentation Kernel

- **Version:** v0 (draft), 2026-09-29, revised 2026-09-30
- **Part:** presentation kernel (D-011). It reads the model through the model kernel's interface (MK section 17) and the runtime's observer output (RC section 15), and directs runs through execution control (RC section 12).
- **Scope:** how a model is observed, projected into representations, arranged in views, narrated over time, and manipulated by a learner or author. How pixels, audio or video frames are produced is a renderer concern outside this specification.

Every section follows accepted decisions (including D-025 and D-026, raised by this document, and D-033 and D-034, raised by the syntax study) or carried-forward resolutions.

## Contents

1. Overview
2. Presentations
3. Observation and data
4. Expectations
5. Projection
6. Representation
7. View
8. Presentation time and animation
9. Explanation timeline
10. Interaction
11. Accessibility
12. Output and media
13. Check against the first-slice reference programs
14. New positions taken in this document
15. Deferred

---

## 1. Overview

- **Restates:** R-07, R-19, R-20, R-37, R-38, R-39, R-40, R-42.
- **Decisions:** D-009, D-011, D-017, D-018, D-019.
- **Prior art:** follows the grammar-of-graphics separation of data, encoding and rendering (Wilkinson; Vega-Lite). Follows Manim and Motion Canvas for author-driven animation over a timeline. Follows PhET simulations for learner-facing interactive models with separate model and view code.

The presentation kernel has these concepts:

| Concept | Role | Section |
|---|---|---|
| Presentation | one way of presenting one model; a model may have several | 2 |
| Observation, data | reading model state into information | 3 |
| Expectation | an observation with an expected result | 4 |
| Projection | mapping sources to representations | 5 |
| Representation | what is shown: a marker, arrow, plot, equation, control | 6 |
| View | a region with a coordinate system, camera and layout | 7 |
| Animation | change of presentation properties over presentation time | 8 |
| Explanation timeline | scenes, beats, reveals, camera, narration, and run direction | 9 |
| Interaction | input turned into model, runtime, view or observation actions | 10 |

- **PK-1.1** Presentation state (camera, selection, visibility, animation progress, layout) is never model state (R-20, R-37 invariants 3, 11).
- **PK-1.2** Nothing in the presentation kernel changes the model except interventions (MK-17.2), which pass through the runtime's validation (RC section 11).
- **PK-1.3** A representation does not own scientific truth. Values shown come from committed model state, dense output or data; a representation MAY compute presentation values from them (an arrow length from a velocity) but never feeds such values back into the model (07 sections 66, 111).

---

## 2. Presentations

- **Restates:** R-39.
- **Decisions:** D-011, D-018, D-019.

- **PK-2.1** A **presentation** is a named, serializable unit that refers to one model and contains: observations, expectations, projections, views, a layout of views, interaction permissions, and optionally an explanation timeline.
- **PK-2.2** A model MAY have any number of presentations (D-011). Presentations do not affect each other or the model. The serializable form keeps them separate from the model (D-019).
- **PK-2.3** A presentation refers to model elements only by identity (MK-7.5), through the model's public interface. A presentation that refers to an element the model no longer has is invalid and reports which references are broken.
- **PK-2.4** Presentation configuration (colors, scales, layout, pacing) is distinct from model parameters (R-37 invariant 20). Changing it never changes model results.

---

## 3. Observation and data

- **Restates:** R-19, R-23, R-31, SEM-05 (R-62).
- **Decisions:** D-011, D-015.
- **Prior art:** follows data-logging practice in physics education (Vernier and PASCO loggers: sampled series with units and timestamps). Follows the W3C PROV idea of recording where each datum came from.

### 3.1 Observations

- **PK-3.1** An **observation** reads model state and produces data. It has identity, a **source** (an expression over public bindings, equation residuals, event payloads or structure, MK-17.1), and a **schedule**:

| Schedule | Produces |
|---|---|
| `live` | the current value at the displayed instant |
| `every(Δ)` | a series sampled at simulation instants `t0 + kΔ`, from the dense output |
| `at(τ)` | one value at simulation instant `τ` |
| `on(E)` | one value at each occurrence of model event `E`, read from the committed state at the event's final microstep, or at a named microstep |
| `over(τ1, τ2)` | a series of every committed event instant and dense-output sample in the interval |

- **PK-3.2** Observations never change the model or the trajectory (R-19, RC-15.2). Sampling from the dense output does not add solver steps.
- **PK-3.3** An observation's value may be a status (MK section 5). Data records statuses as statuses, never as numbers.
- **PK-3.4** `on(E)` resolves the model kernel's open item (MK 19.1): a quantity evaluated at an event instant, such as the projectile's range at landing, is an observation `pos.x on(landed)`. An equation's residual at an event is observed the same way.

### 3.2 Data

- **PK-3.5** **Data** is the result of observations: values with their types and units, the simulation instants they refer to (superdense where relevant), and provenance (run identity, observation identity, schedule).
- **PK-3.6** **Analysis** functions (min, max, mean, interpolation, fitting, differentiation of series) take data and produce derived data with provenance. They are pure functions, provided by libraries (05c section 53).
- **PK-3.7** An observation MAY declare an **instrument**: resolution (values rounded to it), range (values outside are `unavailable`), and a stated uncertainty carried with each value. Instrument noise, which needs randomness, is deferred with the stochastic slice (D-013).

---

## 4. Expectations

- **Restates:** R-19.
- **Decisions:** D-012, D-017.
- **Prior art:** follows unit-testing assertions with tolerances (for example `assert_relative_eq` in Rust's `approx` crate). Follows PhET and other inquiry-based lessons, where learners predict an outcome before observing it (08 section 81).

- **PK-4.1** An **expectation** is an observation with an expected value (an expression, possibly over parameters) and a tolerance (absolute, relative, or both). Evaluating it gives `pass`, `fail` (with the observed value and the difference), or the observation's status.
- **PK-4.2** Reference programs (D-012) state their checkable results as expectations. A headless run evaluates them and reports each result.
- **PK-4.3** An expectation MAY take its expected value from the learner (a prediction entered through a control before the run). The same mechanism then serves self-study and assessment (D-017), without the model knowing about either.

---

## 5. Projection

- **Restates:** R-37, R-38, SEM-05, SEM-06 (R-62).
- **Decisions:** D-018, D-020.
- **Prior art:** follows Vega-Lite encodings (a field mapped through a scale to a visual channel). Follows D3's data join for identity-preserving updates of collections.

- **PK-5.1** A **projection** is `Projection(source, context, configuration) -> representation` (R-38). The source is model state (directly, MK-17.1) or data (section 3). The context is the view, its coordinate mapping and the displayed instant. The configuration is presentation configuration (PK-2.4).
- **PK-5.2** Projections are pure: the same source values, context and configuration give the same representation.
- **PK-5.3** A projection MAY be many-to-one (one representation from several sources) or one-to-many (several representations from one source) (SEM-05).
- **PK-5.4** A projection over a collection produces one representation per member. Each representation's identity is derived from its member's identity (MK-7.5), so members that persist keep their representations when the collection changes, and the representation of a destroyed member is removed (R-37 invariant 18 in reverse: removing the source removes its representation; removing a representation never removes its source).
- **PK-5.5** **Encodings** map source values to presentation values: a spatial mapping (model space to view space, section 7), a scale (quantity to length, color or size), or formatting (quantity to text in a display unit, MK-3.7). Encodings are unit-aware: mapping a `Quantity<L/T>` to an arrow length requires a declared scale with that dimension (for example `1 m/s -> 20 px`).
- **PK-5.6** A projection is **invertible** only if it declares an **inverse**: a function from a presentation value (a pointer position, a slider position) to a proposed source value. Invertibility is declared, never inferred, and is independent of whether interaction is permitted (SEM-06, R-37 invariants 9, 10).
- **PK-5.7** An inverse MAY map onto a restricted set, such as a point constrained to a curve through the curve's parameter. This is how constrained dragging is provided while model constraints are checked, not enforced (D-020).

---

## 6. Representation

- **Restates:** R-37, R-59.
- **Decisions:** D-018, D-034.
- **Prior art:** follows SVG and the Manim object model for geometric primitives and grouping. Follows MathML and TeX for equation layout from a symbolic tree.

- **PK-6.1** A **representation** is a presentation element with identity, a kind, properties, and the sources its properties are bound to. It is not pixels (R-37 invariant 2); a renderer turns it into output. A representation MAY carry an author name, unique within its presentation, by which timeline actions and interactions refer to it; the name is not its identity (MK-6.2).
- **PK-6.2** Each representation property is either **bound** (computed by a projection from sources), **set** (a constant in the presentation configuration), or **animated** (driven by presentation time, section 8). A property has exactly one of these at a time.
- **PK-6.3** The **first-slice representation set**:

| Kind | Shows | Typical source |
|---|---|---|
| `marker` | a point (dot, circle, custom glyph) | `Point<S>` |
| `arrow` | a vector drawn from a point | `Vector<S, D>` with an anchor point |
| `segment`, `polyline`, `polygon` | straight geometry | points |
| `trace` | the path of a point over time | observation `over` or `every` of a point |
| `function_graph` | the graph of `f : Real -> Real` or of a quantity function over a domain | function binding |
| `series_plot` | data against an axis (for example `x(t)`) | data |
| `axes`, `grid` | coordinate reference | view coordinate mapping |
| `label` | text, optionally with a value readout | text, any binding |
| `equation` | a typeset model equation, with symbols linked to bindings | equation (MK section 11) |
| `formula` | a typeset expression, a definition, or a labeled expression (`R = ...`), with symbols linked to bindings | expression, derived binding, derived function value (D-034) |
| `table` | rows of values | data |
| `slider`, `number_input`, `toggle`, `button` | a control | intervenable binding; for `button`, a requestable event (D-027) or a runtime control |
| `group` | a set of representations with a shared transform | representations |

- **PK-6.4** Controls are representations of bindings with a declared inverse (PK-5.6): a slider shows a parameter's value and, when moved, proposes a new one. A control can only target an intervenable binding (D-023).
- **PK-6.5** `equation` and `formula` representations are typeset from symbolic forms in the IR (MK-10.6), never from strings. A `formula` of a derived binding or derived function value shows its definition (`f(x) = a x²`); a `formula` with a label shows `label = expression`, where the label is presentation text and not a binding (D-034). Each symbol that refers to a binding carries that binding's identity, so views can highlight a symbol with the value it stands for, and a label can show live values substituted into the equation.
- **PK-6.6** A representation may exist without a model source (a title, an annotation) (R-37 invariant 19). A line between two objects is not a relation unless the model declares one (R-59).
- **PK-6.7** Libraries MAY add representation kinds. A new kind declares its properties, the source types it accepts, and a description of how a renderer draws it in each supported medium (section 12).

---

## 7. View

- **Restates:** R-07, R-37.
- **Decisions:** D-018, D-022.
- **Prior art:** follows camera and viewport separation in graphics (world, view and screen space) and responsive layout practice on the web.

- **PK-7.1** A **view** is a region of the presentation with its own coordinate system, a set of representations, and presentation state: camera, visibility, selection, and zoom.
- **PK-7.2** A **spatial view** shows one model space (D-022). Its **coordinate mapping** takes model coordinates in a chosen frame of that space to view coordinates: origin, scale (for example `1 m -> 100 px`), rotation and orientation (for example `y` up). Camera movement changes the mapping, never model values (R-07: model space is not render space).
- **PK-7.3** A **plot view** has axes, each bound to a dimension and a display unit, with a range that is fixed, follows the data, or is controlled by the camera.
- **PK-7.4** **Layout** places views in the presentation and adapts them to the output's size and orientation. Layout never changes what a view shows, only where and how large.
- **PK-7.5** **Consistency across views**: every view in one presented frame shows the same simulation instant (and the same microstep when one is selected), and the same committed state or dense-output value. A view never shows a newer state than another view of the same frame (07 section 96).

---

## 8. Presentation time and animation

- **Restates:** R-23, R-31, R-32.
- **Decisions:** D-009, D-015.
- **Prior art:** follows Manim and Motion Canvas (animations as functions of presentation time with easing curves) and the Web Animations model (timing, easing, fill).

- **PK-8.1** **Presentation time** is the clock of a playback or export (RC-2.1). Every presented frame is taken at a presentation instant.
- **PK-8.2** The **time mapping** relates presentation time to simulation time: at each presentation instant the presentation shows one simulation instant. The mapping is piecewise: playing at a rate (simulation seconds per presentation second, `1` for real time), holding (the simulation paused while presentation time runs, for narration), or jumping (a seek). Rates are presentation configuration and never change results (RC-2.3).
- **PK-8.3** Between committed instants, the displayed state is the runtime's dense output at the mapped simulation instant (RC-15.1). A presentation MUST NOT add solver steps to get smoother frames (RC-6.6).
- **PK-8.4** An **animation** changes an animated property (PK-6.2) over an interval of presentation time: from a value, to a value, with a duration and an easing function. Typical uses: reveal (fade, draw), transform (move, morph one shape into another), emphasis (highlight, pulse), camera moves.
- **PK-8.5** An animation cannot drive a bound property. To animate the appearance of a model-bound element, the presentation animates a separate property (opacity, style, a presentation offset) or temporarily replaces the binding by an explicit handover (section 9, `release` and `bind`).
- **PK-8.6** A presentation without a model clock (a static model, or a pure animation with no model) uses presentation time alone (R-32).
- **PK-8.7** Given the model, run configuration, presentation, and intervention log, presented frames are deterministic: the same presentation instant shows the same content. Video export relies on this (D-015, D-018).

---

## 9. Explanation timeline

- **Restates:** R-22, R-23, R-32.
- **Decisions:** D-009, D-017, D-018, D-025, D-027, D-033.
- **Prior art:** follows Manim scenes and Motion Canvas generator timelines (author-sequenced animation with waits). Follows interactive-video and slide tools for learner-paced continue points. Departs from both by synchronizing with a live simulation through events rather than fixed times.

The explanation timeline is a peer of the model (D-009): it observes the model and directs its runs; the model never depends on it.

### 9.1 Structure

- **PK-9.1** A timeline is a sequence of **scenes**. A scene is a sequence of **beats**. A beat is a set of **actions** that start together, and it ends when all of its actions have finished. Actions MAY be grouped as a sequence within a beat.
- **PK-9.2** Action kinds:

| Action | Effect |
|---|---|
| `show`, `hide`, `reveal(style)` | make representations visible, with an optional animation |
| `animate` | an animation of presentation properties (PK-8.4) |
| `camera` | move a view's camera (an animation of its mapping) |
| `highlight` | emphasize representations, equation symbols or bindings |
| `narrate(text)` | a narration cue: caption text and, optionally, recorded or synthesized audio; ends when the audio or the reading time ends |
| `run(rate)`, `hold` | set the time mapping (PK-8.2): play the simulation at a rate, or pause it |
| `seek(τ)`, `reset`, `branch` | execution control on the lesson's run (RC section 12) |
| `intervene(ops)` | an intervention on intervenable bindings (MK-17.2), such as setting the launch angle for the next demonstration |
| `request(E)` | request a requestable model event (MK-17.2a, D-027), such as relaunching the projectile |
| `wait(d)` | wait a presentation duration |
| `wait_until(E)` | wait until model event `E` occurs on the lesson's run |
| `wait_until(t = τ)` | wait until the simulation reaches instant `τ` |
| `wait_for_learner` | wait for the learner to continue (a continue point) |
| `explore` | open a learner exploration period (section 9.3) |
| `bind`, `release` | hand a representation property between a projection and an animation (PK-8.5) |

- **PK-9.2a** **Order at a beat's start** (D-033). A beat's run-directing actions (`seek`, `reset`, `branch`, `intervene`, `request`, `run`, `hold`) take effect in the order written, at the beat's start and before any presentation time passes. The beat's other actions then start together and see the resulting state. `sequence` orders actions that take time; it is not needed for run-directing actions.
- **PK-9.3** `wait_until(E)` with the simulation running makes presentation duration depend on the model: the beat lasts exactly as long as the simulation takes to reach `E` at the current rate. Because runs are deterministic (RC-14.5), this duration is known before playback and is identical in export.
- **PK-9.4** If a `wait_until` can never be satisfied (the run ends or stops first), the timeline reports it at the moment the run ends, continues with the next beat, and records a diagnostic. It never hangs.

### 9.2 The lesson's run

- **PK-9.5** A timeline directs one **lesson run** of its model. Timeline actions act on that run. The lesson run's configuration is part of the presentation.
- **PK-9.6** The timeline reads the model only through observation and events (MK-17.1) and changes it only through interventions (MK-17.2). The model runs unchanged with the timeline removed (D-009, MK-17.5).

### 9.3 Learner control

- **PK-9.7** During a narrated lesson, the learner always has runtime controls over the **timeline**: pause, resume, replay a beat or scene, change narration speed. View actions (zoom, pan) are available where the presentation permits them. These never change the lesson run's results.
- **PK-9.8** Model actions by the learner (interventions) are allowed only inside `explore` beats. An `explore` beat:
  - branches the lesson run at the current instant (RC-12.3); the learner's interventions and run controls act on the branch;
  - declares which controls and intervenable bindings are available, a subset of the presentation's permissions;
  - ends when the learner continues, or after a declared time limit.
- **PK-9.9** When an `explore` beat ends, the timeline returns to the lesson run at the instant where the branch was made, unless the beat declares `keep`, in which case the lesson continues on the learner's branch. A `keep` beat declares which bindings the learner sets; later narration can refer to them (for example "you chose `θ = 50°`").
- **PK-9.10** In linear media (video), `explore` beats and `wait_for_learner` are replaced by their declared fallback: a fixed-duration pause, a scripted intervention demonstrating the exploration, or omission (section 12).

### 9.4 Education layer boundary

- **PK-9.11** Questions, answers, feedback and scoring are the education layer above the core (R-22). They are deferred; the timeline provides what they need: continue points, explore beats, expectations with learner predictions (PK-4.3), and observation of learner interventions.

---

## 10. Interaction

- **Restates:** R-33, R-40, R-41, SEM-07, SEM-08 (R-62).
- **Decisions:** D-020, D-023.
- **Prior art:** follows the model-view-controller separation of input handling from model state, the command pattern for undo (Gamma and others), and GeoGebra and PhET for dragging model elements with validation.

### 10.1 The path from input to action

- **PK-10.1** **Input** (pointer, touch, keyboard, device) is not interaction (R-40). An **interaction** interprets input under a mode or tool, resolves a **target**, and produces an **action** in one of four destinations (08 section 5, R-33):

| Destination | Action | Passes through |
|---|---|---|
| model | intervention | runtime validation (RC section 11) |
| runtime | execution control | RC section 12 |
| view | camera, selection, visibility, layout | presentation state only |
| observation | probe, measure, read out | observation (section 3) |

- **PK-10.2** **Targeting** resolves input to a representation (hit testing in view coordinates), then to the representation's source through its binding. A model action names its semantic target, never a visual one (R-40 invariant 5).
- **PK-10.3** When a target is ambiguous (overlapping representations), resolution uses a declared order (topmost, then nearest), and the interaction MAY offer a choice. It never picks by internal order.

### 10.2 Permissions

- **PK-10.4** A presentation declares which interactions it offers: which bindings are controllable and through which representations, which runtime controls appear, which view actions are allowed. A presentation can offer less than the model permits, never more: model actions target only intervenable bindings (D-023).

### 10.3 Direct manipulation

- **PK-10.5** Dragging a representation is a **transaction**: begin, preview, commit or cancel (R-41). During preview, the representation's inverse (PK-5.6) turns each pointer position into a **proposed** value (SEM-08).
- **PK-10.6** Proposed values are validated against the model's constraints as they arrive (MK section 12). A proposed value that a `reject` constraint would reject is shown as invalid (the drag stops at the boundary or is marked), and is never committed.
- **PK-10.7** Preview shows the effect of the proposed value on derived bindings, computed from the proposed state without committing it. Preview state is presentation state.
- **PK-10.8** On release, the last valid proposed value is committed as one intervention (RC-11.1). On cancel, nothing is committed. Only commits are logged (RC-11.5).
- **PK-10.9** Default drag mode for a running simulation is `hold`: the lesson or exploration run pauses at the current instant when the drag begins, and resumes after the commit. A presentation MAY choose `live` mode, where each proposed value is committed as a separate intervention while the run advances (for steering). `live` commits are logged individually.

### 10.4 Undo and redo

- **PK-10.10** Undo of an intervention on a run is a seek to the intervention's instant with the intervention removed from the log (RC-11.2): the trajectory after that instant is recomputed. Redo restores it. For a static model this is the same as returning to the previous committed state.
- **PK-10.11** View actions have their own undo history in presentation state and do not enter the run's log.

---

## 11. Accessibility

- **Restates:** R-37 (non-visual representations, 07 sections 70-71).
- **Decisions:** D-017, D-018, D-026.
- **Prior art:** follows WCAG 2.2 (text alternatives, keyboard operability, not relying on color alone). Follows PhET's accessible simulations, which describe a simulation's state in text for screen readers and let every interaction be done by keyboard.

- **PK-11.1** Every representation that conveys model information has a **text alternative**, generated by default from its sources: the binding's label, value and display unit, or for a group, a summary of its members. Authors MAY override it.
- **PK-11.2** Every model action and runtime control a presentation offers is available without a pointer (keyboard or equivalent), including dragging: a draggable marker is movable by steps along its inverse (PK-5.6).
- **PK-11.3** Narration has captions (text is required; audio is optional). Timeline beats and event occurrences can be announced to assistive technology.
- **PK-11.4** Color is never the only encoding of a value: a color encoding is paired with a value readout on demand, a legend, or another channel.

---

## 12. Output and media

- **Restates:** R-37 (invariants 2, 14), R-39.
- **Decisions:** D-018.
- **Prior art:** follows the display-list separation between scene description and renderer (SVG, PDF, Vello scenes), and export pipelines that render a deterministic timeline frame by frame (Manim, Motion Canvas).

- **PK-12.1** The presentation produces, for each presentation instant, a **frame description**: the visible representations with their resolved properties, in view coordinates, plus captions, text alternatives and pending announcements. It is medium-independent.
- **PK-12.2** A **renderer** turns frame descriptions into a medium: interactive web player (first target, D-018), later still image, video, vector document, data export. A renderer MUST NOT define representation semantics (R-37 invariant 14).
- **PK-12.3** Each medium declares what it supports: interaction, audio, time. A presentation element that a medium cannot support uses its declared fallback (PK-9.10); an element with no fallback is reported, not silently dropped.
- **PK-12.4** Which forms and media a piece of content uses follows the content and the author's intent (D-018, R-39); the specification makes every medium one target among several.

---

## 13. Check against the first-slice reference programs

Each program (D-012) is checked for expressibility in the presentation kernel. Examples are in the working syntax (D-028), non-binding; the complete programs are in `reference-programs/`.

### 13.1 Projectile, with and without drag

```text
presentation ProjectileLab for Projectile {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    marker(pos)
    arrow(vel, from: pos, scale: 1 m/s -> 4 px)
    trace(pos every 0.02 s)
  }
  view graph: plot(x: elapsed, y: pos.y) {
    series_plot(pos.y every 0.02 s)
  }
  observe { range = pos.x on landed }
}

run no_drag of Projectile with ProjectileLab {
  param  { k = 0 }
  expect { range == speed^2 * sin(2 * angle) / g within rel 1e-6 }
}
```

- The range at landing is an `on(landed)` observation (PK-3.4). The model kernel's gap (MK 19.1) is closed.
- The reference result is an expectation (PK-4.2). The no-drag result holds only when `k = 0`, so it belongs to a named case with that override, not to a conditional expectation (section 15).

### 13.2 Bouncing ball, pendulum, spring-mass

Marker, trace and series plot as in 13.1; energy as a `series_plot` of the derived `energy` binding. Expectations: bounce times `on(bounce)` against the geometric series; period from successive `on(...)` observations of a crossing event (for example an upward crossing of `θ`); energy drift as `max - min` of the `energy` series (an analysis, PK-3.6). Expressible.

### 13.3 Function plot with a draggable parameter

```text
presentation QuadraticPlot for QuadraticDemo {
  view plot: plot(x: [-3, 3], y: [-5, 20]) {
    function_graph(f)
    marker(at: (1, f(1))) {
      on drag as p { propose a = p.y / 1^2 }      // x fixed at 1
    }
  }
  panel controls {
    slider(a, range: [-5, 5], step: 0.1)
    formula(f, live: true)                        // f(x) = a x², D-034
  }
}
```

- The slider and the draggable point both target `a`, the only intervenable binding (D-023). The point's inverse maps a pointer position to a proposed `a` (PK-5.6).
- The constraint `-5 <= a <= 5` (policy `reject`) stops the drag at the boundary (PK-10.6).
- All views show the same committed state (PK-7.5); the formula shows the live value of `a` (PK-6.5). Expressible.

### 13.4 Vector addition

Arrows for `u`, `w`, `sum`; draggable arrow heads with inverse `head -> vector = head - tail`, only if the model marks `u` and `w` intervenable. Invalid operations are model-kernel static errors (MK-E03, MK-E04). Expressible.

### 13.5 Narrated projectile lesson

```text
timeline {
  scene launch {
    beat { in scene { marker(pos) as ball }; narrate "A ball is launched at 45 degrees." for 4 s }
    beat { run rate 1 until landed }
    beat { hold; highlight ball; narrate "It lands here. Why this distance?" for 3 s }
    beat {
      seek t0                                      // applies before the formula appears (PK-9.2a)
      show formula("R", speed^2 * sin(2 * angle) / g, live: true)
      narrate "Watch the horizontal speed." for 3 s
    }
    beat { in scene { arrow((vel.x, 0), from: pos) }; run rate 0.5 until landed }
  }
  scene try_it {
    beat { explore limit 60 s keep angle { slider(angle, range: [10 deg, 80 deg]) } }
    beat { request relaunch; run rate 1 until landed; narrate "Compare with your prediction." for 3 s }
  }
}
```

- The timeline waits on `landed`, holds, seeks back and replays (PK-9.2, PK-9.3), with the same trajectory both times (RC-12.2).
- The `explore ... keep` beat lets the learner set the launch angle, then continues the lesson with it (PK-9.9). Without `keep`, the lesson would return to its own run. In video, the explore beat uses its fallback.
- Gap (resolved): an earlier draft of the model exposed the launch velocity as one vector `v0`, which a slider for the angle cannot target. The model now has `speed` and `angle` parameters (MK 19.1, RP-01), and relaunching is a requestable event (D-027).

### 13.6 Findings

| Program | Presentation kernel |
|---|---|
| Projectile, with and without drag | Expressible; conditional expectations noted |
| Bouncing ball, pendulum, spring-mass | Expressible |
| Function plot | Expressible |
| Vector addition | Expressible |
| Narrated lesson | Expressible |

The model kernel's open item (checks at an event instant) is resolved by `on(E)` observations and expectations.

---

## 14. New positions taken in this document

**Register entries:**

| Entry | Position | Sections |
|---|---|---|
| D-025 | Learner model actions during a narrated lesson happen only in `explore` beats, on a branch; the lesson returns to its own run unless the beat keeps the learner's choice (Accepted) | 9.3 |
| D-026 | Accessibility baseline in v1: generated text alternatives, keyboard operation of every control and drag, captions, no color-only encoding (Accepted) | 11 |
| D-033 | Run-directing actions in a beat apply in written order before presentation time passes (Accepted; raised by syntax study S-8) | 9.1 |
| D-034 | `formula` representation for expressions, definitions and labeled expressions; never strings (Accepted; raised by syntax study S-10) | 6 |

**Elaborations (in this specification only):**

- Observation schedules, including `on(E)` at event instants (PK-3.1, PK-3.4).
- Expectations as the form of reference-program results and learner predictions (section 4).
- Unit-aware encodings with declared scales (PK-5.5); invertibility declared, never inferred (PK-5.6).
- A representation property is bound, set or animated, one at a time (PK-6.2, PK-8.5).
- Controls are representations with inverses (PK-6.4).
- All views of a frame show one simulation instant (PK-7.5).
- Frames between steps come from dense output, never from extra steps (PK-8.3).
- `wait_until` never hangs (PK-9.4).
- Drag default `hold` for running simulations; only commits are logged (PK-10.8, PK-10.9).
- Undo as seek with the intervention removed (PK-10.10).
- Medium fallbacks declared, never silently dropped (PK-12.3).

---

## 15. Deferred

| Item | Where it will be specified |
|---|---|
| Education layer: questions, answers, feedback, scoring (R-22) | Education layer specification |
| Instrument noise (needs randomness, D-013) | Stochastic slice |
| Field representations (heatmaps, vector fields, D-010) | Slice with field state |
| 3D views and projection from 3D to 2D | Slice that needs them |
| Construction tools (creating points, lines, objects by interaction; 08 sections 43-46) | Slice with dynamic collections and relations |
| Conditional expectations (`expect ... when k = 0`) (13.1) | Resolved: expectations belong to named cases (run configurations), `reference-programs/README.md` |
| Video, image and vector renderers | After the web player (D-018) |
| Frame description format and renderer interface | Implementation specification |
| Narration audio (recorded or synthesized), language and localization | Later |
