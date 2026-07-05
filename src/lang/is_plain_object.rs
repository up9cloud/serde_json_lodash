use crate::lib::Value;

/// See lodash [isPlainObject](https://lodash.com/docs/#isPlainObject)
pub fn is_plain_object(v: &Value) -> bool {
    v.is_object()
}

/// Based on [is_plain_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_plain_object!(&json!([1, 2, 3])), false);
/// assert_eq!(is_plain_object!(&json!({ "x": 0, "y": 0 })), true);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_plain_object!(), false);
/// assert_eq!(is_plain_object!(&json!(null)), false);
/// assert_eq!(is_plain_object!(&json!({})), true);
/// ```
#[macro_export]
macro_rules! is_plain_object {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_plain_object($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_plain_object($a)
    };
}
