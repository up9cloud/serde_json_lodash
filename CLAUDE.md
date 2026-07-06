# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Rust port of lodash.js that operates on `serde_json::Value`. Every function mirrors the lodash spec as closely as possible, and all tests are Rust doc tests embedded in the source.

README.md and the crate-level rustdoc (`//!` in `src/lib.rs`) serve the same purpose, so keep them in sync — in particular the `## Usage` example must be identical in both. Feature/behavior descriptions belong in README.md and Cargo.toml stays comment-free (don't duplicate the feature docs as `#` comments in `Cargo.toml`).

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

# Release: commit first, then bump + tag (pushing the tag makes CI publish to crates.io)
cargo bump patch --git-tag   # or minor / major
git push && git push --tags
```

Update the `serde_json_lodash = "..."` version in README.md's Install section when a release changes the required version.

Edition is 2024. Features: `alias` (the `paste`-based aliasing machinery plus snake_case lodash aliases like `first`/`entries`/`has_in`; pulls in the optional `paste` crate — without `alias` those names don't exist, use the canonical name), `camel` (camelCase aliases — the **default** feature — requires and enables `alias`), `lazy_static`, `all` (`camel` + `lazy_static`). Because `camel` depends on `alias`, the default build has both. The crate has `#![deny(missing_docs)]` and `#![deny(warnings)]`, so any public item without a doc comment (even an empty `///`) fails the build. `src/seq`, `src/properties`, and `src/methods` are not re-exported from `src/lib_snake.rs` (they hold only commented-out chaining/template-settings stubs); do not add empty glob re-exports of them or the build fails under `#![deny(warnings)]`.

Functions that return or invoke JS functions cannot be represented as `serde_json::Value` and remain `todo!()` stubs by design: the entire `function/` category (debounce, curry, memoize, …), most of `util/` (flow, iteratee, property, matches, over, …), plus `invoke`, `invoke_map`, `create`, `set_with`, `update_with`, and `string` `template`.

## Architecture

- `src/{array,collection,date,function,lang,math,number,object,seq,string,util,properties,methods}/` — one directory per lodash docs category. Each category's `mod.rs` contains `#[doc(hidden)] todo!()` stubs for every not-yet-implemented lodash function; implementing a function means replacing its stub with `mod <name>; pub use <name>::*;` and creating `<name>.rs`.
- `src/lib_snake.rs` re-exports the implemented category modules (snake_case API). `src/lib_camel.rs` (behind the `camel` feature) generates camelCase aliases via `build_camel_links![ camelName => snake_name … ]`, which expands each pair through `build_camel_link!`. Every multi-word function must be registered there to get a camelCase alias. Category `mod.rs` files also declare aliases next to their targets: `build_link!` for snake aliases (`first => head`, gated on `alias`) and `build_camel_link!` for camelCase ones (`hasIn => has`, gated on `camel`).
- `src/internal.rs` also holds shared helpers used across categories: `words_vec`/`compound_words`/`capitalize_word` (string casing), `compare_values` (JS-style ordering), `value_to_option_number`/`f64_to_number` (numeric coercion), `uniq_by_key` (array dedup), `base_is_match` (partial deep match), and `rand_f64` (std-only RNG for `random`/`sample`/`shuffle`).
- `src/macros.rs` — `with_dollar_sign!` (nested `macro_rules!` `$`-escaping workaround) plus the alias generators `build_link!` (gated on `alias`) and `build_camel_link!` (gated on `camel`), both using `paste` to derive `_x` idents. Each aliases a name's whole family — `$from`/`$from_x` fns and `$from!`/`$from_x!` macros — forwarding to the `$to` equivalents; `build_camel_link!` spells the camelCase `_x` form with an `X` suffix (`hasInX`). This is why every function needs all four forms (see below).
- `src/internal.rs` — JS-compat constants (`MAX_SAFE_INTEGER`, etc.) and helpers like `value_undefined()` (JS `undefined` maps to `Value::Null`).

## Per-function file convention

Each `<name>.rs` implements the base fn and a `_x` output helper, each with a matching `macro_rules!` (macros provide lodash-style optional arguments), so every name has all four forms `name` / `name_x` / `name!` / `name_x!`:

- `name(impl Into<Value>, ...) -> Value` — the canonical form. Data params are generic over `Into<Value>`, so a primitive (`&str`, a number, …) **or** a `json!` value can be passed directly.
- `name_x(...) -> <primitive>` — same input, output downgraded to a primitive (`String`, `bool`, `f64`, `Vec<Value>`, …). Where a result has no single primitive form (a collection, or a value whose type is only known at runtime like `get`/`nth`), `name_x` is an unimplemented `todo!()` void marker that documents why. (Not-ported `todo!()` stubs likewise carry all four forms so aliases can point at them.)

There is no longer any `x_name` / `x_name_x` primitive-**input** helper — the generic `Into<Value>` base subsumes them. Option-like parameters (sizes, indexes, pad chars) take primitive types (`usize`, `isize`, `&str`), not `Value`; predicates take `Fn(&Value) -> bool`. Lodash functions with unlimited optional args keep exactly one in the fn form; the macro accepts more (e.g. `merge!(a, b, c)`).

## Test convention

Tests are doc tests on the macros/fns, in two sections:

- `/// Examples:` — must exactly reproduce the examples from the lodash documentation for that function
- `/// Additional cases:` — everything else worth showing (edge cases, empty/optional args, null, type coercion, JSON specifics)

## Style

`.rustfmt.toml` sets `reorder_imports = false` and `reorder_modules = false` — do not reorder imports or module declarations; the `mod.rs` files keep lodash's alphabetical function order interleaved with stubs.
