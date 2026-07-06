use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [filter](https://lodash.com/docs/#filter)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::filter;
/// # use serde_json::json;
/// assert_eq!(filter(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!([1, 3]));
/// ```
pub fn filter(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    Value::Array(
        collection_values(&collection)
            .into_iter()
            .filter(predicate)
            .collect(),
    )
}

/// Based on [filter()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   filter!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1),
///   json!([1, 3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(filter!(), json!([]));
/// assert_eq!(filter!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(filter!(json!({"a": 1, "b": 2}), |v| v.as_i64().unwrap() > 1), json!([2]));
/// ```
#[macro_export]
macro_rules! filter {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::map($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::filter($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::filter($a, $b)
    };
}

/// `_x` helper for [filter()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [filter()] and read the returned `Value`.
pub fn filter_x() {
    todo!()
}
