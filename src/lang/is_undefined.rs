use crate::lib::{Value, json};

/// Fn form of [is_undefined!](crate::is_undefined!); see it for the full docs
///
/// `_x` forms: [is_undefined_x!](crate::is_undefined_x!), [is_undefined_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_undefined;
/// # use serde_json::json;
/// assert_eq!(is_undefined(&json!(null)), json!(true));
/// ```
pub fn is_undefined(v: &Value) -> Value {
    json!(is_undefined_x(v))
}

/// See lodash [isUndefined](https://lodash.com/docs/#isUndefined)
///
/// Fn form: [is_undefined()] | `_x` forms: [is_undefined_x!](crate::is_undefined_x!), [is_undefined_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_undefined!(), json!(true));
/// assert_eq!(is_undefined!(&json!(null)), json!(true));
/// assert_eq!(is_undefined!(&json!(0)), json!(false));
/// ```
#[macro_export]
macro_rules! is_undefined {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::is_undefined($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_undefined($a)
    };
}

/// `_x` helper for [is_undefined!](crate::is_undefined!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_undefined_x!](crate::is_undefined_x!) | `Value` forms: [is_undefined!](crate::is_undefined!), [is_undefined()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_undefined_x;
/// # use serde_json::json;
/// assert_eq!(is_undefined_x(&json!(null)), true);
/// ```
pub fn is_undefined_x(v: &Value) -> bool {
    v.is_null()
}

/// `_x` helper for [is_undefined!](crate::is_undefined!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_undefined_x()] | `Value` forms: [is_undefined!](crate::is_undefined!), [is_undefined()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_undefined_x!(&json!(null)), true);
/// ```
#[macro_export]
macro_rules! is_undefined_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_undefined_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_undefined_x($a)
    };
}
