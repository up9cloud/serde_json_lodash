use crate::lib::{Value, json};

/// Fn form of [is_object!](crate::is_object!); see it for the full docs
///
/// `_x` forms: [is_object_x!](crate::is_object_x!), [is_object_x()]
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

/// See lodash [isObject](https://lodash.com/docs/#isObject)
///
/// Fn form: [is_object()] | `_x` forms: [is_object_x!](crate::is_object_x!), [is_object_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_object!(json!({})), json!(true));
/// assert_eq!(is_object!(json!([1, 2, 3])), json!(true));
/// assert_eq!(is_object!(json!(null)), json!(false));
/// assert_eq!(is_object!(), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_object!(), json!(false));
/// assert_eq!(is_object!(json!(null)), json!(false));
/// assert_eq!(is_object!(json!(true)), json!(false));
/// assert_eq!(is_object!(json!(0)), json!(false));
/// assert_eq!(is_object!(json!("ab")), json!(false));
/// assert_eq!(is_object!(json!([1, 2])), json!(true));
/// assert_eq!(is_object!(json!({"a": 1})), json!(true));
/// assert_eq!(is_object!(json!("abc")), json!(false));
/// ```
#[macro_export]
macro_rules! is_object {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_object(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object(&$a)
    };
}

/// `_x` helper for [is_object!](crate::is_object!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_object_x!](crate::is_object_x!) | `Value` forms: [is_object!](crate::is_object!), [is_object()]
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

/// `_x` helper for [is_object!](crate::is_object!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_object_x()] | `Value` forms: [is_object!](crate::is_object!), [is_object()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_object_x!(json!({})), true);
/// ```
#[macro_export]
macro_rules! is_object_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_object_x(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object_x(&$a)
    };
}
