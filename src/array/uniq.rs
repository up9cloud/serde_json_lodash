use crate::lib::{Value, json};

use crate::internal::uniq_by_key;

/// Fn form of [uniq!](crate::uniq!); see it for the full docs
///
/// `_x` form: **not provided** — see [uniq_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::uniq;
/// # use serde_json::json;
/// assert_eq!(uniq(json!([2, 1, 2])), json!([2, 1]));
/// ```
pub fn uniq(array: Value) -> Value {
    match array {
        Value::Array(vec) => Value::Array(uniq_by_key(vec, |v| v.clone())),
        _ => json!([]),
    }
}

/// See lodash [uniq](https://lodash.com/docs/#uniq)
///
/// Fn form: [uniq()] | `_x` form: **not provided** — see [uniq_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(uniq!(json!([2, 1, 2])), json!([2, 1]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(uniq!(), json!([]));
/// assert_eq!(uniq!(json!(null)), json!([]));
/// assert_eq!(uniq!(json!([1, 1, 2, 3, 3])), json!([1, 2, 3]));
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(uniq!(json!([1, 1.0, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! uniq {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::uniq($a)
    };
}

build_not_provided_x!(uniq, uniq_x);
