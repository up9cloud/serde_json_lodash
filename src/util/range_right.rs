use crate::lib::{json, Value};

use crate::{to_safe_integer_x, x_range_x};
///
pub fn x_range_right_x(start: isize, end: isize, step: isize) -> Vec<isize> {
    let mut vec = x_range_x(start, end, step);
    vec.reverse();
    vec
}
///
pub fn x_range_right(start: isize, end: isize, step: isize) -> Value {
    Value::Array(
        x_range_right_x(start, end, step)
            .iter()
            .map(|n| json!(n))
            .collect::<Vec<Value>>(),
    )
}
///
pub fn range_right_x(start: Value, end: Value, step: isize) -> Vec<isize> {
    x_range_right_x(to_safe_integer_x(start), to_safe_integer_x(end), step)
}
/// See lodash [rangeRight](https://lodash.com/docs/#rangeRight)
///
/// Examples:
///
/// ```rust
/// use serde_json::json;
/// use serde_json_lodash::range_right;
/// assert_eq!(range_right(json!(null), json!(null), 2), json!([]));
/// assert_eq!(range_right(json!(0), json!(1), 2), json!([0]));
/// ```
pub fn range_right(start: Value, end: Value, step: isize) -> Value {
    x_range_right(to_safe_integer_x(start), to_safe_integer_x(end), step)
}

/// Based on [x_range_right_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   x_range_right_x!(4),
///   vec![3_isize, 2, 1, 0]
/// );
/// assert_eq!(
///   x_range_right_x!(-4),
///   vec![-3_isize, -2, -1, 0]
/// );
/// assert_eq!(
///   x_range_right_x!(1, 5),
///   vec![4_isize, 3, 2, 1]
/// );
/// assert_eq!(
///   x_range_right_x!(0, 20, 5),
///   vec![15_isize, 10, 5, 0]
/// );
/// assert_eq!(
///   x_range_right_x!(0, -4, -1),
///   vec![-3_isize, -2, -1, 0]
/// );
/// assert_eq!(
///   x_range_right_x!(1, 4, 0),
///   vec![1_isize, 1, 1]
/// );
/// let expect: Vec<isize> = vec![];
/// assert_eq!(
///   x_range_right_x!(0),
///   expect
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// let expect: Vec<isize> = vec![];
/// assert_eq!(x_range_right_x!(), expect);
/// ```
#[macro_export]
macro_rules! x_range_right_x {
    () => {{
        let a: Vec<isize> = vec![];
        a
    }};
    ($a:expr $(,)*) => {{
        if $a >= 0 {
            $crate::x_range_right_x(0, $a, 1)
        } else {
            $crate::x_range_right_x(0, $a, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_range_right_x($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_range_right_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_range_right_x($a, $b, $c)
    };
}
/// Based on [x_range_right()]
#[macro_export]
macro_rules! x_range_right {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {{
        if $a >= 0 {
            $crate::x_range_right(0, $a, 1)
        } else {
            $crate::x_range_right(0, $a, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_range_right($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_range_right($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_range_right($a, $b, $c)
    };
}
/// Based on [range_right_x()]
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
/// Based on [range_right()]
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
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(range_right!(), json!([]));
/// ```
#[macro_export]
macro_rules! range_right {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {{
        let end = $crate::to_safe_integer_x($a);
        if end >= 0 {
            $crate::x_range_right(0, end, 1)
        } else {
            $crate::x_range_right(0, end, -1)
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
