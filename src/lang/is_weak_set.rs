use crate::lib::{Value, json};

/// Fn form of [is_weak_set!](crate::is_weak_set!); see it for the full docs
///
/// `_x` forms: [is_weak_set_x!](crate::is_weak_set_x!), [is_weak_set_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_weak_set;
/// # use serde_json::json;
/// assert_eq!(is_weak_set(&json!({})), json!(false));
/// ```
pub fn is_weak_set(_v: &Value) -> Value {
    json!(is_weak_set_x(_v))
}

/// See lodash [isWeakSet](https://lodash.com/docs/#isWeakSet)
///
/// Fn form: [is_weak_set()] | `_x` forms: [is_weak_set_x!](crate::is_weak_set_x!), [is_weak_set_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_weak_set!(&json!({})), json!(false));
/// assert_eq!(is_weak_set!(&json!("a")), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_weak_set!(), json!(false));
/// assert_eq!(is_weak_set!(&json!(null)), json!(false));
/// assert_eq!(is_weak_set!(&json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_weak_set {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_set($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_set($a)
    };
}

/// `_x` helper for [is_weak_set!](crate::is_weak_set!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_weak_set_x!](crate::is_weak_set_x!) | `Value` forms: [is_weak_set!](crate::is_weak_set!), [is_weak_set()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_weak_set_x;
/// # use serde_json::json;
/// assert_eq!(is_weak_set_x(&json!({})), false);
/// ```
pub fn is_weak_set_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_weak_set!](crate::is_weak_set!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_weak_set_x()] | `Value` forms: [is_weak_set!](crate::is_weak_set!), [is_weak_set()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_weak_set_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_weak_set_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_set_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_set_x($a)
    };
}
