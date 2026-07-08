use crate::lib::{Value, json};

/// Fn form of [is_nil!](crate::is_nil!); see it for the full docs
///
/// `_x` forms: [is_nil_x!](crate::is_nil_x!), [is_nil_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nil;
/// # use serde_json::json;
/// assert_eq!(is_nil(&json!(null)), json!(true));
/// ```
pub fn is_nil(v: &Value) -> Value {
    json!(is_nil_x(v))
}

/// See lodash [isNil](https://lodash.com/docs/#isNil)
///
/// Fn form: [is_nil()] | `_x` forms: [is_nil_x!](crate::is_nil_x!), [is_nil_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_nil!(json!(null)), json!(true));
/// assert_eq!(is_nil!(), json!(true));
/// assert_eq!(is_nil!(json!(0)), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_nil!(), json!(true));
/// assert_eq!(is_nil!(json!(null)), json!(true));
/// assert_eq!(is_nil!(json!(true)), json!(false));
/// assert_eq!(is_nil!(json!(0)), json!(false));
/// assert_eq!(is_nil!(json!("ab")), json!(false));
/// assert_eq!(is_nil!(json!([1, 2])), json!(false));
/// assert_eq!(is_nil!(json!({"a": 1})), json!(false));
/// assert_eq!(is_nil!(json!("")), json!(false));
/// ```
#[macro_export]
macro_rules! is_nil {
    () => {
        $crate::lib::json!(true)
    };
    ($a:expr $(,)*) => {
        $crate::is_nil(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nil(&$a)
    };
}

/// `_x` helper for [is_nil!](crate::is_nil!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_nil_x!](crate::is_nil_x!) | `Value` forms: [is_nil!](crate::is_nil!), [is_nil()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nil_x;
/// # use serde_json::json;
/// assert_eq!(is_nil_x(&json!(null)), true);
/// ```
pub fn is_nil_x(v: &Value) -> bool {
    v.is_null()
}

/// `_x` helper for [is_nil!](crate::is_nil!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_nil_x()] | `Value` forms: [is_nil!](crate::is_nil!), [is_nil()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_nil_x!(json!(null)), true);
/// ```
#[macro_export]
macro_rules! is_nil_x {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_nil_x(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nil_x(&$a)
    };
}
