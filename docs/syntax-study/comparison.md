# Syntax Study: Comparison

Comparison of candidates A (flat keyword statements), B (mathematical sections) and C (nested structure) on the D-006 criteria, using all eight first-slice reference programs. Method and criteria: `README.md`.

## 1. Summary

- **A** is the most readable line by line, marks every R-46 distinction locally, and maps almost one to one onto the IR. Its weaknesses: repeated mode conditions in flows, and presentations that need many representation names.
- **B** is the closest to the science and the shortest (22 % fewer tokens than A), but a line's role depends on its section, inferred types move errors away from their cause, and it has the most surface forms per IR element, which makes printing from the IR a matter of choice.
- **C** removes mode conditions and makes presentations read as trees, but modes introduce an implicit hold rule and an undeclared discrete state, need either a new IR construct or pattern recognition to print back, and invite behavior into modes where it does not belong.
- **Recommendation (D-028, proposed):** A as the base, with C's presentation structure (view trees, interactions inside representations, explicit `sequence` in beats), C's optional named processes, and B's default space for a model. Modes are not adopted in v1. Section 8 gives the full position.

## 2. The same element in three candidates

| Element | A | B | C |
|---|---|---|---|
| Parameter with range | `param e: Real = 0.8 where 0 <= e < 1` | `e = 0.8 ∈ [0, 1)` under `parameters` | as A |
| Continuous state | `state vel: Vector<Plane, Velocity> = ...` | `vel : Vector L/T = ...` under `state` | as A |
| Discrete state | `discrete flying: Boolean = true` | `flying = true` under `discrete` | `modes motion { flying {...} grounded {...} }` |
| Defining flow | `flow der(pos) = if flying then vel else (0, 0)` | `pos' = if flying then vel else (0, 0)` | `flow der(pos) = vel` inside `flying` |
| Contribution | `flow der(vel) += -k * \|vel\| * vel` | `vel' += −k * \|vel\| * vel` | `process drag { flow der(vel) += ... }` |
| Derived value | `derived energy: Energy = ...` | `energy := ...` | as A |
| Function-valued derived | `derived f(x: Real): Real = a * x^2` | `f(x : Real) := a * x²` | as A |
| Crossing event with reset | `event landed on falling(pos.y) { set vel = (0, 0) }` | `landed: when pos.y falls through 0` + `vel ← (0, 0)` | as A, with `enter grounded` |
| Zeno policy | `} zeno settle { ... }` | `zeno settle` indented under the event | as A |
| Requestable event | `event relaunch on request { ... }` | `relaunch: on request` | as A |
| Check equation | `equation conservation: energy == ... checked within 2e-5 J` | `conservation: energy = ... checked ± 2e-5 J` under `equations` | as A |
| Constraint | `constraint rod: \|bob - pivot\| == L within 1e-9 m policy report` | `rod: \|bob − pivot\| = L ± 1e-9 m` under `constraints` | as A |
| Observation at microstep 0 | `observe v_land = vel on landed microstep 0` | `v_land = vel just before landed` | as A, in an `observe` block |
| Drag inverse | `on drag u_arrow.head as h { propose u = h - A }` | `drag u_arrow.head to h ⇒ u ← h − A` | inside `arrow(u, from: A) { on drag head as h { propose u = h - A } }` |
| Explore beat | `explore limit 60 s keep angle { ... } fallback { ... }` | `explore for at most 60 s, keeping angle` | `explore(limit: 60 s, keep: angle) { ... } fallback { sequence { ... } }` |
| Run with overrides | `run B_60deg of Projectile with ProjectileChecks { param angle = 60 deg }` | `run B-60deg: Projectile with angle = 60°, observed by ProjectileChecks, ...` | as A |

## 3. Visibility of the R-46 distinctions (C3)

**Local** means the mark is on the line itself. **Sectional** means it is given by an enclosing heading or block. **Implicit** means the reader must infer it.

