use crate::lib::{Value, json};

use std::collections::HashSet;

/// Fn form of [intersection_by!](crate::intersection_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [intersection_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::intersection_by;
/// # use serde_json::json;
/// assert_eq!(intersection_by(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())), json!([2.1]));
/// ```
pub fn intersection_by(array: Value, other: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b = match other {
        Value::Array(v) => v,
        _ => return json!([]),
    };
    let b_keys: HashSet<Value> = b.iter().map(&iteratee).collect();
    let mut out = vec![];
    let mut out_keys: HashSet<Value> = HashSet::new();
    for v in a {
        let k = iteratee(&v);
        if b_keys.contains(&k) && out_keys.insert(k) {
            out.push(v);
        }
    }
    Value::Array(out)
}

/// See lodash [intersectionBy](https://lodash.com/docs/#intersectionBy)
///
/// `iteratee` maps each element to the value used for comparison
///
/// Fn form: [intersection_by()] | `_x` form: **not provided** — see [intersection_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   intersection_by!(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([2.1])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(intersection_by!(), json!([]));
/// assert_eq!(intersection_by!(json!([1, 2])), json!([1, 2]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(intersection_by!(json!([{"a": 1}, {"a": 2}]), json!([{"a": 2}, {"a": 3}]), "a"), json!([{"a":2}]));
/// ```
#[macro_export]
macro_rules! intersection_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::intersection($a, $b)
    };
        ($a:expr, $b:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::intersection_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::intersection_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal $(,)*) => {
        $crate::intersection_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::intersection_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::intersection_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal, $($rest:tt)*) => {
        $crate::intersection_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::intersection_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::intersection_by($a, $b, $c)
    };
}

build_not_provided_x!(intersection_by, intersection_by_x);
