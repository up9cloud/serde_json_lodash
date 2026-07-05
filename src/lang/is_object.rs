use crate::lib::Value;

/// See lodash [isObject](https://lodash.com/docs/#isObject)
///
/// In js world arrays and functions are objects too; for JSON that means
/// objects and arrays
pub fn is_object(v: &Value) -> bool {
    v.is_object() || v.is_array()
}

/// Based on [is_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_object!(&json!({})), true);
/// assert_eq!(is_object!(&json!([1, 2, 3])), true);
/// assert_eq!(is_object!(&json!(null)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_object!(), false);
/// assert_eq!(is_object!(&json!("abc")), false);
/// ```
#[macro_export]
macro_rules! is_object {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_object($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object($a)
    };
}
