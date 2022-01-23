#!/bin/bash -x

# cargo fmt --all -- --check
cargo fmt --all
rustup run nightly cargo clippy -Zunstable-options
