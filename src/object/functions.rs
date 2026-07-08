use crate::lib::{Value, json};

/// Fn form of [functions!](crate::functions!); see it for the full docs
///
/// `_x` form: **not provided** — see [functions_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::functions;
/// # use serde_json::json;
/// assert_eq!(functions(&json!({ "a": 1, "b": 2 })), json!([]));
/// ```
pub fn functions(_object: &Value) -> Value {
    json!([])
}

/// See lodash [functions](https://lodash.com/docs/#functions)
///
/// JSON values never hold functions, so this always returns an empty array
///
/// Fn form: [functions()] | `_x` form: **not provided** — see [functions_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// // js returns the names of function-valued properties; JSON has none
/// assert_eq!(functions!(json!({ "a": 1, "b": 2 })), json!([]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(functions!(), json!([]));
/// assert_eq!(functions!(json!(null)), json!([]));
/// ```
#[macro_export]
macro_rules! functions {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::functions(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::functions(&$a)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [functions!](crate::functions!) and read the returned
/// `Value`.
///
/// Macro form: [functions_x!](crate::functions_x!)
pub fn functions_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [functions!](crate::functions!) and read the returned
/// `Value`.
///
/// Fn form: [functions_x()]
#[macro_export]
macro_rules! functions_x {
    ($($t:tt)*) => {
        $crate::functions_x()
    };
}
