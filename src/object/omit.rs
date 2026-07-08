use crate::lib::{Value, json};

use crate::to_string_x;

/// Fn form of [omit!](crate::omit!); see it for the full docs
///
/// `_x` form: **not provided** — see [omit_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::omit;
/// # use serde_json::json;
/// assert_eq!(omit(json!({"a": 1}), json!(["x"])), json!({"a": 1}));
/// ```
pub fn omit(object: Value, paths: Value) -> Value {
    match object {
        Value::Object(mut o) => {
            if let Value::Array(keys) = paths {
                for k in keys {
                    o.remove(&to_string_x(k));
                }
            }
            Value::Object(o)
        }
        _ => json!({}),
    }
}

/// See lodash [omit](https://lodash.com/docs/#omit)
///
/// `paths` is an array of (top level) property names to drop
///
/// Fn form: [omit()] | `_x` form: **not provided** — see [omit_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2, "c": 3 });
/// assert_eq!(
///   omit!(object, json!(["a", "c"])),
///   json!({ "b": 2 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(omit!(), json!({}));
/// assert_eq!(omit!(json!({"a": 1, "b": 2})), json!({"a": 1, "b": 2}));
/// assert_eq!(omit!(json!({"a": 1}), json!(["x"])), json!({"a": 1}));
/// ```
#[macro_export]
macro_rules! omit {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::omit($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::omit($a, $b)
    };
}

build_not_provided_x!(omit, omit_x);
