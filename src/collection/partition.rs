use crate::lib::{Value, json};

use crate::collection::collect::collection_values;

/// Fn form of [partition!](crate::partition!); see it for the full docs
///
/// `_x` form: **not provided** — see [partition_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::partition;
/// # use serde_json::json;
/// assert_eq!(partition(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!([[1, 3], [2, 4]]));
/// ```
pub fn partition(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    let (yes, no): (Vec<Value>, Vec<Value>) = collection_values(&collection)
        .into_iter()
        .partition(predicate);
    json!([yes, no])
}

/// See lodash [partition](https://lodash.com/docs/#partition)
///
/// Returns `[matched, unmatched]`
///
/// Fn form: [partition()] | `_x` form: **not provided** — see [partition_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   partition!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1),
///   json!([[1, 3], [2, 4]])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(partition!(), json!([[], []]));
/// assert_eq!(partition!(json!([1, 2, 3])), json!([[], [1, 2, 3]]));
/// ```
#[macro_export]
macro_rules! partition {
    () => {
        $crate::lib::json!([[], []])
    };
    ($a:expr $(,)*) => {
        $crate::partition($a, |_| false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::partition($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::partition($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [partition!](crate::partition!) and read the returned
/// `Value`.
///
/// Macro form: [partition_x!](crate::partition_x!)
pub fn partition_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [partition!](crate::partition!) and read the returned
/// `Value`.
///
/// Fn form: [partition_x()]
#[macro_export]
macro_rules! partition_x {
    ($($t:tt)*) => {
        $crate::partition_x()
    };
}
