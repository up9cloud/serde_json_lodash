use crate::lib::{json, Value};

/// `_x` helper for [is_symbol()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_symbol_x;
/// # use serde_json::json;
/// assert_eq!(is_symbol_x(&json!({})), false);
/// ```
pub fn is_symbol_x(_v: &Value) -> bool {
    false
}
/// See lodash [isSymbol](https://lodash.com/docs/#isSymbol)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_symbol;
/// # use serde_json::json;
/// assert_eq!(is_symbol(&json!({})), json!(false));
/// ```
pub fn is_symbol(_v: &Value) -> Value {
    json!(is_symbol_x(_v))
}

/// Based on [is_symbol_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_symbol_x!(&json!({})), false);
/// ```
#[macro_export]
macro_rules! is_symbol_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_symbol_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_symbol_x($a)
    };
}
/// Based on [is_symbol()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_symbol!(&json!({})), json!(false));
/// assert_eq!(is_symbol!(&json!("a")), json!(false));
/// assert_eq!(is_symbol!(), json!(false));
/// assert_eq!(is_symbol!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_symbol {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_symbol($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_symbol($a)
    };
}
