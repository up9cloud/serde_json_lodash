use crate::lib::Value;
use crate::internal::{f64_to_number, value_nan, value_to_option_number};

/// See lodash [subtract](https://lodash.com/docs/#subtract)
pub fn subtract(minuend: Value, subtrahend: Value) -> Value {
    match (
        value_to_option_number(minuend).and_then(|n| n.as_f64()),
        value_to_option_number(subtrahend).and_then(|n| n.as_f64()),
    ) {
        (Some(a), Some(b)) => match f64_to_number(a - b) {
            Some(n) => Value::Number(n),
            None => value_nan(),
        },
        _ => value_nan(),
    }
}

/// Based on [subtract()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   subtract!(json!(6), json!(4)),
///   json!(2)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(subtract!(), json!(0));
/// assert_eq!(subtract!(json!(6)), json!(6));
/// assert_eq!(subtract!(json!(10), json!(3)), json!(7));
/// assert_eq!(subtract!(json!("10"), json!("3")), json!(7));
/// ```
#[macro_export]
macro_rules! subtract {
    () => {
        json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::to_number($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::subtract($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::subtract($a, $b)
    };
}
