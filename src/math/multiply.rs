use crate::lib::Value;
use crate::internal::{f64_to_number, value_nan, value_to_option_number};

/// See lodash [multiply](https://lodash.com/docs/#multiply)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::multiply;
/// # use serde_json::json;
/// assert_eq!(multiply(json!(6), json!(4)), json!(24));
/// ```
pub fn multiply(multiplier: Value, multiplicand: Value) -> Value {
    match (
        value_to_option_number(multiplier).and_then(|n| n.as_f64()),
        value_to_option_number(multiplicand).and_then(|n| n.as_f64()),
    ) {
        (Some(a), Some(b)) => match f64_to_number(a * b) {
            Some(n) => Value::Number(n),
            None => value_nan(),
        },
        _ => value_nan(),
    }
}

/// Based on [multiply()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   multiply!(json!(6), json!(4)),
///   json!(24)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(multiply!(), json!(1));
/// assert_eq!(multiply!(json!(6)), json!(6));
/// assert_eq!(multiply!(json!(3), json!(4)), json!(12));
/// assert_eq!(multiply!(json!("3"), json!("4")), json!(12));
/// ```
#[macro_export]
macro_rules! multiply {
    () => {
        $crate::lib::json!(1)
    };
    ($a:expr $(,)*) => {
        $crate::to_number($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::multiply($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::multiply($a, $b)
    };
}

/// `_x` helper for [multiply()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [multiply()] and read the returned `Value`.
pub fn multiply_x() {
    todo!()
}
