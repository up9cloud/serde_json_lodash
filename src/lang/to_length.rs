use crate::lib::{json, Value};

/// `_x` helper for [to_length()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
pub fn to_length_x(v: Value) -> u64 {
    crate::to_integer_x(v).clamp(0, 4294967295) as u64
}
/// See lodash [toLength](https://lodash.com/docs/#toLength)
pub fn to_length(v: Value) -> Value {
    json!(to_length_x(v))
}

/// Based on [to_length()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(to_length!(json!(3.2)), json!(3));
/// assert_eq!(to_length!(json!(5e-324)), json!(0)); // Number.MIN_VALUE
/// assert_eq!(to_length!(json!("3.2")), json!(3));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_length!(), json!(0));
/// assert_eq!(to_length!(json!(-1)), json!(0));
/// assert_eq!(to_length!(json!(9007199254740991u64)), json!(4294967295u64));
/// ```
#[macro_export]
macro_rules! to_length {
    () => {
        json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::to_length($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_length($a)
    };
}
