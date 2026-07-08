use crate::lib::{Value, json};

use crate::internal::base_is_match;

/// Fn form of [is_match!](crate::is_match!); see it for the full docs
///
/// `_x` forms: [is_match_x!](crate::is_match_x!), [is_match_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_match;
/// # use serde_json::json;
/// assert_eq!(is_match(&json!({"a": 1}), &json!({"a": 1})), json!(true));
/// ```
pub fn is_match(object: &Value, source: &Value) -> Value {
    json!(is_match_x(object, source))
}

/// See lodash [isMatch](https://lodash.com/docs/#isMatch)
///
/// Fn form: [is_match()] | `_x` forms: [is_match_x!](crate::is_match_x!), [is_match_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_match!(json!({"a": 1, "b": 2}), json!({"b": 2})), json!(true));
/// assert_eq!(is_match!(json!({"a": 1, "b": 2}), json!({"b": 1})), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_match!(), json!(true));
/// assert_eq!(is_match!(json!(null)), json!(true));
/// assert_eq!(is_match!(json!(1)), json!(true));
/// assert_eq!(is_match!(json!({"a": 1})), json!(true));
/// assert_eq!(is_match!(json!(null), json!(null)), json!(true));
/// assert_eq!(is_match!(json!(1), json!(1)), json!(true));
/// assert_eq!(is_match!(json!(1), json!(2)), json!(false));
/// assert_eq!(is_match!(json!([1, 2, 3]), json!(2)), json!(false));
/// assert_eq!(is_match!(json!("abc"), json!("bc")), json!(false));
/// assert_eq!(is_match!(json!({"a": [1, 2, 3]}), json!({"a": [1, 3]})), json!(true));
/// assert_eq!(is_match!(json!({"a": {"b": 1, "c": 2}}), json!({"a": {"b": 1}})), json!(true));
/// assert_eq!(is_match!(json!(1), json!({})), json!(true));
/// // SameValueZero: JS has one number type, so 1 == 1.0
/// assert_eq!(is_match!(json!({"a": 1}), json!({"a": 1.0})), json!(true));
/// ```
#[macro_export]
macro_rules! is_match {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(true)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match(&$a, &$b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_match(&$a, &$b)
    };
}

/// `_x` helper for [is_match!](crate::is_match!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_match_x!](crate::is_match_x!) | `Value` forms: [is_match!](crate::is_match!), [is_match()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_match_x;
/// # use serde_json::json;
/// assert_eq!(is_match_x(&json!({"a": 1}), &json!({"a": 1})), true);
/// ```
pub fn is_match_x(object: &Value, source: &Value) -> bool {
    base_is_match(object, source)
}

/// `_x` helper for [is_match!](crate::is_match!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_match_x()] | `Value` forms: [is_match!](crate::is_match!), [is_match()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_match_x!(json!({"a": 1}), json!({"a": 1})), true);
/// ```
#[macro_export]
macro_rules! is_match_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_match_x(&$a, &$b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_match_x(&$a, &$b)
    };
}
