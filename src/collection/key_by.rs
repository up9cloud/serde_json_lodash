use crate::lib::{Map, Value};

use crate::to_string_x;

use crate::collection::collect::collection_values;

/// Fn form of [key_by!](crate::key_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [key_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::key_by;
/// # use serde_json::json;
/// assert_eq!(key_by(json!(["a", "b"]), |v| v.clone()), json!({"a": "a", "b": "b"}));
/// ```
pub fn key_by(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    for v in collection_values(collection) {
        let key = to_string_x(iteratee(&v));
        out.insert(key, v);
    }
    Value::Object(out)
}

/// See lodash [keyBy](https://lodash.com/docs/#keyBy)
///
/// `iteratee` maps each element to its key (coerced to a string)
///
/// Fn form: [key_by()] | `_x` form: **not provided** — see [key_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([
///   { "dir": "left", "code": 97 },
///   { "dir": "right", "code": 100 }
/// ]);
/// assert_eq!(
///   key_by!(array, |o| o["dir"].clone()),
///   json!({
///     "left": { "dir": "left", "code": 97 },
///     "right": { "dir": "right", "code": 100 }
///   })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(key_by!(), json!({}));
/// assert_eq!(key_by!(json!(["a", "b"])), json!({"a": "a", "b": "b"}));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(key_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!({"0":{"a":0,"b":1},"2":{"a":2,"b":1},"3":{"a":3,"b":2}}));
/// assert_eq!(key_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!({"false":{"a":3,"b":2},"true":{"a":2,"b":1}}));
/// ```
#[macro_export]
macro_rules! key_by {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::key_by($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::key_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::key_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::key_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::key_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::key_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::key_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::key_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::key_by($a, $b)
    };
}

build_not_provided_x!(key_by, key_by_x);
