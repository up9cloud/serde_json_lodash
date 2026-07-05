use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [findLast](https://lodash.com/docs/#findLast)
///
/// Like [find()](fn@crate::find) but iterates from the end
pub fn find_last(collection: Value, predicate: fn(&Value) -> bool) -> Value {
    collection_values(&collection)
        .into_iter()
        .rev()
        .find(predicate)
        .unwrap_or(Value::Null)
}

/// Based on [find_last()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   find_last!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() % 2 == 1),
///   json!(3)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_last!(), json!(null));
/// assert_eq!(find_last!(json!([1, 2, 3]), |_| false), json!(null));
/// ```
#[macro_export]
macro_rules! find_last {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        json!(null)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_last($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::find_last($a, $b)
    };
}
