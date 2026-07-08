use crate::lib::Value;

use crate::internal::compare_values;

use crate::collection::collect::collection_values;

use std::cmp::Ordering;

/// Fn form of [order_by!](crate::order_by!); see it for the full docs
///
/// `_x` form: **not provided** — see [order_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::order_by;
/// # use serde_json::json;
/// assert_eq!(order_by(json!([1, 3, 2]), |v| v.clone(), false), json!([3, 2, 1]));
/// ```
pub fn order_by(collection: Value, iteratee: impl Fn(&Value) -> Value, ascending: bool) -> Value {
    // Schwartzian transform, like [sort_by()]
    let mut keyed: Vec<(Value, Value)> = collection_values(collection)
        .into_iter()
        .map(|v| (iteratee(&v), v))
        .collect();
    keyed.sort_by(|(ka, _), (kb, _)| {
        let ord = compare_values(ka, kb).unwrap_or(Ordering::Equal);
        if ascending { ord } else { ord.reverse() }
    });
    Value::Array(keyed.into_iter().map(|(_, v)| v).collect())
}

/// See lodash [orderBy](https://lodash.com/docs/#orderBy)
///
/// `iteratee` maps each element to a sort key; `ascending` picks the
/// direction
///
/// Fn form: [order_by()] | `_x` form: **not provided** — see [order_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!([
///   { "user": "fred",   "age": 48 },
///   { "user": "barney", "age": 34 }
/// ]);
/// assert_eq!(
///   order_by!(users, |o| o["age"].clone(), false),
///   json!([
///     { "user": "fred",   "age": 48 },
///     { "user": "barney", "age": 34 }
///   ])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(order_by!(), json!([]));
/// assert_eq!(order_by!(json!([1, 3, 2])), json!([1, 2, 3]));
/// assert_eq!(order_by!(json!([1, 3, 2]), |v| v.clone(), false), json!([3, 2, 1]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(order_by!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a", false), json!([{"a":3,"b":2},{"a":2,"b":1},{"a":0,"b":1}]));
/// ```
#[macro_export]
macro_rules! order_by {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sort_by($a, |v| v.clone())
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)), true)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)), true)
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($b)), true)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr $(,)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr $(,)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($b)), $c)
    };
    ($a:expr, json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $c:expr, $($rest:tt)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($($__sh)+)), $c)
    };
    ($a:expr, $b:literal, $c:expr, $($rest:tt)*) => {
        $crate::order_by($a, $crate::iteratee($crate::lib::json!($b)), $c)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::order_by($a, $b, true)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::order_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::order_by($a, $b, $c)
    };
}

build_not_provided_x!(order_by, order_by_x);
