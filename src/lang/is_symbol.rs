use crate::lib::{Value, json};

/// Fn form of [is_symbol!](crate::is_symbol!); see it for the full docs
///
/// `_x` forms: [is_symbol_x!](crate::is_symbol_x!), [is_symbol_x()]
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

/// See lodash [isSymbol](https://lodash.com/docs/#isSymbol)
///
/// Fn form: [is_symbol()] | `_x` forms: [is_symbol_x!](crate::is_symbol_x!), [is_symbol_x()]
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

/// `_x` helper for [is_symbol!](crate::is_symbol!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_symbol_x!](crate::is_symbol_x!) | `Value` forms: [is_symbol!](crate::is_symbol!), [is_symbol()]
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

/// `_x` helper for [is_symbol!](crate::is_symbol!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_symbol_x()] | `Value` forms: [is_symbol!](crate::is_symbol!), [is_symbol()]
///
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
