use crate::lib::Value;

/// See lodash [isDate](https://lodash.com/docs/#isDate)
///
/// There is no such type in JSON, so it always returns `false`
pub fn is_date(_v: &Value) -> bool {
    false
}

/// Based on [is_date()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isDate` values, but those are not portable to JSON
/// assert_eq!(is_date!(&json!({})), false);
/// assert_eq!(is_date!(&json!("a")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_date!(), false);
/// assert_eq!(is_date!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_date {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_date($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_date($a)
    };
}
