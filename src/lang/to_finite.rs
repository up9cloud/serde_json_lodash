use crate::lib::{json, Value};
use crate::internal::value_to_option_number;

/// `_x` helper for [to_finite()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn to_finite_x(v: Value) -> f64 {
    match value_to_option_number(v) {
        Some(n) => n.as_f64().unwrap_or(0.0),
        None => 0.0,
    }
}
/// See lodash [toFinite](https://lodash.com/docs/#toFinite)
pub fn to_finite(v: Value) -> Value {
    json!(to_finite_x(v))
}

/// Based on [to_finite()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(to_finite!(json!(3.2)), json!(3.2));
/// assert_eq!(to_finite!(json!(5e-324)), json!(5e-324)); // Number.MIN_VALUE
/// assert_eq!(to_finite!(json!("3.2")), json!(3.2));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_finite!(), json!(0.0));
/// assert_eq!(to_finite!(json!(null)), json!(0.0));
/// assert_eq!(to_finite!(json!("abc")), json!(0.0));
/// ```
#[macro_export]
macro_rules! to_finite {
    () => {
        json!(0.0)
    };
    ($a:expr $(,)*) => {
        $crate::to_finite($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_finite($a)
    };
}
