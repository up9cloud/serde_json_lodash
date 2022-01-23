#!/bin/bash

cargo watch -x "test --features lazy_static $*" -w "Cargo.toml" -w "src"
