use crate::lib::{Value, json};

use crate::internal::compare_values;

/// Fn form of [gte!](crate::gte!); see it for the full docs
///
/// `_x` forms: [gte_x!](crate::gte_x!), [gte_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::gte;
/// # use serde_json::json;
/// assert_eq!(gte(&json!(3), &json!(1)), json!(true));
/// ```
pub fn gte(a: &Value, b: &Value) -> Value {
    json!(gte_x(a, b))
}

/// See lodash [gte](https://lodash.com/docs/#gte)
///
/// Fn form: [gte()] | `_x` forms: [gte_x!](crate::gte_x!), [gte_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(gte!(&json!(3), &json!(1)), json!(true));
/// assert_eq!(gte!(&json!(3), &json!(3)), json!(true));
/// assert_eq!(gte!(&json!(1), &json!(3)), json!(false));
/// assert_eq!(gte!(), json!(false));
/// assert_eq!(gte!(&json!(1)), json!(false));
/// assert_eq!(gte!(&json!("a"), &json!("a")), json!(true));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(gte!(), json!(false));
/// assert_eq!(gte!(json!(null)), json!(false));
/// assert_eq!(gte!(json!({"a": 1})), json!(false));
/// assert_eq!(gte!(&json!(null), &json!(null)), json!(true));
/// assert_eq!(gte!(&json!(1), &json!(1)), json!(true));
/// assert_eq!(gte!(&json!(1), &json!(2)), json!(false));
/// assert_eq!(gte!(&json!([1, 2, 3]), &json!(2)), json!(false));
/// assert_eq!(gte!(&json!("abc"), &json!("bc")), json!(false));
/// ```
#[macro_export]
macro_rules! gte {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::gte($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::gte($a, $b)
    };
}

/// `_x` helper for [gte!](crate::gte!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [gte_x!](crate::gte_x!) | `Value` forms: [gte!](crate::gte!), [gte()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::gte_x;
/// # use serde_json::json;
/// assert_eq!(gte_x(&json!(3), &json!(1)), true);
/// ```
pub fn gte_x(a: &Value, b: &Value) -> bool {
    matches!(
        compare_values(a, b),
        Some(std::cmp::Ordering::Greater) | Some(std::cmp::Ordering::Equal)
    )
}

/// `_x` helper for [gte!](crate::gte!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [gte_x()] | `Value` forms: [gte!](crate::gte!), [gte()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(gte_x!(&json!(3), &json!(1)), true);
/// ```
#[macro_export]
macro_rules! gte_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::gte_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::gte_x($a, $b)
    };
}
