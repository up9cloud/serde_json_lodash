# serde_json_lodash

[serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html) with [lodash.js](https://github.com/lodash/lodash) spec, makes life easier.

## Dev memo

- All functions should be as same as possible as lodash
- All parameters and return value should only be `serde_json::Value` or `std::ops::Fn`
- There might be some useful functions, e.q. `::capitalize`, people don't want just with `serde_json::Value`, but also primitive types. So this lib export additional helper functions:
  - with `x_` prefix means input not Value, e.q. `x_capitalize(&str) -> Value`
  - with `_x` suffix means output not Value, e.q. `capitalize_x(Value) -> String`
  - and with both prefix and suffix, e.q. `x_capitalize_x(&str) -> &str`
- If the original function allow optional parameters:
  - known amount, e.q. `_.get(object, path, [defaultValue])`, the ported fn should be `::get(object, path, defaultValue)`, no optional
  - infinity amount, e.q. `_.merge(object, [...sources])`, the ported fn should be `::merge(object, source)`, no more optionals
- `Examples:` section should be exactly as same as the examples in lodash doc. If need more examples, they should be put in next `More Examples:` section
- Some functions are impassible to implement, consider using macros
