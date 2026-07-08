use crate::lib::{Value, json};

use crate::internal::compare_values;

use std::cmp::Ordering;

pub(crate) fn sorted_index_impl(
    array: &Value,
    value: &Value,
    high: bool,
    key: impl Fn(&Value) -> Value,
) -> usize {
    let vec = match array {
        Value::Array(v) => v,
        _ => return 0,
    };
    let target = key(value);
    let (mut lo, mut hi) = (0usize, vec.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        let ord = compare_values(&key(&vec[mid]), &target).unwrap_or(Ordering::Less);
        let go_right = if high {
            ord != Ordering::Greater
        } else {
            ord == Ordering::Less
        };
        if go_right {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Fn form of [sorted_index!](crate::sorted_index!); see it for the full docs
///
/// `_x` forms: [sorted_index_x!](crate::sorted_index_x!), [sorted_index_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index;
/// # use serde_json::json;
/// assert_eq!(sorted_index(json!([30, 50]), json!(40)), json!(1));
/// ```
pub fn sorted_index(array: Value, value: Value) -> Value {
    json!(sorted_index_x(array, value))
}

/// See lodash [sortedIndex](https://lodash.com/docs/#sortedIndex)
///
/// Fn form: [sorted_index()] | `_x` forms: [sorted_index_x!](crate::sorted_index_x!), [sorted_index_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_index!(json!([30, 50]), json!(40)), json!(1));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index!(), json!(0));
/// assert_eq!(sorted_index!(json!(null)), json!(0));
/// assert_eq!(sorted_index!(json!({"a": 1})), json!(0));
/// assert_eq!(sorted_index!(json!(null), json!(null)), json!(0));
/// assert_eq!(sorted_index!(json!(1), json!(1)), json!(0));
/// assert_eq!(sorted_index!(json!(1), json!(2)), json!(0));
/// assert_eq!(sorted_index!(json!([1, 2, 3]), json!(2)), json!(1));
/// assert_eq!(sorted_index!(json!("abc"), json!("bc")), json!(0));
/// assert_eq!(sorted_index!(json!([20, 30, 50]), json!(10)), json!(0));
/// assert_eq!(sorted_index!(json!([20, 30, 30, 50]), json!(30)), json!(1));
/// ```
#[macro_export]
macro_rules! sorted_index {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_index($a, $b)
    };
}

/// `_x` helper for [sorted_index!](crate::sorted_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [sorted_index_x!](crate::sorted_index_x!) | `Value` forms: [sorted_index!](crate::sorted_index!), [sorted_index()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_x;
/// # use serde_json::json;
/// assert_eq!(sorted_index_x(json!([30, 50]), json!(40)), 1);
/// ```
pub fn sorted_index_x(array: Value, value: Value) -> usize {
    sorted_index_impl(&array, &value, false, |v| v.clone())
}

/// `_x` helper for [sorted_index!](crate::sorted_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [sorted_index_x()] | `Value` forms: [sorted_index!](crate::sorted_index!), [sorted_index()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_x!(json!([30, 50]), json!(40)), 1);
/// ```
#[macro_export]
macro_rules! sorted_index_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_index_x($a, $b)
    };
}
