use crate::lib::Value;

/// See lodash [isElement](https://lodash.com/docs/#isElement)
///
/// There is no such type in JSON, so it always returns `false`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_element;
/// # use serde_json::json;
/// assert_eq!(is_element(&json!({})), false);
/// ```
pub fn is_element(_v: &Value) -> bool {
    false
}

/// Based on [is_element()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isElement` values, but those are not portable to JSON
/// assert_eq!(is_element!(&json!({})), false);
/// assert_eq!(is_element!(&json!("a")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_element!(), false);
/// assert_eq!(is_element!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_element {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_element($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_element($a)
    };
}
