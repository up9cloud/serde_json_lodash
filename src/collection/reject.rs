use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [reject!](crate::reject!); see it for the full docs
///
/// `_x` form: **not provided** — see [reject_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::reject;
/// # use serde_json::json;
/// assert_eq!(reject(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!([2, 4]));
/// ```
pub fn reject(collection: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    Value::Array(
        collection_values(collection)
            .into_iter()
            .filter(|v| !predicate(v))
            .collect(),
    )
}

/// See lodash [reject](https://lodash.com/docs/#reject)
///
/// The opposite of [filter()](fn@crate::filter)
///
/// Fn form: [reject()] | `_x` form: **not provided** — see [reject_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney", "age": 36, "active": false },
///   { "user": "fred",   "age": 40, "active": true }
/// ]);
/// assert_eq!(
///   reject!(users.clone(), |o| !o["active"].as_bool().unwrap()),
///   json!([{ "user": "fred", "age": 40, "active": true }])
/// );
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(
///   reject!(users.clone(), json!({ "age": 40, "active": true })),
///   json!([{ "user": "barney", "age": 36, "active": false }])
/// );
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(
///   reject!(users.clone(), json!(["active", false])),
///   json!([{ "user": "fred", "age": 40, "active": true }])
/// );
/// // The `_.property` iteratee shorthand.
/// assert_eq!(
///   reject!(users, "active"),
///   json!([{ "user": "barney", "age": 36, "active": false }])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(reject!(), json!([]));
/// assert_eq!(reject!(json!([1, 2, 3])), json!([]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(reject!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([{"a":3,"b":2}]));
/// assert_eq!(reject!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!([{"a":0,"b":1},{"a":3,"b":2}]));
/// assert_eq!(reject!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([{"a":0,"b":1}]));
/// ```
#[macro_export]
macro_rules! reject {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::reject($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::reject($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::reject($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::reject($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::reject($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::reject($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::reject($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::reject($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reject!](crate::reject!) and read the returned
/// `Value`.
///
/// Macro form: [reject_x!](crate::reject_x!)
pub fn reject_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [reject!](crate::reject!) and read the returned
/// `Value`.
///
/// Fn form: [reject_x()]
#[macro_export]
macro_rules! reject_x {
    ($($t:tt)*) => {
        $crate::reject_x()
    };
}
