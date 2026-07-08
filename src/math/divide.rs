use crate::lib::Value;

use crate::internal::{f64_to_number, value_nan, value_to_option_number};

/// Fn form of [divide!](crate::divide!); see it for the full docs
///
/// `_x` form: **not provided** — see [divide_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::divide;
/// # use serde_json::json;
/// assert_eq!(divide(json!(6), json!(4)), json!(1.5));
/// ```
pub fn divide(dividend: Value, divisor: Value) -> Value {
    match (
        value_to_option_number(dividend).and_then(|n| n.as_f64()),
        value_to_option_number(divisor).and_then(|n| n.as_f64()),
    ) {
        (Some(a), Some(b)) => match f64_to_number(a / b) {
            Some(n) => Value::Number(n),
            None => value_nan(),
        },
        _ => value_nan(),
    }
}

/// See lodash [divide](https://lodash.com/docs/#divide)
///
/// Fn form: [divide()] | `_x` form: **not provided** — see [divide_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   divide!(json!(6), json!(4)),
///   json!(1.5)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(divide!(), json!(1));
/// assert_eq!(divide!(json!(6)), json!(6));
/// assert_eq!(divide!(json!(12), json!(3)), json!(4));
/// assert_eq!(divide!(json!("6"), json!("4")), json!(1.5));
/// ```
#[macro_export]
macro_rules! divide {
    () => {
        $crate::lib::json!(1)
    };
    ($a:expr $(,)*) => {
        $crate::to_number($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::divide($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::divide($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [divide!](crate::divide!) and read the returned
/// `Value`.
///
/// Macro form: [divide_x!](crate::divide_x!)
pub fn divide_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [divide!](crate::divide!) and read the returned
/// `Value`.
///
/// Fn form: [divide_x()]
#[macro_export]
macro_rules! divide_x {
    ($($t:tt)*) => {
        $crate::divide_x()
    };
}
