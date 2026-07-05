use crate::lib::Value;

/// See lodash [isBuffer](https://lodash.com/docs/#isBuffer)
///
/// There is no such type in JSON, so it always returns `false`
pub fn is_buffer(_v: &Value) -> bool {
    false
}

/// Based on [is_buffer()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isBuffer` values, but those are not portable to JSON
/// assert_eq!(is_buffer!(&json!({})), false);
/// assert_eq!(is_buffer!(&json!("a")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_buffer!(), false);
/// assert_eq!(is_buffer!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_buffer {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_buffer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_buffer($a)
    };
}
