use crate::lib::{Value, json};

/// Fn form of [is_native!](crate::is_native!); see it for the full docs
///
/// `_x` forms: [is_native_x!](crate::is_native_x!), [is_native_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_native;
/// # use serde_json::json;
/// assert_eq!(is_native(&json!({})), json!(false));
/// ```
pub fn is_native(_v: &Value) -> Value {
    json!(is_native_x(_v))
}

/// See lodash [isNative](https://lodash.com/docs/#isNative)
///
/// Fn form: [is_native()] | `_x` forms: [is_native_x!](crate::is_native_x!), [is_native_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_native!(json!({})), json!(false));
/// assert_eq!(is_native!(json!("a")), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_native!(), json!(false));
/// assert_eq!(is_native!(json!(null)), json!(false));
/// assert_eq!(is_native!(json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_native {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_native(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_native(&$a)
    };
}

/// `_x` helper for [is_native!](crate::is_native!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_native_x!](crate::is_native_x!) | `Value` forms: [is_native!](crate::is_native!), [is_native()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_native_x;
/// # use serde_json::json;
/// assert_eq!(is_native_x(&json!({})), false);
/// ```
pub fn is_native_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_native!](crate::is_native!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_native_x()] | `Value` forms: [is_native!](crate::is_native!), [is_native()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_native_x!(json!({})), false);
/// ```
#[macro_export]
macro_rules! is_native_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_native_x(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_native_x(&$a)
    };
}
