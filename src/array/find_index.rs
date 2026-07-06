use crate::lib::{json, Value};

/// `_x` helper for [find_index()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_index_x;
/// # use serde_json::json;
/// assert_eq!(find_index_x(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 0), 1);
/// ```
pub fn find_index_x(array: Value, predicate: fn(&Value) -> bool, from_index: usize) -> isize {
    match array {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => {
            return -1;
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                return -1;
            }
            if from_index >= vec.len() {
                return -1;
            }
            for (i, item) in vec.iter().enumerate().skip(from_index) {
                if predicate(item) {
                    return i as isize;
                }
            }
        }
    };
    -1
}
/// See lodash [findIndex](https://lodash.com/docs/#findIndex)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_index;
/// # use serde_json::json;
/// assert_eq!(find_index(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 0), json!(1));
/// ```
pub fn find_index(array: Value, predicate: fn(&Value) -> bool, from_index: usize) -> Value {
    json!(find_index_x(array, predicate, from_index))
}

/// Based on [find_index_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_index_x!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1, 0), 1);
/// ```
#[macro_export]
macro_rules! find_index_x {
    () => {
        -1
    };
    ($a:expr $(,)*) => {
        -1
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
/// Based on [find_index()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(find_index!(), json!(-1));
/// ```
#[macro_export]
macro_rules! find_index {
    () => {
        $crate::lib::json!(-1)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(-1)
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
