use crate::lib::Value;

/// See lodash [isNaN](https://lodash.com/docs/#isNaN)
///
/// serde_json numbers can never be `NaN` (`NaN` becomes `null` in JSON), so
/// this always returns `false`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_nan;
/// # use serde_json::json;
/// assert_eq!(is_nan(&json!(f64::NAN)), false);
/// ```
pub fn is_nan(_v: &Value) -> bool {
    false
}

/// Based on [is_nan()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_nan!(&json!(f64::NAN)), false); // json!(f64::NAN) is null, js version is true for real NaN
/// assert_eq!(is_nan!(&json!(null)), false); // js version: _.isNaN(undefined) => false
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_nan!(), false);
/// assert_eq!(is_nan!(&json!(1)), false);
/// ```
#[macro_export]
macro_rules! is_nan {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_nan($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_nan($a)
    };
}
