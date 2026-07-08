use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [find!](crate::find!); see it for the full docs
///
/// `_x` form: **not provided** — see [find_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find;
/// # use serde_json::json;
/// assert_eq!(find(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1), json!(2));
/// ```
pub fn find(collection: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    collection_values(collection)
        .into_iter()
        .find(predicate)
        .unwrap_or(Value::Null)
}

/// See lodash [find](https://lodash.com/docs/#find)
///
/// Returns the first matching element, or `Null` if none match
///
/// Fn form: [find()] | `_x` form: **not provided** — see [find_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney",  "age": 36, "active": true },
///   { "user": "fred",    "age": 40, "active": false },
///   { "user": "pebbles", "age": 1,  "active": true }
/// ]);
/// assert_eq!(
///   find!(users.clone(), |o| o["age"].as_i64().unwrap() < 40),
///   json!({ "user": "barney", "age": 36, "active": true })
/// );
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(
///   find!(users.clone(), json!({ "age": 1, "active": true })),
///   json!({ "user": "pebbles", "age": 1, "active": true })
/// );
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(
///   find!(users.clone(), json!(["active", false])),
///   json!({ "user": "fred", "age": 40, "active": false })
/// );
/// // The `_.property` iteratee shorthand.
/// assert_eq!(
///   find!(users, "active"),
///   json!({ "user": "barney", "age": 36, "active": true })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find!(), json!(null));
/// assert_eq!(find!(json!([1, 2, 3]), |_| false), json!(null));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(find!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!({"a":0,"b":1}));
/// assert_eq!(find!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!({"a":2,"b":1}));
/// assert_eq!(find!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!({"a":2,"b":1}));
/// ```
#[macro_export]
macro_rules! find {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(null)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::find($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::find($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::find($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::find($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::find($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::find($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::find($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find!](crate::find!) and read the returned `Value`.
///
/// Macro form: [find_x!](crate::find_x!)
pub fn find_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find!](crate::find!) and read the returned `Value`.
///
/// Fn form: [find_x()]
#[macro_export]
macro_rules! find_x {
    ($($t:tt)*) => {
        $crate::find_x()
    };
}
