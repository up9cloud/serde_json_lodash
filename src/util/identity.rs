use crate::lib::Value;

/// See lodash [identity](https://lodash.com/docs/#identity)
pub fn identity(value: Value) -> Value {
    value
}

/// Based on [identity()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(identity!(json!({ "a": 1 })), json!({ "a": 1 }));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(identity!(), json!(null));
/// assert_eq!(identity!(json!(1)), json!(1));
/// ```
#[macro_export]
macro_rules! identity {
    () => {
        serde_json::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::identity($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::identity($a)
    };
}
