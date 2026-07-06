use crate::lib::Value;
use crate::internal::value_to_option_number;

/// See lodash [inRange](https://lodash.com/docs/#inRange)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::in_range;
/// # use serde_json::json;
/// assert_eq!(in_range(json!(3), json!(2), json!(4)), true);
/// ```
pub fn in_range(number: Value, start: Value, end: Value) -> bool {
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

/// Based on [in_range()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(in_range!(json!(3), json!(2), json!(4)), true);
/// assert_eq!(in_range!(json!(4), json!(8)), true);
/// assert_eq!(in_range!(json!(4), json!(2)), false);
/// assert_eq!(in_range!(json!(2), json!(2)), false);
/// assert_eq!(in_range!(json!(-3), json!(-2), json!(-6)), true);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(in_range!(), false);
/// assert_eq!(in_range!(json!(1.2), json!(2)), true);
/// assert_eq!(in_range!(json!(5.2), json!(4)), false);
/// ```
#[macro_export]
macro_rules! in_range {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
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
