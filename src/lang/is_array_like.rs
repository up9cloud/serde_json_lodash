use crate::lib::Value;

/// See lodash [isArrayLike](https://lodash.com/docs/#isArrayLike)
///
/// In JSON world only arrays and strings have a length
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_array_like;
/// # use serde_json::json;
/// assert_eq!(is_array_like(&json!([1, 2, 3])), true);
/// ```
pub fn is_array_like(v: &Value) -> bool {
    v.is_array() || v.is_string()
}

/// Based on [is_array_like()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array_like!(&json!([1, 2, 3])), true);
/// assert_eq!(is_array_like!(&json!("abc")), true);
/// assert_eq!(is_array_like!(&json!({})), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array_like!(), false);
/// assert_eq!(is_array_like!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_array_like {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array_like($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array_like($a)
    };
}
