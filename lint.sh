#!/bin/bash -x

cargo fmt --all -- --check
rustup run nightly cargo clippy -Zunstable-options
