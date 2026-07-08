use crate::lib::{Value, json};

/// Fn form of [is_equal!](crate::is_equal!); see it for the full docs
///
/// `_x` forms: [is_equal_x!](crate::is_equal_x!), [is_equal_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_equal;
/// # use serde_json::json;
/// assert_eq!(is_equal(&json!(1), &json!(1)), json!(true));
/// ```
pub fn is_equal(a: &Value, b: &Value) -> Value {
    json!(is_equal_x(a, b))
}

/// See lodash [isEqual](https://lodash.com/docs/#isEqual)
///
/// Fn form: [is_equal()] | `_x` forms: [is_equal_x!](crate::is_equal_x!), [is_equal_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_equal!(&json!({"a": 1}), &json!({"a": 1})), json!(true));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_equal!(), json!(true));
/// assert_eq!(is_equal!(&json!(null)), json!(true));
/// assert_eq!(is_equal!(&json!(1)), json!(false));
/// assert_eq!(is_equal!(&json!(true)), json!(false));
/// assert_eq!(is_equal!(&json!(0)), json!(false));
/// assert_eq!(is_equal!(&json!("ab")), json!(false));
/// assert_eq!(is_equal!(&json!([1, 2])), json!(false));
/// assert_eq!(is_equal!(&json!({"a": 1})), json!(false));
/// assert_eq!(is_equal!(&json!(null), &json!(null)), json!(true));
/// assert_eq!(is_equal!(&json!(1), &json!(1)), json!(true));
/// assert_eq!(is_equal!(&json!(1), &json!(2)), json!(false));
/// assert_eq!(is_equal!(&json!([1, 2, 3]), &json!(2)), json!(false));
/// assert_eq!(is_equal!(&json!("abc"), &json!("bc")), json!(false));
/// assert_eq!(is_equal!(&json!([1, [2]]), &json!([1, [2]])), json!(true));
/// assert_eq!(is_equal!(&json!(1), &json!("1")), json!(false));
/// ```
#[macro_export]
macro_rules! is_equal {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::is_equal($a, &$crate::lib::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_equal($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_equal($a, $b)
    };
}

/// `_x` helper for [is_equal!](crate::is_equal!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_equal_x!](crate::is_equal_x!) | `Value` forms: [is_equal!](crate::is_equal!), [is_equal()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_equal_x;
/// # use serde_json::json;
/// assert_eq!(is_equal_x(&json!(1), &json!(1)), true);
/// ```
pub fn is_equal_x(a: &Value, b: &Value) -> bool {
    a == b
}

/// `_x` helper for [is_equal!](crate::is_equal!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_equal_x()] | `Value` forms: [is_equal!](crate::is_equal!), [is_equal()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_equal_x!(&json!(1), &json!(1)), true);
/// ```
#[macro_export]
macro_rules! is_equal_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_equal_x($a, &$crate::lib::json!(null))
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::is_equal_x($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::is_equal_x($a, $b)
    };
}
