use crate::lib::Value;

/// See lodash [defaultTo](https://lodash.com/docs/#defaultTo)
///
/// Returns `value` unless it is `Value::Null` (js `null`/`undefined`/`NaN`),
/// in which case `default_value` is returned
pub fn default_to(value: Value, default_value: Value) -> Value {
    if value.is_null() {
        default_value
    } else {
        value
    }
}

/// Based on [default_to()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(default_to!(json!(1), json!(10)), json!(1));
/// assert_eq!(default_to!(json!(null), json!(10)), json!(10));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(default_to!(), json!(null));
/// assert_eq!(default_to!(json!(5)), json!(5));
/// assert_eq!(default_to!(json!(0), json!(10)), json!(0));
/// ```
#[macro_export]
macro_rules! default_to {
    () => {
        serde_json::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::default_to($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::default_to($a, $b)
    };
}
