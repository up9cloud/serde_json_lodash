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
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(sum_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!(5));
/// assert_eq!(sum_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!(2));
/// ```
#[macro_export]
macro_rules! sum_by {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::sum($a)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::sum_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::sum_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::sum_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sum_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sum_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::sum_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sum_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sum_by($a, $b)
    };
}

build_not_provided_x!(sum_by, sum_by_x);
