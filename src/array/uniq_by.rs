use crate::lib::{json, Value};
use crate::internal::uniq_by_key;

/// See lodash [uniqBy](https://lodash.com/docs/#uniqBy)
///
/// `iteratee` maps each element to the value used for uniqueness
pub fn uniq_by(array: Value, iteratee: fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => Value::Array(uniq_by_key(vec, iteratee)),
        _ => json!([]),
    }
}

/// Based on [uniq_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   uniq_by!(json!([2.1, 1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([2.1, 1.2])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(uniq_by!(), json!([]));
/// assert_eq!(uniq_by!(json!([1, 2, 1])), json!([1, 2]));
/// ```
#[macro_export]
macro_rules! uniq_by {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::uniq_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::uniq_by($a, $b)
    };
}
