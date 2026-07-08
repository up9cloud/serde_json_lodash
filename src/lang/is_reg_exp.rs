use crate::lib::{Value, json};

/// Fn form of [is_reg_exp!](crate::is_reg_exp!); see it for the full docs
///
/// `_x` forms: [is_reg_exp_x!](crate::is_reg_exp_x!), [is_reg_exp_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_reg_exp;
/// # use serde_json::json;
/// assert_eq!(is_reg_exp(&json!({})), json!(false));
/// ```
pub fn is_reg_exp(_v: &Value) -> Value {
    json!(is_reg_exp_x(_v))
}

/// See lodash [isRegExp](https://lodash.com/docs/#isRegExp)
///
/// Fn form: [is_reg_exp()] | `_x` forms: [is_reg_exp_x!](crate::is_reg_exp_x!), [is_reg_exp_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_reg_exp!(&json!({})), json!(false));
/// assert_eq!(is_reg_exp!(&json!("a")), json!(false));
/// assert_eq!(is_reg_exp!(), json!(false));
/// assert_eq!(is_reg_exp!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_reg_exp {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_reg_exp($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_reg_exp($a)
    };
}

/// `_x` helper for [is_reg_exp!](crate::is_reg_exp!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_reg_exp_x!](crate::is_reg_exp_x!) | `Value` forms: [is_reg_exp!](crate::is_reg_exp!), [is_reg_exp()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_reg_exp_x;
/// # use serde_json::json;
/// assert_eq!(is_reg_exp_x(&json!({})), false);
/// ```
pub fn is_reg_exp_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_reg_exp!](crate::is_reg_exp!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_reg_exp_x()] | `Value` forms: [is_reg_exp!](crate::is_reg_exp!), [is_reg_exp()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_reg_exp_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_reg_exp_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_reg_exp_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_reg_exp_x($a)
    };
}
