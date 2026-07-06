use crate::lib::{json, Value};

/// `_x` helper for [is_string()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_string_x;
/// # use serde_json::json;
/// assert_eq!(is_string_x(&json!("abc")), true);
/// ```
pub fn is_string_x(v: &Value) -> bool {
    v.is_string()
}
/// See lodash [isString](https://lodash.com/docs/#isString)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_string;
/// # use serde_json::json;
/// assert_eq!(is_string(&json!("abc")), json!(true));
/// ```
pub fn is_string(v: &Value) -> Value {
    json!(is_string_x(v))
}

/// Based on [is_string_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_string_x!(&json!("abc")), true);
/// ```
#[macro_export]
macro_rules! is_string_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_string_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_string_x($a)
    };
}
/// Based on [is_string()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_string!(&json!("abc")), json!(true));
/// assert_eq!(is_string!(&json!(1)), json!(false));
/// assert_eq!(is_string!(), json!(false));
/// assert_eq!(is_string!(&json!("")), json!(true));
/// ```
#[macro_export]
macro_rules! is_string {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_string($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_string($a)
    };
}
