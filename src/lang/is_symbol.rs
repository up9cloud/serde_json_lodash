use crate::lib::Value;

/// See lodash [isSymbol](https://lodash.com/docs/#isSymbol)
///
/// There is no such type in JSON, so it always returns `false`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_symbol;
/// # use serde_json::json;
/// assert_eq!(is_symbol(&json!({})), false);
/// ```
pub fn is_symbol(_v: &Value) -> bool {
    false
}

/// Based on [is_symbol()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js version could be true for real `isSymbol` values, but those are not portable to JSON
/// assert_eq!(is_symbol!(&json!({})), false);
/// assert_eq!(is_symbol!(&json!("a")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_symbol!(), false);
/// assert_eq!(is_symbol!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_symbol {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_symbol($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_symbol($a)
    };
}
