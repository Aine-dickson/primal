# Prismal Web Player

The first renderer of Prismal (D-018): the presentation kernel compiled to WebAssembly, with a browser front end that draws its frame descriptions (PK-12.1).

- `crates/prismal-web`: the player logic (`Player`), independent of the browser and tested natively, plus its JavaScript bindings. It compiles a program from source text, opens a presentation, and answers with layout and frame descriptions as JSON. Formulas are typeset as MathML from the IR (D-034).
- `web/`: the front end (`index.html`, `player.js`, `style.css`), with no framework and no build step of its own. It draws spatial and plot views as SVG, panels as HTML controls, and forwards gestures to the kernel. It holds no model logic.

## Build and run

Requirements: the `wasm32-unknown-unknown` Rust target and the `wasm-bindgen` command at the version pinned in `crates/prismal-web/Cargo.toml` (0.2.126).

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
./web/build.sh                        # writes web/pkg (not versioned)
python -m http.server 8000 -d web     # or any static file server
```

Then open `http://localhost:8000/`. A reference program can be chosen in the address: `#rp06`, `#rp07`, `#rp08`, `#rp01` to `#rp05`.

## What the player does

| Presentation | Mode | The learner can |
|---|---|---|
| With a timeline (RP-08) | lesson | play, pause, seek by time or beat, change speed; in explore beats use the declared controls and continue; restart without inputs; choose the video medium, where explore beats play their fallbacks (PK-9.10); zoom and pan where the presentation permits them |
| With views, no timeline, dynamic model (the labs of RP-01 to RP-05) | session | play, pause, seek and reset the run; change parameters while it runs (the change applies at the instant shown and the rest of the run is recomputed); undo and redo; a drag pauses the run until it is committed |
| With views, no timeline, static model (RP-06, RP-07) | interactive | use controls, drag representations that declare an inverse (previews marked valid or invalid, the runtime's reason shown), move them with the arrow keys (PK-11.2a), undo and redo |
| Without views (the checks presentations) | - | run the program's cases in the Cases tab |

Every tab works on any program: the Source tab edits and recompiles the program (diagnostics are located in the text), Observations shows the presentation's observations, Cases runs every case headless and reports each expectation, Description lists the text alternatives of the frame shown (PK-11.1). Captions show narration; event occurrences are announced to assistive technology (PK-11.3a).

A lesson is recomputed from the learner's inputs each time one is added. Playback is deterministic (PK-8.7), so frames before an input never change.

## Limits

- Loading new source text keeps the previous program in memory until the page is reloaded.
- A session's run is computed 60 s ahead, or to the end of the longest time axis its plots show; each intervention recomputes it from the start.
- Representations and timeline actions the prototype does not implement are listed in `docs/prototype.md`.
