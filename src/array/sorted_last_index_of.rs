use crate::lib::{json, Value};
use crate::array::sorted_index::sorted_index_impl;

/// `_x` helper for [sorted_last_index_of()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index_of_x;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_of_x(json!([4, 5, 5, 5, 6]), json!(5)), 3);
/// ```
pub fn sorted_last_index_of_x(array: Value, value: Value) -> isize {
    let i = sorted_index_impl(&array, &value, true, |v| v.clone());
    if let Value::Array(vec) = &array
        && i > 0
        && vec[i - 1] == value
    {
        return (i - 1) as isize;
    }
    -1
}
/// See lodash [sortedLastIndexOf](https://lodash.com/docs/#sortedLastIndexOf)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index_of;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_of(json!([4, 5, 5, 5, 6]), json!(5)), json!(3));
/// ```
pub fn sorted_last_index_of(array: Value, value: Value) -> Value {
    json!(sorted_last_index_of_x(array, value))
}

/// Based on [sorted_last_index_of_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_of_x!(json!([4, 5, 5, 5, 6]), json!(5)), 3);
/// ```
#[macro_export]
macro_rules! sorted_last_index_of_x {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index_of_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_last_index_of_x($a, $b)
    };
}
/// Based on [sorted_last_index_of()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_last_index_of!(json!([4, 5, 5, 5, 6]), json!(5)), json!(3));
/// assert_eq!(sorted_last_index_of!(), json!(-1));
/// assert_eq!(sorted_last_index_of!(json!([1, 2, 3]), json!(9)), json!(-1));
/// ```
#[macro_export]
macro_rules! sorted_last_index_of {
    () => {
        $crate::lib::json!(-1)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(-1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index_of($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_last_index_of($a, $b)
    };
}
