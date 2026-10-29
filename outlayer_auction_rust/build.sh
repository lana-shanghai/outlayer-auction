#!/bin/sh
set -eu

rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1

echo
echo "WASM:"
echo "target/wasm32-wasip1/release/outlayer_auction.wasm"
echo
shasum -a 256 target/wasm32-wasip1/release/outlayer_auction.wasm
