use crate::lib::{json, Value};
use crate::{to_safe_integer_x};
///
pub fn x_range_x(start: isize, end: isize, step: isize) -> Vec<isize> {
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
///
pub fn x_range(start: isize, end: isize, step: isize) -> Value {
    Value::Array(
        x_range_x(start, end, step)
            .iter()
            .map(|n| json!(n))
            .collect::<Vec<Value>>(),
    )
}
///
pub fn range_x(start: Value, end: Value, step: isize) -> Vec<isize> {
    x_range_x(to_safe_integer_x(start), to_safe_integer_x(end), step)
}
/// See lodash [range](https://lodash.com/docs/#range)
pub fn range(start: Value, end: Value, step: isize) -> Value {
    x_range(to_safe_integer_x(start), to_safe_integer_x(end), step)
}

/// Based on [x_range_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   x_range_x!(4),
///   vec![0_isize, 1, 2, 3]
/// );
/// assert_eq!(
///   x_range_x!(-4),
///   vec![0_isize, -1, -2, -3]
/// );
/// assert_eq!(
///   x_range_x!(1, 5),
///   vec![1_isize, 2, 3, 4]
/// );
/// assert_eq!(
///   x_range_x!(0, 20, 5),
///   vec![0_isize, 5, 10, 15]
/// );
/// assert_eq!(
///   x_range_x!(0, -4, -1),
///   vec![0_isize, -1, -2, -3]
/// );
/// assert_eq!(
///   x_range_x!(1, 4, 0),
///   vec![1_isize, 1, 1]
/// );
/// let expect: Vec<isize> = vec![];
/// assert_eq!(
///   x_range_x!(0),
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
/// assert_eq!(x_range_x!(), expect);
/// ```
#[macro_export]
macro_rules! x_range_x {
    () => {{
        let a: Vec<isize> = vec![];
        a
    }};
    ($a:expr $(,)*) => {{
        if $a >= 0 {
            $crate::x_range_x(0, $a, 1)
        } else {
            $crate::x_range_x(0, $a, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_range_x($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_range_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_range_x($a, $b, $c)
    };
}
/// Based on [x_range()]
#[macro_export]
macro_rules! x_range {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {{
        if $a >= 0 {
            $crate::x_range(0, $a, 1)
        } else {
            $crate::x_range(0, $a, -1)
        }
    }};
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_range($a, $b, 1)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::x_range($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::x_range($a, $b, $c)
    };
}
/// Based on [range_x()]
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
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(range!(), json!([]));
/// ```
#[macro_export]
macro_rules! range {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {{
        let end = $crate::to_safe_integer_x($a);
        if end >= 0 {
            $crate::x_range(0, end, 1)
        } else {
            $crate::x_range(0, end, -1)
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
