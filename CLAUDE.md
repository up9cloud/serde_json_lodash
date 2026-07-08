# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Rust port of lodash.js that operates on `serde_json::Value`. Every function mirrors the lodash spec as closely as possible, and all tests are Rust doc tests embedded in the source.

## Design priorities

When goals conflict, they rank in this order — never trade a higher one for a lower one:

1. **lodash API fidelity** — names, signatures, argument order, and behavior follow the lodash docs as closely as Rust allows.
2. **Ergonomics** — easy to call: `Into<Value>` inputs (primitives or `json!` values), macros for lodash-style optional/variadic args, `impl Fn` callbacks that accept capturing closures.
3. **Performance** — optimize only within the bounds of 1 and 2 (e.g. the owned `Into<Value>` API stays even though by-ref variants would be faster).

README.md and the crate-level rustdoc (`//!` in `src/lib.rs`) serve the same purpose, so keep them in sync — in particular the `## Usage` example must be identical in both. Feature/behavior descriptions belong in README.md and Cargo.toml stays comment-free (don't duplicate the feature docs as `#` comments in `Cargo.toml`).

## Commands

Dev shortcuts are cargo aliases in `.cargo/config.toml` (keep them and CONTRIBUTING.md's Dev memo in sync).

```bash
# Run all tests (doc tests are the test suite; lazy_static feature is needed by some examples)
cargo t                 # = cargo test --features lazy_static

# Test a single function's doc tests (e.g. `set`)
cargo t --doc set

# Watch mode (needs cargo-watch)
cargo dev               # all tests

# Lint — CI enforces both of these
cargo fmt --all -- --check
cargo lint              # = cargo clippy --all-features -- -D warnings

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

README.md's Install section uses `cargo add serde_json_lodash` (no pinned version), so releases never require a README version bump.

Edition is 2024. **No features are enabled by default** (snake_case fns/macros only). Features: `alias` (the `paste`-based aliasing machinery plus snake_case lodash aliases like `first`/`entries`/`has_in`; pulls in the optional `paste` crate — without `alias` those names don't exist, use the canonical name), `camel` (camelCase aliases — requires and enables `alias`), `lazy_static`, `all` (`camel` + `lazy_static`). The crate has `#![deny(missing_docs)]` and `#![deny(warnings)]`, so any public item without a doc comment (even an empty `///`) fails the build. `src/seq`, `src/properties`, and `src/methods` are not re-exported from `src/lib_snake.rs` (they hold only commented-out chaining/template-settings stubs); do not add empty glob re-exports of them or the build fails under `#![deny(warnings)]`.

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

There is no longer any `x_name` / `x_name_x` primitive-**input** helper — the generic `Into<Value>` base subsumes them. Option-like parameters (sizes, indexes, pad chars) take primitive types (`usize`, `isize`, `&str`), not `Value`; predicates/iteratees take `impl Fn(...)` generics (closures may capture their environment; pass `&f` when a helper needs the callback more than once). Exception: `conforms_to`'s `Conform` pairs use `&dyn Fn` — a `Vec` can hold only one generic closure type, and different keys need different predicates, so callers borrow each closure (`vec![("a", &|v: &Value| …)]`). When a fn takes `&Value` (the `is_*` family, `eq`/`gt`/`lt`, `includes`, `has`, `clone*`, …) its macro arms wrap the argument themselves (`$crate::f(&$a)`), so callers may pass owned or borrowed values (`is_boolean!(json!(true))` and `is_boolean!(&v)` both work — `&&Value` deref-coerces); doc examples use the owned style. This applies to `&Value` params only, not `&str` ones. Lodash functions with unlimited optional args keep exactly one in the fn form; the macro accepts more (e.g. `merge!(a, b, c)`).

Item order within a file is fixed: internal helpers first, then `fn name`, `name!`, `fn name_x`, `name_x!`. `name!` is the recommended entry point (closest to the lodash experience), so it carries the primary docs: its first doc line is `See lodash [camelName](https://lodash.com/docs/#camelName)` plus any behavior notes, followed by a `Fn form: [name()] | `_x` forms: …` link line. The other three forms start with a pointer back to the macro (`Fn form of [name!](crate::name!)…` / `` `_x` helper for [name!](crate::name!)… ``) and cross-link their sibling forms so readers can switch quickly; macro links must use the qualified `[name!](crate::name!)` form (unqualified `[concat!]` etc. can collide with std macros). Void `_x` markers start their docs with `**Not provided.**` + the reason; not-ported stub macros in `mod.rs` repeat the fn's `**Not ported.**` line (never a bare `Based on […]`). Doc lines always come before attributes like `#[macro_export]`, and section headers (`Examples:`/`Additional cases:`) must be preceded by a blank `///` line so the rustdoc summary stays a single line.

## Test convention

Tests are doc tests on the macros/fns, in two sections:

- `/// Examples:` — must exactly reproduce the examples from the lodash documentation for that function
- `/// Additional cases:` — everything else worth showing (edge cases, empty/optional args, null, type coercion, JSON specifics)

## Style

Default rustfmt (no `.rustfmt.toml`). The `mod.rs` files keep lodash's doc order (`mod x; pub use x::*;` pairs interleaved with stubs) — rustfmt can't disturb it because it only reorders *consecutive* `mod`/`use` items, so keep that interleaved structure when adding functions.
