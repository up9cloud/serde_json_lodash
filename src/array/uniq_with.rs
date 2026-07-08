use crate::lib::{Value, json};

/// Fn form of [uniq_with!](crate::uniq_with!); see it for the full docs
///
/// `_x` form: **not provided** — see [uniq_with_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::uniq_with;
/// # use serde_json::json;
/// assert_eq!(uniq_with(json!([1, 2, 3, 2]), |a, b| a == b), json!([1, 2, 3]));
/// ```
pub fn uniq_with(array: Value, comparator: impl Fn(&Value, &Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => {
            let mut out: Vec<Value> = vec![];
            for v in vec {
                if !out.iter().any(|kept| comparator(kept, &v)) {
                    out.push(v);
                }
            }
            Value::Array(out)
        }
        _ => json!([]),
    }
}

/// See lodash [uniqWith](https://lodash.com/docs/#uniqWith)
///
/// `comparator` is invoked to compare elements for uniqueness
///
/// Fn form: [uniq_with()] | `_x` form: **not provided** — see [uniq_with_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   uniq_with!(json!([1, 2, 3, 2]), |a, b| a == b),
///   json!([1, 2, 3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(uniq_with!(), json!([]));
/// assert_eq!(uniq_with!(json!([1, 1])), json!([1, 1]));
/// ```
#[macro_export]
macro_rules! uniq_with {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::uniq_with($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::uniq_with($a, $b)
    };
}

build_not_provided_x!(uniq_with, uniq_with_x);
