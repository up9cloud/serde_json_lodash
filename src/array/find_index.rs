use crate::lib::{Value, json};

use crate::internal::resolve_from_index;

/// Fn form of [find_index!](crate::find_index!); see it for the full docs
///
/// `_x` forms: [find_index_x!](crate::find_index_x!), [find_index_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_index;
/// # use serde_json::json;
/// assert_eq!(find_index(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 0), json!(1));
/// ```
pub fn find_index(array: Value, predicate: impl Fn(&Value) -> bool, from_index: isize) -> Value {
    json!(find_index_x(array, predicate, from_index))
}

/// See lodash [findIndex](https://lodash.com/docs/#findIndex)
///
/// Fn form: [find_index()] | `_x` forms: [find_index_x!](crate::find_index_x!), [find_index_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "barney",  "active": false },
///   { "user": "fred",    "active": false },
///   { "user": "pebbles", "active": true }
/// ]);
/// assert_eq!(
///   find_index!(users.clone(), |o| o["user"] == json!("barney")),
///   json!(0)
/// );
/// // The `_.matches` iteratee shorthand.
/// assert_eq!(
///   find_index!(users.clone(), json!({ "user": "fred", "active": false })),
///   json!(1)
/// );
/// // The `_.matchesProperty` iteratee shorthand.
/// assert_eq!(find_index!(users.clone(), json!(["active", false])), json!(0));
/// // The `_.property` iteratee shorthand.
/// assert_eq!(find_index!(users, "active"), json!(2));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_index!(), json!(-1));
/// assert_eq!(find_index!(json!(null)), json!(-1));
/// assert_eq!(find_index!(json!({"a": 1})), json!(-1));
/// // negative fromIndex counts back from the end
/// assert_eq!(find_index!(json!([1, 2, 1, 2]), |v| v == &json!(2), -2), json!(3));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(find_index!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["b", 1]), 1), json!(1));
/// assert_eq!(find_index!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a", 1), json!(1));
/// ```
#[macro_export]
macro_rules! find_index {
    () => {
        $crate::lib::json!(-1)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(-1)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), 0)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), 0)
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)), 0)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr $(,)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)), $c)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr, $($rest:tt)*) => {
        $crate::find_index($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)), $c)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_index($a, $b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::find_index($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::find_index($a, $b, $c)
    };
}

/// `_x` helper for [find_index!](crate::find_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [find_index_x!](crate::find_index_x!) | `Value` forms: [find_index!](crate::find_index!), [find_index()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_index_x;
/// # use serde_json::json;
/// assert_eq!(find_index_x(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 0), 1);
/// ```
pub fn find_index_x(array: Value, predicate: impl Fn(&Value) -> bool, from_index: isize) -> isize {
    match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return -1;
        }
        Value::Array(vec) => {
            let start = resolve_from_index(vec.len(), from_index);
            for (i, item) in vec.iter().enumerate().skip(start) {
                if predicate(item) {
                    return i as isize;
                }
            }
        }
    };
    -1
}

/// `_x` helper for [find_index!](crate::find_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [find_index_x()] | `Value` forms: [find_index!](crate::find_index!), [find_index()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_index_x!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 0), 1);
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(find_index_x!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), 1);
/// ```
#[macro_export]
macro_rules! find_index_x {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), 0)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), 0)
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)), 0)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr $(,)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)), $c)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr, $($rest:tt)*) => {
        $crate::find_index_x($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)), $c)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_index_x($a, $b, 0)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::find_index_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::find_index_x($a, $b, $c)
    };
}
