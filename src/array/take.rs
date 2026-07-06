use crate::lib::{json, Value};

/// See lodash [take](https://lodash.com/docs/#take)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::take;
/// # use serde_json::json;
/// assert_eq!(take(json!([1, 2, 3]), 2), json!([1, 2]));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!([])
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

/// `_x` helper for [take()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [take()] and read the returned `Value`.
pub fn take_x() {
    todo!()
}
/// Based on [take_x()]
#[macro_export]
macro_rules! take_x {
    ($($t:tt)*) => {
        $crate::take_x()
    };
}
