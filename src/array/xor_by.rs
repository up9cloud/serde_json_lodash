use crate::lib::Value;

use std::collections::HashSet;

/// Fn form of [xor_by!](crate::xor_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [xor_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::xor_by;
/// # use serde_json::json;
/// assert_eq!(xor_by(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())), json!([1.2, 3.4]));
/// ```
pub fn xor_by(array: Value, other: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let a = match array {
        Value::Array(v) => v,
        _ => vec![],
    };
    let b = match other {
        Value::Array(v) => v,
        _ => vec![],
    };
    let a_keys: Vec<Value> = a.iter().map(&iteratee).collect();
    let b_keys: Vec<Value> = b.iter().map(&iteratee).collect();
    let a_key_set: HashSet<&Value> = a_keys.iter().collect();
    let b_key_set: HashSet<&Value> = b_keys.iter().collect();
    let mut seen: HashSet<&Value> = HashSet::new();
    let mut out = vec![];
    for (v, k) in a.iter().zip(a_keys.iter()) {
        if !b_key_set.contains(k) && seen.insert(k) {
            out.push(v.clone());
        }
    }
    for (v, k) in b.iter().zip(b_keys.iter()) {
        if !a_key_set.contains(k) && seen.insert(k) {
            out.push(v.clone());
        }
    }
    Value::Array(out)
}

/// See lodash [xorBy](https://lodash.com/docs/#xorBy)
///
/// `iteratee` maps each element to the value used for comparison
///
/// Fn form: [xor_by()] | `_x` form: **not provided** — see [xor_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   xor_by!(json!([2.1, 1.2]), json!([2.3, 3.4]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([1.2, 3.4])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(xor_by!(), json!([]));
/// assert_eq!(xor_by!(json!([1, 2])), json!([1, 2]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(xor_by!(json!([{"a": 1}, {"a": 2}]), json!([{"a": 2}, {"a": 3}]), "a"), json!([{"a":1},{"a":3}]));
/// ```
#[macro_export]
macro_rules! xor_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::xor($a, $b)
    };
        ($a:expr, $b:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::xor_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::xor_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal $(,)*) => {
        $crate::xor_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::xor_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::xor_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal, $($rest:tt)*) => {
        $crate::xor_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::xor_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::xor_by($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [xor_by!](crate::xor_by!) and read the returned
/// `Value`.
///
/// Macro form: [xor_by_x!](crate::xor_by_x!)
pub fn xor_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [xor_by!](crate::xor_by!) and read the returned
/// `Value`.
///
/// Fn form: [xor_by_x()]
#[macro_export]
macro_rules! xor_by_x {
    ($($t:tt)*) => {
        $crate::xor_by_x()
    };
}
