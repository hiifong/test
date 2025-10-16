#!/bin/bash

cd typst-rs && cargo run \
  --features=cli \
  --bin uniffi-bindgen-go \
  "src/typst.udl" \
  --out-dir "target/go"
