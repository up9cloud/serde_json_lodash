use crate::lib::{Map, Value, json};

use crate::to_string_x;

/// Fn form of [zip_object!](crate::zip_object!); see it for the full docs
///
/// `_x` form: **not provided** — see [zip_object_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::zip_object;
/// # use serde_json::json;
/// assert_eq!(zip_object(json!(["a", "b"]), json!([1, 2])), json!({ "a": 1, "b": 2 }));
/// ```
pub fn zip_object(keys: Value, values: Value) -> Value {
    let ks = match keys {
        Value::Array(v) => v,
        _ => return json!({}),
    };
    let vs = match values {
        Value::Array(v) => v,
        _ => vec![],
    };
    let mut out = Map::new();
    for (i, k) in ks.into_iter().enumerate() {
        out.insert(to_string_x(k), vs.get(i).cloned().unwrap_or(Value::Null));
    }
    Value::Object(out)
}

/// See lodash [zipObject](https://lodash.com/docs/#zipObject)
///
/// Builds an object from `keys` and `values` arrays
///
/// Fn form: [zip_object()] | `_x` form: **not provided** — see [zip_object_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   zip_object!(json!(["a", "b"]), json!([1, 2])),
///   json!({ "a": 1, "b": 2 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(zip_object!(), json!({}));
/// assert_eq!(zip_object!(json!(["a"])), json!({ "a": null }));
/// ```
#[macro_export]
macro_rules! zip_object {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::zip_object($a, $crate::lib::json!([]))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::zip_object($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::zip_object($a, $b)
    };
}

build_not_provided_x!(zip_object, zip_object_x);
