use crate::lib::{json, Value};
use crate::set;

/// See lodash [zipObjectDeep](https://lodash.com/docs/#zipObjectDeep)
///
/// Like `zip_object` but `keys` may be property paths
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::zip_object_deep;
/// # use serde_json::json;
/// assert_eq!(zip_object_deep(json!(["a.b[0].c", "a.b[1].d"]), json!([1, 2])), json!({ "a": { "b": [{ "c": 1 }, { "d": 2 }] } }));
/// ```
pub fn zip_object_deep(keys: Value, values: Value) -> Value {
    let ks = match keys {
        Value::Array(v) => v,
        _ => return json!({}),
    };
    let vs = match values {
        Value::Array(v) => v,
        _ => vec![],
    };
    let mut out = json!({});
    for (i, k) in ks.into_iter().enumerate() {
        out = set(out, k, vs.get(i).cloned().unwrap_or(Value::Null));
    }
    out
}

/// Based on [zip_object_deep()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   zip_object_deep!(json!(["a.b[0].c", "a.b[1].d"]), json!([1, 2])),
///   json!({ "a": { "b": [{ "c": 1 }, { "d": 2 }] } })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(zip_object_deep!(), json!({}));
/// assert_eq!(zip_object_deep!(json!(["a.b"]), json!([1])), json!({ "a": { "b": 1 } }));
/// ```
#[macro_export]
macro_rules! zip_object_deep {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::zip_object_deep($a, $crate::lib::json!([]))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::zip_object_deep($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::zip_object_deep($a, $b)
    };
}
