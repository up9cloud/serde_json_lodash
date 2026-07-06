use crate::lib::Value;

/// See lodash [isBoolean](https://lodash.com/docs/#isBoolean)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_boolean;
/// # use serde_json::json;
/// assert_eq!(is_boolean(&json!(false)), true);
/// ```
pub fn is_boolean(v: &Value) -> bool {
    v.is_boolean()
}

/// Based on [is_boolean()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_boolean!(&json!(false)), true);
/// assert_eq!(is_boolean!(&json!(null)), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_boolean!(), false);
/// assert_eq!(is_boolean!(&json!(true)), true);
/// assert_eq!(is_boolean!(&json!(0)), false);
/// ```
#[macro_export]
macro_rules! is_boolean {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_boolean($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_boolean($a)
    };
}
