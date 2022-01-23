use crate::lib::{Value, Number};
use crate::internal::{number_nan, value_nan, value_to_option_number};

///
pub fn x_ceil_x(n: Number, precision: isize) -> Number {
    if n.is_u64() {
        if precision < 0 {
            let f = n.as_f64().unwrap();
            let x = 10_f64.powi(precision as i32);
            let result = (f * x).ceil() / x;
            return Number::from(result as u64);
        } else {
            return n;
        }
    }
    if n.is_i64() {
        if precision < 0 {
            let f = n.as_f64().unwrap();
            let x = 10_f64.powi(precision as i32);
            let result = (f * x).ceil() / x;
            return Number::from(result as i64);
        } else {
            return n;
        }
    }
    // f64
    if precision == 0 {
        let result = n.as_f64().unwrap().ceil();
        if result < 0.0 {
            return Number::from(result as i64);
        } else {
            return Number::from(result as u64);
        }
    }
    let f = n.as_f64().unwrap();
    let x = 10_f64.powi(precision as i32);
    let result = (f * x).ceil() / x;
    if precision < 0 {
        if result < 0.0 {
            return Number::from(result as i64);
        } else {
            return Number::from(result as u64);
        }
    }
    Number::from_f64(result).unwrap_or_else(number_nan)
}
///
pub fn x_ceil(number: Number, precision: isize) -> Value {
    Value::Number(x_ceil_x(number, precision))
}
/// See lodash [ceil](https://lodash.com/docs/#ceil)
pub fn ceil(number: Value, precision: isize) -> Value {
    match value_to_option_number(number) {
        Some(n) => x_ceil(n, precision),
        None => value_nan(),
    }
}

/// Based on [x_ceil_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::Number;
/// assert_eq!(
///   x_ceil_x!(Number::from_f64(4.006).unwrap()),
///   Number::from(5)
/// );
/// assert_eq!(
///   x_ceil_x!(Number::from_f64(6.004).unwrap(), 2),
///   Number::from_f64(6.01).unwrap()
/// );
/// assert_eq!(
///   x_ceil_x!(Number::from(6040), -2),
///   Number::from(6100)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::{json, Number};
/// assert_eq!(
///   x_ceil_x!(),
///   Number::from(0)
/// );
/// ```
#[macro_export]
macro_rules! x_ceil_x {
    () => {
        $crate::internal::number_nan()
    };
    ($a:expr $(,)*) => {
        $crate::x_ceil_x($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::x_ceil_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::x_ceil_x($a, $b)
    };
}
/// Based on [ceil()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   ceil!(json!(4.006)),
///   json!(5)
/// );
/// assert_eq!(
///   ceil!(json!(6.004), 2),
///   json!(6.01)
/// );
/// assert_eq!(
///   ceil!(json!(6040), -2),
///   json!(6100)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(ceil!(), json!(f64::NAN));
/// assert_eq!(ceil!(json!(null)), json!(0));
/// assert_eq!(ceil!(json!(false)), json!(0));
/// assert_eq!(ceil!(json!(true)), json!(1));
/// assert_eq!(ceil!(json!(0)), json!(0));
/// assert_eq!(ceil!(json!(1.1)), json!(2));
/// assert_eq!(ceil!(json!("")), json!(0));
/// assert_eq!(ceil!(json!("1.1")), json!(2));
/// assert_eq!(ceil!(json!("a")), json!(f64::NAN));
/// assert_eq!(ceil!(json!([])), json!(0));
/// assert_eq!(ceil!(json!([1.1])), json!(2));
/// assert_eq!(ceil!(json!(["a"])), json!(f64::NAN));
/// assert_eq!(ceil!(json!({})), json!(f64::NAN));
/// assert_eq!(ceil!(json!({"a":1})), json!(f64::NAN));
/// ```
#[macro_export]
macro_rules! ceil {
    () => {
        $crate::internal::value_nan()
    };
    ($a:expr $(,)*) => {
        $crate::ceil($a, 0)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::ceil($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::ceil($a, $b)
    };
}
