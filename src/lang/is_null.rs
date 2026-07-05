use crate::lib::Value;

/// See lodash [isNull](https://lodash.com/docs/#isNull)
///
/// *Note:* `undefined` and `null` are both `Value::Null` in this crate, so
/// unlike the js version, `is_null` cannot tell them apart
pub fn is_null(v: &Value) -> bool {
    v.is_null()
}

/// Based on [is_null()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(is_null!(&json!(null)), true);
/// assert_eq!(is_null!(&json!(1)), false);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(is_null!(), false); // js version: _.isNull(void 0) => false
/// assert_eq!(is_null!(&json!("")), false);
/// ```
#[macro_export]
macro_rules! is_null {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        $crate::is_null($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::is_null($a)
    };
}
