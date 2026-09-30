# Syntax Study

The syntax study required by D-006: two or three candidate syntaxes in R-44's hybrid style, each used to write every first-slice reference program (D-012), compared on the D-006 criteria.

- **Status:** done. D-028 accepted with amendment; the working syntax is `working-syntax.md`.
- **Started:** 2026-09-29.
- **Normative status:** none. D-028 fixes the direction; the working syntax stays non-binding until the syntax is frozen. Candidate notations are not the sketch notation of the specification; the sketch gets no preference (D-006).

## Documents

| File | Content |
|---|---|
| `README.md` | Method, fixed parts, criteria |
| `candidate-a.md` | Candidate A: flat keyword statements. All eight reference programs. |
| `candidate-b.md` | Candidate B: mathematical sections. All eight reference programs. |
| `candidate-c.md` | Candidate C: nested structure (modes, processes, view trees). All eight reference programs. |
| `working-syntax.md` | Working syntax: A amended per D-028. All eight reference programs. |
| `comparison.md` | Side-by-side comparison, round-trip analysis, findings for the specification, recommendation |
| `tools/metrics.py` | Counts lines and tokens per program and candidate (standard library only) |

## Method

1. **One baseline, two variations.** Comparing three unrelated designs cannot tell which feature caused which difference. So the candidates are built as controlled variations:
   - **A (baseline): flat keyword statements.** Every statement starts with a keyword naming its kind. Behaviors are listed flat. Braces delimit blocks. ASCII first.
   - **B: mathematical sections.** Varies notation and role marking. Roles are given by section headings (`parameters`, `state`, `dynamics`, ...), distinctions by operators (`:=`, `'`, `←`), with mathematical forms (intervals, `x'`, `falls through`). Indentation delimits blocks. Unicode first, with ASCII spellings.
   - **C: nested structure.** Varies structure. Declarations as in A, but behavior is grouped into named modes and processes (hybrid-automaton style), representations are nested inside their views, and interactions are nested inside the representations they belong to.
2. **Every program in every candidate.** RP-01 to RP-08: models, observations, presentations, timelines, and the cases with representative expectations. Diagnostic variants are compared line by line in `comparison.md`.
3. **Each candidate at its best.** A candidate is written as its own advocate would write it, not as a straw man. Where writing a program exposed a weakness, the weakness is recorded rather than patched in one candidate only.
4. **Findings for the specification** (gaps and ambiguities found while writing) are collected in `comparison.md`, section 7. They are candidate-independent.

## Fixed across candidates

To keep the comparison about structure and role marking, these parts are shared by all three candidates. They are open questions of their own, settled after the direction is chosen.

- **Expression sublanguage.** Literals with units by juxtaposition (`9.81 m/s^2`, `45 deg`, `0.01 /m`); operators `+ - * / ^`; norm `|v|`; calls `sin(θ)`; tuples `(a, b)` as vector literals typed by context; components `v.x`; conditionals `if c then a else b`; chained comparisons `0 <= e < 1`; `and`, `or`, `not`; `2π` (a numeral followed by a constant). No implicit multiplication. B additionally accepts Unicode spellings of the same operators (`²`, `−`, `≤`, `°`); the meaning is identical.
- **Type expressions.** Built-in and named types of MK section 2 (`Point<Plane>`, `Vector<Plane, Velocity>`, `Quantity<1/L>`). `Angle` is a library alias of `Real` (MK-3.3, D-021). B writes types in its own shorter form where the model declares a default space.
- **Identifiers.** Unicode letters are allowed (`θ`, `ω`).
- **Comments.** `//` to end of line.
- **Semantics.** Everything in `docs/spec/`. A candidate may add surface conveniences that lower into the kernel (R-45); it may not change meaning.

## Criteria (D-006)

| # | Criterion | What is looked at |
|---|---|---|
| C1 | Readability for programming-literate authors (D-002) | Familiarity of forms, whether a line can be understood without its surroundings, error recovery and tooling, typing effort |
| C2 | Closeness to the science being taught | Whether the source reads like the physics and mathematics a teacher would write on a board |
| C3 | Visibility of the R-46 distinctions | Whether declaration, definition, initialization, assignment, derivation, equation, constraint and evolution; observation, projection and representation; input, interaction and operation each have a distinct, local mark |
| C4 | Round-tripping with the serializable IR (D-019, R-47) | Whether every IR element has one canonical surface form, what sugar needs pattern recognition to print back, and whether an IR edited by Mava Studio prints to readable source |

Line and token counts (`tools/metrics.py`) are reported as supporting data. Brevity is not a criterion by itself.

## History

- 2026-09-29 study started; candidates A, B, C written; comparison and D-028 proposed.
- 2026-09-29 D-028 accepted with amendment (grouped declaration and flow blocks, interval ranges, timeline shorthand); working syntax written.
