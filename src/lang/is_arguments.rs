use crate::lib::Value;

/// See lodash [isArguments](https://lodash.com/docs/#isArguments)
///
/// There is no such type in JSON, so it always returns `false`
pub fn is_arguments(_v: &Value) -> bool {
    false
}

/// Based on [is_arguments()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isArguments` values, but those are not portable to JSON
/// assert_eq!(is_arguments!(&json!({})), false);
/// assert_eq!(is_arguments!(&json!("a")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_arguments!(), false);
/// assert_eq!(is_arguments!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_arguments {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_arguments($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_arguments($a)
    };
}
