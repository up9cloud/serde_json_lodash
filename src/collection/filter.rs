use crate::lib::Value;

use crate::collection::collect::collection_values;

/// Fn form of [filter!](crate::filter!); see it for the full docs
///
/// `_x` form: **not provided** — see [filter_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::filter;
/// # use serde_json::json;
/// assert_eq!(filter(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!([1, 3]));
/// ```
pub fn filter(collection: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    Value::Array(
        collection_values(collection)
            .into_iter()
            .filter(predicate)
            .collect(),
    )
}

/// See lodash [filter](https://lodash.com/docs/#filter)
///
/// Fn form: [filter()] | `_x` form: **not provided** — see [filter_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney", "age": 36, "active": true },
///   { "user": "fred",   "age": 40, "active": false }
/// ]);
/// assert_eq!(
///   filter!(users.clone(), |o| !o["active"].as_bool().unwrap()),
///   json!([{ "user": "fred", "age": 40, "active": false }])
/// );
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(
///   filter!(users.clone(), json!({ "age": 36, "active": true })),
///   json!([{ "user": "barney", "age": 36, "active": true }])
/// );
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(
///   filter!(users.clone(), json!(["active", false])),
///   json!([{ "user": "fred", "age": 40, "active": false }])
/// );
/// // The `_.property` iteratee shorthand.
/// assert_eq!(
///   filter!(users, "active"),
///   json!([{ "user": "barney", "age": 36, "active": true }])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(filter!(), json!([]));
/// assert_eq!(filter!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(filter!(json!({"a": 1, "b": 2}), |v| v.as_i64().unwrap() > 1), json!([2]));
/// // predicates are `impl Fn`, so closures may capture their environment
/// let threshold = 2;
/// assert_eq!(filter!(json!([1, 2, 3, 4]), |v| v.as_i64().unwrap() >= threshold), json!([2, 3, 4]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(filter!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([{"a":0,"b":1},{"a":2,"b":1}]));
/// assert_eq!(filter!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!([{"a":2,"b":1}]));
/// assert_eq!(filter!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([{"a":2,"b":1},{"a":3,"b":2}]));
/// // a fully qualified `serde_json::json!` works as a shorthand too
/// assert_eq!(
///   filter!(json!([{"a": 1}, {"a": 2}]), serde_json::json!({"a": 1})),
///   json!([{"a": 1}])
/// );
/// // a shorthand held in a variable must be wrapped explicitly: predicate
/// // positions take the bool combinators [matches()](fn@crate::matches) /
/// // [matches_property()](fn@crate::matches_property)
/// use serde_json_lodash::matches;
/// let spec = json!({"a": 1});
/// assert_eq!(filter!(json!([{"a": 1}, {"a": 2}]), matches(spec)), json!([{"a": 1}]));
/// ```
#[macro_export]
macro_rules! filter {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::map($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::filter($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::filter($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::filter($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::filter($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::filter($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::filter($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::filter($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::filter($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [filter!](crate::filter!) and read the returned
/// `Value`.
///
/// Macro form: [filter_x!](crate::filter_x!)
pub fn filter_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [filter!](crate::filter!) and read the returned
/// `Value`.
///
/// Fn form: [filter_x()]
#[macro_export]
macro_rules! filter_x {
    ($($t:tt)*) => {
        $crate::filter_x()
    };
}
