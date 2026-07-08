use crate::lib::{Value, json};

/// Fn form of [includes!](crate::includes!); see it for the full docs
///
/// `_x` forms: [includes_x!](crate::includes_x!), [includes_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::includes;
/// # use serde_json::json;
/// assert_eq!(includes(&json!([1, 2, 3]), &json!(1)), json!(true));
/// ```
pub fn includes(collection: &Value, value: &Value) -> Value {
    json!(includes_x(collection, value))
}

/// See lodash [includes](https://lodash.com/docs/#includes)
///
/// Fn form: [includes()] | `_x` forms: [includes_x!](crate::includes_x!), [includes_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(includes!(&json!([1, 2, 3]), &json!(1)), json!(true));
/// assert_eq!(includes!(&json!({ "a": 1, "b": 2 }), &json!(1)), json!(true));
/// assert_eq!(includes!(&json!("abcd"), &json!("bc")), json!(true));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(includes!(), json!(false));
/// assert_eq!(includes!(json!(null)), json!(false));
/// assert_eq!(includes!(json!({"a": 1})), json!(false));
/// assert_eq!(includes!(&json!(null), &json!(null)), json!(false));
/// assert_eq!(includes!(&json!(1), &json!(1)), json!(false));
/// assert_eq!(includes!(&json!(1), &json!(2)), json!(false));
/// assert_eq!(includes!(&json!([1, 2, 3]), &json!(2)), json!(true));
/// assert_eq!(includes!(&json!("abc"), &json!("bc")), json!(true));
/// assert_eq!(includes!(&json!([1, 2, 3]), &json!(9)), json!(false));
/// ```
#[macro_export]
macro_rules! includes {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::includes($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::includes($a, $b)
    };
}

/// `_x` helper for [includes!](crate::includes!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [includes_x!](crate::includes_x!) | `Value` forms: [includes!](crate::includes!), [includes()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::includes_x;
/// # use serde_json::json;
/// assert_eq!(includes_x(&json!([1, 2, 3]), &json!(1)), true);
/// ```
pub fn includes_x(collection: &Value, value: &Value) -> bool {
    match collection {
        Value::String(s) => match value {
            Value::String(sub) => s.contains(sub.as_str()),
            _ => false,
        },
        Value::Array(vec) => vec.contains(value),
        Value::Object(o) => o.values().any(|v| v == value),
        _ => false,
    }
}

/// `_x` helper for [includes!](crate::includes!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [includes_x()] | `Value` forms: [includes!](crate::includes!), [includes()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(includes_x!(&json!([1, 2, 3]), &json!(1)), true);
/// ```
#[macro_export]
macro_rules! includes_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::includes_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::includes_x($a, $b)
    };
}
