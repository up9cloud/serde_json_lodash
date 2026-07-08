use crate::lib::{Value, json};

use crate::collection::collect::collection_values;

/// Fn form of [partition!](crate::partition!); see it for the full docs
///
/// `_x` form: **not provided** — see [partition_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::partition;
/// # use serde_json::json;
/// assert_eq!(partition(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!([[1, 3], [2, 4]]));
/// ```
pub fn partition(collection: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    let (yes, no): (Vec<Value>, Vec<Value>) = collection_values(collection)
        .into_iter()
        .partition(predicate);
    json!([yes, no])
}

/// See lodash [partition](https://lodash.com/docs/#partition)
///
/// Returns `[matched, unmatched]`
///
/// Fn form: [partition()] | `_x` form: **not provided** — see [partition_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney",  "age": 36, "active": false },
///   { "user": "fred",    "age": 40, "active": true },
///   { "user": "pebbles", "age": 1,  "active": false }
/// ]);
/// assert_eq!(
///   partition!(users.clone(), |o| o["active"].as_bool().unwrap()),
///   json!([
///     [{ "user": "fred", "age": 40, "active": true }],
///     [{ "user": "barney", "age": 36, "active": false }, { "user": "pebbles", "age": 1, "active": false }]
///   ])
/// );
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(
///   partition!(users.clone(), json!({ "age": 1, "active": false })),
///   json!([
///     [{ "user": "pebbles", "age": 1, "active": false }],
///     [{ "user": "barney", "age": 36, "active": false }, { "user": "fred", "age": 40, "active": true }]
///   ])
/// );
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(
///   partition!(users.clone(), json!(["active", false])),
///   json!([
///     [{ "user": "barney", "age": 36, "active": false }, { "user": "pebbles", "age": 1, "active": false }],
///     [{ "user": "fred", "age": 40, "active": true }]
///   ])
/// );
/// // The `_.property` iteratee shorthand.
/// assert_eq!(
///   partition!(users, "active"),
///   json!([
///     [{ "user": "fred", "age": 40, "active": true }],
///     [{ "user": "barney", "age": 36, "active": false }, { "user": "pebbles", "age": 1, "active": false }]
///   ])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(partition!(), json!([[], []]));
/// assert_eq!(partition!(json!([1, 2, 3])), json!([[], [1, 2, 3]]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(partition!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([[{"a":0,"b":1},{"a":2,"b":1}],[{"a":3,"b":2}]]));
/// assert_eq!(partition!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!([[{"a":2,"b":1}],[{"a":0,"b":1},{"a":3,"b":2}]]));
/// assert_eq!(partition!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([[{"a":2,"b":1},{"a":3,"b":2}],[{"a":0,"b":1}]]));
/// ```
#[macro_export]
macro_rules! partition {
    () => {
        $crate::lib::json!([[], []])
    };
    ($a:expr $(,)*) => {
        $crate::partition($a, |_| false)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::partition($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::partition($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::partition($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::partition($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::partition($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::partition($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::partition($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::partition($a, $b)
    };
}

build_not_provided_x!(partition, partition_x);
