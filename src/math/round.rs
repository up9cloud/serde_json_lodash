use crate::lib::{Value, Number};
use crate::internal::{f64_to_number, number_nan, value_nan, value_to_option_number};

/// `x_`/`_x` helper for [round()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_round_x;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_round_x(Number::from_f64(4.006).unwrap(), 0), Number::from(4));
/// ```
pub fn x_round_x(n: Number, precision: isize) -> Number {
    let f = match n.as_f64() {
        Some(f) => f,
        None => return number_nan(),
    };
    let x = 10_f64.powi(precision as i32);
    f64_to_number((f * x).round() / x).unwrap_or_else(number_nan)
}
/// `x_` helper for [round()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_round;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_round(Number::from_f64(4.006).unwrap(), 0), json!(4));
/// ```
pub fn x_round(number: Number, precision: isize) -> Value {
    Value::Number(x_round_x(number, precision))
}
/// See lodash [round](https://lodash.com/docs/#round)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::round;
/// # use serde_json::json;
/// assert_eq!(round(json!(4.006), 2), json!(4.01));
/// ```
pub fn round(number: Value, precision: isize) -> Value {
    match value_to_option_number(number) {
        Some(n) => x_round(n, precision),
        None => value_nan(),
    }
}

/// Based on [round()]
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

/// Based on [x_round_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_round_x!(Number::from_f64(4.006).unwrap()), Number::from(4));
/// ```
#[macro_export]
macro_rules! x_round_x {
    () => {
        $crate::internal::number_nan()
    };
    ($a:expr $(,)*) => {
        $crate::x_round_x($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_round_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_round_x($a, $b)
    };
}
/// Based on [x_round()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_round!(Number::from_f64(4.006).unwrap()), json!(4));
/// ```
#[macro_export]
macro_rules! x_round {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::x_round($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_round($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_round($a, $b)
    };
}
