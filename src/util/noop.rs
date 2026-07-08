use crate::lib::{Value, json};

/// Fn form of [noop!](crate::noop!); see it for the full docs
///
/// `_x` form: **not provided** — see [noop_x()]
///
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

/// See lodash [noop](https://lodash.com/docs/#noop)
///
/// Always returns `Value::Null` (js `undefined`), ignoring any arguments
///
/// Fn form: [noop()] | `_x` form: **not provided** — see [noop_x()]
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

build_not_provided_x!(noop, noop_x);
