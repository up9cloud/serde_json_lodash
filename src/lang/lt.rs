use crate::lib::{Value, json};

use crate::internal::compare_values;

/// Fn form of [lt!](crate::lt!); see it for the full docs
///
/// `_x` forms: [lt_x!](crate::lt_x!), [lt_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lt;
/// # use serde_json::json;
/// assert_eq!(lt(&json!(1), &json!(3)), json!(true));
/// ```
pub fn lt(a: &Value, b: &Value) -> Value {
    json!(lt_x(a, b))
}

/// See lodash [lt](https://lodash.com/docs/#lt)
///
/// Fn form: [lt()] | `_x` forms: [lt_x!](crate::lt_x!), [lt_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(lt!(&json!(1), &json!(3)), json!(true));
/// assert_eq!(lt!(&json!(3), &json!(3)), json!(false));
/// assert_eq!(lt!(&json!(3), &json!(1)), json!(false));
/// assert_eq!(lt!(), json!(false));
/// assert_eq!(lt!(&json!(1)), json!(false));
/// assert_eq!(lt!(&json!("a"), &json!("b")), json!(true));
/// ```
#[macro_export]
macro_rules! lt {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::lt($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::lt($a, $b)
    };
}

/// `_x` helper for [lt!](crate::lt!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [lt_x!](crate::lt_x!) | `Value` forms: [lt!](crate::lt!), [lt()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::lt_x;
/// # use serde_json::json;
/// assert_eq!(lt_x(&json!(1), &json!(3)), true);
/// ```
pub fn lt_x(a: &Value, b: &Value) -> bool {
    matches!(compare_values(a, b), Some(std::cmp::Ordering::Less))
}

/// `_x` helper for [lt!](crate::lt!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [lt_x()] | `Value` forms: [lt!](crate::lt!), [lt()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(lt_x!(&json!(1), &json!(3)), true);
/// ```
#[macro_export]
macro_rules! lt_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::lt_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::lt_x($a, $b)
    };
}
