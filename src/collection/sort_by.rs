use crate::lib::Value;

use crate::internal::compare_values;

use crate::collection::collect::collection_values;

use std::cmp::Ordering;

/// Fn form of [sort_by!](crate::sort_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [sort_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sort_by;
/// # use serde_json::json;
/// assert_eq!(sort_by(json!([3, 1, 2]), |v| v.clone()), json!([1, 2, 3]));
/// ```
pub fn sort_by(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    // Schwartzian transform: compute each key once (n iteratee calls instead
    // of ~2·n·log n from inside the comparator)
    let mut keyed: Vec<(Value, Value)> = collection_values(collection)
        .into_iter()
        .map(|v| (iteratee(&v), v))
        .collect();
    keyed.sort_by(|(ka, _), (kb, _)| compare_values(ka, kb).unwrap_or(Ordering::Equal));
    Value::Array(keyed.into_iter().map(|(_, v)| v).collect())
}

/// See lodash [sortBy](https://lodash.com/docs/#sortBy)
///
/// `iteratee` maps each element to the value used for sorting (a stable,
/// ascending sort)
///
/// Fn form: [sort_by()] | `_x` form: **not provided** — see [sort_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "fred",   "age": 48 },
///   { "user": "barney", "age": 36 },
///   { "user": "fred",   "age": 30 },
///   { "user": "barney", "age": 34 }
/// ]);
/// assert_eq!(
///   sort_by!(users.clone(), |o| o["user"].clone()),
///   json!([
///     { "user": "barney", "age": 36 }, { "user": "barney", "age": 34 },
///     { "user": "fred",   "age": 48 }, { "user": "fred",   "age": 30 }
///   ])
/// );
/// // The `_.property` iteratee shorthand.
/// assert_eq!(
///   sort_by!(users, "user"),
///   json!([
///     { "user": "barney", "age": 36 }, { "user": "barney", "age": 34 },
///     { "user": "fred",   "age": 48 }, { "user": "fred",   "age": 30 }
///   ])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sort_by!(), json!([]));
/// assert_eq!(sort_by!(json!([3, 1, 2])), json!([1, 2, 3]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(sort_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([{"a":0,"b":1},{"a":2,"b":1},{"a":3,"b":2}]));
/// assert_eq!(sort_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([{"a":3,"b":2},{"a":0,"b":1},{"a":2,"b":1}]));
/// ```
#[macro_export]
macro_rules! sort_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sort_by($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::sort_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::sort_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::sort_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sort_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sort_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::sort_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sort_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sort_by($a, $b)
    };
}

build_not_provided_x!(sort_by, sort_by_x);
