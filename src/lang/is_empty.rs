use crate::lib::Value;

/// See lodash [isEmpty](https://lodash.com/docs/#isEmpty)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_empty;
/// # use serde_json::json;
/// assert_eq!(is_empty(&json!(null)), true);
/// ```
pub fn is_empty(v: &Value) -> bool {
    match v {
        Value::Null | Value::Bool(_) | Value::Number(_) => true,
        Value::String(s) => s.is_empty(),
        Value::Array(vec) => vec.is_empty(),
        Value::Object(o) => o.is_empty(),
    }
}

/// Based on [is_empty()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_empty!(&json!(null)), true);
/// assert_eq!(is_empty!(&json!(true)), true);
/// assert_eq!(is_empty!(&json!(1)), true);
/// assert_eq!(is_empty!(&json!([1, 2, 3])), false);
/// assert_eq!(is_empty!(&json!({"a": 1})), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_empty!(), true);
/// assert_eq!(is_empty!(&json!("")), true);
/// assert_eq!(is_empty!(&json!("abc")), false);
/// assert_eq!(is_empty!(&json!([])), true);
/// assert_eq!(is_empty!(&json!({})), true);
/// ```
#[macro_export]
macro_rules! is_empty {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_empty($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_empty($a)
    };
}
