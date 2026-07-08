use crate::lib::{Value, json};

/// Fn form of [is_integer!](crate::is_integer!); see it for the full docs
///
/// `_x` forms: [is_integer_x!](crate::is_integer_x!), [is_integer_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_integer;
/// # use serde_json::json;
/// assert_eq!(is_integer(&json!(3)), json!(true));
/// ```
pub fn is_integer(v: &Value) -> Value {
    json!(is_integer_x(v))
}

/// See lodash [isInteger](https://lodash.com/docs/#isInteger)
///
/// Fn form: [is_integer()] | `_x` forms: [is_integer_x!](crate::is_integer_x!), [is_integer_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_integer!(&json!(3)), json!(true));
/// assert_eq!(is_integer!(&json!(5e-324)), json!(false));
/// assert_eq!(is_integer!(&json!("3")), json!(false));
/// assert_eq!(is_integer!(), json!(false));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_integer!(), json!(false));
/// assert_eq!(is_integer!(&json!(null)), json!(false));
/// assert_eq!(is_integer!(&json!(true)), json!(false));
/// assert_eq!(is_integer!(&json!(0)), json!(true));
/// assert_eq!(is_integer!(&json!("ab")), json!(false));
/// assert_eq!(is_integer!(&json!([1, 2])), json!(false));
/// assert_eq!(is_integer!(&json!({"a": 1})), json!(false));
/// assert_eq!(is_integer!(&json!(3.0)), json!(true));
/// assert_eq!(is_integer!(&json!(3.2)), json!(false));
/// assert_eq!(is_integer!(&json!(-3)), json!(true));
/// ```
#[macro_export]
macro_rules! is_integer {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_integer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_integer($a)
    };
}

/// `_x` helper for [is_integer!](crate::is_integer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [is_integer_x!](crate::is_integer_x!) | `Value` forms: [is_integer!](crate::is_integer!), [is_integer()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_integer_x;
/// # use serde_json::json;
/// assert_eq!(is_integer_x(&json!(3)), true);
/// ```
pub fn is_integer_x(v: &Value) -> bool {
    match v {
        Value::Number(n) => {
            n.is_i64() || n.is_u64() || n.as_f64().is_some_and(|f| f.fract() == 0.0)
        }
        _ => false,
    }
}

/// `_x` helper for [is_integer!](crate::is_integer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [is_integer_x()] | `Value` forms: [is_integer!](crate::is_integer!), [is_integer()]
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_integer_x!(&json!(3)), true);
/// ```
#[macro_export]
macro_rules! is_integer_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_integer_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_integer_x($a)
    };
}
