use crate::lib::Value;
use crate::internal::{value_to_option_number, value_nan};

/// See lodash [toNumber](https://lodash.com/docs/#toNumber)
///
/// Unconvertible values return `Value::Null` (there is no `NaN` in
/// serde_json)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_number;
/// # use serde_json::json;
/// assert_eq!(to_number(json!(3.2)), json!(3.2));
/// ```
pub fn to_number(v: Value) -> Value {
    match value_to_option_number(v) {
        Some(n) => Value::Number(n),
        None => value_nan(),
    }
}

/// Based on [to_number()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(to_number!(json!(3.2)), json!(3.2));
/// assert_eq!(to_number!(json!(5e-324)), json!(5e-324)); // Number.MIN_VALUE
/// assert_eq!(to_number!(json!("3.2")), json!(3.2));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_number!(), json!(null)); // NaN
/// assert_eq!(to_number!(json!(null)), json!(0));
/// assert_eq!(to_number!(json!(true)), json!(1));
/// assert_eq!(to_number!(json!("abc")), json!(null)); // NaN
/// ```
#[macro_export]
macro_rules! to_number {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::to_number($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_number($a)
    };
}
