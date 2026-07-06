use crate::lib::Value;

/// See lodash [identity](https://lodash.com/docs/#identity)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::identity;
/// # use serde_json::json;
/// assert_eq!(identity(json!({ "a": 1 })), json!({ "a": 1 }));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::identity($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::identity($a)
    };
}

/// `_x` helper for [identity()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [identity()] and read the returned `Value`.
pub fn identity_x() {
    todo!()
}
/// Based on [identity_x()]
#[macro_export]
macro_rules! identity_x {
    ($($t:tt)*) => {
        $crate::identity_x()
    };
}
