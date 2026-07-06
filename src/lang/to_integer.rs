use crate::lib::{json, Value};

/// `_x` helper for [to_integer()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
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
/// See lodash [toInteger](https://lodash.com/docs/#toInteger)
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

/// Based on [to_integer()]
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

/// Based on [to_integer_x()]
#[macro_export]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_integer_x!(json!(3.2)), 3);
/// ```
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
