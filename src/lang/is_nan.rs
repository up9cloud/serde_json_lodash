use crate::lib::{Value, json};

/// Fn form of [is_nan!](crate::is_nan!); see it for the full docs
///
/// `_x` forms: [is_nan_x!](crate::is_nan_x!), [is_nan_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nan;
/// # use serde_json::json;
/// assert_eq!(is_nan(&json!(f64::NAN)), json!(false));
/// ```
pub fn is_nan(_v: &Value) -> Value {
    json!(is_nan_x(_v))
}

/// See lodash [isNaN](https://lodash.com/docs/#isNaN)
///
/// Fn form: [is_nan()] | `_x` forms: [is_nan_x!](crate::is_nan_x!), [is_nan_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_nan!(&json!(f64::NAN)), json!(false));
/// assert_eq!(is_nan!(&json!(null)), json!(false));
/// assert_eq!(is_nan!(), json!(false));
/// assert_eq!(is_nan!(&json!(1)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_nan!(), json!(false));
/// assert_eq!(is_nan!(&json!(null)), json!(false));
/// assert_eq!(is_nan!(&json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_nan {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_nan($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nan($a)
    };
}

/// `_x` helper for [is_nan!](crate::is_nan!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_nan_x!](crate::is_nan_x!) | `Value` forms: [is_nan!](crate::is_nan!), [is_nan()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nan_x;
/// # use serde_json::json;
/// assert_eq!(is_nan_x(&json!(f64::NAN)), false);
/// ```
pub fn is_nan_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_nan!](crate::is_nan!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_nan_x()] | `Value` forms: [is_nan!](crate::is_nan!), [is_nan()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_nan_x!(&json!(f64::NAN)), false);
/// ```
#[macro_export]
macro_rules! is_nan_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_nan_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nan_x($a)
    };
}
