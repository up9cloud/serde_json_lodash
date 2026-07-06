use crate::lib::{json, Value};
use crate::internal::uniq_by_key;

/// See lodash [sortedUniq](https://lodash.com/docs/#sortedUniq)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::sorted_uniq;
/// # use serde_json::json;
/// assert_eq!(sorted_uniq(json!([1, 1, 2])), json!([1, 2]));
/// ```
pub fn sorted_uniq(array: Value) -> Value {
    match array {
        Value::Array(vec) => Value::Array(uniq_by_key(vec, |v| v.clone())),
        _ => json!([]),
    }
}

/// Based on [sorted_uniq()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(sorted_uniq!(json!([1, 1, 2])), json!([1, 2]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(sorted_uniq!(), json!([]));
/// assert_eq!(sorted_uniq!(json!([1, 2, 2, 3])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! sorted_uniq {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::sorted_uniq($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::sorted_uniq($a)
    };
}

/// `_x` helper for [sorted_uniq()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [sorted_uniq()] and read the returned `Value`.
pub fn sorted_uniq_x() {
    todo!()
}
