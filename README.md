# serde_json_lodash

[![Documentation](https://img.shields.io/crates/v/serde_json_lodash?label=latest)](https://docs.rs/serde_json_lodash)
[![build status](https://github.com/up9cloud/serde_json_lodash/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/up9cloud/serde_json_lodash/actions)
![Downloads](https://img.shields.io/crates/d/serde_json_lodash.svg)

[serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html) with [lodash.js](https://github.com/lodash/lodash) spec, makes life easier.

## Usage

> Cargo.toml

```toml
[dependencies]
serde_json_lodash = "0.1"
```

> main.rs

```rust
#[macro_use] extern crate serde_json_lodash;
use serde_json::json;
fn main() {
  // macro style, optional parameters
  assert_eq!(
    merge!(json!({'a':1}), json!({'b':2}), json!({'c':3})),
    json!({'a': 1, 'b': 2, 'c': 3})
  );

  // fn style, fixed parameters
  use serde_json_lodash::merge;
  assert_eq!(
    merge(json!({'a':1}), json!({'b':2})),
    json!({'a': 1, 'b': 2})
  );

  // `x_`, `_x` helpers for simple types
  assert_eq!(capitalize!(json!("FRED")), json!("Fred"));
  assert_eq!(x_capitalize!("FRED"), json!("Fred"));
  assert_eq!(capitalize_x!(json!("FRED")), "Fred".to_owned());
  assert_eq!(x_capitalize_x!("FRED"), "Fred".to_owned());
}
```

## Concepts

All implements should be same as lodash as possible

How?

- Every function from lodash.js should be implemented both `fn` and `macro`
  - marco is for optional parameters usages
- The main inputs and return values should be *`serde_json::Value`*, excepts:
  - Inputs:
    - If the input parameters are options, not data, always using *primitive type* instead Value
      - e.q. `_.chunk(array, [size=1])` => `::chunk(json!([1,2,3]), 2)`, size should be `usize`, not `Value::Number`
    - Some cases we use *`std::ops::Fn`* as input parameter
      - e.q. `_.findIndex(array, predicate, ...)` => `::find_index(..., predicate: fn(&Value) -> bool, ...)`
  - Retune values:
    - If return value is statistic, using *primitive type* instead Value
      - e.q. `_.findIndex(...)` => `::find_index(...) -> isize`, return value should be `isize`, not `Value::Number`
    - Because there is no `undefined` type in serde_json, so if the original function return `undefined`, the ported version should return Value::Null
- If the original function allows optional parameters:
  - known amount, then the ported fn should *should be as required*
    - e.q. `_.get(object, path, [defaultValue])` => `::get(object, path, defaultValue)`
  - infinity amount, the ported fn should *only keep one, and no more optionals*
    - e.q. `_.merge(object, [...sources])` => `::merge(object, source)`, but macro could `::merge!(object, source1, source2, ...)`
- It might implement helper functions, for different input and output types:
  - with *`x_` prefix*: input is not Value, will be downgrade type
    - e.q. `x_capitalize(&str) -> Value`
  - with *`_x` suffix*: output is not Value, will be downgrade type
    - e.q. `capitalize_x(Value) -> String`
  - with *both `x_` and `_x`*
    - e.q. `x_capitalize_x(&str) -> &str`, `x_add_x(n: Number, n2: Number) -> Number`
  - If the function accept multiple types, the helper functions will only choose one type to implement
    - e.q. `_.toString([1,2])`, `_.toString(123)` => `::x_to_string(v: &str) -> Value`
- About the test cases:
  - `Examples:` section should be exactly same as the examples in lodash doc.
  - More test cases should all be put in the `More examples` section, we relied on powerful rust's doc test

## Dev memo

```bash
# Up
./dev.sh

# Watch and test single file
./dev.sh --doc set

# Lint
./lint.sh

# Preview doc
cargo doc --open

# Bump patch version and push
./bump_push.sh
```

> Check lodash.js api

```console
$ npm i
$ node
Welcome to Node.js v15.14.0.
Type ".help" for more information.
> const l = require('lodash')
undefined
> l.toString()
''
>
```
