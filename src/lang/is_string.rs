use crate::lib::Value;

/// See lodash [isString](https://lodash.com/docs/#isString)
pub fn is_string(v: &Value) -> bool {
    v.is_string()
}

/// Based on [is_string()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_string!(&json!("abc")), true);
/// assert_eq!(is_string!(&json!(1)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_string!(), false);
/// assert_eq!(is_string!(&json!("")), true);
/// ```
#[macro_export]
macro_rules! is_string {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_string($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_string($a)
    };
}
