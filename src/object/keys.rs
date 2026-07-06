use crate::lib::{json, Value};

/// See lodash [keys](https://lodash.com/docs/#keys)
///
/// For arrays (and strings) the indexes are returned as string keys
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

/// Based on [keys()]
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

/// `_x` helper for [keys()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [keys()] and read the returned `Value`.
pub fn keys_x() {
    todo!()
}
