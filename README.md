# Prismal

Prismal is a modeling language for mathematics, science and interactive systems. You describe what a system is and how it behaves (a model), how it is shown (a presentation), and what it should do (cases). Prismal checks the units, runs the model, draws it, lets a learner interact with it, and can export it as a video.

The language is young: the syntax is usable and tested, but not frozen.

## Get started

### 1. Install Rust

Install Rust 1.94 or later from <https://rustup.rs>. Then, in this folder:

```sh
cargo test
```

Every test should pass. This also checks every program in the guide.

### 2. Write a first program

Save this as `fall.prismal`:

```text
model Fall {
  param {
    g:  Acceleration = 9.81 m/s^2
    h0: Length       = 20 m
  }
  state {
    h: Length   = h0
    v: Velocity = 0 m/s
  }
  flow {
    der(h) = v
    der(v) = -g
  }
  event landed on falling(h)
}

presentation Watch for Fall {
  observe {
    fall_time = elapsed on landed
  }
}

run drop of Fall with Watch {
  until t0 + 5 s
  expect {
    fall_time[1] == 2.01928 s within 1e-5 s
  }
}
```

- `model` says what exists: parameters, state, how the state changes (`flow`), and events.
- `presentation` says what to observe or show. Here it records the time of landing.
- `run` is a test case: it runs the model and checks the result against `sqrt(2 h0 / g)`.

Units are checked: writing `der(h) = g` is an error, because a length cannot change at the rate of an acceleration.

### 3. Run it

```sh
cargo run -p prismal-web --bin prismal -- cases fall.prismal
```

Each expectation prints `pass` or `FAIL` with the reason. Errors in the program are printed with their line and column.

### 4. Run the examples

The examples are the reference programs (`rp01` to `rp08`) and every program of the guide. List them, then run the cases of any one by name:

```sh
cargo run -p prismal-web --bin prismal -- list
cargo run -p prismal-web --bin prismal -- cases rp03
cargo run -p prismal-web --bin prismal -- cases g9-drops
```

The same command runs a file: `cases fall.prismal`, or a Markdown file whose `text` and `cases` blocks form one program.

### 5. See it in the browser

The web player shows presentations, lets you drag and change values, and plays lessons. Building it needs the WebAssembly target and the `wasm-bindgen` command at the pinned version:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
./web/build.sh
python -m http.server 8000 -d web
```

Open <http://localhost:8000>, choose a program from the list (every reference program and guide program is there), or paste your own into the Source tab and press Compile and open (Ctrl+Enter).

### 6. Make a picture or a video

```sh
cargo run --release -p prismal-media -- rp08 ProjectileLesson lesson.png --at 5
cargo run --release -p prismal-media -- rp08 ProjectileLesson lesson.mp4 --fps 30 --speech system
```

The first argument is a program file or the name of a built-in example (`rp01` to `rp08`, `g7-wheel`, `g8-freefall`, `g9-drops` ...); the second names a presentation. Videos need [ffmpeg](https://ffmpeg.org) installed (or `--encoder PATH`); without it, give a folder name instead of `lesson.mp4` to get the frames as images.

## Learn more

- [Learn Prismal](docs/guide/README.md): the guide, from a first program to lessons and systems of objects, with exercises. Every program in it is tested.
- [Reference](docs/guide/10-reference.md): every form on one page.
- [Project state](docs/PROJECT-STATE.md): where the project stands and what comes next.
- [Specification](docs/spec/): the meaning of the language, and [decisions](docs/decisions/divergence-register.md) taken along the way.

## Repository layout

| Folder | Content |
|---|---|
| `crates/prismal-syntax` | the parser and formatter |
| `crates/prismal-kernel`, `crates/prismal-runtime` | checking and running models |
| `crates/prismal-present` | presentations, lessons, cases |
| `crates/prismal-host` | the engine that hosts embed (Rust API and JSON protocol) |
| `crates/prismal-web`, `web/` | the web player |
| `crates/prismal-svg`, `crates/prismal-media` | SVG frames, images and videos |
| `docs/` | guide, specification, decisions |
