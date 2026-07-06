# serde_json_lodash

[![Documentation](https://img.shields.io/crates/v/serde_json_lodash?label=latest)](https://docs.rs/serde_json_lodash)
[![build status](https://github.com/up9cloud/serde_json_lodash/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/up9cloud/serde_json_lodash/actions)
![Downloads](https://img.shields.io/crates/d/serde_json_lodash.svg)

[lodash.js](https://lodash.com/docs) ported to Rust, operating on [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html).

## Install

```toml
[dependencies]
serde_json_lodash = "0.2"
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

### Naming: `_x` primitive-output helpers

Everything is `serde_json::Value` in and out. A data argument may also be a
primitive (the base fn/macro are generic over `Into<Value>`), and every function
has a `_x` variant that returns a primitive instead of a `Value`:

| Form           | Output   | Example (either arg form works)         |
| -------------- | -------- | --------------------------------------- |
| `capitalize`   | `Value`  | `capitalize("FRED")   // json!("Fred")` |
| `capitalize_x` | `String` | `capitalize_x("FRED") // "Fred"`        |

Options that aren't data (sizes, indexes, pad chars) stay primitive (`usize`,
`isize`, `&str`); predicates use `Fn(&Value) -> bool`. Where a result has no
single primitive form (a collection, or a runtime-typed value like `get`/`nth`),
`_x` is an unimplemented `todo!()` marker. JSON has no `undefined`, so functions
that would return it return `Value::Null`.

Every name exists as a `fn` and a macro, for both the base and `_x` form.

## Features

| Feature       | Default | Description                                                                                                                                                       |
| ------------- | :-----: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `alias`       |         | Aliasing machinery + snake_case lodash aliases (`first`, `entries`, `has_in`, …). Without it, use the canonical name (`head`, not `first`). Pulls in the optional [`paste`](https://crates.io/crates/paste) crate. |
| `camel`       |    ✓    | camelCase aliases (`isEmpty`, `hasIn`, …) and their `X`-suffixed `_x` forms (`isEmptyX`). Requires (and enables) `alias`.                                          |
| `lazy_static` |         | Enables `unique_id` / `uniqueId`.                                                                                                                                 |
| `all`         |         | `camel` + `lazy_static`.                                                                                                                                          |

Each alias re-exports the whole family (`fn`, macro, and both `_x` variants);
snake aliases keep the `_x` suffix (`has_in_x`), camelCase aliases use `X` (`hasInX`).

## What isn't ported

Functions whose result is itself a **function** (`debounce`, `curry`,
`memoize`, `flow`, `iteratee`, `property`, …) or that invoke object
**methods** (`invoke`, `invokeMap`, `create`) have no meaningful mapping onto
`serde_json::Value` and are intentionally left unimplemented. Each such stub is
annotated in the source with the reason.

## Dev memo

```bash
./dev.sh              # watch + test everything
./dev.sh --doc set    # watch + test one function's doc tests
cargo test --doc set  # run one function's doc tests once

./lint.sh             # cargo fmt + clippy
cargo doc --open      # preview docs
```

Tests live in the doc comments: the `Examples:` block mirrors the lodash
documentation for that function, and `Additional cases:` covers everything
else worth showing (edge cases, empty/optional args, type coercion, JSON
specifics).

### Releasing a new version

- Commit all your changes first (a clean working tree keeps the tag accurate).
- Bump the version and create the git tag with [`cargo-bump`](https://crates.io/crates/cargo-bump):

   ```bash
   cargo bump patch --git-tag   # 0.1.16 -> 0.1.17 (bug fixes)
   cargo bump minor --git-tag   # 0.1.16 -> 0.2.0  (new features)
   cargo bump major --git-tag   # 0.1.16 -> 1.0.0  (breaking changes)
   ```

- Update the install version in this README's [Install](#install) section if the new version needs a different `serde_json_lodash = "..."` requirement.
- Push the commit and the tag — pushing the tag triggers CI to publish to crates.io:

   ```bash
   git push && git push --tags
   ```

### Checking lodash's real behavior

```console
# node
Welcome to Node.js v24.1.0.
Type ".help" for more information.
> const {default: l} = await import('lodash')
undefined
> l.capitalize('FRED')
'Fred'
```
