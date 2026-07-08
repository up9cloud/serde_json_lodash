use crate::lib::{Value, json};

/// Fn form of [is_array!](crate::is_array!); see it for the full docs
///
/// `_x` forms: [is_array_x!](crate::is_array_x!), [is_array_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array;
/// # use serde_json::json;
/// assert_eq!(is_array(&json!([1, 2, 3])), json!(true));
/// ```
pub fn is_array(v: &Value) -> Value {
    json!(is_array_x(v))
}

/// See lodash [isArray](https://lodash.com/docs/#isArray)
///
/// Fn form: [is_array()] | `_x` forms: [is_array_x!](crate::is_array_x!), [is_array_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array!(&json!([1, 2, 3])), json!(true));
/// assert_eq!(is_array!(&json!("abc")), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array!(), json!(false));
/// assert_eq!(is_array!(&json!(null)), json!(false));
/// assert_eq!(is_array!(&json!([])), json!(true));
/// ```
#[macro_export]
macro_rules! is_array {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array($a)
    };
}

/// `_x` helper for [is_array!](crate::is_array!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_array_x!](crate::is_array_x!) | `Value` forms: [is_array!](crate::is_array!), [is_array()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_x;
/// # use serde_json::json;
/// assert_eq!(is_array_x(&json!([1, 2, 3])), true);
/// ```
pub fn is_array_x(v: &Value) -> bool {
    v.is_array()
}

/// `_x` helper for [is_array!](crate::is_array!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_array_x()] | `Value` forms: [is_array!](crate::is_array!), [is_array()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_x!(&json!([1, 2, 3])), true);
/// ```
#[macro_export]
macro_rules! is_array_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_x($a)
    };
}
