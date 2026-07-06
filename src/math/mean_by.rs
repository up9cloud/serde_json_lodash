use crate::lib::Value;
use crate::math::mean::mean_values;

/// See lodash [meanBy](https://lodash.com/docs/#meanBy)
///
/// `iteratee` maps each element to the value to be averaged
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

/// Based on [mean_by()]
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

/// `_x` helper for [mean_by()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [mean_by()] and read the returned `Value`.
pub fn mean_by_x() {
    todo!()
}
/// Based on [mean_by_x()]
#[macro_export]
macro_rules! mean_by_x {
    ($($t:tt)*) => {
        $crate::mean_by_x()
    };
}
