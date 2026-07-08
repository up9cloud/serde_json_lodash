use crate::lib::{Value, json};

/// Fn form of [is_equal_with!](crate::is_equal_with!); see it for the full docs
///
/// `_x` forms: [is_equal_with_x!](crate::is_equal_with_x!), [is_equal_with_x()]
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

/// See lodash [isEqualWith](https://lodash.com/docs/#isEqualWith)
///
/// Fn form: [is_equal_with()] | `_x` forms: [is_equal_with_x!](crate::is_equal_with_x!), [is_equal_with_x()]
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

/// `_x` helper for [is_equal_with!](crate::is_equal_with!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_equal_with_x!](crate::is_equal_with_x!) | `Value` forms: [is_equal_with!](crate::is_equal_with!), [is_equal_with()]
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

/// `_x` helper for [is_equal_with!](crate::is_equal_with!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_equal_with_x()] | `Value` forms: [is_equal_with!](crate::is_equal_with!), [is_equal_with()]
///
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
