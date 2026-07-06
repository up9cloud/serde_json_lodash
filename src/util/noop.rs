use crate::lib::{json, Value};

/// See lodash [noop](https://lodash.com/docs/#noop)
///
/// Always returns `Value::Null` (js `undefined`), ignoring any arguments
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::noop;
/// # use serde_json::json;
/// assert_eq!(noop(), json!(null));
/// ```
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
/// Additional cases:
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
        $crate::lib::json!(null)
    };
    ($($rest:tt)*) => {
        $crate::noop()
    };
}

/// `_x` helper for [noop()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [noop()] and read the returned `Value`.
pub fn noop_x() {
    todo!()
}
/// Based on [noop_x()]
#[macro_export]
macro_rules! noop_x {
    ($($t:tt)*) => {
        $crate::noop_x()
    };
}
