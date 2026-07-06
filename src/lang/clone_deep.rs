use crate::lib::Value;

/// See lodash [cloneDeep](https://lodash.com/docs/#cloneDeep)
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

/// Based on [clone_deep()]
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
/// assert_eq!(clone_deep!(&json!({"a": {"b": 1}})), json!({"a": {"b": 1}}));
/// ```
#[macro_export]
macro_rules! clone_deep {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::clone_deep($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::clone_deep($a)
    };
}

/// `_x` helper for [clone_deep()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [clone_deep()] and read the returned `Value`.
pub fn clone_deep_x() {
    todo!()
}
