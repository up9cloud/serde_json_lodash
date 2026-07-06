use crate::lib::{json, Value};
use crate::to_string_x;

/// See lodash [omit](https://lodash.com/docs/#omit)
///
/// `paths` is an array of (top level) property names to drop
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

/// Based on [omit()]
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

/// `_x` helper for [omit()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [omit()] and read the returned `Value`.
pub fn omit_x() {
    todo!()
}
