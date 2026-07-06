use crate::lib::Value;

/// See lodash [isNil](https://lodash.com/docs/#isNil)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nil;
/// # use serde_json::json;
/// assert_eq!(is_nil(&json!(null)), true);
/// ```
pub fn is_nil(v: &Value) -> bool {
    v.is_null()
}

/// Based on [is_nil()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_nil!(&json!(null)), true);
/// assert_eq!(is_nil!(), true); // void 0 => undefined => null in this crate
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_nil!(&json!(0)), false);
/// assert_eq!(is_nil!(&json!("")), false);
/// ```
#[macro_export]
macro_rules! is_nil {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_nil($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nil($a)
    };
}
