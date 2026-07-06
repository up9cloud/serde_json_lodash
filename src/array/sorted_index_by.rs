use crate::lib::{json, Value};
use crate::array::sorted_index::sorted_index_impl;

/// `_x` helper for [sorted_index_by()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_by_x;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by_x(json!([30, 50]), json!(40), |v| v.clone()), 1);
/// ```
pub fn sorted_index_by_x(array: Value, value: Value, iteratee: fn(&Value) -> Value) -> usize {
    sorted_index_impl(&array, &value, false, iteratee)
}
/// See lodash [sortedIndexBy](https://lodash.com/docs/#sortedIndexBy)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_by;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by(json!([30, 50]), json!(40), |v| v.clone()), json!(1));
/// ```
pub fn sorted_index_by(array: Value, value: Value, iteratee: fn(&Value) -> Value) -> Value {
    json!(sorted_index_by_x(array, value, iteratee))
}

/// Based on [sorted_index_by_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by_x!(json!([30, 50]), json!(40), |v| v.clone()), 1);
/// ```
#[macro_export]
macro_rules! sorted_index_by_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_index_by_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_index_by_x($a, $b, $c)
    };
}
/// Based on [sorted_index_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_index_by!(), json!(0));
/// assert_eq!(sorted_index_by!(json!([30, 50]), json!(40)), json!(1));
/// ```
#[macro_export]
macro_rules! sorted_index_by {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_index_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_index_by($a, $b, $c)
    };
}
