use crate::lib::Value;

/// See lodash [isWeakMap](https://lodash.com/docs/#isWeakMap)
///
/// There is no such type in JSON, so it always returns `false`
pub fn is_weak_map(_v: &Value) -> bool {
    false
}

/// Based on [is_weak_map()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isWeakMap` values, but those are not portable to JSON
/// assert_eq!(is_weak_map!(&json!({})), false);
/// assert_eq!(is_weak_map!(&json!("a")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_weak_map!(), false);
/// assert_eq!(is_weak_map!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_weak_map {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_weak_map($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_weak_map($a)
    };
}
