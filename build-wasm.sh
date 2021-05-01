#!/bin/sh

xargo build --target=wasm32-unknown-unknown --release
wasm-opt -Os --strip-debug target/wasm32-unknown-unknown/release/graphical-solve.wasm -o graphical-solve.wasm