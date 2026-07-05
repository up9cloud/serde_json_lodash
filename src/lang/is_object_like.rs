use crate::lib::Value;

/// See lodash [isObjectLike](https://lodash.com/docs/#isObjectLike)
pub fn is_object_like(v: &Value) -> bool {
    v.is_object() || v.is_array()
}

/// Based on [is_object_like()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_object_like!(&json!({})), true);
/// assert_eq!(is_object_like!(&json!([1, 2, 3])), true);
/// assert_eq!(is_object_like!(&json!(null)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_object_like!(), false);
/// assert_eq!(is_object_like!(&json!("abc")), false);
/// ```
#[macro_export]
macro_rules! is_object_like {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_object_like($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_object_like($a)
    };
}
