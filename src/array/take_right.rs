use crate::lib::{json, Value};

/// See lodash [takeRight](https://lodash.com/docs/#takeRight)
pub fn take_right(array: Value, n: usize) -> Value {
    match array {
        Value::Array(vec) => {
            let start = vec.len().saturating_sub(n);
            Value::Array(vec[start..].to_vec())
        }
        _ => json!([]),
    }
}

/// Based on [take_right()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(take_right!(json!([1, 2, 3])), json!([3]));
/// assert_eq!(take_right!(json!([1, 2, 3]), 2), json!([2, 3]));
/// assert_eq!(take_right!(json!([1, 2, 3]), 5), json!([1, 2, 3]));
/// assert_eq!(take_right!(json!([1, 2, 3]), 0), json!([]));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(take_right!(), json!([]));
/// assert_eq!(take_right!(json!(null)), json!([]));
/// ```
#[macro_export]
macro_rules! take_right {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::take_right($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::take_right($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::take_right($a, $b)
    };
}
