use crate::lib::Value;

use crate::internal::uniq_by_key;

/// Fn form of [union_by!](crate::union_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [union_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::union_by;
/// # use serde_json::json;
/// assert_eq!(union_by(json!([2.1]), json!([1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())), json!([2.1, 1.2]));
/// ```
pub fn union_by(array: Value, other: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    let mut all = vec![];
    if let Value::Array(vec) = array {
        all.extend(vec);
    }
    if let Value::Array(vec) = other {
        all.extend(vec);
    }
    Value::Array(uniq_by_key(all, iteratee))
}

/// See lodash [unionBy](https://lodash.com/docs/#unionBy)
///
/// `iteratee` maps each element to the value used for uniqueness
///
/// Fn form: [union_by()] | `_x` form: **not provided** — see [union_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   union_by!(json!([2.1]), json!([1.2, 2.3]), |n| json!(n.as_f64().unwrap().floor())),
///   json!([2.1, 1.2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(union_by!(), json!([]));
/// assert_eq!(union_by!(json!([1, 1])), json!([1]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(union_by!(json!([{"a": 1}, {"a": 2}]), json!([{"a": 2}, {"a": 3}]), "a"), json!([{"a":1},{"a":2},{"a":3}]));
/// ```
#[macro_export]
macro_rules! union_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::uniq($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::union($a, $b)
    };
        ($a:expr, $b:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::union_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::union_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal $(,)*) => {
        $crate::union_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::union_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::union_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal, $($rest:tt)*) => {
        $crate::union_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::union_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::union_by($a, $b, $c)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [union_by!](crate::union_by!) and read the returned
/// `Value`.
///
/// Macro form: [union_by_x!](crate::union_by_x!)
pub fn union_by_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [union_by!](crate::union_by!) and read the returned
/// `Value`.
///
/// Fn form: [union_by_x()]
#[macro_export]
macro_rules! union_by_x {
    ($($t:tt)*) => {
        $crate::union_by_x()
    };
}
