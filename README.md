# serde_json_lodash

[serde_json::Value](https://docs.serde.rs/serde_json/value/enum.Value.html) with [lodash.js](https://github.com/lodash/lodash) spec, makes life easier.

## Rules

- All functions should be same as possible as lodash
- If the original functions allow dynamical amount of parameters:
  - known amount, e.q. `_.get(object, path, [defaultValue])`, the ported fn should be designed as `::get(object, path, defaultValue)`
  - infinity amount, e.q. `_.merge(object, [...sources])`, the ported fn should be designed as `::merge(object, sources)`
- All parameters and return value should only be `serde_json::Value` or `std::ops::Fn`
- Some functions are impassible to implement, consider using macros
