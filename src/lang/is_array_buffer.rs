use crate::lib::{Value, json};

/// Fn form of [is_array_buffer!](crate::is_array_buffer!); see it for the full docs
///
/// `_x` forms: [is_array_buffer_x!](crate::is_array_buffer_x!), [is_array_buffer_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_buffer;
/// # use serde_json::json;
/// assert_eq!(is_array_buffer(&json!({})), json!(false));
/// ```
pub fn is_array_buffer(_v: &Value) -> Value {
    json!(is_array_buffer_x(_v))
}

/// See lodash [isArrayBuffer](https://lodash.com/docs/#isArrayBuffer)
///
/// Fn form: [is_array_buffer()] | `_x` forms: [is_array_buffer_x!](crate::is_array_buffer_x!), [is_array_buffer_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array_buffer!(&json!({})), json!(false));
/// assert_eq!(is_array_buffer!(&json!("a")), json!(false));
/// assert_eq!(is_array_buffer!(), json!(false));
/// assert_eq!(is_array_buffer!(&json!(null)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_buffer!(), json!(false));
/// assert_eq!(is_array_buffer!(&json!(null)), json!(false));
/// assert_eq!(is_array_buffer!(&json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_array_buffer {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_array_buffer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_buffer($a)
    };
}

/// `_x` helper for [is_array_buffer!](crate::is_array_buffer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_array_buffer_x!](crate::is_array_buffer_x!) | `Value` forms: [is_array_buffer!](crate::is_array_buffer!), [is_array_buffer()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_buffer_x;
/// # use serde_json::json;
/// assert_eq!(is_array_buffer_x(&json!({})), false);
/// ```
pub fn is_array_buffer_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_array_buffer!](crate::is_array_buffer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_array_buffer_x()] | `Value` forms: [is_array_buffer!](crate::is_array_buffer!), [is_array_buffer()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_buffer_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_array_buffer_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_buffer_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_buffer_x($a)
    };
}
