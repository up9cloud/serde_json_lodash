use crate::lib::{json, Value};

/// `_x` helper for [is_null()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_null_x;
/// # use serde_json::json;
/// assert_eq!(is_null_x(&json!(null)), true);
/// ```
pub fn is_null_x(v: &Value) -> bool {
    v.is_null()
}
/// See lodash [isNull](https://lodash.com/docs/#isNull)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_null;
/// # use serde_json::json;
/// assert_eq!(is_null(&json!(null)), json!(true));
/// ```
pub fn is_null(v: &Value) -> Value {
    json!(is_null_x(v))
}

/// Based on [is_null_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_null_x!(&json!(null)), true);
/// ```
#[macro_export]
macro_rules! is_null_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_null_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_null_x($a)
    };
}
/// Based on [is_null()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_null!(&json!(null)), json!(true));
/// assert_eq!(is_null!(&json!(1)), json!(false));
/// assert_eq!(is_null!(), json!(false));
/// assert_eq!(is_null!(&json!("")), json!(false));
/// ```
#[macro_export]
macro_rules! is_null {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_null($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_null($a)
    };
}
