use crate::lib::{json, Value};

/// `_x` helper for [is_safe_integer()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_safe_integer_x;
/// # use serde_json::json;
/// assert_eq!(is_safe_integer_x(&json!(3)), true);
/// ```
pub fn is_safe_integer_x(v: &Value) -> bool {
    match v {
        Value::Number(n) => match n.as_i64() {
            Some(i) => i.abs() <= 9007199254740991, // Number.MAX_SAFE_INTEGER
            None => n
                .as_f64()
                .is_some_and(|f| f.fract() == 0.0 && f.abs() <= 9007199254740991.0),
        },
        _ => false,
    }
}
/// See lodash [isSafeInteger](https://lodash.com/docs/#isSafeInteger)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_safe_integer;
/// # use serde_json::json;
/// assert_eq!(is_safe_integer(&json!(3)), json!(true));
/// ```
pub fn is_safe_integer(v: &Value) -> Value {
    json!(is_safe_integer_x(v))
}

/// Based on [is_safe_integer_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_safe_integer_x!(&json!(3)), true);
/// ```
#[macro_export]
macro_rules! is_safe_integer_x {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_safe_integer_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_safe_integer_x($a)
    };
}
/// Based on [is_safe_integer()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_safe_integer!(&json!(3)), json!(true));
/// assert_eq!(is_safe_integer!(&json!(5e-324)), json!(false));
/// assert_eq!(is_safe_integer!(&json!("3")), json!(false));
/// assert_eq!(is_safe_integer!(), json!(false));
/// assert_eq!(is_safe_integer!(&json!(9007199254740991u64)), json!(true));
/// assert_eq!(is_safe_integer!(&json!(9007199254740992u64)), json!(false));
/// ```
#[macro_export]
macro_rules! is_safe_integer {
    () => {
        $crate::lib::json!(false)
    };
    ($a:expr $(,)*) => {
        $crate::is_safe_integer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_safe_integer($a)
    };
}
