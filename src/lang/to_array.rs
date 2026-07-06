use crate::lib::{json, Value};

/// See lodash [toArray](https://lodash.com/docs/#toArray)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_array;
/// # use serde_json::json;
/// assert_eq!(to_array(json!({"a": 1, "b": 2})), json!([1, 2]));
/// ```
pub fn to_array(v: Value) -> Value {
    match v {
        Value::Array(_) => v,
        Value::Object(o) => Value::Array(o.into_iter().map(|(_, v)| v).collect()),
        Value::String(s) => Value::Array(s.chars().map(|c| json!(c.to_string())).collect()),
        _ => json!([]),
    }
}

/// Based on [to_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(to_array!(json!({"a": 1, "b": 2})), json!([1, 2]));
/// assert_eq!(to_array!(json!("abc")), json!(["a", "b", "c"]));
/// assert_eq!(to_array!(json!(1)), json!([]));
/// assert_eq!(to_array!(json!(null)), json!([]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_array!(), json!([]));
/// assert_eq!(to_array!(json!([1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! to_array {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_array($a)
    };
}
