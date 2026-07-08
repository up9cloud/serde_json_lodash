use crate::lib::{Map, Value};

use crate::to_string_x;

/// Fn form of [pick!](crate::pick!); see it for the full docs
///
/// `_x` form: **not provided** — see [pick_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::pick;
/// # use serde_json::json;
/// assert_eq!(pick(json!({"a": 1, "b": 2}), json!(["x"])), json!({}));
/// ```
pub fn pick(object: Value, paths: Value) -> Value {
    let mut out = Map::new();
    if let (Value::Object(mut o), Value::Array(keys)) = (object, paths) {
        for k in keys {
            let key = to_string_x(k);
            if let Some(v) = o.remove(&key) {
                out.insert(key, v);
            }
        }
    }
    Value::Object(out)
}

/// See lodash [pick](https://lodash.com/docs/#pick)
///
/// `paths` is an array of (top level) property names to keep
///
/// Fn form: [pick()] | `_x` form: **not provided** — see [pick_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2, "c": 3 });
/// assert_eq!(
///   pick!(object, json!(["a", "c"])),
///   json!({ "a": 1, "c": 3 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pick!(), json!({}));
/// assert_eq!(pick!(json!({"a": 1})), json!({}));
/// assert_eq!(pick!(json!({"a": 1, "b": 2}), json!(["x"])), json!({}));
/// ```
#[macro_export]
macro_rules! pick {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!({})
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pick($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pick($a, $b)
    };
}

build_not_provided_x!(pick, pick_x);
