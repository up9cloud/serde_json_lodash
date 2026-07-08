use crate::lib::{Value, json};

use crate::array::sorted_index::sorted_index_impl;

/// Fn form of [sorted_index_of!](crate::sorted_index_of!); see it for the full docs
///
/// `_x` forms: [sorted_index_of_x!](crate::sorted_index_of_x!), [sorted_index_of_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_of;
/// # use serde_json::json;
/// assert_eq!(sorted_index_of(json!([4, 5, 5, 5, 6]), json!(5)), json!(1));
/// ```
pub fn sorted_index_of(array: Value, value: Value) -> Value {
    json!(sorted_index_of_x(array, value))
}

/// See lodash [sortedIndexOf](https://lodash.com/docs/#sortedIndexOf)
///
/// Fn form: [sorted_index_of()] | `_x` forms: [sorted_index_of_x!](crate::sorted_index_of_x!), [sorted_index_of_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_index_of!(json!([4, 5, 5, 5, 6]), json!(5)), json!(1));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_of!(), json!(-1));
/// assert_eq!(sorted_index_of!(json!(null)), json!(-1));
/// assert_eq!(sorted_index_of!(json!({"a": 1})), json!(-1));
/// assert_eq!(sorted_index_of!(json!(null), json!(null)), json!(-1));
/// assert_eq!(sorted_index_of!(json!(1), json!(1)), json!(-1));
/// assert_eq!(sorted_index_of!(json!(1), json!(2)), json!(-1));
/// assert_eq!(sorted_index_of!(json!([1, 2, 3]), json!(2)), json!(1));
/// assert_eq!(sorted_index_of!(json!("abc"), json!("bc")), json!(-1));
/// assert_eq!(sorted_index_of!(json!([1, 2, 3]), json!(9)), json!(-1));
/// ```
#[macro_export]
macro_rules! sorted_index_of {
    () => {
        $crate::lib::json!(-1)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(-1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index_of($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_index_of($a, $b)
    };
}

/// `_x` helper for [sorted_index_of!](crate::sorted_index_of!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [sorted_index_of_x!](crate::sorted_index_of_x!) | `Value` forms: [sorted_index_of!](crate::sorted_index_of!), [sorted_index_of()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_of_x;
/// # use serde_json::json;
/// assert_eq!(sorted_index_of_x(json!([4, 5, 5, 5, 6]), json!(5)), 1);
/// ```
pub fn sorted_index_of_x(array: Value, value: Value) -> isize {
    let i = sorted_index_impl(&array, &value, false, |v| v.clone());
    if let Value::Array(vec) = &array
        && i < vec.len()
        && vec[i] == value
    {
        return i as isize;
    }
    -1
}

/// `_x` helper for [sorted_index_of!](crate::sorted_index_of!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [sorted_index_of_x()] | `Value` forms: [sorted_index_of!](crate::sorted_index_of!), [sorted_index_of()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_of_x!(json!([4, 5, 5, 5, 6]), json!(5)), 1);
/// ```
#[macro_export]
macro_rules! sorted_index_of_x {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index_of_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_index_of_x($a, $b)
    };
}
