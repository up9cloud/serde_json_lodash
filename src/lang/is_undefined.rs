use crate::lib::{json, Value};

/// `_x` helper for [is_undefined()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_undefined_x;
/// # use serde_json::json;
/// assert_eq!(is_undefined_x(&json!(null)), true);
/// ```
pub fn is_undefined_x(v: &Value) -> bool {
    v.is_null()
}
/// See lodash [isUndefined](https://lodash.com/docs/#isUndefined)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_undefined;
/// # use serde_json::json;
/// assert_eq!(is_undefined(&json!(null)), json!(true));
/// ```
pub fn is_undefined(v: &Value) -> Value {
    json!(is_undefined_x(v))
}

/// Based on [is_undefined_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_undefined_x!(&json!(null)), true);
/// ```
#[macro_export]
macro_rules! is_undefined_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_undefined_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_undefined_x($a)
    };
}
/// Based on [is_undefined()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_undefined!(), json!(true));
/// assert_eq!(is_undefined!(&json!(null)), json!(true));
/// assert_eq!(is_undefined!(&json!(0)), json!(false));
/// ```
#[macro_export]
macro_rules! is_undefined {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::is_undefined($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_undefined($a)
    };
}
