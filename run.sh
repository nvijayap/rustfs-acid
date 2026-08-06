#!/usr/bin/env bash

# run.sh

export RUST_LOG=info; echo

rustfmt --edition 2024 src/main.rs && \
    cargo run --release -- "$@"
