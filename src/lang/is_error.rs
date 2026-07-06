use crate::lib::Value;

/// See lodash [isError](https://lodash.com/docs/#isError)
///
/// There is no such type in JSON, so it always returns `false`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_error;
/// # use serde_json::json;
/// assert_eq!(is_error(&json!({})), false);
/// ```
pub fn is_error(_v: &Value) -> bool {
    false
}

/// Based on [is_error()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isError` values, but those are not portable to JSON
/// assert_eq!(is_error!(&json!({})), false);
/// assert_eq!(is_error!(&json!("a")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_error!(), false);
/// assert_eq!(is_error!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_error {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_error($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_error($a)
    };
}
