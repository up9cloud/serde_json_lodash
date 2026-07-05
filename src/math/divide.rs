use crate::lib::Value;
use crate::internal::{f64_to_number, value_nan, value_to_option_number};

/// See lodash [divide](https://lodash.com/docs/#divide)
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

/// Based on [divide()]
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
/// More examples:
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
        json!(1)
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
