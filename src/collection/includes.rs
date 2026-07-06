use crate::lib::{json, Value};
use crate::collection::collect::collection_values;

/// `_x` helper for [includes()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
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
        _ => collection_values(collection).contains(value),
    }
}
/// See lodash [includes](https://lodash.com/docs/#includes)
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

/// Based on [includes_x()]
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
/// Based on [includes()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(includes!(&json!([1, 2, 3]), &json!(1)), json!(true));
/// assert_eq!(includes!(&json!({ "a": 1, "b": 2 }), &json!(1)), json!(true));
/// assert_eq!(includes!(&json!("abcd"), &json!("bc")), json!(true));
/// assert_eq!(includes!(), json!(false));
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
