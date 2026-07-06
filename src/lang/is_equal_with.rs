use crate::lib::{json, Value};

/// `_x` helper for [is_equal_with()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_equal_with_x;
/// # use serde_json::json;
/// assert_eq!(is_equal_with_x(&json!(1), &json!(1), |_, _| None), true);
/// ```
pub fn is_equal_with_x(
    a: &Value,
    b: &Value,
    customizer: fn(&Value, &Value) -> Option<bool>,
) -> bool {
    match customizer(a, b) {
        Some(result) => result,
        None => a == b,
    }
}
/// See lodash [isEqualWith](https://lodash.com/docs/#isEqualWith)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_equal_with;
/// # use serde_json::json;
/// assert_eq!(is_equal_with(&json!(1), &json!(1), |_, _| None), json!(true));
/// ```
pub fn is_equal_with(
    a: &Value,
    b: &Value,
    customizer: fn(&Value, &Value) -> Option<bool>,
) -> Value {
    json!(is_equal_with_x(a, b, customizer))
}

/// Based on [is_equal_with_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_equal_with_x!(&json!(1), &json!(1), |_, _| None), true);
/// ```
#[macro_export]
macro_rules! is_equal_with_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_equal($a, &$crate::lib::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_equal($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::is_equal_with_x($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::is_equal_with_x($a, $b, $c)
    };
}
/// Based on [is_equal_with()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_equal_with!(), json!(true));
/// assert_eq!(is_equal_with!(&json!(1)), json!(false));
/// assert_eq!(is_equal_with!(&json!(1), &json!(1)), json!(true));
/// ```
#[macro_export]
macro_rules! is_equal_with {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::is_equal($a, &$crate::lib::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_equal($a, $b)
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::is_equal_with($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::is_equal_with($a, $b, $c)
    };
}
