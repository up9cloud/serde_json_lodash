use crate::lib::Value;

/// See lodash [isArrayLikeObject](https://lodash.com/docs/#isArrayLikeObject)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_like_object;
/// # use serde_json::json;
/// assert_eq!(is_array_like_object(&json!([1, 2, 3])), true);
/// ```
pub fn is_array_like_object(v: &Value) -> bool {
    v.is_array()
}

/// Based on [is_array_like_object()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array_like_object!(&json!([1, 2, 3])), true);
/// assert_eq!(is_array_like_object!(&json!("abc")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_like_object!(), false);
/// assert_eq!(is_array_like_object!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_array_like_object {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_like_object($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_like_object($a)
    };
}
