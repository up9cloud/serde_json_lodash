use crate::lib::{json, Value, Map};

/// See lodash [toPlainObject](https://lodash.com/docs/#toPlainObject)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_plain_object;
/// # use serde_json::json;
/// assert_eq!(to_plain_object(json!({"b": 2, "c": 3})), json!({"b": 2, "c": 3}));
/// ```
pub fn to_plain_object(v: Value) -> Value {
    match v {
        Value::Object(_) => v,
        Value::Array(vec) => {
            let mut o = Map::new();
            for (i, item) in vec.into_iter().enumerate() {
                o.insert(i.to_string(), item);
            }
            Value::Object(o)
        }
        Value::String(s) => {
            let mut o = Map::new();
            for (i, c) in s.chars().enumerate() {
                o.insert(i.to_string(), json!(c.to_string()));
            }
            Value::Object(o)
        }
        _ => json!({}),
    }
}

/// Based on [to_plain_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_plain_object!(json!({"b": 2, "c": 3})),
///   json!({"b": 2, "c": 3})
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_plain_object!(), json!({}));
/// assert_eq!(to_plain_object!(json!(null)), json!({}));
/// assert_eq!(to_plain_object!(json!([1, 2])), json!({"0": 1, "1": 2}));
/// assert_eq!(to_plain_object!(json!("ab")), json!({"0": "a", "1": "b"}));
/// ```
#[macro_export]
macro_rules! to_plain_object {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_plain_object($a)
    };
}

/// `_x` helper for [to_plain_object()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [to_plain_object()] and read the returned `Value`.
pub fn to_plain_object_x() {
    todo!()
}
