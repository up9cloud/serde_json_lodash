use crate::lib::Value;

/// See lodash [isTypedArray](https://lodash.com/docs/#isTypedArray)
///
/// There is no such type in JSON, so it always returns `false`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_typed_array;
/// # use serde_json::json;
/// assert_eq!(is_typed_array(&json!({})), false);
/// ```
pub fn is_typed_array(_v: &Value) -> bool {
    false
}

/// Based on [is_typed_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isTypedArray` values, but those are not portable to JSON
/// assert_eq!(is_typed_array!(&json!({})), false);
/// assert_eq!(is_typed_array!(&json!("a")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_typed_array!(), false);
/// assert_eq!(is_typed_array!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_typed_array {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_typed_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_typed_array($a)
    };
}
