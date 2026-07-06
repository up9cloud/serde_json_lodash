use crate::lib::Value;

/// See lodash [isSet](https://lodash.com/docs/#isSet)
///
/// There is no such type in JSON, so it always returns `false`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_set;
/// # use serde_json::json;
/// assert_eq!(is_set(&json!({})), false);
/// ```
pub fn is_set(_v: &Value) -> bool {
    false
}

/// Based on [is_set()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isSet` values, but those are not portable to JSON
/// assert_eq!(is_set!(&json!({})), false);
/// assert_eq!(is_set!(&json!("a")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_set!(), false);
/// assert_eq!(is_set!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_set {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_set($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_set($a)
    };
}
