use crate::lib::{Value, json};

/// Fn form of [to_safe_integer!](crate::to_safe_integer!); see it for the full docs
///
/// `_x` forms: [to_safe_integer_x!](crate::to_safe_integer_x!), [to_safe_integer_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_safe_integer;
/// # use serde_json::json;
/// assert_eq!(to_safe_integer(json!(3.2)), json!(3));
/// ```
pub fn to_safe_integer(v: Value) -> Value {
    json!(to_safe_integer_x(v))
}

/// See lodash [toSafeInteger](https://lodash.com/docs/#toSafeInteger)
///
/// Fn form: [to_safe_integer()] | `_x` forms: [to_safe_integer_x!](crate::to_safe_integer_x!), [to_safe_integer_x()]
///
/// Examples:
///
/// ```rust
/// # #![allow(overflowing_literals)]
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_safe_integer!(json!(3.2)),
///   json!(3)
/// );
/// //assert_eq!(
/// //  to_safe_integer!(json!(isize::MIN)), // serde_json will convert this to Number(-9223372036854775808)
/// //  json!(0)
/// //);
/// //assert_eq!(
/// //  to_safe_integer!(json!(f64::INFINITY)), // serde_json will convert this to Value::Null
/// //  json!(9007199254740991) // serde_json will convert this to Number(-1)
/// //);
/// assert_eq!(
///   to_safe_integer!(json!("3.2")),
///   json!(3)
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_safe_integer!(), json!(0));
/// ```
#[macro_export]
macro_rules! to_safe_integer {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::to_safe_integer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_safe_integer($a)
    };
}

/// `_x` helper for [to_safe_integer!](crate::to_safe_integer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [to_safe_integer_x!](crate::to_safe_integer_x!) | `Value` forms: [to_safe_integer!](crate::to_safe_integer!), [to_safe_integer()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::to_safe_integer_x;
/// # use serde_json::json;
/// assert_eq!(to_safe_integer_x(json!(3.2)), 3);
/// ```
pub fn to_safe_integer_x(v: Value) -> isize {
    match v {
        Value::Null => 0,
        Value::Bool(b) => {
            if b {
                1
            } else {
                0
            }
        }
        Value::Number(n) => {
            if n.is_u64() {
                n.as_u64().unwrap() as isize
            } else if n.is_i64() {
                n.as_i64().unwrap() as isize
            } else {
                n.as_f64().unwrap() as isize
            }
        }
        Value::String(s) => {
            if let Ok(v) = s.parse::<isize>() {
                v
            } else if let Ok(v) = s.parse::<f64>() {
                v as isize
            } else {
                0
            }
        }
        Value::Array(vec) => match vec.len() {
            1 => to_safe_integer_x(vec[0].clone()),
            _ => 0,
        },
        Value::Object(_) => 0,
    }
}

/// `_x` helper for [to_safe_integer!](crate::to_safe_integer!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [to_safe_integer_x()] | `Value` forms: [to_safe_integer!](crate::to_safe_integer!), [to_safe_integer()]
///
/// Examples:
///
/// ```rust
/// # #![allow(overflowing_literals)]
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   to_safe_integer_x!(json!(3.2)),
///   3
/// );
/// //assert_eq!(
/// //  to_safe_integer_x!(json!(isize::MIN)), // serde_json will convert this to Number(-9223372036854775808)
/// //  0
/// //);
/// //assert_eq!(
/// //  to_safe_integer_x!(json!(f64::INFINITY)), // serde_json will convert this to Value::Null
/// //  9007199254740991 // serde_json will convert this to Number(-1)
/// //);
/// assert_eq!(
///   to_safe_integer_x!(json!("3.2")),
///   3
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(to_safe_integer_x!(), 0);
/// ```
#[macro_export]
macro_rules! to_safe_integer_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        $crate::to_safe_integer_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::to_safe_integer_x($a)
    };
}
