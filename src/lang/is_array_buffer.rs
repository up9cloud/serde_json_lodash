use crate::lib::Value;

/// See lodash [isArrayBuffer](https://lodash.com/docs/#isArrayBuffer)
///
/// There is no such type in JSON, so it always returns `false`
pub fn is_array_buffer(_v: &Value) -> bool {
    false
}

/// Based on [is_array_buffer()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isArrayBuffer` values, but those are not portable to JSON
/// assert_eq!(is_array_buffer!(&json!({})), false);
/// assert_eq!(is_array_buffer!(&json!("a")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_buffer!(), false);
/// assert_eq!(is_array_buffer!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_array_buffer {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_buffer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_buffer($a)
    };
}
