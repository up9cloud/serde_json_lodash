use crate::lib::{Value, json};

/// Fn form of [take_while!](crate::take_while!); see it for the full docs
///
/// `_x` form: **not provided** — see [take_while_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::take_while;
/// # use serde_json::json;
/// assert_eq!(take_while(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3), json!([1, 2]));
/// ```
pub fn take_while(array: Value, predicate: impl Fn(&Value) -> bool) -> Value {
    match array {
        Value::Array(vec) => Value::Array(vec.into_iter().take_while(predicate).collect()),
        _ => json!([]),
    }
}

/// See lodash [takeWhile](https://lodash.com/docs/#takeWhile)
///
/// Takes elements from the start while `predicate` returns `true`
///
/// Fn form: [take_while()] | `_x` form: **not provided** — see [take_while_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   take_while!(json!([1, 2, 3, 4]), |n| n.as_i64().unwrap() < 3),
///   json!([1, 2])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(take_while!(), json!([]));
/// assert_eq!(take_while!(json!([1, 2, 3])), json!([]));
/// assert_eq!(take_while!(json!([1, 2, 3]), |_| true), json!([1, 2, 3]));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(take_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!({"b": 1})), json!([{"a":0,"b":1},{"a":2,"b":1}]));
/// assert_eq!(take_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), json!(["a", 2])), json!([]));
/// assert_eq!(take_while!(json!([{"a": 0, "b": 1}, {"a": 2, "b": 1}, {"a": 3, "b": 2}]), "a"), json!([]));
/// ```
#[macro_export]
macro_rules! take_while {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!([])
    };
    ($a:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::take_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::take_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal $(,)*) => {
        $crate::take_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::take_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::take_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:literal, $($rest:tt)*) => {
        $crate::take_while($a, $crate::internal::predicate_shorthand($crate::lib::json!($b)))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::take_while($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::take_while($a, $b)
    };
}

build_not_provided_x!(take_while, take_while_x);
