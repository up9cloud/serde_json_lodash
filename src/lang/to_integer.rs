use crate::lib::{Value, json};

/// Fn form of [to_integer!](crate::to_integer!); see it for the full docs
///
/// `_x` forms: [to_integer_x!](crate::to_integer_x!), [to_integer_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_integer;
/// # use serde_json::json;
/// assert_eq!(to_integer(json!(3.2)), json!(3));
/// ```
pub fn to_integer(v: Value) -> Value {
    json!(to_integer_x(v))
}

/// See lodash [toInteger](https://lodash.com/docs/#toInteger)
///
/// Fn form: [to_integer()] | `_x` forms: [to_integer_x!](crate::to_integer_x!), [to_integer_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(to_integer!(json!(3.2)), json!(3));
/// assert_eq!(to_integer!(json!(5e-324)), json!(0)); // Number.MIN_VALUE
/// assert_eq!(to_integer!(json!("3.2")), json!(3));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_integer!(), json!(0));
/// assert_eq!(to_integer!(json!(null)), json!(0));
/// assert_eq!(to_integer!(json!(-3.9)), json!(-3));
/// ```
#[macro_export]
macro_rules! to_integer {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::to_integer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_integer($a)
    };
}

/// `_x` helper for [to_integer!](crate::to_integer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [to_integer_x!](crate::to_integer_x!) | `Value` forms: [to_integer!](crate::to_integer!), [to_integer()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_integer_x;
/// # use serde_json::json;
/// assert_eq!(to_integer_x(json!(3.2)), 3);
/// ```
pub fn to_integer_x(v: Value) -> i64 {
    crate::to_finite_x(v).trunc() as i64
}

/// `_x` helper for [to_integer!](crate::to_integer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [to_integer_x()] | `Value` forms: [to_integer!](crate::to_integer!), [to_integer()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_integer_x!(json!(3.2)), 3);
/// ```
#[macro_export]
macro_rules! to_integer_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        $crate::to_integer_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_integer_x($a)
    };
}
