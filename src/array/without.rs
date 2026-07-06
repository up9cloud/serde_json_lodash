use crate::lib::{json, Value};

/// See lodash [without](https://lodash.com/docs/#without)
///
/// `values` is an array of elements to exclude
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::without;
/// # use serde_json::json;
/// assert_eq!(without(json!([2, 1, 2, 3]), json!([1, 2])), json!([3]));
/// ```
pub fn without(array: Value, values: Value) -> Value {
    match array {
        Value::Array(vec) => {
            let excluded = match values {
                Value::Array(v) => v,
                _ => vec![],
            };
            Value::Array(vec.into_iter().filter(|v| !excluded.contains(v)).collect())
        }
        _ => json!([]),
    }
}

/// Based on [without()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   without!(json!([2, 1, 2, 3]), json!([1, 2])),
///   json!([3])
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(without!(), json!([]));
/// assert_eq!(without!(json!([1, 2, 3])), json!([1, 2, 3]));
/// assert_eq!(without!(json!([1, 2, 3]), json!([9])), json!([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! without {
    () => {
        $crate::lib::json!([])
    };
    ($a:expr $(,)*) => {
        $crate::to_array($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::without($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::without($a, $b)
    };
}
