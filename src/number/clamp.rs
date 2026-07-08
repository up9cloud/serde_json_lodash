use crate::lib::Value;

use crate::internal::{f64_to_number, value_nan, value_to_option_number};

/// Fn form of [clamp!](crate::clamp!); see it for the full docs
///
/// `_x` form: **not provided** — see [clamp_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::clamp;
/// # use serde_json::json;
/// assert_eq!(clamp(json!(-10), json!(-5), json!(5)), json!(-5));
/// ```
pub fn clamp(number: Value, lower: Value, upper: Value) -> Value {
    let n = value_to_option_number(number).and_then(|n| n.as_f64());
    let lo = value_to_option_number(lower).and_then(|n| n.as_f64());
    let up = value_to_option_number(upper).and_then(|n| n.as_f64());
    match (n, lo, up) {
        (Some(n), Some(lo), Some(up)) => {
            let (lo, up) = if lo <= up { (lo, up) } else { (up, lo) };
            match f64_to_number(n.clamp(lo, up)) {
                Some(num) => Value::Number(num),
                None => value_nan(),
            }
        }
        _ => value_nan(),
    }
}

/// See lodash [clamp](https://lodash.com/docs/#clamp)
///
/// Fn form: [clamp()] | `_x` form: **not provided** — see [clamp_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(clamp!(json!(-10), json!(-5), json!(5)), json!(-5));
/// assert_eq!(clamp!(json!(10), json!(-5), json!(5)), json!(5));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(clamp!(), json!(f64::NAN));
/// assert_eq!(clamp!(json!(3), json!(-5), json!(5)), json!(3));
/// assert_eq!(clamp!(json!(3), json!(5), json!(-5)), json!(3)); // bounds get swapped
/// ```
#[macro_export]
macro_rules! clamp {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::to_number($a)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::clamp($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::clamp($a, $b, $c)
    };
}

build_not_provided_x!(clamp, clamp_x);
