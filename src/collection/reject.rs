use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [reject](https://lodash.com/docs/#reject)
///
/// The opposite of [filter()](fn@crate::filter)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::reject;
/// # use serde_json::json;
/// assert_eq!(reject(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1), json!([2, 4]));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::reject($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::reject($a, $b)
    };
}

/// `_x` helper for [reject()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [reject()] and read the returned `Value`.
pub fn reject_x() {
    todo!()
}
/// Based on [reject_x()]
#[macro_export]
macro_rules! reject_x {
    ($($t:tt)*) => {
        $crate::reject_x()
    };
}
