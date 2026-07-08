use crate::lib::{Value, json};

/// Fn form of [is_string!](crate::is_string!); see it for the full docs
///
/// `_x` forms: [is_string_x!](crate::is_string_x!), [is_string_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_string;
/// # use serde_json::json;
/// assert_eq!(is_string(&json!("abc")), json!(true));
/// ```
pub fn is_string(v: &Value) -> Value {
    json!(is_string_x(v))
}

/// See lodash [isString](https://lodash.com/docs/#isString)
///
/// Fn form: [is_string()] | `_x` forms: [is_string_x!](crate::is_string_x!), [is_string_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_string!(&json!("abc")), json!(true));
/// assert_eq!(is_string!(&json!(1)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_string!(), json!(false));
/// assert_eq!(is_string!(&json!(null)), json!(false));
/// assert_eq!(is_string!(&json!(true)), json!(false));
/// assert_eq!(is_string!(&json!(0)), json!(false));
/// assert_eq!(is_string!(&json!("ab")), json!(true));
/// assert_eq!(is_string!(&json!([1, 2])), json!(false));
/// assert_eq!(is_string!(&json!({"a": 1})), json!(false));
/// assert_eq!(is_string!(&json!("")), json!(true));
/// ```
#[macro_export]
macro_rules! is_string {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_string($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_string($a)
    };
}

/// `_x` helper for [is_string!](crate::is_string!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_string_x!](crate::is_string_x!) | `Value` forms: [is_string!](crate::is_string!), [is_string()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_string_x;
/// # use serde_json::json;
/// assert_eq!(is_string_x(&json!("abc")), true);
/// ```
pub fn is_string_x(v: &Value) -> bool {
    v.is_string()
}

/// `_x` helper for [is_string!](crate::is_string!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_string_x()] | `Value` forms: [is_string!](crate::is_string!), [is_string()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_string_x!(&json!("abc")), true);
/// ```
#[macro_export]
macro_rules! is_string_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_string_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_string_x($a)
    };
}
