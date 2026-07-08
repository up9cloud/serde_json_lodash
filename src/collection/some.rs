use crate::lib::{Value, json};

use crate::collection::collect::collection_values;

/// Fn form of [some!](crate::some!); see it for the full docs
///
/// `_x` forms: [some_x!](crate::some_x!), [some_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::some;
/// # use serde_json::json;
/// assert_eq!(some(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), json!(true));
/// ```
pub fn some(collection: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    json!(some_x(collection, predicate))
}

/// See lodash [some](https://lodash.com/docs/#some)
///
/// Fn form: [some()] | `_x` forms: [some_x!](crate::some_x!), [some_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // `Boolean` is JS truthiness — the identity (`null`) shorthand here
/// assert_eq!(some!(json!([null, 0, "yes", false]), json!(null)), json!(true));
/// let users = json!([
///   { "user": "barney", "active": true },
///   { "user": "fred",   "active": false }
/// ]);
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(some!(users.clone(), json!({ "user": "barney", "active": false })), json!(false));
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(some!(users.clone(), json!(["active", false])), json!(true));
/// // The `_.property` iteratee shorthand.
/// assert_eq!(some!(users, "active"), json!(true));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(some!(), json!(false));
/// assert_eq!(some!(json!(null)), json!(false));
/// assert_eq!(some!(json!({"a": 1})), json!(false));
/// assert_eq!(some!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), json!(true));
/// assert_eq!(some!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 5), json!(false));
/// assert_eq!(some!(json!([])), json!(false));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(some!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!(true));
/// assert_eq!(some!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!(true));
/// assert_eq!(some!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!(true));
/// ```
#[macro_export]
macro_rules! some {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::some($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::some($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::some($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::some($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::some($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::some($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::some($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::some($a, $b)
    };
}

/// `_x` helper for [some!](crate::some!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [some_x!](crate::some_x!) | `Value` forms: [some!](crate::some!), [some()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::some_x;
/// # use serde_json::json;
/// assert_eq!(some_x(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), true);
/// ```
pub fn some_x(collection: Value, predicate: impl Fn(&Value) -> bool) -> bool {
    collection_values(collection).iter().any(predicate)
}

/// `_x` helper for [some!](crate::some!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [some_x()] | `Value` forms: [some!](crate::some!), [some()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(some_x!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), true);
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(some_x!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), true);
/// ```
#[macro_export]
macro_rules! some_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::some_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::some_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::some_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::some_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::some_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::some_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::some_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::some_x($a, $b)
    };
}
