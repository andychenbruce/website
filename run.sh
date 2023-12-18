#!/usr/bin/env bash
set -e

ROOT_DIR="root_server"
WASM_DIR=$ROOT_DIR"/andy_wasm"

cargo build --package "icosahedron" --target "wasm32-unknown-unknown"
rm -fr $WASM_DIR
wasm-bindgen --target web ./target/wasm32-unknown-unknown/debug/icosahedron.wasm --out-dir $WASM_DIR
cargo run --bin "andy_http_server" -- --port 1234 --root-serve-path $ROOT_DIR
