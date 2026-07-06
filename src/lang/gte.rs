use crate::lib::{json, Value};
use crate::internal::compare_values;

/// `_x` helper for [gte()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
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
/// See lodash [gte](https://lodash.com/docs/#gte)
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

/// Based on [gte_x()]
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
/// Based on [gte()]
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
