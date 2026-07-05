use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [some](https://lodash.com/docs/#some)
pub fn some(collection: Value, predicate: fn(&Value) -> bool) -> bool {
    collection_values(&collection).iter().any(predicate)
}

/// Based on [some()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   some!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 2),
///   true
/// );
/// assert_eq!(
///   some!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 5),
///   false
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(some!(), false);
/// assert_eq!(some!(json!([])), false);
/// ```
#[macro_export]
macro_rules! some {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::some($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::some($a, $b)
    };
}
