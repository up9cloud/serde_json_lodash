use crate::lib::{json, Value};

/// `_x` helper for [is_native()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_native_x;
/// # use serde_json::json;
/// assert_eq!(is_native_x(&json!({})), false);
/// ```
pub fn is_native_x(_v: &Value) -> bool {
    false
}
/// See lodash [isNative](https://lodash.com/docs/#isNative)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_native;
/// # use serde_json::json;
/// assert_eq!(is_native(&json!({})), json!(false));
/// ```
pub fn is_native(_v: &Value) -> Value {
    json!(is_native_x(_v))
}

/// Based on [is_native_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_native_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_native_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_native_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_native_x($a)
    };
}
/// Based on [is_native()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_native!(&json!({})), json!(false));
/// assert_eq!(is_native!(&json!("a")), json!(false));
/// assert_eq!(is_native!(), json!(false));
/// assert_eq!(is_native!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_native {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_native($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_native($a)
    };
}
