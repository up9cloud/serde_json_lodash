use crate::lib::{Value, Number};
use crate::internal::{f64_to_number, number_nan, value_nan, value_to_option_number};

/// `x_`/`_x` helper for [floor()]: takes a primitive argument and returns a primitive value.
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_floor_x;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_floor_x(Number::from_f64(4.006).unwrap(), 0), Number::from(4));
/// ```
pub fn x_floor_x(n: Number, precision: isize) -> Number {
    let f = match n.as_f64() {
        Some(f) => f,
        None => return number_nan(),
    };
    let x = 10_f64.powi(precision as i32);
    f64_to_number((f * x).floor() / x).unwrap_or_else(number_nan)
}
/// `x_` helper for [floor()]: takes a primitive argument instead of a [`Value`](crate::lib::Value).
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::x_floor;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_floor(Number::from_f64(4.006).unwrap(), 0), json!(4));
/// ```
pub fn x_floor(number: Number, precision: isize) -> Value {
    Value::Number(x_floor_x(number, precision))
}
/// See lodash [floor](https://lodash.com/docs/#floor)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::floor;
/// # use serde_json::json;
/// assert_eq!(floor(json!(0.046), 2), json!(0.04));
/// ```
pub fn floor(number: Value, precision: isize) -> Value {
    match value_to_option_number(number) {
        Some(n) => x_floor(n, precision),
        None => value_nan(),
    }
}

/// Based on [floor()]
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

/// Based on [x_floor_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_floor_x!(Number::from_f64(4.006).unwrap()), Number::from(4));
/// ```
#[macro_export]
macro_rules! x_floor_x {
    () => {
        $crate::internal::number_nan()
    };
    ($a:expr $(,)*) => {
        $crate::x_floor_x($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_floor_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_floor_x($a, $b)
    };
}
/// Based on [x_floor()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// # use serde_json::Number;
/// assert_eq!(x_floor!(Number::from_f64(4.006).unwrap()), json!(4));
/// ```
#[macro_export]
macro_rules! x_floor {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::x_floor($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_floor($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_floor($a, $b)
    };
}
