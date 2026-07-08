use crate::lib::{Value, json};

use crate::{to_safe_integer_x, x_range_x};

// internal worker for [range_right()].
pub(crate) fn x_range_right_x(start: isize, end: isize, step: isize) -> Vec<isize> {
    let mut vec = x_range_x(start, end, step);
    vec.reverse();
    vec
}

// internal worker for [range_right()].
fn x_range_right(start: isize, end: isize, step: isize) -> Value {
    Value::Array(
        x_range_right_x(start, end, step)
            .iter()
            .map(|n| json!(n))
            .collect::<Vec<Value>>(),
    )
}

/// Fn form of [range_right!](crate::range_right!); see it for the full docs
///
/// `_x` forms: [range_right_x!](crate::range_right_x!), [range_right_x()]
///
/// Examples:
///
/// ```rust
/// use serde_json::json;
/// use serde_json_lodash::range_right;
/// assert_eq!(range_right(json!(null), json!(null), 2), json!([]));
/// assert_eq!(range_right(json!(0), json!(1), 2), json!([0]));
/// ```
pub fn range_right<A: Into<Value>, B: Into<Value>>(start: A, end: B, step: isize) -> Value {
    let start = start.into();
    let end = end.into();
    x_range_right(to_safe_integer_x(start), to_safe_integer_x(end), step)
}

/// See lodash [rangeRight](https://lodash.com/docs/#rangeRight)
///
/// Fn form: [range_right()] | `_x` forms: [range_right_x!](crate::range_right_x!), [range_right_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   range_right!(json!(4)),
///   json!([3, 2, 1, 0])
/// );
/// assert_eq!(
///   range_right!(json!(-4)),
///   json!([-3, -2, -1, 0])
/// );
/// assert_eq!(
///   range_right!(json!(1), json!(5)),
///   json!([4, 3, 2, 1])
/// );
/// assert_eq!(
///   range_right!(json!(0), json!(20), 5),
///   json!([15, 10, 5, 0])
/// );
/// assert_eq!(
///   range_right!(json!(0), json!(-4), -1),
///   json!([-3, -2, -1, 0])
/// );
/// assert_eq!(
///   range_right!(json!(1), json!(4), 0),
///   json!([1, 1, 1])
/// );
/// assert_eq!(
///   range_right!(json!(0)),
///   json!([])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(range_right!(), json!([]));
/// ```
#[macro_export]
macro_rules! range_right {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {{
        let end = $crate::to_safe_integer_x($a);
        if end >= 0 {
            $crate::range_right(0, end, 1)
        } else {
            $crate::range_right(0, end, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::range_right($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::range_right($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::range_right($a, $b, $c)
    };
}

/// `_x` helper for [range_right!](crate::range_right!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [range_right_x!](crate::range_right_x!) | `Value` forms: [range_right!](crate::range_right!), [range_right()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::range_right_x;
/// # use serde_json::json;
/// assert_eq!(range_right_x(json!(0), json!(4), 1), vec![3_isize, 2, 1, 0]);
/// ```
pub fn range_right_x<A: Into<Value>, B: Into<Value>>(start: A, end: B, step: isize) -> Vec<isize> {
    let start = start.into();
    let end = end.into();
    x_range_right_x(to_safe_integer_x(start), to_safe_integer_x(end), step)
}

/// `_x` helper for [range_right!](crate::range_right!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [range_right_x()] | `Value` forms: [range_right!](crate::range_right!), [range_right()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(range_right_x!(json!(0), json!(4)), vec![3_isize, 2, 1, 0]);
/// ```
#[macro_export]
macro_rules! range_right_x {
    () => {{
        let a: Vec<isize> = vec![];
        a
    }};
    ($a:expr $(,)*) => {{
        let end = $crate::to_safe_integer_x($a);
        if end >= 0 {
            $crate::x_range_right_x(0, end, 1)
        } else {
            $crate::x_range_right_x(0, end, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::range_right_x($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::range_right_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::range_right_x($a, $b, $c)
    };
}
