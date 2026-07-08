use crate::lib::{Value, json};

/// Fn form of [to_length!](crate::to_length!); see it for the full docs
///
/// `_x` forms: [to_length_x!](crate::to_length_x!), [to_length_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_length;
/// # use serde_json::json;
/// assert_eq!(to_length(json!(3.2)), json!(3));
/// ```
pub fn to_length(v: Value) -> Value {
    json!(to_length_x(v))
}

/// See lodash [toLength](https://lodash.com/docs/#toLength)
///
/// Fn form: [to_length()] | `_x` forms: [to_length_x!](crate::to_length_x!), [to_length_x()]
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
/// Additional cases:
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
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::to_length($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_length($a)
    };
}

/// `_x` helper for [to_length!](crate::to_length!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [to_length_x!](crate::to_length_x!) | `Value` forms: [to_length!](crate::to_length!), [to_length()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_length_x;
/// # use serde_json::json;
/// assert_eq!(to_length_x(json!(3.2)), 3);
/// ```
pub fn to_length_x(v: Value) -> u64 {
    crate::to_integer_x(v).clamp(0, 4294967295) as u64
}

/// `_x` helper for [to_length!](crate::to_length!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [to_length_x()] | `Value` forms: [to_length!](crate::to_length!), [to_length()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_length_x!(json!(3.2)), 3);
/// ```
#[macro_export]
macro_rules! to_length_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        $crate::to_length_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_length_x($a)
    };
}
