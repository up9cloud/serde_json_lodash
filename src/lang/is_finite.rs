use crate::lib::{json, Value};

/// `_x` helper for [is_finite()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_finite_x;
/// # use serde_json::json;
/// assert_eq!(is_finite_x(&json!(3)), true);
/// ```
pub fn is_finite_x(v: &Value) -> bool {
    v.is_number()
}
/// See lodash [isFinite](https://lodash.com/docs/#isFinite)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_finite;
/// # use serde_json::json;
/// assert_eq!(is_finite(&json!(3)), json!(true));
/// ```
pub fn is_finite(v: &Value) -> Value {
    json!(is_finite_x(v))
}

/// Based on [is_finite_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_finite_x!(&json!(3)), true);
/// ```
#[macro_export]
macro_rules! is_finite_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_finite_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_finite_x($a)
    };
}
/// Based on [is_finite()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_finite!(&json!(3)), json!(true));
/// assert_eq!(is_finite!(&json!(5e-324)), json!(true));
/// assert_eq!(is_finite!(&json!("3")), json!(false));
/// assert_eq!(is_finite!(), json!(false));
/// assert_eq!(is_finite!(&json!(null)), json!(false));
/// ```
#[macro_export]
macro_rules! is_finite {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_finite($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_finite($a)
    };
}
