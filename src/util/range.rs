use crate::lib::{json, Value};
use crate::{to_safe_integer_x};
// internal worker for [range()].
pub(crate) fn x_range_x(start: isize, end: isize, step: isize) -> Vec<isize> {
    let mut result = vec![];
    // handle infinity cases
    match step {
        step if step > 0 => {
            if start >= end {
                return result;
            }
            // increase
            let mut cur = start;
            loop {
                result.push(cur);
                cur += step;
                if cur >= end {
                    break;
                }
            }
        }
        step if step < 0 => {
            if start <= end {
                return result;
            }
            // decrease
            let mut cur = start;
            loop {
                result.push(cur);
                cur += step;
                if cur <= end {
                    break;
                }
            }
        }
        _ => {
            if start >= end {
                return result;
            } else {
                return vec![start; (end - start) as usize];
            }
        }
    }
    result
}

// internal worker for [range()].
fn x_range(start: isize, end: isize, step: isize) -> Value {
    Value::Array(
        x_range_x(start, end, step)
            .iter()
            .map(|n| json!(n))
            .collect::<Vec<Value>>(),
    )
}

/// `_x` helper for [range()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::range_x;
/// # use serde_json::json;
/// assert_eq!(range_x(json!(0), json!(4), 1), vec![0_isize, 1, 2, 3]);
/// ```
pub fn range_x<A: Into<Value>, B: Into<Value>>(start: A, end: B, step: isize) -> Vec<isize> {
    let start = start.into();
    let end = end.into();
    x_range_x(to_safe_integer_x(start), to_safe_integer_x(end), step)
}

/// See lodash [range](https://lodash.com/docs/#range)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::range;
/// # use serde_json::json;
/// assert_eq!(range(json!(0), json!(20), 5), json!([0, 5, 10, 15]));
/// ```
pub fn range<A: Into<Value>, B: Into<Value>>(start: A, end: B, step: isize) -> Value {
    let start = start.into();
    let end = end.into();
    x_range(to_safe_integer_x(start), to_safe_integer_x(end), step)
}

/// Based on [range_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(range_x!(json!(0), json!(4)), vec![0_isize, 1, 2, 3]);
/// ```
#[macro_export]
macro_rules! range_x {
    () => {{
        let a: Vec<isize> = vec![];
        a
    }};
    ($a:expr $(,)*) => {{
        let end = $crate::to_safe_integer_x($a);
        if end >= 0 {
            $crate::x_range_x(0, end, 1)
        } else {
            $crate::x_range_x(0, end, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::range_x($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::range_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::range_x($a, $b, $c)
    };
}

/// Based on [range()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   range!(json!(4)),
///   json!([0, 1, 2, 3])
/// );
/// assert_eq!(
///   range!(json!(-4)),
///   json!([0, -1, -2, -3])
/// );
/// assert_eq!(
///   range!(json!(1), json!(5)),
///   json!([1, 2, 3, 4])
/// );
/// assert_eq!(
///   range!(json!(0), json!(20), 5),
///   json!([0, 5, 10, 15])
/// );
/// assert_eq!(
///   range!(json!(0), json!(-4), -1),
///   json!([0, -1, -2, -3])
/// );
/// assert_eq!(
///   range!(json!(1), json!(4), 0),
///   json!([1, 1, 1])
/// );
/// assert_eq!(
///   range!(json!(0)),
///   json!([])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(range!(), json!([]));
/// ```
#[macro_export]
macro_rules! range {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {{
        let end = $crate::to_safe_integer_x($a);
        if end >= 0 {
            $crate::range(0, end, 1)
        } else {
            $crate::range(0, end, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::range($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::range($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::range($a, $b, $c)
    };
}
