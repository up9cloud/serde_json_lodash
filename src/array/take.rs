use crate::lib::{Value, json};

/// Fn form of [take!](crate::take!); see it for the full docs
///
/// `_x` form: **not provided** — see [take_x()]
///
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

/// See lodash [take](https://lodash.com/docs/#take)
///
/// Fn form: [take()] | `_x` form: **not provided** — see [take_x()]
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

build_not_provided_x!(take, take_x);
