use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [find](https://lodash.com/docs/#find)
///
/// Returns the first matching element, or `Null` if none match
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find;
/// # use serde_json::json;
/// assert_eq!(find(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1), json!(2));
/// ```
pub fn find(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    collection_values(&collection)
        .into_iter()
        .find(predicate)
        .unwrap_or(Value::Null)
}

/// Based on [find()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   find!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1),
///   json!(2)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find!(), json!(null));
/// assert_eq!(find!(json!([1, 2, 3]), |_| false), json!(null));
/// ```
#[macro_export]
macro_rules! find {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(null)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::find($a, $b)
    };
}

/// `_x` helper for [find()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [find()] and read the returned `Value`.
pub fn find_x() {
    todo!()
}
/// Based on [find_x()]
#[macro_export]
macro_rules! find_x {
    ($($t:tt)*) => {
        $crate::find_x()
    };
}
