use crate::lib::{json, Value};

/// `_x` helper for [is_element()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_element_x;
/// # use serde_json::json;
/// assert_eq!(is_element_x(&json!({})), false);
/// ```
pub fn is_element_x(_v: &Value) -> bool {
    false
}
/// See lodash [isElement](https://lodash.com/docs/#isElement)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_element;
/// # use serde_json::json;
/// assert_eq!(is_element(&json!({})), json!(false));
/// ```
pub fn is_element(_v: &Value) -> Value {
    json!(is_element_x(_v))
}

/// Based on [is_element_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_element_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_element_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_element_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_element_x($a)
    };
}
/// Based on [is_element()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_element!(&json!({})), json!(false));
/// assert_eq!(is_element!(&json!("a")), json!(false));
/// assert_eq!(is_element!(), json!(false));
/// assert_eq!(is_element!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_element {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_element($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_element($a)
    };
}
