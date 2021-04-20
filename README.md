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
  assert_eq!(
    capitalize!(json!("FRED")),
    json!("Fred")
  );
}
```

## Contributes

All implements should be same as lodash as possible

How?

- Every function from lodash.js should be implemented both `fn` and `macro` (for optional parameters usages)
- The main inputs and return value should be *`serde_json::Value`*, except:
  - If the input parameters are options, not data, using *primitive type* instead Value
    - e.q. `_.chunk(array, [size=1])` => `::check!(json!([1,2,3]), 2)`, size should be `usize`, not `Value::Number`
  - Some cases we use *`std::ops::Fn`* as input parameter
    - e.q. `_.findIndex(array, predicate, ...)` => `::find_index(..., predicate: fn(&Value) -> bool, ...)`
  - If return value is statistic, using *primitive type* instead Value
    - e.q. `_.findIndex(...)` => `::find_index(...) -> isize`, return value should be `isize`, not `Value::Number`
  - Because there is no `undefined` type in json, so if original function return `undefined`, the ported fn should always return Value::Null
- If the original function allows optional parameters:
  - known amount, e.q. `_.get(object, path, [defaultValue])`, the ported version fn should be `::get(object, path, defaultValue)`, *optional should become required*
  - infinity amount, e.q. `_.merge(object, [...sources])`, the ported version fn should be `::merge(object, source)`, *should only keep one, depends on how the function works, and no more optionals*
- It might implement helper functions, e.q.:
  - `fn` with *`x_` prefix*: input is not Value
    - e.q. `x_capitalize(&str) -> Value`
  - `fn` with *`_x` suffix*: output is not Value
    - e.q. `capitalize_x(Value) -> String`
  - `fn` with *both `x_` and `_x`*
    - e.q. `x_capitalize_x(&str) -> &str`
  - If the function accept multiple types, we can only choose one to implement
    - e.q. `_.merge({a:1}, {b:2})`, `_.merge([1], [2])` => ...
- `Examples:` section should be exactly same as the examples in lodash doc.
- Test cases should all be written in the `More examples` section, we relied on powerful rust's doc test

## Dev

```bash
# Up
cargo watch -x "test" -w "Cargo.toml" -w "src"

# Lint
./lint.sh

# Bump version and push
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
