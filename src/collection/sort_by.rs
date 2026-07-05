use crate::lib::Value;
use crate::internal::compare_values;
use crate::collection::collect::collection_values;
use std::cmp::Ordering;

/// See lodash [sortBy](https://lodash.com/docs/#sortBy)
///
/// `iteratee` maps each element to the value used for sorting (a stable,
/// ascending sort)
pub fn sort_by(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut vec = collection_values(&collection);
    vec.sort_by(|a, b| compare_values(&iteratee(a), &iteratee(b)).unwrap_or(Ordering::Equal));
    Value::Array(vec)
}

/// Based on [sort_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "fred",   "age": 48 },
///   { "user": "barney", "age": 36 }
/// ]);
/// assert_eq!(
///   sort_by!(users, |o| o["age"].clone()),
///   json!([
///     { "user": "barney", "age": 36 },
///     { "user": "fred",   "age": 48 }
///   ])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sort_by!(), json!([]));
/// assert_eq!(sort_by!(json!([3, 1, 2])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! sort_by {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sort_by($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sort_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sort_by($a, $b)
    };
}
