use crate::lib::{json, Value};

/// `_x` helper for [is_weak_set()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_weak_set_x;
/// # use serde_json::json;
/// assert_eq!(is_weak_set_x(&json!({})), false);
/// ```
pub fn is_weak_set_x(_v: &Value) -> bool {
    false
}
/// See lodash [isWeakSet](https://lodash.com/docs/#isWeakSet)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_weak_set;
/// # use serde_json::json;
/// assert_eq!(is_weak_set(&json!({})), json!(false));
/// ```
pub fn is_weak_set(_v: &Value) -> Value {
    json!(is_weak_set_x(_v))
}

/// Based on [is_weak_set_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_weak_set_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_weak_set_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_set_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_set_x($a)
    };
}
/// Based on [is_weak_set()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_weak_set!(&json!({})), json!(false));
/// assert_eq!(is_weak_set!(&json!("a")), json!(false));
/// assert_eq!(is_weak_set!(), json!(false));
/// assert_eq!(is_weak_set!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_weak_set {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_set($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_set($a)
    };
}
