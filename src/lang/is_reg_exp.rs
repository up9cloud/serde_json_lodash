use crate::lib::Value;

/// See lodash [isRegExp](https://lodash.com/docs/#isRegExp)
///
/// There is no such type in JSON, so it always returns `false`
pub fn is_reg_exp(_v: &Value) -> bool {
    false
}

/// Based on [is_reg_exp()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isRegExp` values, but those are not portable to JSON
/// assert_eq!(is_reg_exp!(&json!({})), false);
/// assert_eq!(is_reg_exp!(&json!("a")), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_reg_exp!(), false);
/// assert_eq!(is_reg_exp!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_reg_exp {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_reg_exp($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_reg_exp($a)
    };
}
