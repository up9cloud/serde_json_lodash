use crate::lib::{json, Value};

/// `_x` helper for [is_function()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_function_x;
/// # use serde_json::json;
/// assert_eq!(is_function_x(&json!({})), false);
/// ```
pub fn is_function_x(_v: &Value) -> bool {
    false
}
/// See lodash [isFunction](https://lodash.com/docs/#isFunction)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_function;
/// # use serde_json::json;
/// assert_eq!(is_function(&json!({})), json!(false));
/// ```
pub fn is_function(_v: &Value) -> Value {
    json!(is_function_x(_v))
}

/// Based on [is_function_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_function_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_function_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_function_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_function_x($a)
    };
}
/// Based on [is_function()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_function!(&json!({})), json!(false));
/// assert_eq!(is_function!(&json!("a")), json!(false));
/// assert_eq!(is_function!(), json!(false));
/// assert_eq!(is_function!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_function {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_function($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_function($a)
    };
}
