use crate::lib::{Number, Value};

use crate::internal::{f64_to_number, number_nan, value_nan, value_to_option_number};

// internal `Number` worker for [floor()].
fn x_floor_x(n: Number, precision: isize) -> Number {
    let f = match n.as_f64() {
        Some(f) => f,
        None => return number_nan(),
    };
    let x = 10_f64.powi(precision as i32);
    f64_to_number((f * x).floor() / x).unwrap_or_else(number_nan)
}

/// Fn form of [floor!](crate::floor!); see it for the full docs
///
/// `_x` form: **not provided** — see [floor_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::floor;
/// # use serde_json::json;
/// assert_eq!(floor(json!(0.046), 2), json!(0.04));
/// ```
pub fn floor<A: Into<Value>>(number: A, precision: isize) -> Value {
    match value_to_option_number(number.into()) {
        Some(n) => Value::Number(x_floor_x(n, precision)),
        None => value_nan(),
    }
}

/// See lodash [floor](https://lodash.com/docs/#floor)
///
/// Fn form: [floor()] | `_x` form: **not provided** — see [floor_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   floor!(json!(4.006)),
///   json!(4)
/// );
/// assert_eq!(
///   floor!(json!(0.046), 2),
///   json!(0.04)
/// );
/// assert_eq!(
///   floor!(json!(4060), -2),
///   json!(4000)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(floor!(), json!(f64::NAN));
/// assert_eq!(floor!(json!(null)), json!(0));
/// assert_eq!(floor!(json!("a")), json!(f64::NAN));
/// assert_eq!(floor!(json!("4.7")), json!(4));
/// ```
#[macro_export]
macro_rules! floor {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::floor($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::floor($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::floor($a, $b)
    };
}

build_not_provided_x!(floor, floor_x);
