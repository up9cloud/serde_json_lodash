use crate::lib::Value;
use crate::internal::compare_values;
use crate::collection::collect::collection_values;
use std::cmp::Ordering;

/// See lodash [orderBy](https://lodash.com/docs/#orderBy)
///
/// `iteratee` maps each element to a sort key; `ascending` picks the
/// direction
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::order_by;
/// # use serde_json::json;
/// assert_eq!(order_by(json!([1, 3, 2]), |v| v.clone(), false), json!([3, 2, 1]));
/// ```
pub fn order_by(collection: Value, iteratee: fn(&Value) -> Value, ascending: bool) -> Value {
    let mut vec = collection_values(&collection);
    vec.sort_by(|a, b| {
        let ord = compare_values(&iteratee(a), &iteratee(b)).unwrap_or(Ordering::Equal);
        if ascending { ord } else { ord.reverse() }
    });
    Value::Array(vec)
}

/// Based on [order_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "fred",   "age": 48 },
///   { "user": "barney", "age": 34 }
/// ]);
/// assert_eq!(
///   order_by!(users, |o| o["age"].clone(), false),
///   json!([
///     { "user": "fred",   "age": 48 },
///     { "user": "barney", "age": 34 }
///   ])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(order_by!(), json!([]));
/// assert_eq!(order_by!(json!([1, 3, 2])), json!([1, 2, 3]));
/// assert_eq!(order_by!(json!([1, 3, 2]), |v| v.clone(), false), json!([3, 2, 1]));
/// ```
#[macro_export]
macro_rules! order_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sort_by($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::order_by($a, $b, true)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::order_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::order_by($a, $b, $c)
    };
}
