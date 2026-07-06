use crate::lib::{json, Value};
use crate::internal::base_is_match;

/// `_x` helper for [is_match()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_match_x;
/// # use serde_json::json;
/// assert_eq!(is_match_x(&json!({"a": 1}), &json!({"a": 1})), true);
/// ```
pub fn is_match_x(object: &Value, source: &Value) -> bool {
    base_is_match(object, source)
}
/// See lodash [isMatch](https://lodash.com/docs/#isMatch)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_match;
/// # use serde_json::json;
/// assert_eq!(is_match(&json!({"a": 1}), &json!({"a": 1})), json!(true));
/// ```
pub fn is_match(object: &Value, source: &Value) -> Value {
    json!(is_match_x(object, source))
}

/// Based on [is_match_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_match_x!(&json!({"a": 1}), &json!({"a": 1})), true);
/// ```
#[macro_export]
macro_rules! is_match_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_match_x($a, $b)
    };
}
/// Based on [is_match()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_match!(), json!(true));
/// assert_eq!(is_match!(&json!(1)), json!(true));
/// assert_eq!(is_match!(&json!({"a": [1, 2, 3]}), &json!({"a": [1, 3]})), json!(true));
/// assert_eq!(is_match!(&json!({"a": {"b": 1, "c": 2}}), &json!({"a": {"b": 1}})), json!(true));
/// assert_eq!(is_match!(&json!(1), &json!({})), json!(true));
/// ```
#[macro_export]
macro_rules! is_match {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(true)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_match($a, $b)
    };
}
