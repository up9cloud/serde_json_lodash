use crate::lib::Value;
use crate::collection::collect::collection_values;

/// See lodash [includes](https://lodash.com/docs/#includes)
///
/// Checks if `value` is in the collection (array elements, object values); for
/// strings, checks for a substring
pub fn includes(collection: &Value, value: &Value) -> bool {
    match collection {
        Value::String(s) => match value {
            Value::String(sub) => s.contains(sub.as_str()),
            _ => false,
        },
        _ => collection_values(collection).contains(value),
    }
}

/// Based on [includes()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(includes!(&json!([1, 2, 3]), &json!(1)), true);
/// assert_eq!(includes!(&json!({ "a": 1, "b": 2 }), &json!(1)), true);
/// assert_eq!(includes!(&json!("abcd"), &json!("bc")), true);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(includes!(), false);
/// assert_eq!(includes!(&json!([1, 2, 3]), &json!(9)), false);
/// ```
#[macro_export]
macro_rules! includes {
    () => {
        false
    };
    ($a:expr $(,)*) => {
        false
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::includes($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::includes($a, $b)
    };
}
