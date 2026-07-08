use crate::lib::{Value, json};

/// Fn form of [is_null!](crate::is_null!); see it for the full docs
///
/// `_x` forms: [is_null_x!](crate::is_null_x!), [is_null_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_null;
/// # use serde_json::json;
/// assert_eq!(is_null(&json!(null)), json!(true));
/// ```
pub fn is_null(v: &Value) -> Value {
    json!(is_null_x(v))
}

/// See lodash [isNull](https://lodash.com/docs/#isNull)
///
/// Fn form: [is_null()] | `_x` forms: [is_null_x!](crate::is_null_x!), [is_null_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_null!(&json!(null)), json!(true));
/// assert_eq!(is_null!(&json!(1)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_null!(), json!(false));
/// assert_eq!(is_null!(&json!(null)), json!(true));
/// assert_eq!(is_null!(&json!(true)), json!(false));
/// assert_eq!(is_null!(&json!(0)), json!(false));
/// assert_eq!(is_null!(&json!("ab")), json!(false));
/// assert_eq!(is_null!(&json!([1, 2])), json!(false));
/// assert_eq!(is_null!(&json!({"a": 1})), json!(false));
/// assert_eq!(is_null!(&json!("")), json!(false));
/// ```
#[macro_export]
macro_rules! is_null {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_null($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_null($a)
    };
}

/// `_x` helper for [is_null!](crate::is_null!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_null_x!](crate::is_null_x!) | `Value` forms: [is_null!](crate::is_null!), [is_null()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_null_x;
/// # use serde_json::json;
/// assert_eq!(is_null_x(&json!(null)), true);
/// ```
pub fn is_null_x(v: &Value) -> bool {
    v.is_null()
}

/// `_x` helper for [is_null!](crate::is_null!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_null_x()] | `Value` forms: [is_null!](crate::is_null!), [is_null()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_null_x!(&json!(null)), true);
/// ```
#[macro_export]
macro_rules! is_null_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_null_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_null_x($a)
    };
}
