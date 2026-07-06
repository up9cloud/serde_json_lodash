use crate::lib::Value;
use crate::internal::base_is_match;

/// See lodash [isMatch](https://lodash.com/docs/#isMatch)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_match;
/// # use serde_json::json;
/// assert_eq!(is_match(&json!({"a": [1, 2, 3]}), &json!({"a": [1, 3]})), true);
/// ```
pub fn is_match(object: &Value, source: &Value) -> bool {
    base_is_match(object, source)
}

/// Based on [is_match()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2 });
/// assert_eq!(is_match!(&object, &json!({ "b": 2 })), true);
/// assert_eq!(is_match!(&object, &json!({ "b": 1 })), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_match!(), true);
/// assert_eq!(is_match!(&json!(1)), true);
/// assert_eq!(is_match!(&json!({"a": [1, 2, 3]}), &json!({"a": [1, 3]})), true); // partial array match
/// assert_eq!(is_match!(&json!({"a": {"b": 1, "c": 2}}), &json!({"a": {"b": 1}})), true);
/// assert_eq!(is_match!(&json!(1), &json!({})), true);
/// ```
#[macro_export]
macro_rules! is_match {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_match($a, $b)
    };
}
