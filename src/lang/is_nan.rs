use crate::lib::{json, Value};

/// `_x` helper for [is_nan()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
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
/// See lodash [isNaN](https://lodash.com/docs/#isNaN)
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

/// Based on [is_nan_x()]
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
/// Based on [is_nan()]
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
