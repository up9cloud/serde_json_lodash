use crate::lib::{Value, json};

/// Fn form of [slice!](crate::slice!); see it for the full docs
///
/// `_x` form: **not provided** — see [slice_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::slice;
/// # use serde_json::json;
/// assert_eq!(slice(json!([1, 2, 3, 4]), 1, 3), json!([2, 3]));
/// ```
pub fn slice(array: Value, start: isize, end: isize) -> Value {
    match array {
        Value::Array(vec) => {
            let len = vec.len() as isize;
            let s = if start < 0 {
                (len + start).max(0)
            } else {
                start.min(len)
            };
            let e = if end < 0 {
                (len + end).max(0)
            } else {
                end.min(len)
            };
            if s >= e {
                json!([])
            } else {
                Value::Array(vec[s as usize..e as usize].to_vec())
            }
        }
        _ => json!([]),
    }
}

/// See lodash [slice](https://lodash.com/docs/#slice)
///
/// `start` and `end` may be negative to count from the end
///
/// Fn form: [slice()] | `_x` form: **not provided** — see [slice_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(slice!(json!([1, 2, 3, 4]), 1, 3), json!([2, 3]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(slice!(), json!([]));
/// assert_eq!(slice!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(slice!(json!([1, 2, 3, 4]), 2), json!([3, 4]));
/// assert_eq!(slice!(json!([1, 2, 3, 4]), -2), json!([3, 4]));
/// assert_eq!(slice!(json!([1, 2, 3, 4]), 1, -1), json!([2, 3]));
/// ```
#[macro_export]
macro_rules! slice {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::slice($a, $b, isize::MAX)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::slice($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::slice($a, $b, $c)
    };
}

build_not_provided_x!(slice, slice_x);
