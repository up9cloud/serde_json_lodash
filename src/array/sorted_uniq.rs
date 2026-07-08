use crate::lib::{Value, json};

use crate::internal::uniq_by_key;

/// Fn form of [sorted_uniq!](crate::sorted_uniq!); see it for the full docs
///
/// `_x` form: **not provided** — see [sorted_uniq_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_uniq;
/// # use serde_json::json;
/// assert_eq!(sorted_uniq(json!([1, 1, 2])), json!([1, 2]));
/// ```
pub fn sorted_uniq(array: Value) -> Value {
    match array {
        Value::Array(vec) => Value::Array(uniq_by_key(vec, |v| v.clone())),
        _ => json!([]),
    }
}

/// See lodash [sortedUniq](https://lodash.com/docs/#sortedUniq)
///
/// Fn form: [sorted_uniq()] | `_x` form: **not provided** — see [sorted_uniq_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_uniq!(json!([1, 1, 2])), json!([1, 2]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_uniq!(), json!([]));
/// assert_eq!(sorted_uniq!(json!([1, 2, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! sorted_uniq {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sorted_uniq($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::sorted_uniq($a)
    };
}

build_not_provided_x!(sorted_uniq, sorted_uniq_x);