| Distinction | A | B | C |
|---|---|---|---|
| Declaration | local (role keyword) | sectional | local |
| Definition (functions) | local (`fn`, `derived f(x)`) | local (`:=`) | local |
| Initialization | local (`=` in a declaration) | sectional (`=` under `state`, `parameters`) | local |
| Assignment | local (`set`) | local (`←`) | local (`set`, `enter`) |
| Derivation | local (`derived`) | local (`:=`) | local |
| Equation | local (`equation`, `==`) | sectional (`=` under `equations`) | local |
| Constraint | local (`constraint`, `where`) | local on parameters (`∈`), sectional otherwise | local |
| Evolution | local (`flow`, `=` or `+=`) | local (`'`) | local; the flow's condition is sectional (the mode) |
| Continuous vs discrete state | local (`state`, `discrete`) | sectional | discrete state is implicit (the mode variable is never declared) |
| Observation | local (`observe`) | sectional | sectional (`observe { }`) |
| Projection and representation | local (`show kind(...) in view`) | sectional (lines under a view) | sectional (tree under a view) |
| Input / interaction / operation | local / local / local | sectional / sectional / local | local / local, nested in its representation / local |

A marks every distinction locally. B's operators (`:=`, `'`, `←`) are the clearest single marks in the study, but `=` carries four meanings chosen by section. C loses one distinction that A has: the mode variable is discrete state, but it is never declared as such.

**`=` versus `==`.** A and C reserve `=` for giving a value to something being declared or set, and `==` for asserting equality (equations, constraints, expectations). This keeps "equation" and "definition" apart on the line, at the cost of writing physics equations with `==`. B writes equations with `=`, as in mathematics, and relies on sections. The typeset equation shown to learners uses `=` in every case (PK-6.5), so `==` is only an authoring mark.

## 4. Diagnostic variants

How the one-line changes of the diagnostic variants are written, and where the error is reported.

| Variant | A and C | B | Note |
|---|---|---|---|
| RP-01.D1 level trigger | `event landed on pos.y <= 0 m` is not a trigger form; parse error naming MK-E13 | `when pos.y ≤ 0` is not a trigger form; same | No candidate has a level-style trigger, so MK-15.6 lowering is not needed. The variant is tested in IR form, as RP-01 already allows. |
| RP-01.D2 bare literal | `flow der(vel) += (1, -g)`, MK-E03 on the line | same | Shared expression language. |
| RP-01.D3 define and contribute | `flow der(vel) = ...` next to `+=`; MK-E10 on the `=` line | `vel' = ...` next to `vel' +=`; same | In C, the defining flow may sit in a mode and the contribution outside it; both lines are reported. |
| RP-01.D4 flow condition on continuous state | `if pos.y > 0 m then ...` in a flow; MK-E12 | same | C makes the correct form (a mode) the natural one, so the error is less likely to be written. |
| RP-01.D5 handler sets a parameter | `set g = 0 m/s^2`; MK-E08 | `g ← 0 m/s²`; MK-E08 | - |
| RP-03.D1 no Zeno policy | remove the `zeno` clause; MK-E16 on the event | same | - |
| RP-03.D3 two sets in one handler | two `set v` lines; MK-E19 | two `v ←` lines; MK-E19 | - |
| RP-04.D1 dimensioned angle | `state θ: Length = 10 cm`; MK-E02 at `sin(θ)` | `θ = 10 cm` with the type inferred; MK-E02 at `sin(θ)`, several sections away from the cause | With inference, B's diagnostic must point back to the declaration to be useful. |
| RP-05.D1 equation dimensions | `T == 2π * sqrt(k / m)`; MK-E01 | `T = 2π * sqrt(k / m)`; MK-E01 | - |
| RP-07.D1 to D6 | shared expression language; identical | identical | D6 (`u + (0, 0)`) is accepted in all three. |

Diagnostics are the same kernel diagnostics in every candidate, because all three lower to the same IR. The candidates differ in how far the reported line is from the cause (B, with inferred types) and in which errors the surface makes easy to write (C prevents D4-style errors by offering modes).

## 5. Round-tripping with the IR (C4)

D-019 requires Mava Studio to edit the model through the IR. So text must lower to the IR, and an IR (possibly edited by a visual tool, with no source text) must print back to text a person can read and keep editing. Four questions:

| Question | A | B | C |
|---|---|---|---|
| R1. Does every IR element have one canonical surface form? | Yes, with one choice: a parameter's `reject` constraint prints as `where` when it mentions only that parameter | No: crossings (`y falls through h` or `y − h falls through 0`), ranges (interval or constraint line), types (written or omitted), spellings (Unicode or ASCII), and timeline pairs (`run ... until` or two actions) all have two forms | Mostly: modes print only if the IR records them (see R4) |
| R2. Sugar that needs pattern recognition to print | `where` only | intervals, omitted types, `falls through h`, `run ... until`, `just before` | modes (an enumeration discrete state, conditional flows and enabling conditions that together form a mode) |
| R3. Author choices that are not meaning | comments, blank lines, alignment | as A, plus section order, Unicode or ASCII, written or omitted types | as A, plus which flows are grouped into processes (the IR carries processes, MK-14.1, so this is meaning) |
| R4. IR from a visual editor, printed | Always prints: every element has a flat form | Prints, with a formatter choosing each alternative | A flow conditioned on two mode variables, or a mode test inside a larger condition, has no mode form; C then needs A's flat form as a fallback, so C contains A |

Consequences:

- **B** needs a formatter with a fixed choice for every alternative, and the author's choices are lost on every visual edit unless the IR stores them as presentation-of-source metadata. That is the "syntax tree records what the author wrote" half of R-47 leaking into the IR.
- **C** needs a decision: either modes become an IR construct (a hybrid-automaton location with its own flows), which the model kernel does not have and which lowers again into discrete state and conditions, or modes are recovered by pattern recognition, which fails on IR edited outside the pattern.
- **A** needs almost nothing beyond the IR as specified.

Two round-trip needs are common to all three candidates and are recorded as open items (section 8.3): comments must survive a visual edit, so the IR needs author notes attached to elements; and identities must survive a rename made in a text editor, so the text form needs a way to carry or recover IR identities (MK-6.2, MK-7.6).

## 6. Assessment

| Criterion | A | B | C |
|---|---|---|---|
| C1 Readability for programming-literate authors | **Strong.** Familiar keyword-and-brace form; every line self-describing; robust to copy and paste; simple error recovery. | **Adequate.** Short and pleasant to read whole, but lines depend on their section; indentation-sensitive; Unicode input needs editor support. | **Strong for presentations, adequate for models.** View trees read like SwiftUI or JSX; modes need the implicit hold rule to be learned; deeper nesting. |
| C2 Closeness to the science | **Adequate.** Physics is recognizable (`der(θ)`, `sin(θ)`, units), with keyword noise and repeated mode conditions. | **Strong.** Reads like a problem statement: `θ' = ω`, `e ∈ [0, 1)`, `when y falls through 0`. | **Strong for moded systems.** Flight and rest phases read as the physical story; no difference from A for unmoded systems. |
| C3 Visibility of R-46 distinctions | **Strong.** Every distinction local. | **Adequate.** Best operators, but four meanings of `=` chosen by section. | **Adequate.** As A, except the undeclared mode variable and implicit flow conditions. |
| C4 Round-tripping with the IR | **Strong.** Nearly one to one; one piece of sugar. | **Weak.** Many alternatives; needs a formatter and loses author choices on visual edits. | **Adequate.** Needs a new IR construct or pattern recognition, with A's flat form as fallback. |

