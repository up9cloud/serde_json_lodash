use crate::lib::{json, Value};

/// See lodash [takeRight](https://lodash.com/docs/#takeRight)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::take_right;
/// # use serde_json::json;
/// assert_eq!(take_right(json!([1, 2, 3]), 2), json!([2, 3]));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!([])
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

/// `_x` helper for [take_right()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [take_right()] and read the returned `Value`.
pub fn take_right_x() {
    todo!()
}
/// Based on [take_right_x()]
#[macro_export]
macro_rules! take_right_x {
    ($($t:tt)*) => {
        $crate::take_right_x()
    };
}
