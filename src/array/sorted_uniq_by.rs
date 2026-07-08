use crate::lib::{Value, json};

use crate::internal::uniq_by_key;

/// Fn form of [sorted_uniq_by!](crate::sorted_uniq_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [sorted_uniq_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_uniq_by;
/// # use serde_json::json;
/// assert_eq!(sorted_uniq_by(json!([1.1, 1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())), json!([1.1, 2.3]));
/// ```
pub fn sorted_uniq_by(array: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    match array {
        Value::Array(vec) => Value::Array(uniq_by_key(vec, iteratee)),
        _ => json!([]),
    }
}

/// See lodash [sortedUniqBy](https://lodash.com/docs/#sortedUniqBy)
///
/// `iteratee` maps each element to the value used for uniqueness
///
/// Fn form: [sorted_uniq_by()] | `_x` form: **not provided** — see [sorted_uniq_by_x()]
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
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(sorted_uniq_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([{"a":0,"b":1},{"a":2,"b":1},{"a":3,"b":2}]));
/// assert_eq!(sorted_uniq_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([{"a":0,"b":1},{"a":3,"b":2}]));
/// ```
#[macro_export]
macro_rules! sorted_uniq_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sorted_uniq($a)
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::sorted_uniq_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::sorted_uniq_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::sorted_uniq_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sorted_uniq_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sorted_uniq_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::sorted_uniq_by($a, $crate::iteratee($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_uniq_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::sorted_uniq_by($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sorted_uniq_by!](crate::sorted_uniq_by!) and read the
/// returned `Value`.
///
/// Macro form: [sorted_uniq_by_x!](crate::sorted_uniq_by_x!)
pub fn sorted_uniq_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [sorted_uniq_by!](crate::sorted_uniq_by!) and read the
/// returned `Value`.
///
/// Fn form: [sorted_uniq_by_x()]
#[macro_export]
macro_rules! sorted_uniq_by_x {
    ($($t:tt)*) => {
        $crate::sorted_uniq_by_x()
    };
}