Closeness to the science (C2) matters most where learners see it. Learners see representations, not source: equations are typeset from the IR (PK-6.5) with `=`, primes or dots, and display symbols, whatever the source looks like. So the source can favor its authors (C1) and its editor (C4) while the displayed mathematics stays as close to the science as B's.

Supporting data (`tools/metrics.py`, program code without runs and expectations):

| | A | B | C |
|---|---|---|---|
| Lines, all eight programs | 191 | 195 | 227 |
| Tokens, all eight programs | 1592 | 1248 | 1603 |
| Deepest nesting (block depth at line start) | 5 | 6 | 5 (7 within one line in RP-08 `b7`) |

B is shortest in tokens because of omitted types and sections; C is longest in lines because of mode, process and view blocks. The difference between A and C is small; B's advantage is real but comes from exactly the features that weaken C3 and C4.

**Familiarity bias.** A is closest to the sketch notation of the specification, which every reader of this project has seen for weeks. D-006 gives the sketch no preference. The recommendation rests on C3 and C4 (local marks, one-to-one printing), which are checkable properties of the tables above, not on familiarity. A also departs from the sketch where the sketch was weak: explicit `discrete`, `==` for equality, `where` for parameter ranges, no level-style triggers.

## 7. Findings for the specification

Writing every program in three candidates exposed these gaps and ambiguities in the specification and reference programs. They do not depend on which candidate is chosen.

