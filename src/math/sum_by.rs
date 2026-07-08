use crate::lib::{Value, json};

use crate::math::sum::sum_values;

/// Fn form of [sum_by!](crate::sum_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [sum_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sum_by;
/// # use serde_json::json;
/// assert_eq!(sum_by(json!([1, 2, 3]), |v| v.clone()), json!(6));
/// ```
pub fn sum_by(array: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => sum_values(vec.iter().map(iteratee).collect()),
        _ => json!(0),
    }
}

/// See lodash [sumBy](https://lodash.com/docs/#sumBy)
///
/// `iteratee` maps each element to the value to be summed
///
/// Fn form: [sum_by()] | `_x` form: **not provided** — see [sum_by_x()]
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
/// Additional cases:
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
        $crate::lib::json!(0)
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sum_by!](crate::sum_by!) and read the returned
/// `Value`.
///
/// Macro form: [sum_by_x!](crate::sum_by_x!)
pub fn sum_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sum_by!](crate::sum_by!) and read the returned
/// `Value`.
///
/// Fn form: [sum_by_x()]
#[macro_export]
macro_rules! sum_by_x {
    ($($t:tt)*) => {
        $crate::sum_by_x()
    };
}
