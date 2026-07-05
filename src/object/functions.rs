use crate::lib::{json, Value};

/// See lodash [functions](https://lodash.com/docs/#functions)
///
/// JSON values never hold functions, so this always returns an empty array
pub fn functions(_object: &Value) -> Value {
    json!([])
}

/// Based on [functions()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js returns the names of function-valued properties; JSON has none
/// assert_eq!(functions!(&json!({ "a": 1, "b": 2 })), json!([]));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(functions!(), json!([]));
/// assert_eq!(functions!(&json!(null)), json!([]));
/// ```
#[macro_export]
macro_rules! functions {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        $crate::functions($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::functions($a)
    };
}
