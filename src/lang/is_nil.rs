use crate::lib::{json, Value};

/// `_x` helper for [is_nil()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nil_x;
/// # use serde_json::json;
/// assert_eq!(is_nil_x(&json!(null)), true);
/// ```
pub fn is_nil_x(v: &Value) -> bool {
    v.is_null()
}
/// See lodash [isNil](https://lodash.com/docs/#isNil)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nil;
/// # use serde_json::json;
/// assert_eq!(is_nil(&json!(null)), json!(true));
/// ```
pub fn is_nil(v: &Value) -> Value {
    json!(is_nil_x(v))
}

/// Based on [is_nil_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_nil_x!(&json!(null)), true);
/// ```
#[macro_export]
macro_rules! is_nil_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_nil_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nil_x($a)
    };
}
/// Based on [is_nil()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_nil!(&json!(null)), json!(true));
/// assert_eq!(is_nil!(), json!(true));
/// assert_eq!(is_nil!(&json!(0)), json!(false));
/// assert_eq!(is_nil!(&json!("")), json!(false));
/// ```
#[macro_export]
macro_rules! is_nil {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::is_nil($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nil($a)
    };
}
