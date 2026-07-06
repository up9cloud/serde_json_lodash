use crate::lib::Value;

/// See lodash [isInteger](https://lodash.com/docs/#isInteger)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_integer;
/// # use serde_json::json;
/// assert_eq!(is_integer(&json!(3)), true);
/// ```
pub fn is_integer(v: &Value) -> bool {
    match v {
        Value::Number(n) => {
            n.is_i64() || n.is_u64() || n.as_f64().is_some_and(|f| f.fract() == 0.0)
        }
        _ => false,
    }
}

/// Based on [is_integer()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_integer!(&json!(3)), true);
/// assert_eq!(is_integer!(&json!(5e-324)), false); // Number.MIN_VALUE
/// assert_eq!(is_integer!(&json!("3")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_integer!(), false);
/// assert_eq!(is_integer!(&json!(3.0)), true);
/// assert_eq!(is_integer!(&json!(3.2)), false);
/// assert_eq!(is_integer!(&json!(-3)), true);
/// ```
#[macro_export]
macro_rules! is_integer {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_integer($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_integer($a)
    };
}
