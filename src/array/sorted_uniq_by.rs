use crate::lib::{json, Value};
use crate::internal::uniq_by_key;

/// See lodash [sortedUniqBy](https://lodash.com/docs/#sortedUniqBy)
///
/// `iteratee` maps each element to the value used for uniqueness
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_uniq_by;
/// # use serde_json::json;
/// assert_eq!(sorted_uniq_by(json!([1.1, 1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())), json!([1.1, 2.3]));
/// ```
pub fn sorted_uniq_by(array: Value, iteratee: fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => Value::Array(uniq_by_key(vec, iteratee)),
        _ => json!([]),
    }
}

/// Based on [sorted_uniq_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   sorted_uniq_by!(json!([1.1, 1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([1.1, 2.3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_uniq_by!(), json!([]));
/// assert_eq!(sorted_uniq_by!(json!([1, 1, 2])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! sorted_uniq_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sorted_uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_uniq_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_uniq_by($a, $b)
    };
}

/// `_x` helper for [sorted_uniq_by()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [sorted_uniq_by()] and read the returned `Value`.
pub fn sorted_uniq_by_x() {
    todo!()
}
