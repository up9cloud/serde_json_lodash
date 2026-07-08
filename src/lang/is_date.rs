use crate::lib::{Value, json};

/// Fn form of [is_date!](crate::is_date!); see it for the full docs
///
/// `_x` forms: [is_date_x!](crate::is_date_x!), [is_date_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_date;
/// # use serde_json::json;
/// assert_eq!(is_date(&json!({})), json!(false));
/// ```
pub fn is_date(_v: &Value) -> Value {
    json!(is_date_x(_v))
}

/// See lodash [isDate](https://lodash.com/docs/#isDate)
///
/// Fn form: [is_date()] | `_x` forms: [is_date_x!](crate::is_date_x!), [is_date_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_date!(&json!({})), json!(false));
/// assert_eq!(is_date!(&json!("a")), json!(false));
/// assert_eq!(is_date!(), json!(false));
/// assert_eq!(is_date!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_date {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_date($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_date($a)
    };
}

/// `_x` helper for [is_date!](crate::is_date!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_date_x!](crate::is_date_x!) | `Value` forms: [is_date!](crate::is_date!), [is_date()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_date_x;
/// # use serde_json::json;
/// assert_eq!(is_date_x(&json!({})), false);
/// ```
pub fn is_date_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_date!](crate::is_date!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_date_x()] | `Value` forms: [is_date!](crate::is_date!), [is_date()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_date_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_date_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_date_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_date_x($a)
    };
}
