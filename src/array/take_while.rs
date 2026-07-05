use crate::lib::{json, Value};

/// See lodash [takeWhile](https://lodash.com/docs/#takeWhile)
///
/// Takes elements from the start while `predicate` returns `true`
pub fn take_while(array: Value, predicate: fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => Value::Array(vec.into_iter().take_while(predicate).collect()),
        _ => json!([]),
    }
}

/// Based on [take_while()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   take_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3),
///   json!([1, 2])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(take_while!(), json!([]));
/// assert_eq!(take_while!(json!([1, 2, 3])), json!([]));
/// assert_eq!(take_while!(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! take_while {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::take_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::take_while($a, $b)
    };
}
