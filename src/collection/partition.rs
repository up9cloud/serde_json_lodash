use crate::lib::{json, Value};
use crate::collection::collect::collection_values;

/// See lodash [partition](https://lodash.com/docs/#partition)
///
/// Returns `[matched, unmatched]`
pub fn partition(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    let (yes, no): (Vec<Value>, Vec<Value>) = collection_values(&collection)
        .into_iter()
        .partition(predicate);
    json!([yes, no])
}

/// Based on [partition()]
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
/// More examples:
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
        json!([[], []])
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
