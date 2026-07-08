use crate::lib::{Value, json};

/// Fn form of [reverse!](crate::reverse!); see it for the full docs
///
/// `_x` form: **not provided** — see [reverse_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::reverse;
/// # use serde_json::json;
/// assert_eq!(reverse(json!([1, 2, 3])), json!([3, 2, 1]));
/// ```
pub fn reverse(array: Value) -> Value {
    match array {
        Value::Array(mut vec) => {
            vec.reverse();
            Value::Array(vec)
        }
        _ => json!([]),
    }
}

/// See lodash [reverse](https://lodash.com/docs/#reverse)
///
/// Fn form: [reverse()] | `_x` form: **not provided** — see [reverse_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(reverse!(json!([1, 2, 3])), json!([3, 2, 1]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(reverse!(), json!([]));
/// assert_eq!(reverse!(json!(null)), json!([]));
/// assert_eq!(reverse!(json!([])), json!([]));
/// ```
#[macro_export]
macro_rules! reverse {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::reverse($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::reverse($a)
    };
}

build_not_provided_x!(reverse, reverse_x);
