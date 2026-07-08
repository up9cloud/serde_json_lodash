use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [flat_map!](crate::flat_map!); see it for the full docs
///
/// `_x` form: **not provided** — see [flat_map_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::flat_map;
/// # use serde_json::json;
/// assert_eq!(flat_map(json!([1, 2]), |n| json!([n.clone(), n.clone()])), json!([1, 1, 2, 2]));
/// ```
pub fn flat_map(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let mut out = vec![];
    for v in collection_values(collection) {
        match iteratee(&v) {
            Value::Array(inner) => out.extend(inner),
            other => out.push(other),
        }
    }
    Value::Array(out)
}

/// See lodash [flatMap](https://lodash.com/docs/#flatMap)
///
/// Maps each element with `iteratee`, then flattens the result one level
///
/// Fn form: [flat_map()] | `_x` form: **not provided** — see [flat_map_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   flat_map!(json!([1, 2]), |n| json!([n.clone(), n.clone()])),
///   json!([1, 1, 2, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(flat_map!(), json!([]));
/// assert_eq!(flat_map!(json!([1, 2, 3])), json!([1, 2, 3]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(flat_map!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([0,2,3]));
/// assert_eq!(flat_map!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([true,true,false]));
/// ```
#[macro_export]
macro_rules! flat_map {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::flat_map($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::flat_map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::flat_map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::flat_map($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::flat_map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::flat_map($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::flat_map($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::flat_map($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::flat_map($a, $b)
    };
}

build_not_provided_x!(flat_map, flat_map_x);
