use crate::lib::{json, Value};

/// `_x` helper for [is_length()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_length_x;
/// # use serde_json::json;
/// assert_eq!(is_length_x(&json!(3)), true);
/// ```
pub fn is_length_x(v: &Value) -> bool {
    match v {
        Value::Number(n) => match n.as_u64() {
            Some(u) => u <= 9007199254740991, // Number.MAX_SAFE_INTEGER
            None => false,
        },
        _ => false,
    }
}
/// See lodash [isLength](https://lodash.com/docs/#isLength)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_length;
/// # use serde_json::json;
/// assert_eq!(is_length(&json!(3)), json!(true));
/// ```
pub fn is_length(v: &Value) -> Value {
    json!(is_length_x(v))
}

/// Based on [is_length_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_length_x!(&json!(3)), true);
/// ```
#[macro_export]
macro_rules! is_length_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_length_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_length_x($a)
    };
}
/// Based on [is_length()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_length!(&json!(3)), json!(true));
/// assert_eq!(is_length!(&json!(5e-324)), json!(false));
/// assert_eq!(is_length!(&json!("3")), json!(false));
/// assert_eq!(is_length!(), json!(false));
/// assert_eq!(is_length!(&json!(-1)), json!(false));
/// assert_eq!(is_length!(&json!(3.2)), json!(false));
/// ```
#[macro_export]
macro_rules! is_length {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_length($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_length($a)
    };
}
