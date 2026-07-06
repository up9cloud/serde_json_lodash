# serde_json_lodash

[![Documentation](https://img.shields.io/crates/v/serde_json_lodash?label=latest)](https://docs.rs/serde_json_lodash)
[![build status](https://github.com/up9cloud/serde_json_lodash/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/up9cloud/serde_json_lodash/actions)
![Downloads](https://img.shields.io/crates/d/serde_json_lodash.svg)

[lodash.js](https://lodash.com/docs) ported to Rust, operating on [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html).

## Install

```toml
[dependencies]
serde_json_lodash = "0.3"
# for camelCase aliases (isEmpty, hasIn, …):
# serde_json_lodash = { version = "0.3", features = ["camel"] }
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
- Update version in Cargo.toml
- Update the install version in this README's [Install](#install) section if the new version needs a different `serde_json_lodash = "..."` requirement.
- Add git tag (without v prefix), e.g. `0.0.1`, NOT v0.0.1
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
