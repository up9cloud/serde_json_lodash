use crate::lib::Value;

use crate::internal::compare_values;

use crate::collection::collect::collection_values;

use std::cmp::Ordering;

/// Fn form of [sort_by!](crate::sort_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [sort_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sort_by;
/// # use serde_json::json;
/// assert_eq!(sort_by(json!([3, 1, 2]), |v| v.clone()), json!([1, 2, 3]));
/// ```
pub fn sort_by(collection: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    // Schwartzian transform: compute each key once (n iteratee calls instead
    // of ~2·n·log n from inside the comparator)
    let mut keyed: Vec<(Value, Value)> = collection_values(collection)
        .into_iter()
        .map(|v| (iteratee(&v), v))
        .collect();
    keyed.sort_by(|(ka, _), (kb, _)| compare_values(ka, kb).unwrap_or(Ordering::Equal));
    Value::Array(keyed.into_iter().map(|(_, v)| v).collect())
}

/// See lodash [sortBy](https://lodash.com/docs/#sortBy)
///
/// `iteratee` maps each element to the value used for sorting (a stable,
/// ascending sort)
///
/// Fn form: [sort_by()] | `_x` form: **not provided** — see [sort_by_x()]
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
/// Additional cases:
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
        $crate::lib::json!([])
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sort_by!](crate::sort_by!) and read the returned
/// `Value`.
///
/// Macro form: [sort_by_x!](crate::sort_by_x!)
pub fn sort_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sort_by!](crate::sort_by!) and read the returned
/// `Value`.
///
/// Fn form: [sort_by_x()]
#[macro_export]
macro_rules! sort_by_x {
    ($($t:tt)*) => {
        $crate::sort_by_x()
    };
}
