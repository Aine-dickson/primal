# Learn Prismal

A guide to writing Prismal programs in the working syntax (D-028), from a first formula to a narrated lesson. Each chapter introduces a few forms, shows a complete program that uses them, and ends with exercises. After the last chapter, `10-reference.md` lists every form on one page.

- **Status:** living. Written 2026-09-30 against the prototype (`crates/`). The syntax is the working syntax: usable and implemented, not yet frozen.
- **Checked:** every program in the guide is compiled and every case in it is run by `cargo test` (`crates/prismal-web/tests/guide.rs`). A program that fails, or an example error that is not reported as stated, fails the test suite.

## Chapters

| Chapter | Introduces |
|---|---|
| [1. A first program](01-first-program.md) | models, parameters, derived values, functions, a plot, a slider, a formula, a test case |
| [2. Quantities and units](02-quantities-and-units.md) | dimensions, units, named types, ranges, how unit errors are reported |
| [3. Space, points and vectors](03-space-and-vectors.md) | spaces, points, vectors, components, the spatial view, dragging |
| [4. Motion](04-motion.md) | state, flows, time, solvers, traces and series plots |
| [5. Events and modes](05-events-and-modes.md) | discrete state, events and triggers, operations, Zeno behavior |
| [6. Checks and tests](06-checks-and-tests.md) | constraints, equations, observations, run cases, expectations |
| [7. Presentations](07-presentations.md) | views, representations, controls, inverses, what the learner may do |
| [8. Lessons](08-lessons.md) | timelines, beats, narration, explore beats, requested events, learner scripts |
| [9. Systems of objects](09-objects.md) | object types, parts, collections, members, aggregates, forces between members |
| [10. Reference](10-reference.md) | every form, reserved words, diagnostic codes |
| [11. Solutions](11-solutions.md) | worked answers to the chapters' exercises, each checked by the tests |

## Running a program

**In the web player.** Build and serve the player (`web/README.md`), open the Source tab, paste a program, and press Compile and open (Ctrl+Enter). Diagnostics are listed with their line and column; clicking one selects the text. The guide's programs are also in the player's Program list.

**From the command line.** A program saved in a file (or a Markdown file whose `text` and `cases` blocks form the program) runs its cases headless:

```sh
cargo run -p prismal-present --example cases -- my-program.prismal
```

Each expectation is printed as `pass` or `FAIL` with the reason. `cargo run -p prismal-syntax --example prismalc -- FILE` prints the IR, or the diagnostics.

## How a program is organized

A Prismal program has up to four kinds of top-level items:

1. `space`: a geometric space, such as the plane.
2. `model`: what exists and how it behaves: parameters, state, derived values, flows, events, equations, constraints. A model never mentions how it is shown.
3. `presentation`: how a model is shown and what the learner may do: views, representations, controls, observations, and optionally a timeline (a lesson).
4. `run`: a test case: a configuration of a run of the model, with expected results.

One model can have several presentations (a lab, a lesson, a presentation that only observes for tests). The guide builds each of these in turn.

## Conventions in this guide

- Programs are in `text` blocks and test cases in `cases` blocks. A block that starts with `space` or `model` begins a new program; the blocks after it belong to it.
- Fragments that are not whole programs (one declaration shown on its own) are in `prismal` blocks and are not compiled by themselves; the same forms appear in the chapter's full programs.
- Examples of mistakes are in `error` blocks. Each starts with a comment naming the diagnostic the compiler reports, `// error: MK-E01`.
- Numbers in expectations were computed independently (by hand or with `tools/refvals.py`-style closed forms), not copied from the implementation.
