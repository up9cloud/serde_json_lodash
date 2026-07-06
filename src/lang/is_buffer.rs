use crate::lib::{json, Value};

/// `_x` helper for [is_buffer()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_buffer_x;
/// # use serde_json::json;
/// assert_eq!(is_buffer_x(&json!({})), false);
/// ```
pub fn is_buffer_x(_v: &Value) -> bool {
    false
}
/// See lodash [isBuffer](https://lodash.com/docs/#isBuffer)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_buffer;
/// # use serde_json::json;
/// assert_eq!(is_buffer(&json!({})), json!(false));
/// ```
pub fn is_buffer(_v: &Value) -> Value {
    json!(is_buffer_x(_v))
}

/// Based on [is_buffer_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_buffer_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_buffer_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_buffer_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_buffer_x($a)
    };
}
/// Based on [is_buffer()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_buffer!(&json!({})), json!(false));
/// assert_eq!(is_buffer!(&json!("a")), json!(false));
/// assert_eq!(is_buffer!(), json!(false));
/// assert_eq!(is_buffer!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_buffer {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_buffer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_buffer($a)
    };
}
