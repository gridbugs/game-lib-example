#!/bin/bash

set -e

CRATE_NAME=web_frontend

cargo build --target=wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/$CRATE_NAME.wasm web/$CRATE_NAME.wasm
