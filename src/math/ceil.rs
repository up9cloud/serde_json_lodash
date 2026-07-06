use crate::lib::{Value, Number};
use crate::internal::{number_nan, value_nan, value_to_option_number};

// internal `Number` worker for [ceil()].
fn x_ceil_x(n: Number, precision: isize) -> Number {
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

/// `_x` helper for [ceil()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [ceil()] and read the returned `Value`.
pub fn ceil_x() {
    todo!()
}

/// See lodash [ceil](https://lodash.com/docs/#ceil)
///
/// Accepts anything convertible into a `Value` — an `f64`/`Number` primitive or a `json!` value.
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::ceil;
/// # use serde_json::json;
/// assert_eq!(ceil(6.004, 2), json!(6.01));
/// assert_eq!(ceil(json!(6.004), 2), json!(6.01));
/// ```
pub fn ceil<A: Into<Value>>(number: A, precision: isize) -> Value {
    match value_to_option_number(number.into()) {
        Some(n) => Value::Number(x_ceil_x(n, precision)),
        None => value_nan(),
    }
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
/// // a primitive `f64` argument is accepted too
/// assert_eq!(ceil!(6.004, 2), json!(6.01));
/// ```
///
/// Additional cases:
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
