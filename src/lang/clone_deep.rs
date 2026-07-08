use crate::lib::Value;

/// Fn form of [clone_deep!](crate::clone_deep!); see it for the full docs
///
/// `_x` form: **not provided** — see [clone_deep_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::clone_deep;
/// # use serde_json::json;
/// assert_eq!(clone_deep(&json!({"a": {"b": 1}})), json!({"a": {"b": 1}}));
/// ```
pub fn clone_deep(v: &Value) -> Value {
    v.clone()
}

/// See lodash [cloneDeep](https://lodash.com/docs/#cloneDeep)
///
/// Fn form: [clone_deep()] | `_x` form: **not provided** — see [clone_deep_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let objects = json!([{ "a": 1 }, { "b": 2 }]);
/// let deep = clone_deep!(&objects);
/// assert_eq!(deep[0], objects[0]);
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(clone_deep!(), json!(null));
/// assert_eq!(clone_deep!(json!({"a": {"b": 1}})), json!({"a": {"b": 1}}));
/// ```
#[macro_export]
macro_rules! clone_deep {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::clone_deep(&$a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::clone_deep(&$a)
    };
}

build_not_provided_x!(clone_deep, clone_deep_x);
