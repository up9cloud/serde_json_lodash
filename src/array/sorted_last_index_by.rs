use crate::lib::{Value, json};

use crate::array::sorted_index::sorted_index_impl;

/// Fn form of [sorted_last_index_by!](crate::sorted_last_index_by!); see it for the full docs
///
/// `_x` forms: [sorted_last_index_by_x!](crate::sorted_last_index_by_x!), [sorted_last_index_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index_by;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_by(json!([4, 5, 5, 6]), json!(5), |v| v.clone()), json!(3));
/// ```
pub fn sorted_last_index_by(array: Value, value: Value, iteratee: fn(&Value) -> Value) -> Value {
    json!(sorted_last_index_by_x(array, value, iteratee))
}

/// See lodash [sortedLastIndexBy](https://lodash.com/docs/#sortedLastIndexBy)
///
/// Fn form: [sorted_last_index_by()] | `_x` forms: [sorted_last_index_by_x!](crate::sorted_last_index_by_x!), [sorted_last_index_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_last_index_by!(), json!(0));
/// assert_eq!(sorted_last_index_by!(json!([4, 5, 5, 6]), json!(5)), json!(3));
/// ```
#[macro_export]
macro_rules! sorted_last_index_by {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_last_index_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_last_index_by($a, $b, $c)
    };
}

/// `_x` helper for [sorted_last_index_by!](crate::sorted_last_index_by!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [sorted_last_index_by_x!](crate::sorted_last_index_by_x!) | `Value` forms: [sorted_last_index_by!](crate::sorted_last_index_by!), [sorted_last_index_by()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index_by_x;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_by_x(json!([4, 5, 5, 6]), json!(5), |v| v.clone()), 3);
/// ```
pub fn sorted_last_index_by_x(array: Value, value: Value, iteratee: fn(&Value) -> Value) -> usize {
    sorted_index_impl(&array, &value, true, iteratee)
}

/// `_x` helper for [sorted_last_index_by!](crate::sorted_last_index_by!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [sorted_last_index_by_x()] | `Value` forms: [sorted_last_index_by!](crate::sorted_last_index_by!), [sorted_last_index_by()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_by_x!(json!([4, 5, 5, 6]), json!(5), |v| v.clone()), 3);
/// ```
#[macro_export]
macro_rules! sorted_last_index_by_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_last_index_by_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_last_index_by_x($a, $b, $c)
    };
}
