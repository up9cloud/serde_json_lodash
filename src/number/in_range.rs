use crate::lib::{Value, json};

use crate::internal::value_to_option_number;

/// Fn form of [in_range!](crate::in_range!); see it for the full docs
///
/// `_x` forms: [in_range_x!](crate::in_range_x!), [in_range_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::in_range;
/// # use serde_json::json;
/// assert_eq!(in_range(json!(3), json!(2), json!(4)), json!(true));
/// ```
pub fn in_range(number: Value, start: Value, end: Value) -> Value {
    json!(in_range_x(number, start, end))
}

/// See lodash [inRange](https://lodash.com/docs/#inRange)
///
/// Fn form: [in_range()] | `_x` forms: [in_range_x!](crate::in_range_x!), [in_range_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(in_range!(json!(3), json!(2), json!(4)), json!(true));
/// assert_eq!(in_range!(json!(4), json!(8)), json!(true));
/// assert_eq!(in_range!(json!(4), json!(2)), json!(false));
/// assert_eq!(in_range!(json!(2), json!(2)), json!(false));
/// assert_eq!(in_range!(json!(-3), json!(-2), json!(-6)), json!(true));
/// assert_eq!(in_range!(), json!(false));
/// assert_eq!(in_range!(json!(1.2), json!(2)), json!(true));
/// assert_eq!(in_range!(json!(5.2), json!(4)), json!(false));
/// ```
#[macro_export]
macro_rules! in_range {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(false)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::in_range($a, $crate::lib::json!(0), $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::in_range($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::in_range($a, $b, $c)
    };
}

/// `_x` helper for [in_range!](crate::in_range!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [in_range_x!](crate::in_range_x!) | `Value` forms: [in_range!](crate::in_range!), [in_range()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::in_range_x;
/// # use serde_json::json;
/// assert_eq!(in_range_x(json!(3), json!(2), json!(4)), true);
/// ```
pub fn in_range_x(number: Value, start: Value, end: Value) -> bool {
    let n = value_to_option_number(number).and_then(|n| n.as_f64());
    let s = value_to_option_number(start).and_then(|n| n.as_f64());
    let e = value_to_option_number(end).and_then(|n| n.as_f64());
    match (n, s, e) {
        (Some(n), Some(s), Some(e)) => {
            let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
            n >= lo && n < hi
        }
        _ => false,
    }
}

/// `_x` helper for [in_range!](crate::in_range!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [in_range_x()] | `Value` forms: [in_range!](crate::in_range!), [in_range()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(in_range_x!(json!(3), json!(2), json!(4)), true);
/// ```
#[macro_export]
macro_rules! in_range_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::in_range_x($a, $crate::lib::json!(0), $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::in_range_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::in_range_x($a, $b, $c)
    };
}
