use crate::lib::Value;

/// See lodash [isWeakSet](https://lodash.com/docs/#isWeakSet)
///
/// There is no such type in JSON, so it always returns `false`
pub fn is_weak_set(_v: &Value) -> bool {
    false
}

/// Based on [is_weak_set()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isWeakSet` values, but those are not portable to JSON
/// assert_eq!(is_weak_set!(&json!({})), false);
/// assert_eq!(is_weak_set!(&json!("a")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_weak_set!(), false);
/// assert_eq!(is_weak_set!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_weak_set {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_set($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_set($a)
    };
}
