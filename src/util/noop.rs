use crate::lib::{json, Value};

/// See lodash [noop](https://lodash.com/docs/#noop)
///
/// Always returns `Value::Null` (js `undefined`), ignoring any arguments
pub fn noop() -> Value {
    json!(null)
}

/// Based on [noop()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(noop!(), json!(null));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(noop!(json!(1)), json!(null));
/// assert_eq!(noop!(json!(1), json!(2)), json!(null));
/// ```
#[macro_export]
macro_rules! noop {
    () => {
        serde_json::json!(null)
    };
    ($($rest:tt)*) => {
        $crate::noop()
    };
}
