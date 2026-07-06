use crate::lib::{json, Value};

/// `_x` helper for [is_object()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_object_x;
/// # use serde_json::json;
/// assert_eq!(is_object_x(&json!({})), true);
/// ```
pub fn is_object_x(v: &Value) -> bool {
    v.is_object() || v.is_array()
}
/// See lodash [isObject](https://lodash.com/docs/#isObject)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_object;
/// # use serde_json::json;
/// assert_eq!(is_object(&json!({})), json!(true));
/// ```
pub fn is_object(v: &Value) -> Value {
    json!(is_object_x(v))
}

/// Based on [is_object_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_object_x!(&json!({})), true);
/// ```
#[macro_export]
macro_rules! is_object_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_object_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object_x($a)
    };
}
/// Based on [is_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_object!(&json!({})), json!(true));
/// assert_eq!(is_object!(&json!([1, 2, 3])), json!(true));
/// assert_eq!(is_object!(&json!(null)), json!(false));
/// assert_eq!(is_object!(), json!(false));
/// assert_eq!(is_object!(&json!("abc")), json!(false));
/// ```
#[macro_export]
macro_rules! is_object {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_object($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object($a)
    };
}
