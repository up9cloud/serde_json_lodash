use crate::lib::{json, Value};

/// `_x` helper for [is_reg_exp()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
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
/// See lodash [isRegExp](https://lodash.com/docs/#isRegExp)
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

/// Based on [is_reg_exp_x()]
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
/// Based on [is_reg_exp()]
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
