use crate::lib::{Value, json};

/// Fn form of [eq!](crate::eq!); see it for the full docs
///
/// `_x` forms: [eq_x!](crate::eq_x!), [eq_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::eq;
/// # use serde_json::json;
/// assert_eq!(eq(&json!("a"), &json!("a")), json!(true));
/// ```
pub fn eq(a: &Value, b: &Value) -> Value {
    json!(eq_x(a, b))
}

/// See lodash [eq](https://lodash.com/docs/#eq)
///
/// Fn form: [eq()] | `_x` forms: [eq_x!](crate::eq_x!), [eq_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(eq!(&json!("a"), &json!("a")), json!(true));
/// assert_eq!(eq!(), json!(true));
/// assert_eq!(eq!(&json!(null)), json!(true));
/// assert_eq!(eq!(&json!(1), &json!("1")), json!(false));
/// ```
#[macro_export]
macro_rules! eq {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::eq($a, &$crate::lib::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::eq($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::eq($a, $b)
    };
}

/// `_x` helper for [eq!](crate::eq!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [eq_x!](crate::eq_x!) | `Value` forms: [eq!](crate::eq!), [eq()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::eq_x;
/// # use serde_json::json;
/// assert_eq!(eq_x(&json!("a"), &json!("a")), true);
/// ```
pub fn eq_x(a: &Value, b: &Value) -> bool {
    a == b
}

/// `_x` helper for [eq!](crate::eq!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [eq_x()] | `Value` forms: [eq!](crate::eq!), [eq()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(eq_x!(&json!("a"), &json!("a")), true);
/// ```
#[macro_export]
macro_rules! eq_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::eq_x($a, &$crate::lib::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::eq_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::eq_x($a, $b)
    };
}
