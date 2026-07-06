use crate::lib::{json, Value};

/// `_x` helper for [is_arguments()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_arguments_x;
/// # use serde_json::json;
/// assert_eq!(is_arguments_x(&json!({})), false);
/// ```
pub fn is_arguments_x(_v: &Value) -> bool {
    false
}
/// See lodash [isArguments](https://lodash.com/docs/#isArguments)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_arguments;
/// # use serde_json::json;
/// assert_eq!(is_arguments(&json!({})), json!(false));
/// ```
pub fn is_arguments(_v: &Value) -> Value {
    json!(is_arguments_x(_v))
}

/// Based on [is_arguments_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_arguments_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_arguments_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_arguments_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_arguments_x($a)
    };
}
/// Based on [is_arguments()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_arguments!(&json!({})), json!(false));
/// assert_eq!(is_arguments!(&json!("a")), json!(false));
/// assert_eq!(is_arguments!(), json!(false));
/// assert_eq!(is_arguments!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_arguments {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_arguments($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_arguments($a)
    };
}
