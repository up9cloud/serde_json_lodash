use crate::lib::{Value, json};

/// Fn form of [is_boolean!](crate::is_boolean!); see it for the full docs
///
/// `_x` forms: [is_boolean_x!](crate::is_boolean_x!), [is_boolean_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_boolean;
/// # use serde_json::json;
/// assert_eq!(is_boolean(&json!(false)), json!(true));
/// ```
pub fn is_boolean(v: &Value) -> Value {
    json!(is_boolean_x(v))
}

/// See lodash [isBoolean](https://lodash.com/docs/#isBoolean)
///
/// Fn form: [is_boolean()] | `_x` forms: [is_boolean_x!](crate::is_boolean_x!), [is_boolean_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_boolean!(json!(false)), json!(true));
/// assert_eq!(is_boolean!(json!(null)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_boolean!(), json!(false));
/// assert_eq!(is_boolean!(json!(null)), json!(false));
/// assert_eq!(is_boolean!(json!(true)), json!(true));
/// assert_eq!(is_boolean!(json!(0)), json!(false));
/// assert_eq!(is_boolean!(json!("ab")), json!(false));
/// assert_eq!(is_boolean!(json!([1, 2])), json!(false));
/// assert_eq!(is_boolean!(json!({"a": 1})), json!(false));
/// ```
#[macro_export]
macro_rules! is_boolean {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_boolean(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_boolean(&$a)
    };
}

/// `_x` helper for [is_boolean!](crate::is_boolean!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_boolean_x!](crate::is_boolean_x!) | `Value` forms: [is_boolean!](crate::is_boolean!), [is_boolean()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_boolean_x;
/// # use serde_json::json;
/// assert_eq!(is_boolean_x(&json!(false)), true);
/// ```
pub fn is_boolean_x(v: &Value) -> bool {
    v.is_boolean()
}

/// `_x` helper for [is_boolean!](crate::is_boolean!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_boolean_x()] | `Value` forms: [is_boolean!](crate::is_boolean!), [is_boolean()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_boolean_x!(json!(false)), true);
/// ```
#[macro_export]
macro_rules! is_boolean_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_boolean_x(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_boolean_x(&$a)
    };
}
