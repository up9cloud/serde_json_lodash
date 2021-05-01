use crate::lib::{json, Value};

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
pub fn range(start: isize, end: isize, step: isize) -> Value {
    x_range(start, end, step)
}

/// Description can be found in [lodash range](https://lodash.com/docs/#range)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
// => []
/// assert_eq!(
///   range!(4),
///   json!([0, 1, 2, 3])
/// );
/// assert_eq!(
///   range!(-4),
///   json!([0, -1, -2, -3])
/// );
/// assert_eq!(
///   range!(1, 5),
///   json!([1, 2, 3, 4])
/// );
/// assert_eq!(
///   range!(0, 20, 5),
///   json!([0, 5, 10, 15])
/// );
/// assert_eq!(
///   range!(0, -4, -1),
///   json!([0, -1, -2, -3])
/// );
/// assert_eq!(
///   range!(1, 4, 0),
///   json!([1, 1, 1])
/// );
/// assert_eq!(
///   range!(0),
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
        if $a >= 0 {
            $crate::range(0, $a, 1)
        } else {
            $crate::range(0, $a, -1)
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
