use crate::lib::{json, Value};
use crate::collection::collect::collection_values;

/// `_x` helper for [some()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::some_x;
/// # use serde_json::json;
/// assert_eq!(some_x(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), true);
/// ```
pub fn some_x(collection: Value, predicate: fn(&Value) -> bool) -> bool {
    collection_values(&collection).iter().any(predicate)
}
/// See lodash [some](https://lodash.com/docs/#some)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::some;
/// # use serde_json::json;
/// assert_eq!(some(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), json!(true));
/// ```
pub fn some(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    json!(some_x(collection, predicate))
}

/// Based on [some_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(some_x!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), true);
/// ```
#[macro_export]
macro_rules! some_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::some_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::some_x($a, $b)
    };
}
/// Based on [some()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(some!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2), json!(true));
/// assert_eq!(some!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 5), json!(false));
/// assert_eq!(some!(), json!(false));
/// assert_eq!(some!(json!([])), json!(false));
/// ```
#[macro_export]
macro_rules! some {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::some($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::some($a, $b)
    };
}
