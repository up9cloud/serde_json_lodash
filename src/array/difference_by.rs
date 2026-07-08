use crate::lib::{Value, json};

use std::collections::HashSet;

/// Fn form of [difference_by!](crate::difference_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [difference_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::difference_by;
/// # use serde_json::json;
/// assert_eq!(difference_by(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())), json!([1.2]));
/// ```
pub fn difference_by(array: Value, other: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b = match other {
        Value::Array(v) => v,
        _ => return Value::Array(a),
    };
    let b_keys: HashSet<Value> = b.iter().map(&iteratee).collect();
    Value::Array(
        a.into_iter()
            .filter(|v| !b_keys.contains(&iteratee(v)))
            .collect(),
    )
}

/// See lodash [differenceBy](https://lodash.com/docs/#differenceBy)
///
/// `iteratee` maps each element to the value used for comparison
///
/// Fn form: [difference_by()] | `_x` form: **not provided** — see [difference_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   difference_by!(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([1.2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(difference_by!(), json!([]));
/// assert_eq!(difference_by!(json!([1, 2])), json!([1, 2]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(difference_by!(json!([{"a": 1}, {"a": 2}]), json!([{"a": 2}, {"a": 3}]), "a"), json!([{"a":1}]));
/// ```
#[macro_export]
macro_rules! difference_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::difference($a, $b)
    };
        ($a:expr, $b:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::difference_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::difference_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal $(,)*) => {
        $crate::difference_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::difference_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::difference_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal, $($rest:tt)*) => {
        $crate::difference_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::difference_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::difference_by($a, $b, $c)
    };
}

build_not_provided_x!(difference_by, difference_by_x);
