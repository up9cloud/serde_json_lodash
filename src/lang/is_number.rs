use crate::lib::{json, Value};

/// `_x` helper for [is_number()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_number_x;
/// # use serde_json::json;
/// assert_eq!(is_number_x(&json!(3)), true);
/// ```
pub fn is_number_x(v: &Value) -> bool {
    v.is_number()
}
/// See lodash [isNumber](https://lodash.com/docs/#isNumber)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_number;
/// # use serde_json::json;
/// assert_eq!(is_number(&json!(3)), json!(true));
/// ```
pub fn is_number(v: &Value) -> Value {
    json!(is_number_x(v))
}

/// Based on [is_number_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_number_x!(&json!(3)), true);
/// ```
#[macro_export]
macro_rules! is_number_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_number_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_number_x($a)
    };
}
/// Based on [is_number()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_number!(&json!(3)), json!(true));
/// assert_eq!(is_number!(&json!(5e-324)), json!(true));
/// assert_eq!(is_number!(&json!("3")), json!(false));
/// assert_eq!(is_number!(), json!(false));
/// assert_eq!(is_number!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_number {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_number($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_number($a)
    };
}
