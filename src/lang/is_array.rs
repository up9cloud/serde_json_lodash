use crate::lib::{json, Value};

/// `_x` helper for [is_array()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_x;
/// # use serde_json::json;
/// assert_eq!(is_array_x(&json!([1, 2, 3])), true);
/// ```
pub fn is_array_x(v: &Value) -> bool {
    v.is_array()
}
/// See lodash [isArray](https://lodash.com/docs/#isArray)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array;
/// # use serde_json::json;
/// assert_eq!(is_array(&json!([1, 2, 3])), json!(true));
/// ```
pub fn is_array(v: &Value) -> Value {
    json!(is_array_x(v))
}

/// Based on [is_array_x()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_x!(&json!([1, 2, 3])), true);
/// ```
#[macro_export]
macro_rules! is_array_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_x($a)
    };
}
/// Based on [is_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array!(&json!([1, 2, 3])), json!(true));
/// assert_eq!(is_array!(&json!("abc")), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array!(), json!(false));
/// assert_eq!(is_array!(&json!(null)), json!(false));
/// assert_eq!(is_array!(&json!([])), json!(true));
/// ```
#[macro_export]
macro_rules! is_array {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array($a)
    };
}
