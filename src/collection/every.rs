use crate::lib::{Value, json};

use crate::collection::collect::collection_values;

/// Fn form of [every!](crate::every!); see it for the full docs
///
/// `_x` forms: [every_x!](crate::every_x!), [every_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::every;
/// # use serde_json::json;
/// assert_eq!(every(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), json!(true));
/// ```
pub fn every(collection: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    json!(every_x(collection, predicate))
}

/// See lodash [every](https://lodash.com/docs/#every)
///
/// Fn form: [every()] | `_x` forms: [every_x!](crate::every_x!), [every_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // `Boolean` is JS truthiness — the identity (`null`) shorthand here
/// assert_eq!(every!(json!([true, 1, null, "yes"]), json!(null)), json!(false));
/// let users = json!([
///   { "user": "barney", "age": 36, "active": false },
///   { "user": "fred",   "age": 40, "active": false }
/// ]);
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(every!(users.clone(), json!({ "user": "barney", "active": false })), json!(false));
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(every!(users.clone(), json!(["active", false])), json!(true));
/// // The `_.property` iteratee shorthand.
/// assert_eq!(every!(users, "active"), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(every!(), json!(true));
/// assert_eq!(every!(json!(null)), json!(true));
/// assert_eq!(every!(json!({"a": 1})), json!(true));
/// assert_eq!(every!(json!([])), json!(true));
/// assert_eq!(every!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), json!(true));
/// assert_eq!(every!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1), json!(false));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(every!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!(false));
/// assert_eq!(every!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!(false));
/// assert_eq!(every!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!(false));
/// ```
#[macro_export]
macro_rules! every {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(true)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::every($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::every($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::every($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::every($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::every($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::every($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::every($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::every($a, $b)
    };
}

/// `_x` helper for [every!](crate::every!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [every_x!](crate::every_x!) | `Value` forms: [every!](crate::every!), [every()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::every_x;
/// # use serde_json::json;
/// assert_eq!(every_x(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), true);
/// ```
pub fn every_x(collection: Value, predicate: impl Fn(&Value) -> bool) -> bool {
    collection_values(collection).iter().all(predicate)
}

/// `_x` helper for [every!](crate::every!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [every_x()] | `Value` forms: [every!](crate::every!), [every()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(every_x!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), true);
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(every_x!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), false);
/// ```
#[macro_export]
macro_rules! every_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::every_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::every_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::every_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::every_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::every_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::every_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::every_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::every_x($a, $b)
    };
}
