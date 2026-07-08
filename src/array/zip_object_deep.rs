use crate::lib::{Value, json};

use crate::set;

/// Fn form of [zip_object_deep!](crate::zip_object_deep!); see it for the full docs
///
/// `_x` form: **not provided** — see [zip_object_deep_x()]
///
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

/// See lodash [zipObjectDeep](https://lodash.com/docs/#zipObjectDeep)
///
/// Like `zip_object` but `keys` may be property paths
///
/// Fn form: [zip_object_deep()] | `_x` form: **not provided** — see [zip_object_deep_x()]
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [zip_object_deep!](crate::zip_object_deep!) and read
/// the returned `Value`.
///
/// Macro form: [zip_object_deep_x!](crate::zip_object_deep_x!)
pub fn zip_object_deep_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [zip_object_deep!](crate::zip_object_deep!) and read
/// the returned `Value`.
///
/// Fn form: [zip_object_deep_x()]
#[macro_export]
macro_rules! zip_object_deep_x {
    ($($t:tt)*) => {
        $crate::zip_object_deep_x()
    };
}
