use crate::lib::Value;

/// See lodash [isArray](https://lodash.com/docs/#isArray)
pub fn is_array(v: &Value) -> bool {
    v.is_array()
}

/// Based on [is_array()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_array!(&json!([1, 2, 3])), true);
/// assert_eq!(is_array!(&json!("abc")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_array!(), false);
/// assert_eq!(is_array!(&json!(null)), false);
/// assert_eq!(is_array!(&json!([])), true);
/// ```
#[macro_export]
macro_rules! is_array {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_array($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_array($a)
    };
}
