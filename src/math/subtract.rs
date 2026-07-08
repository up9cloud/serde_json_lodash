use crate::lib::Value;

use crate::internal::{f64_to_number, value_nan, value_to_option_number};

/// Fn form of [subtract!](crate::subtract!); see it for the full docs
///
/// `_x` form: **not provided** — see [subtract_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::subtract;
/// # use serde_json::json;
/// assert_eq!(subtract(json!(6), json!(4)), json!(2));
/// ```
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

/// See lodash [subtract](https://lodash.com/docs/#subtract)
///
/// Fn form: [subtract()] | `_x` form: **not provided** — see [subtract_x()]
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
/// Additional cases:
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
        $crate::lib::json!(0)
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [subtract!](crate::subtract!) and read the returned
/// `Value`.
///
/// Macro form: [subtract_x!](crate::subtract_x!)
pub fn subtract_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [subtract!](crate::subtract!) and read the returned
/// `Value`.
///
/// Fn form: [subtract_x()]
#[macro_export]
macro_rules! subtract_x {
    ($($t:tt)*) => {
        $crate::subtract_x()
    };
}
