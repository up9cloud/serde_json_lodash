use crate::lib::{Value, json};

/// Fn form of [find_last_index!](crate::find_last_index!); see it for the full docs
///
/// `_x` forms: [find_last_index_x!](crate::find_last_index_x!), [find_last_index_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_last_index;
/// # use serde_json::json;
/// assert_eq!(find_last_index(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 2), json!(2));
/// ```
pub fn find_last_index(array: Value, predicate: fn(&Value) -> bool, from_index: usize) -> Value {
    json!(find_last_index_x(array, predicate, from_index))
}

/// See lodash [findLastIndex](https://lodash.com/docs/#findLastIndex)
///
/// Fn form: [find_last_index()] | `_x` forms: [find_last_index_x!](crate::find_last_index_x!), [find_last_index_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(find_last_index!(), json!(-1));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_last_index!(), json!(-1));
/// assert_eq!(find_last_index!(json!(null)), json!(-1));
/// assert_eq!(find_last_index!(json!({"a": 1})), json!(-1));
/// ```
#[macro_export]
macro_rules! find_last_index {
    () => {
        $crate::lib::json!(-1)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(-1)
    };
    ($a:expr, $b:expr $(,)*) => {{
        let from_index = $a.as_array().unwrap_or(&vec![]).len() - 1;
        $crate::find_last_index($a, $b, from_index)
    }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::find_last_index($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::find_last_index($a, $b, $c)
    };
}

/// `_x` helper for [find_last_index!](crate::find_last_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [find_last_index_x!](crate::find_last_index_x!) | `Value` forms: [find_last_index!](crate::find_last_index!), [find_last_index()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_last_index_x;
/// # use serde_json::json;
/// assert_eq!(find_last_index_x(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 2), 2);
/// ```
pub fn find_last_index_x(array: Value, predicate: fn(&Value) -> bool, from_index: usize) -> isize {
    match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return -1;
        }
        Value::Array(ref vec) => {
            if vec.is_empty() {
                return -1;
            }
            let mut real_from_index = from_index;
            if from_index >= vec.len() {
                real_from_index = vec.len() - 1;
            }
            for i in (0..=real_from_index).rev() {
                if predicate(&vec[i]) {
                    return i as isize;
                }
            }
        }
    };
    -1
}

/// `_x` helper for [find_last_index!](crate::find_last_index!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [find_last_index_x()] | `Value` forms: [find_last_index!](crate::find_last_index!), [find_last_index()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_last_index_x!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 2), 2);
/// ```
#[macro_export]
macro_rules! find_last_index_x {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
    };
    ($a:expr, $b:expr $(,)*) => {{
        let from_index = $a.as_array().unwrap_or(&vec![]).len() - 1;
        $crate::find_last_index_x($a, $b, from_index)
    }};
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::find_last_index_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::find_last_index_x($a, $b, $c)
    };
}
