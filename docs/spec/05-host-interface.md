# 05 Host Interface

- **Version:** v0 (draft), 2026-09-30
- **Part:** how a host system embeds Prismal: loading and editing programs, running presentations, and receiving what to render (D-044, D-045). It adds no meaning to `01` to `04`.
- **Scope:** the operations every host needs, their data, and the bindings through which they are offered. The reference implementation is the `prismal-host` crate (`crates/prismal-host`).

---

## 1. Principles

- **Decisions:** D-018, D-019, D-036, D-037, D-044, D-045.
- **Prior art:** follows the Language Server Protocol (one JSON protocol, transport left to the host, capabilities declared) and FMI (a simulation unit embedded by any tool through a small, versioned interface). Follows embeddable engines such as SQLite and Lua in keeping storage, files and user interface with the host.

- **HI-1.1** A **host** is any system that embeds Prismal: an authoring studio, a learning platform, a document viewer, a test harness, a command-line tool. Prismal knows no particular host (D-044). The reference web player (`web/`) is a host like any other and uses only this interface.
- **HI-1.2** The interface has three layers: **authoring** (programs as text or IR, checked and identified, section 3), **execution** (presentations opened as instances, driven by the learner's inputs and the host's clock, section 4) and **output** (layouts, frame descriptions and observations, section 5).
- **HI-1.3** Authoring by text and authoring by a visual tool are equal (D-019, D-036). A host may send source text, an IR document, or both over time; Prismal returns the IR, the canonical text and diagnostics either way.
- **HI-1.4** **Division of responsibility.** The host owns storage, files, projects, assets, the editor and its edit history, users and collaboration, and the choice of renderer. Prismal owns the meaning of programs: checking, identities, runs, sessions (including undo of the learner's interventions, RC section 11), lesson playback and frame descriptions.
- **HI-1.5** The same operations are offered through every binding (section 6), with the same data. A host never needs a binding-specific feature.

---

## 2. Engine and handles

- **HI-2.1** An **engine** holds documents and instances, each named by a **handle** chosen by the engine (an opaque string). A host may hold several documents and several open instances at once; instances of one document are independent.
- **HI-2.2** Handles stay valid until the host closes them. Closing a document closes its instances. Nothing leaks when a host loads, edits and closes documents repeatedly.
- **HI-2.3** An engine is used from one thread at a time. A host that needs parallel work uses several engines.

---

## 3. Authoring

- **HI-3.1** `load` takes source text in the working syntax, a Markdown document whose `text` and `cases` blocks form a program, or an IR document (04). It returns a document handle, or diagnostics when the program does not compile. A document that compiles but fails its checks (MK, PK diagnostics) is still loaded, so that an editor can show the diagnostics against it.
- **HI-3.2** `update` replaces a document's content with new text or IR. The new IR keeps the identities of elements that match the previous version by declaration path (`reconcile`, D-036); `update` returns the identities that disappeared, so that a host can report broken references (PK-2.3).
- **HI-3.3** `rename` renames a declared element by identity and keeps its identity (D-036). It is the operation a visual editor uses; a text edit that renames is a removal and an addition (HI-3.2).
- **HI-3.4** A document answers with its **IR** (04), its **canonical text** (working syntax section 1.4), its **catalogue** (models, presentations with their kinds, cases) and its **diagnostics**. Each diagnostic has a code, a message, the identity of the element it concerns, and a source span when the document came from text.
- **HI-3.5** Cases (`run` blocks) run headless on request and report each expectation (PK-4.2).

---

## 4. Execution

- **HI-4.1** `open` opens a presentation of a checked document as an **instance**: a **lesson** when it has a timeline (PK sections 8, 9; in the interactive or the video medium, PK-9.10), otherwise an **interactive session** (PK section 10). It returns the instance's handle and layout (HI-5.1).
- **HI-4.2** The host owns the clock. A lesson is asked for the frame at a presentation instant; an interactive session is told which simulation instant to show (`seek`, RC section 12). Frames are deterministic (PK-8.7): the same instant gives the same frame.
- **HI-4.3** Learner inputs name representations by identity or author name: set a control, press a button, pointer down, move and up on a draggable representation, a keyboard step (PK-11.2), cancel, undo and redo. In a lesson, inputs carry their presentation instant and are refused outside explore beats (D-025, PK-9.8a). Each answers with what happened: applied, previewed (with validity and reason, PK-10.6) or refused (with reason).
- **HI-4.4** `restart` replays a lesson without the learner's inputs; `reset` starts an interactive session's run again with an empty log.

---

## 5. Output

- **HI-5.1** A **layout** lists an instance's views with their coordinate systems (spatial scale and orientation, plot ranges and units), the extent of their content over the run (for an initial viewport), the learner's permissions, and for a lesson its beats, captions and explore windows.
- **HI-5.2** A **frame** is the frame description of PK-12.1, with what any renderer needs to draw it without the model: labels, the part a representation is dragged by, a control's symbol and display unit, and for formulas and equations both the symbolic IR (renderers may typeset it themselves, D-034) and MathML.
- **HI-5.3** A host may draw frames with its own renderer, or embed the reference renderer (`web/`), which draws frames in any web view.
- **HI-5.4** **Observations** (PK section 3) of an instance's current run are available by name, as values and as text.

---

## 6. Bindings and protocol

- **HI-6.1** **Rust API.** The `prismal-host` crate offers the engine as a Rust type. Native hosts in Rust (a desktop studio, a server, a command-line tool) link it directly.
- **HI-6.2** **Protocol.** Every operation is also a JSON request answered by a JSON response, through one entry point: `handle(request) -> response`. The protocol is independent of transport: a host may call it in process, over WebAssembly, over a pipe or a socket.

  ```json
  { "protocol": 1, "op": "open", "document": "d1", "presentation": "ProjectileLab" }
  { "ok": { "instance": "i1", "layout": { ... } } }
  { "error": [ { "code": "HI-E02", "message": "no presentation `Lab`" } ] }
  ```

- **HI-6.3** **WebAssembly.** The `prismal-web` crate exports the engine's protocol to JavaScript, for browsers and web views.
- **HI-6.4** Further bindings (a C ABI for other languages, a standalone process speaking the protocol on standard input and output) are added when a host needs them; they carry the protocol unchanged.
- **HI-6.5** **Versions.** Requests carry the protocol version. `capabilities` answers the protocol and IR versions (D-037), the representation kinds and timeline actions the implementation supports, and its limits, so that a host can adapt without trial and error.

### 6.1 Operations

| Operation | Takes | Answers |
|---|---|---|
| `capabilities` | - | protocol and IR versions, supported kinds and actions |
| `load` | `text` or `ir` | `document`, `catalogue`, `diagnostics` |
| `update` | `document`, `text` or `ir` | `catalogue`, `diagnostics`, `removed` identities |
| `rename` | `document`, `element`, `name` | `diagnostics` |
| `ir`, `text` | `document` | the IR, the canonical text |
| `locate` | `document`, `element` | its source span |
| `run_cases` | `document` | each case's expectations |
| `close` | `document` or `instance` | - |
| `open` | `document`, `presentation`, `medium`? | `instance`, `layout` |
| `layout`, `frame` | `instance`, `time`?, `dt`? | the layout, the frame |
| `seek`, `reset`, `restart` | `instance`, `time`? | the clock or the layout |
| `set_control`, `press`, `key`, `pointer_down`, `pointer_move`, `pointer_up`, `cancel`, `undo`, `redo`, `continue` | `instance`, `rep`, value, `time` in a lesson | the outcome |
| `observations` | `instance` | observations by name |

### 6.2 Diagnostics of the interface

| Code | Meaning |
|---|---|
| HI-E01 | malformed request: not JSON, unknown operation, a missing or mistyped field, an unsupported protocol version |
| HI-E02 | unknown handle, presentation, representation or element |
| HI-E03 | the operation does not apply: an interactive input to a lesson, a lesson input to a session, an instance of a document with errors |

---

## History

- 2026-09-30 written with D-044 and D-045.
