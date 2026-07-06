use crate::lib::{json, Value};

/// `_x` helper for [is_typed_array()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_typed_array_x;
/// # use serde_json::json;
/// assert_eq!(is_typed_array_x(&json!({})), false);
/// ```
pub fn is_typed_array_x(_v: &Value) -> bool {
    false
}
/// See lodash [isTypedArray](https://lodash.com/docs/#isTypedArray)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_typed_array;
/// # use serde_json::json;
/// assert_eq!(is_typed_array(&json!({})), json!(false));
/// ```
pub fn is_typed_array(_v: &Value) -> Value {
    json!(is_typed_array_x(_v))
}

/// Based on [is_typed_array_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_typed_array_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_typed_array_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_typed_array_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_typed_array_x($a)
    };
}
/// Based on [is_typed_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_typed_array!(&json!({})), json!(false));
/// assert_eq!(is_typed_array!(&json!("a")), json!(false));
/// assert_eq!(is_typed_array!(), json!(false));
/// assert_eq!(is_typed_array!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_typed_array {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_typed_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_typed_array($a)
    };
}
