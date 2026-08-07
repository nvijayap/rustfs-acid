#!/usr/bin/env bash

# run.sh

export RUST_LOG=info; echo

cargo run --release -- "$@"
