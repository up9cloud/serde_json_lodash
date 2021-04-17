# serde_json_lodash

[serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html) with [lodash.js](https://github.com/lodash/lodash) spec, makes life easier.

## Contributes

- All functions should be same as lodash as possible
- Every function from lodash.js should be implemented with 1 `macro`, 1 `fn`:
  - `macro` should be exactly same api as lodash.js version. (the main inputs and return value should only be `serde_json::Value` or `std::ops::Fn`)
    - e.q. `capitalize!(Value) -> Value`
  - `fn`
    - e.q. `capitalize(Value) -> Value`
- If the input parameters are options, we don't use Value as type, use primitive type
  - e.q. `_.chunk(array, [size=1])` => `::check(json!([1,2,3]), 2)`, size should be `usize`, not `Value::Number`
- If the original function allows optional parameters:
  - known amount, e.q. `_.get(object, path, [defaultValue])`, the ported version fn should be `::get(object, path, defaultValue)`, no optional
  - infinity amount, e.q. `_.merge(object, [...sources])`, the ported version fn should be `::merge(object, source)`, no more optionals
- It might implement helper functions also.
  - `fn` with `x_` prefix: input is not Value
    - e.q. `x_capitalize(&str) -> Value`
  - `fn` with `_x` suffix: output is not Value
    - e.q. `capitalize_x(Value) -> String`
  - `fn` with both `x_` and `_x`
    - e.q. `x_capitalize_x(&str) -> &str`
  - Implementing depends on that function accept multiple types or not.
    - e.q. `_.merge({a:1}, {b:2})`, `_.merge([1], [2])`, we don't know which type should be for, so we don't implement
- `Examples:` section should be exactly same as the examples in lodash doc. If we need more examples, they should be put in the `More Examples:` section
- Test cases should all be in the `More examples` section, we relied on rust's powerful doc testing

## Dev

```bash
# Up
cargo watch -x "test" -w "Cargo.toml" -w "src"

# Lint
./lint.sh

# Bump version
cargo bump patch --git-tag # same as npm version
# and remember do git commit and push!
```

> verify lodash.js api

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
