use crate::lib::{json, Value};

/// See lodash [dropWhile](https://lodash.com/docs/#dropWhile)
///
/// Drops elements from the start while `predicate` returns `true`
pub fn drop_while(array: Value, predicate: fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => Value::Array(vec.into_iter().skip_while(predicate).collect()),
        _ => json!([]),
    }
}

/// Based on [drop_while()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   drop_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3),
///   json!([3, 4])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(drop_while!(), json!([]));
/// assert_eq!(drop_while!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(drop_while!(json!([1, 2, 3]), |_| true), json!([]));
/// ```
#[macro_export]
macro_rules! drop_while {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::drop_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::drop_while($a, $b)
    };
}
