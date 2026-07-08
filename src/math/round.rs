use crate::lib::{Number, Value};

use crate::internal::{f64_to_number, number_nan, value_nan, value_to_option_number};

// internal `Number` worker for [round()].
fn x_round_x(n: Number, precision: isize) -> Number {
    let f = match n.as_f64() {
        Some(f) => f,
        None => return number_nan(),
    };
    let x = 10_f64.powi(precision as i32);
    f64_to_number((f * x).round() / x).unwrap_or_else(number_nan)
}

/// Fn form of [round!](crate::round!); see it for the full docs
///
/// `_x` form: **not provided** — see [round_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::round;
/// # use serde_json::json;
/// assert_eq!(round(json!(4.006), 2), json!(4.01));
/// ```
pub fn round<A: Into<Value>>(number: A, precision: isize) -> Value {
    match value_to_option_number(number.into()) {
        Some(n) => Value::Number(x_round_x(n, precision)),
        None => value_nan(),
    }
}

/// See lodash [round](https://lodash.com/docs/#round)
///
/// Fn form: [round()] | `_x` form: **not provided** — see [round_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   round!(json!(4.006)),
///   json!(4)
/// );
/// assert_eq!(
///   round!(json!(4.006), 2),
///   json!(4.01)
/// );
/// assert_eq!(
///   round!(json!(4060), -2),
///   json!(4100)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(round!(), json!(f64::NAN));
/// assert_eq!(round!(json!(null)), json!(0));
/// assert_eq!(round!(json!("a")), json!(f64::NAN));
/// assert_eq!(round!(json!("4.5")), json!(5));
/// ```
#[macro_export]
macro_rules! round {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::round($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::round($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::round($a, $b)
    };
}

build_not_provided_x!(round, round_x);
