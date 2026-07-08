use crate::lib::{Value, json};

/// Fn form of [is_arguments!](crate::is_arguments!); see it for the full docs
///
/// `_x` forms: [is_arguments_x!](crate::is_arguments_x!), [is_arguments_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_arguments;
/// # use serde_json::json;
/// assert_eq!(is_arguments(&json!({})), json!(false));
/// ```
pub fn is_arguments(_v: &Value) -> Value {
    json!(is_arguments_x(_v))
}

/// See lodash [isArguments](https://lodash.com/docs/#isArguments)
///
/// Fn form: [is_arguments()] | `_x` forms: [is_arguments_x!](crate::is_arguments_x!), [is_arguments_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_arguments!(json!({})), json!(false));
/// assert_eq!(is_arguments!(json!("a")), json!(false));
/// assert_eq!(is_arguments!(), json!(false));
/// assert_eq!(is_arguments!(json!(null)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_arguments!(), json!(false));
/// assert_eq!(is_arguments!(json!(null)), json!(false));
/// assert_eq!(is_arguments!(json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_arguments {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_arguments(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_arguments(&$a)
    };
}

/// `_x` helper for [is_arguments!](crate::is_arguments!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_arguments_x!](crate::is_arguments_x!) | `Value` forms: [is_arguments!](crate::is_arguments!), [is_arguments()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_arguments_x;
/// # use serde_json::json;
/// assert_eq!(is_arguments_x(&json!({})), false);
/// ```
pub fn is_arguments_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_arguments!](crate::is_arguments!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_arguments_x()] | `Value` forms: [is_arguments!](crate::is_arguments!), [is_arguments()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_arguments_x!(json!({})), false);
/// ```
#[macro_export]
macro_rules! is_arguments_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_arguments_x(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_arguments_x(&$a)
    };
}
