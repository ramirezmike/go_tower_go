#!/bin/bash

echo Building release..
cargo build  --features web$1 --release --target wasm32-unknown-unknown
echo Binding wasm
wasm-bindgen --out-name go_tower_go --out-dir wasm/ --target web target/wasm32-unknown-unknown/release/go_tower_go.wasm
echo Removing existing assets
rm ./wasm/assets -r
echo Copying assets
cp ./assets ./wasm/ -r
echo Running wasm
basic-http-server wasm/ 