| ID | Finding | Where | Suggested resolution |
|---|---|---|---|
| S-1 | (Resolved by D-029.) RP-06's `f = fn(x) => a * x^2` reads the parameter `a`, but MK-10.3 says functions read only their parameters and constants. The kernel does not say whether a derived binding may build a function that reads bindings. | MK-10.3, RP-06 | State that MK-10.3 applies to declared functions; a derived binding whose value is a function may read bindings, and its dependencies are the derived binding's. |
| S-2 | (Corrected: specification and reference programs use `discrete`.) The sketch notation writes discrete and continuous state with the same word `state`; the role is stated only in comments. MK-6.2 makes them different roles, and MK-14.11 means the role cannot be inferred from the absence of flows. | Sketch, RP-01, RP-03 | Every candidate marks the role; the sketch in the specification should too. |
| S-3 | (Resolved by D-030.) Whether a bare `0` adopts a vector type, as it adopts a dimension (MK-3.8). All candidates wrote `(0, 0)`; `0` would be clearer. | MK-3.8 | Extend the literal rule: `0` adopts the zero of the required quantity or vector type. |
| S-4 | (Resolved by D-031.) RP-03 case `B-stop` changes the Zeno policy, which is model structure, not a parameter or run configuration. The reference-program format defines cases as overrides and settings only. | RP-03, reference-program README | Either allow model variants in cases (named, one-line changes, like diagnostic variants) or make `B-stop` a separate variant program. |
| S-5 | (Resolved by D-032 (MK-4.8, MK-E21).) Tuples are used as vector literals, typed by the expected type (`param u: Vector<Plane, Length> = (3 m, 0 m)`). Where there is no declared type, as in `pivot + L * (sin(θ), -cos(θ))`, the space must come from the other operand. The rule is not written down. | MK-4 | State that a tuple literal takes its space and dimension from the expected type, or from the other operand of `+`, `-`, `*`; otherwise it is a static error. |
| S-6 | (Corrected: `Plane` declared in RP-04; models declare a default space.) The sketches write `origin` without naming its space; with two spaces (RP-07.D5) it is ambiguous. RP-04 uses `Plane` without declaring it. | RP-01, RP-04, RP-07 | Qualify (`Plane.origin`) or give the model a default space; declare `Plane` in RP-04. |
| S-7 | (Corrected: MK-E22.) MK-14.7 says a flow target is defined by exactly one flow, but MK section 18 has no diagnostic for two defining flows on one target (only MK-E10 for defining and contributing). | MK-14.7, MK section 18 | Add a diagnostic code for duplicate definitions of a flow target (and of any binding). |
| S-8 | (Resolved by D-033 (PK-9.2a).) A beat's actions start together (PK-9.1). RP-08 `b4` has `seek(t0)` and a live formula in the same beat, and `b8` has `request(relaunch)` and `run(rate 1)`. The specification does not say how actions that act on the run are ordered when they start together. | PK-9.1, RP-08 | State that run-directing actions in one beat apply in written order at the beat's start, or require an explicit sequence; candidates should make sequence visible. |
| S-9 | (Corrected in RP-08.) RP-08 `b5` passes `vel.x`, a scalar, to `arrow`, whose source is a vector (PK-6.3). | RP-08 | Write `(vel.x, 0)` in RP-08, as every candidate did. |
| S-10 | (Resolved by D-034.) RP-06 shows the equation as a string (`"y = a x^2"`), and RP-08 `b4` shows `R = v^2 sin(2θ) / g` with symbols that are not bindings. PK-6.5 requires a symbolic form linked to bindings. | RP-06, RP-08, PK-6.5 | Show the definition of `f` (typeset from the IR), and for RP-08 a labeled expression over `speed`, `angle`, `g` with display symbols `v` and `θ` (binding metadata, MK-6.1). |
| S-11 | (Corrected: MK-14.1 now says "process kind".) "Mode" names two things: the execution mode of a process (MK-14.1: continuous, discrete, algorithmic, stochastic) and the discrete state that selects flows (MK-14.4). | MK-14.1, MK-14.4 | Rename the first (for example "process kind"). |
| S-12 | (Corrected: PK-6.1 allows author names.) Timelines and interactions refer to representations (`highlight(marker)`), but PK-6.1 gives representations identity without an author-facing name. | PK-6.1, RP-08 | Representations may carry an author name, unique within the presentation. |

