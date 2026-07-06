use crate::lib::Value;

/// See lodash [isFunction](https://lodash.com/docs/#isFunction)
///
/// There is no such type in JSON, so it always returns `false`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_function;
/// # use serde_json::json;
/// assert_eq!(is_function(&json!({})), false);
/// ```
pub fn is_function(_v: &Value) -> bool {
    false
}

/// Based on [is_function()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isFunction` values, but those are not portable to JSON
/// assert_eq!(is_function!(&json!({})), false);
/// assert_eq!(is_function!(&json!("a")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_function!(), false);
/// assert_eq!(is_function!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_function {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_function($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_function($a)
    };
}
