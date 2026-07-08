use crate::lib::{Value, json};

/// Fn form of [is_map!](crate::is_map!); see it for the full docs
///
/// `_x` forms: [is_map_x!](crate::is_map_x!), [is_map_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_map;
/// # use serde_json::json;
/// assert_eq!(is_map(&json!({})), json!(false));
/// ```
pub fn is_map(_v: &Value) -> Value {
    json!(is_map_x(_v))
}

/// See lodash [isMap](https://lodash.com/docs/#isMap)
///
/// Fn form: [is_map()] | `_x` forms: [is_map_x!](crate::is_map_x!), [is_map_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_map!(&json!({})), json!(false));
/// assert_eq!(is_map!(&json!("a")), json!(false));
/// assert_eq!(is_map!(), json!(false));
/// assert_eq!(is_map!(&json!(null)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_map!(), json!(false));
/// assert_eq!(is_map!(&json!(null)), json!(false));
/// assert_eq!(is_map!(&json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_map {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_map($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_map($a)
    };
}

/// `_x` helper for [is_map!](crate::is_map!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_map_x!](crate::is_map_x!) | `Value` forms: [is_map!](crate::is_map!), [is_map()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_map_x;
/// # use serde_json::json;
/// assert_eq!(is_map_x(&json!({})), false);
/// ```
pub fn is_map_x(_v: &Value) -> bool {
    false
}

/// `_x` helper for [is_map!](crate::is_map!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_map_x()] | `Value` forms: [is_map!](crate::is_map!), [is_map()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_map_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_map_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_map_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_map_x($a)
    };
}
