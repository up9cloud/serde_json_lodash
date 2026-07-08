use crate::lib::{Value, json};

/// Fn form of [keys!](crate::keys!); see it for the full docs
///
/// `_x` form: **not provided** — see [keys_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::keys;
/// # use serde_json::json;
/// assert_eq!(keys(json!({"a": 1, "b": 2})), json!(["a", "b"]));
/// ```
pub fn keys(v: Value) -> Value {
    match v {
        Value::Object(o) => Value::Array(o.keys().map(|k| json!(k)).collect()),
        Value::Array(vec) => Value::Array((0..vec.len()).map(|i| json!(i.to_string())).collect()),
        Value::String(s) => Value::Array(
            (0..s.chars().count())
                .map(|i| json!(i.to_string()))
                .collect(),
        ),
        _ => json!([]),
    }
}

/// See lodash [keys](https://lodash.com/docs/#keys)
///
/// For arrays (and strings) the indexes are returned as string keys
///
/// Fn form: [keys()] | `_x` form: **not provided** — see [keys_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(keys!(json!({"a": 1, "b": 2})), json!(["a", "b"]));
/// assert_eq!(keys!(json!("hi")), json!(["0", "1"]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(keys!(), json!([]));
/// assert_eq!(keys!(json!(null)), json!([]));
/// assert_eq!(keys!(json!([1, 2, 3])), json!(["0", "1", "2"]));
/// ```
#[macro_export]
macro_rules! keys {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::keys($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::keys($a)
    };
}

build_not_provided_x!(keys, keys_x);
