use crate::lib::{Value, json};

use crate::array::sorted_index::sorted_index_impl;

/// Fn form of [sorted_index_by!](crate::sorted_index_by!); see it for the full docs
///
/// `_x` forms: [sorted_index_by_x!](crate::sorted_index_by_x!), [sorted_index_by_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_by;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by(json!([30, 50]), json!(40), |v| v.clone()), json!(1));
/// ```
pub fn sorted_index_by(array: Value, value: Value, iteratee: impl Fn(&Value) -> Value) -> Value {
    json!(sorted_index_by_x(array, value, iteratee))
}

/// See lodash [sortedIndexBy](https://lodash.com/docs/#sortedIndexBy)
///
/// Fn form: [sorted_index_by()] | `_x` forms: [sorted_index_by_x!](crate::sorted_index_by_x!), [sorted_index_by_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_index_by!(), json!(0));
/// assert_eq!(sorted_index_by!(json!([30, 50]), json!(40)), json!(1));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by!(), json!(0));
/// assert_eq!(sorted_index_by!(json!(null)), json!(0));
/// assert_eq!(sorted_index_by!(json!({"a": 1})), json!(0));
/// assert_eq!(sorted_index_by!(json!(null), json!(null)), json!(0));
/// assert_eq!(sorted_index_by!(json!(1), json!(1)), json!(0));
/// assert_eq!(sorted_index_by!(json!(1), json!(2)), json!(0));
/// assert_eq!(sorted_index_by!(json!([1, 2, 3]), json!(2)), json!(1));
/// assert_eq!(sorted_index_by!(json!("abc"), json!("bc")), json!(0));
/// // iteratee shorthands: a json! object is `_.matches`, a [path, value] pair is
/// // `_.matchesProperty`, a literal is `_.property`
/// assert_eq!(sorted_index_by!(json!([{"a": 10}, {"a": 20}]), json!({"a": 15}), "a"), json!(1));
/// ```
#[macro_export]
macro_rules! sorted_index_by {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index($a, $b)
    };
        ($a:expr, $b:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::sorted_index_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::sorted_index_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal $(,)*) => {
        $crate::sorted_index_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sorted_index_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sorted_index_by($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal, $($rest:tt)*) => {
        $crate::sorted_index_by($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_index_by($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_index_by($a, $b, $c)
    };
}

/// `_x` helper for [sorted_index_by!](crate::sorted_index_by!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [sorted_index_by_x!](crate::sorted_index_by_x!) | `Value` forms: [sorted_index_by!](crate::sorted_index_by!), [sorted_index_by()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_index_by_x;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by_x(json!([30, 50]), json!(40), |v| v.clone()), 1);
/// ```
pub fn sorted_index_by_x(array: Value, value: Value, iteratee: impl Fn(&Value) -> Value) -> usize {
    sorted_index_impl(&array, &value, false, iteratee)
}

/// `_x` helper for [sorted_index_by!](crate::sorted_index_by!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [sorted_index_by_x()] | `Value` forms: [sorted_index_by!](crate::sorted_index_by!), [sorted_index_by()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_index_by_x!(json!([30, 50]), json!(40), |v| v.clone()), 1);
/// ```
#[macro_export]
macro_rules! sorted_index_by_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        0
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::sorted_index($a, $b)
    };
        ($a:expr, $b:expr, json!($($__sh:tt)+) $(,)*) => {
        $crate::sorted_index_by_x($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+) $(,)*) => {
        $crate::sorted_index_by_x($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal $(,)*) => {
        $crate::sorted_index_by_x($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sorted_index_by_x($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, serde_json::json!($($__sh:tt)+), $($rest:tt)*) => {
        $crate::sorted_index_by_x($a, $b, $crate::iteratee($crate::lib::json!($($__sh)+)))
    };
    ($a:expr, $b:expr, $c:literal, $($rest:tt)*) => {
        $crate::sorted_index_by_x($a, $b, $crate::iteratee($crate::lib::json!($c)))
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::sorted_index_by_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::sorted_index_by_x($a, $b, $c)
    };
}
