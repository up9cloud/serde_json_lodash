use crate::lib::{Value, json};

use crate::array::sorted_index::sorted_index_impl;

/// Fn form of [sorted_last_index!](crate::sorted_last_index!); see it for the full docs
///
/// `_x` forms: [sorted_last_index_x!](crate::sorted_last_index_x!), [sorted_last_index_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index(json!([4, 5, 5, 5, 6]), json!(5)), json!(4));
/// ```
pub fn sorted_last_index(array: Value, value: Value) -> Value {
    json!(sorted_last_index_x(array, value))
}

/// See lodash [sortedLastIndex](https://lodash.com/docs/#sortedLastIndex)
///
/// Fn form: [sorted_last_index()] | `_x` forms: [sorted_last_index_x!](crate::sorted_last_index_x!), [sorted_last_index_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_last_index!(json!([4, 5, 5, 5, 6]), json!(5)), json!(4));
/// assert_eq!(sorted_last_index!(), json!(0));
/// assert_eq!(sorted_last_index!(json!([20, 30, 30, 50]), json!(30)), json!(3));
/// ```
#[macro_export]
macro_rules! sorted_last_index {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_last_index($a, $b)
    };
}

/// `_x` helper for [sorted_last_index!](crate::sorted_last_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [sorted_last_index_x!](crate::sorted_last_index_x!) | `Value` forms: [sorted_last_index!](crate::sorted_last_index!), [sorted_last_index()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_last_index_x;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_x(json!([4, 5, 5, 5, 6]), json!(5)), 4);
/// ```
pub fn sorted_last_index_x(array: Value, value: Value) -> usize {
    sorted_index_impl(&array, &value, true, |v| v.clone())
}

/// `_x` helper for [sorted_last_index!](crate::sorted_last_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [sorted_last_index_x()] | `Value` forms: [sorted_last_index!](crate::sorted_last_index!), [sorted_last_index()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_last_index_x!(json!([4, 5, 5, 5, 6]), json!(5)), 4);
/// ```
#[macro_export]
macro_rules! sorted_last_index_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_last_index_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_last_index_x($a, $b)
    };
}
