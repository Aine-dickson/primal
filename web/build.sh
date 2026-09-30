#!/bin/sh
# Builds the player's WebAssembly package into web/pkg.
# Needs the wasm32-unknown-unknown target and the wasm-bindgen command at the version
# pinned in crates/prismal-web/Cargo.toml.
set -e
cd "$(dirname "$0")/.."
cargo build -p prismal-web --target wasm32-unknown-unknown --release
wasm-bindgen --target web --no-typescript --out-dir web/pkg target/wasm32-unknown-unknown/release/prismal_web.wasm
