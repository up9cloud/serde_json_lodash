use crate::lib::{json, Value, Map};
use crate::to_string_x;

/// See lodash [zipObject](https://lodash.com/docs/#zipObject)
///
/// Builds an object from `keys` and `values` arrays
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

/// Based on [zip_object()]
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

/// `_x` helper for [zip_object()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [zip_object()] and read the returned `Value`.
pub fn zip_object_x() {
    todo!()
}
