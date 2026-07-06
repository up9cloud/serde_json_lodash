use crate::lib::{json, Value};

/// `_x` helper for [is_boolean()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_boolean_x;
/// # use serde_json::json;
/// assert_eq!(is_boolean_x(&json!(false)), true);
/// ```
pub fn is_boolean_x(v: &Value) -> bool {
    v.is_boolean()
}
/// See lodash [isBoolean](https://lodash.com/docs/#isBoolean)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_boolean;
/// # use serde_json::json;
/// assert_eq!(is_boolean(&json!(false)), json!(true));
/// ```
pub fn is_boolean(v: &Value) -> Value {
    json!(is_boolean_x(v))
}

/// Based on [is_boolean_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_boolean_x!(&json!(false)), true);
/// ```
#[macro_export]
macro_rules! is_boolean_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_boolean_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_boolean_x($a)
    };
}
/// Based on [is_boolean()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_boolean!(&json!(false)), json!(true));
/// assert_eq!(is_boolean!(&json!(null)), json!(false));
/// assert_eq!(is_boolean!(), json!(false));
/// assert_eq!(is_boolean!(&json!(true)), json!(true));
/// assert_eq!(is_boolean!(&json!(0)), json!(false));
/// ```
#[macro_export]
macro_rules! is_boolean {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_boolean($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_boolean($a)
    };
}
