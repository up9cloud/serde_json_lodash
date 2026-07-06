use crate::lib::{json, Value};

/// `_x` helper for [is_date()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
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
/// See lodash [isDate](https://lodash.com/docs/#isDate)
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

/// Based on [is_date_x()]
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
/// Based on [is_date()]
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
