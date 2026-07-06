use crate::lib::{json, Value};

/// `_x` helper for [is_plain_object()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_plain_object_x;
/// # use serde_json::json;
/// assert_eq!(is_plain_object_x(&json!([1, 2, 3])), false);
/// ```
pub fn is_plain_object_x(v: &Value) -> bool {
    v.is_object()
}
/// See lodash [isPlainObject](https://lodash.com/docs/#isPlainObject)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_plain_object;
/// # use serde_json::json;
/// assert_eq!(is_plain_object(&json!([1, 2, 3])), json!(false));
/// ```
pub fn is_plain_object(v: &Value) -> Value {
    json!(is_plain_object_x(v))
}

/// Based on [is_plain_object_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_plain_object_x!(&json!([1, 2, 3])), false);
/// ```
#[macro_export]
macro_rules! is_plain_object_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_plain_object_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_plain_object_x($a)
    };
}
/// Based on [is_plain_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_plain_object!(&json!([1, 2, 3])), json!(false));
/// assert_eq!(is_plain_object!(&json!({ "x": 0, "y": 0 })), json!(true));
/// assert_eq!(is_plain_object!(), json!(false));
/// assert_eq!(is_plain_object!(&json!(null)), json!(false));
/// assert_eq!(is_plain_object!(&json!({})), json!(true));
/// ```
#[macro_export]
macro_rules! is_plain_object {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_plain_object($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_plain_object($a)
    };
}
