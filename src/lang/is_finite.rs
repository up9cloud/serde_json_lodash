use crate::lib::Value;

/// See lodash [isFinite](https://lodash.com/docs/#isFinite)
///
/// serde_json numbers are always finite, so this is a number type check
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_finite;
/// # use serde_json::json;
/// assert_eq!(is_finite(&json!(3)), true);
/// ```
pub fn is_finite(v: &Value) -> bool {
    v.is_number()
}

/// Based on [is_finite()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_finite!(&json!(3)), true);
/// assert_eq!(is_finite!(&json!(5e-324)), true); // Number.MIN_VALUE
/// assert_eq!(is_finite!(&json!("3")), false);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_finite!(), false);
/// assert_eq!(is_finite!(&json!(null)), false); // Infinity is not representable in JSON, it becomes null
/// ```
#[macro_export]
macro_rules! is_finite {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_finite($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_finite($a)
    };
}
