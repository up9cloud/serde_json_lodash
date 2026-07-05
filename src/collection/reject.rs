use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [reject](https://lodash.com/docs/#reject)
///
/// The opposite of [filter()](fn@crate::filter)
pub fn reject(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    Value::Array(
        collection_values(&collection)
            .into_iter()
            .filter(|v| !predicate(v))
            .collect(),
    )
}

/// Based on [reject()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   reject!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1),
///   json!([2, 4])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(reject!(), json!([]));
/// assert_eq!(reject!(json!([1, 2, 3])), json!([]));
/// ```
#[macro_export]
macro_rules! reject {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::reject($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::reject($a, $b)
    };
}
