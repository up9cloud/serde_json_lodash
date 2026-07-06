use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [every](https://lodash.com/docs/#every)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::every;
/// # use serde_json::json;
/// assert_eq!(every(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0), true);
/// ```
pub fn every(collection: Value, predicate: fn(&Value) -> bool) -> bool {
    collection_values(&collection).iter().all(predicate)
}

/// Based on [every()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   every!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 0),
///   true
/// );
/// assert_eq!(
///   every!(json!([1, 2, 3]), |n| n.as_i64().unwrap() > 1),
///   false
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(every!(), true);
/// assert_eq!(every!(json!([])), true);
/// ```
#[macro_export]
macro_rules! every {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        true
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::every($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::every($a, $b)
    };
}
