use crate::lib::{json, Value};

/// See lodash [reverse](https://lodash.com/docs/#reverse)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::reverse;
/// # use serde_json::json;
/// assert_eq!(reverse(json!([1, 2, 3])), json!([3, 2, 1]));
/// ```
pub fn reverse(array: Value) -> Value {
    match array {
        Value::Array(mut vec) => {
            vec.reverse();
            Value::Array(vec)
        }
        _ => json!([]),
    }
}

/// Based on [reverse()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(reverse!(json!([1, 2, 3])), json!([3, 2, 1]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(reverse!(), json!([]));
/// assert_eq!(reverse!(json!(null)), json!([]));
/// assert_eq!(reverse!(json!([])), json!([]));
/// ```
#[macro_export]
macro_rules! reverse {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::reverse($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::reverse($a)
    };
}

/// `_x` helper for [reverse()]: not provided — the result is a composite
/// or runtime-dynamic `Value` with no single primitive to downgrade to;
/// use [reverse()] and read the returned `Value`.
pub fn reverse_x() {
    todo!()
}
