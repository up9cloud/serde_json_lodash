use crate::lib::{Value, json};

use crate::collection::collect::collection_values;

/// Fn form of [every!](crate::every!); see it for the full docs
///
/// `_x` forms: [every_x!](crate::every_x!), [every_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::every;
/// # use serde_json::json;
/// assert_eq!(every(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), json!(true));
/// ```
pub fn every(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    json!(every_x(collection, predicate))
}

/// See lodash [every](https://lodash.com/docs/#every)
///
/// Fn form: [every()] | `_x` forms: [every_x!](crate::every_x!), [every_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(every!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), json!(true));
/// assert_eq!(every!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1), json!(false));
/// assert_eq!(every!(), json!(true));
/// assert_eq!(every!(json!([])), json!(true));
/// ```
#[macro_export]
macro_rules! every {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(true)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::every($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::every($a, $b)
    };
}

/// `_x` helper for [every!](crate::every!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [every_x!](crate::every_x!) | `Value` forms: [every!](crate::every!), [every()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::every_x;
/// # use serde_json::json;
/// assert_eq!(every_x(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), true);
/// ```
pub fn every_x(collection: Value, predicate: fn(&Value) -> bool) -> bool {
    collection_values(collection).iter().all(predicate)
}

/// `_x` helper for [every!](crate::every!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [every_x()] | `Value` forms: [every!](crate::every!), [every()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(every_x!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), true);
/// ```
#[macro_export]
macro_rules! every_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::every_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::every_x($a, $b)
    };
}
