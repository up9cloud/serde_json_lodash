use crate::lib::{Value, json};

/// Fn form of [is_empty!](crate::is_empty!); see it for the full docs
///
/// `_x` forms: [is_empty_x!](crate::is_empty_x!), [is_empty_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_empty;
/// # use serde_json::json;
/// assert_eq!(is_empty(&json!(null)), json!(true));
/// ```
pub fn is_empty(v: &Value) -> Value {
    json!(is_empty_x(v))
}

/// See lodash [isEmpty](https://lodash.com/docs/#isEmpty)
///
/// Fn form: [is_empty()] | `_x` forms: [is_empty_x!](crate::is_empty_x!), [is_empty_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_empty!(&json!(null)), json!(true));
/// assert_eq!(is_empty!(&json!(true)), json!(true));
/// assert_eq!(is_empty!(&json!(1)), json!(true));
/// assert_eq!(is_empty!(&json!([1, 2, 3])), json!(false));
/// assert_eq!(is_empty!(&json!({"a": 1})), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_empty!(), json!(true));
/// assert_eq!(is_empty!(&json!(null)), json!(true));
/// assert_eq!(is_empty!(&json!(true)), json!(true));
/// assert_eq!(is_empty!(&json!(0)), json!(true));
/// assert_eq!(is_empty!(&json!("ab")), json!(false));
/// assert_eq!(is_empty!(&json!([1, 2])), json!(false));
/// assert_eq!(is_empty!(&json!({"a": 1})), json!(false));
/// assert_eq!(is_empty!(&json!("")), json!(true));
/// assert_eq!(is_empty!(&json!("abc")), json!(false));
/// assert_eq!(is_empty!(&json!([])), json!(true));
/// assert_eq!(is_empty!(&json!({})), json!(true));
/// ```
#[macro_export]
macro_rules! is_empty {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::is_empty($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_empty($a)
    };
}

/// `_x` helper for [is_empty!](crate::is_empty!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_empty_x!](crate::is_empty_x!) | `Value` forms: [is_empty!](crate::is_empty!), [is_empty()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_empty_x;
/// # use serde_json::json;
/// assert_eq!(is_empty_x(&json!(null)), true);
/// ```
pub fn is_empty_x(v: &Value) -> bool {
    match v {
        Value::Null | Value::Bool(_) | Value::Number(_) => true,
        Value::String(s) => s.is_empty(),
        Value::Array(vec) => vec.is_empty(),
        Value::Object(o) => o.is_empty(),
    }
}

/// `_x` helper for [is_empty!](crate::is_empty!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_empty_x()] | `Value` forms: [is_empty!](crate::is_empty!), [is_empty()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_empty_x!(&json!(null)), true);
/// ```
#[macro_export]
macro_rules! is_empty_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_empty_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_empty_x($a)
    };
}
