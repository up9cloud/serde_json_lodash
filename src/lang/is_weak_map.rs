use crate::lib::{Value, json};

/// Fn form of [is_weak_map!](crate::is_weak_map!); see it for the full docs
///
/// `_x` forms: [is_weak_map_x!](crate::is_weak_map_x!), [is_weak_map_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_weak_map;
/// # use serde_json::json;
/// assert_eq!(is_weak_map(&json!({})), json!(false));
/// ```
pub fn is_weak_map(_v: &Value) -> Value {
    json!(is_weak_map_x(_v))
}

/// See lodash [isWeakMap](https://lodash.com/docs/#isWeakMap)
///
/// Fn form: [is_weak_map()] | `_x` forms: [is_weak_map_x!](crate::is_weak_map_x!), [is_weak_map_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_weak_map!(&json!({})), json!(false));
/// assert_eq!(is_weak_map!(&json!("a")), json!(false));
/// assert_eq!(is_weak_map!(), json!(false));
/// assert_eq!(is_weak_map!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_weak_map {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_map($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_map($a)
    };
}

/// `_x` helper for [is_weak_map!](crate::is_weak_map!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_weak_map_x!](crate::is_weak_map_x!) | `Value` forms: [is_weak_map!](crate::is_weak_map!), [is_weak_map()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_weak_map_x;
/// # use serde_json::json;
/// assert_eq!(is_weak_map_x(&json!({})), false);
/// ```
pub fn is_weak_map_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_weak_map!](crate::is_weak_map!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_weak_map_x()] | `Value` forms: [is_weak_map!](crate::is_weak_map!), [is_weak_map()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_weak_map_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_weak_map_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_map_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_map_x($a)
    };
}
