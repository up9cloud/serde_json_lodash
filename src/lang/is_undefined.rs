use crate::lib::Value;

/// See lodash [isUndefined](https://lodash.com/docs/#isUndefined)
///
/// *Note:* `undefined` maps to `Value::Null` in this crate, so this is the
/// same as [is_null()](fn@crate::is_null)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::is_undefined;
/// # use serde_json::json;
/// assert_eq!(is_undefined(&json!(null)), true);
/// ```
pub fn is_undefined(v: &Value) -> bool {
    v.is_null()
}

/// Based on [is_undefined()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_undefined!(), true); // _.isUndefined(void 0) => true
/// assert_eq!(is_undefined!(&json!(null)), true); // js version is false, undefined => null in this crate
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_undefined!(&json!(0)), false);
/// ```
#[macro_export]
macro_rules! is_undefined {
    () => {
        true
    };
    ($a:expr $(,)*) => {
        $crate::is_undefined($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_undefined($a)
    };
}
