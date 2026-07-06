use crate::lib::{json, Value};

/// See lodash [tail](https://lodash.com/docs/#tail)
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::tail;
/// # use serde_json::json;
/// assert_eq!(tail(json!([1, 2, 3])), json!([2, 3]));
/// ```
pub fn tail(array: Value) -> Value {
    match array {
        Value::Array(mut vec) => {
            if !vec.is_empty() {
                vec.remove(0);
            }
            Value::Array(vec)
        }
        _ => json!([]),
    }
}

/// Based on [tail()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(tail!(json!([1, 2, 3])), json!([2, 3]));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(tail!(), json!([]));
/// assert_eq!(tail!(json!([])), json!([]));
/// assert_eq!(tail!(json!([1])), json!([]));
/// assert_eq!(tail!(json!(null)), json!([]));
/// ```
#[macro_export]
macro_rules! tail {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::tail($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::tail($a)
    };
}
