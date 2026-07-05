use crate::lib::Value;

/// See lodash [castArray](https://lodash.com/docs/#castArray)
pub fn cast_array(v: Value) -> Value {
    match v {
        Value::Array(_) => v,
        _ => Value::Array(vec![v]),
    }
}

/// Based on [cast_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(cast_array!(json!(1)), json!([1]));
/// assert_eq!(cast_array!(json!({"a": 1})), json!([{"a": 1}]));
/// assert_eq!(cast_array!(json!("abc")), json!(["abc"]));
/// assert_eq!(cast_array!(json!(null)), json!([null]));
/// assert_eq!(cast_array!(), json!([]));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(cast_array!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! cast_array {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::cast_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::cast_array($a)
    };
}
