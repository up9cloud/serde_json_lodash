use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [map](https://lodash.com/docs/#map)
///
/// Works on arrays (elements) and objects (values), always returning an array
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::map;
/// # use serde_json::json;
/// assert_eq!(map(json!([1, 2]), |n| json!(n.as_i64().unwrap() * 2)), json!([2, 4]));
/// ```
pub fn map(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    Value::Array(
        collection_values(&collection)
            .iter()
            .map(iteratee)
            .collect(),
    )
}

/// Based on [map()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   map!(json!([1, 2]), |n| json!(n.as_i64().unwrap() * 2)),
///   json!([2, 4])
/// );
/// assert_eq!(
///   map!(json!({ "a": 1, "b": 2 }), |n| json!(n.as_i64().unwrap() * 2)),
///   json!([2, 4])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(map!(), json!([]));
/// assert_eq!(map!(json!([1, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! map {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::map($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::map($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::map($a, $b)
    };
}

/// `_x` helper for [map()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [map()] and read the returned `Value`.
pub fn map_x() {
    todo!()
}
