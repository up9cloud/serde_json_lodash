use crate::lib::Value;

/// See lodash [isLength](https://lodash.com/docs/#isLength)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_length;
/// # use serde_json::json;
/// assert_eq!(is_length(&json!(3)), true);
/// ```
pub fn is_length(v: &Value) -> bool {
    match v {
        Value::Number(n) => match n.as_u64() {
            Some(u) => u <= 9007199254740991, // Number.MAX_SAFE_INTEGER
            None => false,
        },
        _ => false,
    }
}

/// Based on [is_length()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_length!(&json!(3)), true);
/// assert_eq!(is_length!(&json!(5e-324)), false); // Number.MIN_VALUE
/// assert_eq!(is_length!(&json!("3")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_length!(), false);
/// assert_eq!(is_length!(&json!(-1)), false);
/// assert_eq!(is_length!(&json!(3.2)), false);
/// ```
#[macro_export]
macro_rules! is_length {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_length($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_length($a)
    };
}
