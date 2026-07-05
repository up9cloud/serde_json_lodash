use crate::lib::{json, Value};
use crate::math::sum::sum_values;

/// See lodash [sumBy](https://lodash.com/docs/#sumBy)
///
/// `iteratee` maps each element to the value to be summed
pub fn sum_by(array: Value, iteratee: fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => sum_values(vec.iter().map(iteratee).collect()),
        _ => json!(0),
    }
}

/// Based on [sum_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "n": 4 }, { "n": 2 }, { "n": 8 }, { "n": 6 }]);
/// assert_eq!(
///   sum_by!(objects, |o| o["n"].clone()),
///   json!(20)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sum_by!(), json!(0));
/// assert_eq!(sum_by!(json!([1, 2, 3])), json!(6));
/// ```
#[macro_export]
macro_rules! sum_by {
    () => {
        json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::sum($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sum_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sum_by($a, $b)
    };
}
