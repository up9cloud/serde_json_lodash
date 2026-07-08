use crate::lib::{Value, json};

/// Fn form of [is_error!](crate::is_error!); see it for the full docs
///
/// `_x` forms: [is_error_x!](crate::is_error_x!), [is_error_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_error;
/// # use serde_json::json;
/// assert_eq!(is_error(&json!({})), json!(false));
/// ```
pub fn is_error(_v: &Value) -> Value {
    json!(is_error_x(_v))
}

/// See lodash [isError](https://lodash.com/docs/#isError)
///
/// Fn form: [is_error()] | `_x` forms: [is_error_x!](crate::is_error_x!), [is_error_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_error!(json!({})), json!(false));
/// assert_eq!(is_error!(json!("a")), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_error!(), json!(false));
/// assert_eq!(is_error!(json!(null)), json!(false));
/// assert_eq!(is_error!(json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_error {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_error(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_error(&$a)
    };
}

/// `_x` helper for [is_error!](crate::is_error!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_error_x!](crate::is_error_x!) | `Value` forms: [is_error!](crate::is_error!), [is_error()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_error_x;
/// # use serde_json::json;
/// assert_eq!(is_error_x(&json!({})), false);
/// ```
pub fn is_error_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_error!](crate::is_error!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_error_x()] | `Value` forms: [is_error!](crate::is_error!), [is_error()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_error_x!(json!({})), false);
/// ```
#[macro_export]
macro_rules! is_error_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_error_x(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_error_x(&$a)
    };
}