## 8. Recommendation

### 8.1 Position (proposed as D-028)

Adopt **candidate A as the base syntax direction**, amended as follows:

1. **From C, presentation structure:** representations are written inside the view that shows them; an interaction is written inside the representation it acts on; a beat's actions start together, and a sequence inside a beat is written with `sequence { }`.
2. **From C, named processes:** `process P { ... }` is an optional grouping of flows and events. It maps to the process concept the kernel already has (MK-14.1), so it round-trips one to one, and presentations can refer to a process (the drag contribution alone).
3. **From B, a default space:** `model Name in Space` lets `origin`, `Point` and `Vector<D>` refer to that space. It resolves S-6 without qualification noise. Explicit forms remain valid.
4. **Not adopted in v1:** C's modes (they need a new IR construct or pattern recognition, add an implicit hold rule and hide a discrete state); B's sections, inferred types, interval ranges and Unicode-first spelling (they weaken local visibility and canonical printing). Each can be revisited as a separate proposal with evidence from a program that needs it.
5. **Scientific notation for learners** is carried by typeset representations (PK-6.5), not by the source.

### 8.2 What stays open after D-028

The direction fixes the structure and the role marks. These remain to be settled, as separate entries, one at a time:

- **Keyword and spelling pass:** final keywords (`derived` or another word, `discrete`, `policy`, `checked`, `where`), trigger spelling (`falling(y)` or a phrase such as `y falls through 0`), and `microstep 0` versus a named form such as `before handlers`.
- **Expression sublanguage details:** S-3 (zero vectors), S-5 (tuple literals), conditionals and chained comparisons, whether Unicode operator spellings are accepted.
- **Specification findings** S-1 to S-12 (section 7): corrections to the specification and reference programs.

### 8.3 Open items for the IR (all candidates)

- **Comments across visual edits:** the IR should carry author notes attached to elements, so that a model edited in Mava Studio prints back with its comments.
- **Identity in text:** a rename made in a text editor must not change an element's identity (MK-6.2, MK-7.6). Options: identities kept in a sidecar file, optional identity annotations printed by the formatter, or identity recovery by a language server. To be decided with the IR format.

### 8.4 Next step if D-028 is accepted

Write the combined candidate (A with the amendments) for all eight programs, as the working syntax for the Rust prototype's parser, and run the keyword pass. The sketch notation in the specification is then replaced by the working syntax, still labeled non-binding until the syntax is frozen.

## 9. Outcome

D-028 was accepted with an owner amendment: A remains the base, and B and C forms that improve the author's experience are adopted where they still lower to the same IR and print canonically. Adopted beyond section 8.1: grouped declaration blocks (`param { }`, `state { }`, ...; the one-line form stays valid and the formatter prints blocks), grouped flows (`flow { }`), interval ranges (also the single range notation in presentations and runs), and the timeline shorthand `run rate r until E`. Declined: types inferred from unit literals. The result is `working-syntax.md`.

On 2026-09-30 all findings S-1 to S-12 were resolved: S-1, S-3, S-4, S-5, S-8 and S-10 by register entries D-029 to D-034, the rest as corrections to the specification and reference programs.
