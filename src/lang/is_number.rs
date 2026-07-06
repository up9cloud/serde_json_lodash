use crate::lib::Value;

/// See lodash [isNumber](https://lodash.com/docs/#isNumber)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_number;
/// # use serde_json::json;
/// assert_eq!(is_number(&json!(3)), true);
/// ```
pub fn is_number(v: &Value) -> bool {
    v.is_number()
}

/// Based on [is_number()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_number!(&json!(3)), true);
/// assert_eq!(is_number!(&json!(5e-324)), true); // Number.MIN_VALUE
/// assert_eq!(is_number!(&json!("3")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_number!(), false);
/// assert_eq!(is_number!(&json!(null)), false);
/// ```
#[macro_export]
macro_rules! is_number {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_number($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_number($a)
    };
}
