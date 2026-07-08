use crate::lib::Value;

use crate::math::mean::mean_values;

/// Fn form of [mean_by!](crate::mean_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [mean_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::mean_by;
/// # use serde_json::json;
/// assert_eq!(mean_by(json!([2, 4]), |v| v.clone()), json!(3));
/// ```
pub fn mean_by(array: Value, iteratee: fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => mean_values(vec.iter().map(iteratee).collect()),
        _ => crate::internal::value_nan(),
    }
}

/// See lodash [meanBy](https://lodash.com/docs/#meanBy)
///
/// `iteratee` maps each element to the value to be averaged
///
/// Fn form: [mean_by()] | `_x` form: **not provided** — see [mean_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "n": 4 }, { "n": 2 }, { "n": 8 }, { "n": 6 }]);
/// assert_eq!(
///   mean_by!(objects, |o| o["n"].clone()),
///   json!(5)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(mean_by!(), json!(f64::NAN));
/// assert_eq!(mean_by!(json!([2, 4])), json!(3));
/// ```
#[macro_export]
macro_rules! mean_by {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::mean($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::mean_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::mean_by($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [mean_by!](crate::mean_by!) and read the returned
/// `Value`.
///
/// Macro form: [mean_by_x!](crate::mean_by_x!)
pub fn mean_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [mean_by!](crate::mean_by!) and read the returned
/// `Value`.
///
/// Fn form: [mean_by_x()]
#[macro_export]
macro_rules! mean_by_x {
    ($($t:tt)*) => {
        $crate::mean_by_x()
    };
}
