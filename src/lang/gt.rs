use crate::lib::{Value, json};

use crate::internal::compare_values;

/// Fn form of [gt!](crate::gt!); see it for the full docs
///
/// `_x` forms: [gt_x!](crate::gt_x!), [gt_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::gt;
/// # use serde_json::json;
/// assert_eq!(gt(&json!(3), &json!(1)), json!(true));
/// ```
pub fn gt(a: &Value, b: &Value) -> Value {
    json!(gt_x(a, b))
}

/// See lodash [gt](https://lodash.com/docs/#gt)
///
/// Fn form: [gt()] | `_x` forms: [gt_x!](crate::gt_x!), [gt_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(gt!(&json!(3), &json!(1)), json!(true));
/// assert_eq!(gt!(&json!(3), &json!(3)), json!(false));
/// assert_eq!(gt!(&json!(1), &json!(3)), json!(false));
/// assert_eq!(gt!(), json!(false));
/// assert_eq!(gt!(&json!(1)), json!(false));
/// assert_eq!(gt!(&json!("b"), &json!("a")), json!(true));
/// assert_eq!(gt!(&json!("3"), &json!(1)), json!(true));
/// assert_eq!(gt!(&json!({}), &json!(1)), json!(false));
/// ```
#[macro_export]
macro_rules! gt {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::gt($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::gt($a, $b)
    };
}

/// `_x` helper for [gt!](crate::gt!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [gt_x!](crate::gt_x!) | `Value` forms: [gt!](crate::gt!), [gt()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::gt_x;
/// # use serde_json::json;
/// assert_eq!(gt_x(&json!(3), &json!(1)), true);
/// ```
pub fn gt_x(a: &Value, b: &Value) -> bool {
    matches!(compare_values(a, b), Some(std::cmp::Ordering::Greater))
}

/// `_x` helper for [gt!](crate::gt!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [gt_x()] | `Value` forms: [gt!](crate::gt!), [gt()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(gt_x!(&json!(3), &json!(1)), true);
/// ```
#[macro_export]
macro_rules! gt_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::gt_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::gt_x($a, $b)
    };
}
