# serde_json_lodash

[![Documentation](https://img.shields.io/crates/v/serde_json_lodash?label=latest)](https://docs.rs/serde_json_lodash)
[![build status](https://github.com/up9cloud/serde_json_lodash/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/up9cloud/serde_json_lodash/actions)
![Downloads](https://img.shields.io/crates/d/serde_json_lodash.svg)

[lodash.js](https://lodash.com/docs) ported to Rust, operating on [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html).

## Install

```bash
cargo add serde_json_lodash
# for camelCase aliases (isEmpty, hasIn, …):
cargo add serde_json_lodash --features camel
```

## Usage

```rust
#[macro_use] extern crate serde_json_lodash;
use serde_json::json;

fn main() {
    // Macro form: variadic / optional arguments, like the JS original
    assert_eq!(
        merge!(json!({"a": 1}), json!({"b": 2}), json!({"c": 3})),
        json!({"a": 1, "b": 2, "c": 3})
    );

    // Function form: fixed arguments
    use serde_json_lodash::capitalize;
    assert_eq!(capitalize(json!("FRED")), json!("Fred"));

    // A data argument can be a primitive instead of a `json!` value; the
    // `_x` form returns a primitive instead of a `Value`.
    assert_eq!(capitalize!("FRED"), json!("Fred"));
    assert_eq!(capitalize_x!("FRED"), "Fred".to_owned());
}
```

Every function comes in a `fn` and a `macro` flavor. Use the macro for lodash's
optional/variadic arguments; use the function for a fixed signature.

**Full documentation** — the naming convention (`_x` helpers, `Into<Value>`
inputs), the Cargo features, what isn't ported, and every function with its
examples — is on **[docs.rs](https://docs.rs/serde_json_lodash)**.

## TODO

Known gaps against lodash, in priority order:

- **Number unification (SameValueZero)** — JS has a single number type, so
  lodash treats `1` and `1.0` as the same value; `serde_json::Number` keeps
  them distinct, so every equality/hash-based function deviates today
  (`uniq!(json!([1, 1.0]))` → `[1, 1.0]` instead of `[1]`, `eq!(json!(1),
  json!(1.0))` → `false` instead of `true`, same for
  `intersection`/`difference`/`xor`/`includes`/`index_of`/`is_equal`/…).
  Fix: canonicalize integral floats to integer `Number`s in a shared
  SameValueZero helper and use it everywhere values are compared or hashed.
- **Iteratee shorthands (`_.matches` / `_.property`)** — lodash collection
  functions accept `{ 'active': true }` or `'active'` in place of a callback;
  this port only takes closures, which is also why some official doc examples
  can't be mirrored yet. Sketch: a `Predicate` trait with impls for
  `Fn(&Value) -> bool` closures, `Value` (partial deep match via the existing
  `base_is_match`) and `&str` (property access), accepted by
  `filter`/`find`/`every`/`some`/`reject`/`partition`/….

## Contributing

Dev commands, the release flow, and how to check lodash's real behavior are in
[CONTRIBUTING.md](CONTRIBUTING.md).
