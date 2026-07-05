# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Rust port of lodash.js that operates on `serde_json::Value`. Every function mirrors the lodash spec as closely as possible, and all tests are Rust doc tests embedded in the source.

## Commands

```bash
# Run all tests (doc tests are the test suite; lazy_static feature is needed by some examples)
cargo test --features lazy_static

# Test a single function's doc tests (e.g. `set`)
cargo test --features lazy_static --doc set

# Watch mode (wraps the above in cargo-watch)
./dev.sh              # all tests
./dev.sh --doc set    # single function

# Lint — CI enforces both of these
cargo fmt --all -- --check
cargo clippy -- -D warnings
./lint.sh             # local variant: fmt (writing) + nightly clippy

# Benchmarks (criterion)
cargo bench

# Preview docs
cargo doc --open

# Check real lodash.js behavior when porting a function
npm i && node
> const l = require('lodash')

# Bump patch version, tag, and push (CI publishes to crates.io on tag push)
./bump_push.sh
```

Edition is 2024. Features: `camel` (camelCase aliases — the default feature), `lazy_static`, `all` (both). The crate has `#![deny(missing_docs)]` and `#![deny(warnings)]`, so any public item without a doc comment (even an empty `///`) fails the build. `src/seq`, `src/properties`, and `src/methods` are not re-exported from `src/lib_snake.rs` (they hold only commented-out chaining/template-settings stubs); do not add empty glob re-exports of them or the build fails under `#![deny(warnings)]`.

Functions that return or invoke JS functions cannot be represented as `serde_json::Value` and remain `todo!()` stubs by design: the entire `function/` category (debounce, curry, memoize, …), most of `util/` (flow, iteratee, property, matches, over, …), plus `invoke`, `invoke_map`, `create`, `set_with`, `update_with`, and `string` `template`.

## Architecture

- `src/{array,collection,date,function,lang,math,number,object,seq,string,util,properties,methods}/` — one directory per lodash docs category. Each category's `mod.rs` contains `#[doc(hidden)] todo!()` stubs for every not-yet-implemented lodash function; implementing a function means replacing its stub with `mod <name>; pub use <name>::*;` and creating `<name>.rs`.
- `src/lib_snake.rs` re-exports the implemented category modules (snake_case API). `src/lib_camel.rs` (behind the `camel` feature) generates camelCase fn and macro aliases via the `build_camel_case!`/`build_multi!` macros, which take explicit `camelName => snake_name` pairs — every multi-word function's macro must be registered there to get a camelCase alias. (Single-word camelCase fn aliases for still-stubbed functions live as `pub use snake as camelName` lines in each category `mod.rs`.)
- `src/internal.rs` also holds shared helpers used across categories: `words_vec`/`compound_words`/`capitalize_word` (string casing), `compare_values` (JS-style ordering), `value_to_option_number`/`f64_to_number` (numeric coercion), `uniq_by_key` (array dedup), `base_is_match` (partial deep match), and `rand_f64` (std-only RNG for `random`/`sample`/`shuffle`).
- `src/macros.rs` — `with_dollar_sign!` and `build_link!` helpers for defining macro aliases (workaround for nested `macro_rules!` `$` escaping).
- `src/internal.rs` — JS-compat constants (`MAX_SAFE_INTEGER`, etc.) and helpers like `value_undefined()` (JS `undefined` maps to `Value::Null`).

## Per-function file convention

Each `<name>.rs` implements up to four fn variants plus a matching `macro_rules!` for each (macros provide lodash-style optional arguments):

- `name(Value, ...) -> Value` — the canonical form
- `x_name(...)` — input downgraded to a primitive type (e.g. `&str` instead of `Value`)
- `name_x(...)` — output downgraded to a primitive type
- `x_name_x(...)` — both

Option-like parameters (sizes, indexes) take primitive types (`usize`, `isize`), not `Value`; predicates take `Fn(&Value) -> bool`. Lodash functions with unlimited optional args keep exactly one in the fn form; the macro accepts more (e.g. `merge!(a, b, c)`).

## Test convention

Tests are doc tests on the macros/fns, in two sections:

- `/// Examples:` — must exactly reproduce the examples from the lodash documentation for that function
- `/// More examples:` — additional edge cases (empty args, null, wrong types, etc.)

## Style

`.rustfmt.toml` sets `reorder_imports = false` and `reorder_modules = false` — do not reorder imports or module declarations; the `mod.rs` files keep lodash's alphabetical function order interleaved with stubs.
