use crate::lib::{json, Value};

/// See lodash [take](https://lodash.com/docs/#take)
pub fn take(array: Value, n: usize) -> Value {
    match array {
        Value::Array(mut vec) => {
            vec.truncate(n);
            Value::Array(vec)
        }
        _ => json!([]),
    }
}

/// Based on [take()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(take!(json!([1, 2, 3])), json!([1]));
/// assert_eq!(take!(json!([1, 2, 3]), 2), json!([1, 2]));
/// assert_eq!(take!(json!([1, 2, 3]), 5), json!([1, 2, 3]));
/// assert_eq!(take!(json!([1, 2, 3]), 0), json!([]));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(take!(), json!([]));
/// assert_eq!(take!(json!(null)), json!([]));
/// ```
#[macro_export]
macro_rules! take {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::take($a, 1)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::take($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::take($a, $b)
    };
}
