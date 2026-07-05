use crate::lib::Value;

/// See lodash [size](https://lodash.com/docs/#size)
pub fn size(collection: Value) -> usize {
    match collection {
        Value::Array(vec) => vec.len(),
        Value::Object(o) => o.len(),
        Value::String(s) => s.chars().count(),
        _ => 0,
    }
}

/// Based on [size()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(size!(json!([1, 2, 3])), 3);
/// assert_eq!(size!(json!({ "a": 1, "b": 2 })), 2);
/// assert_eq!(size!(json!("pebbles")), 7);
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(size!(), 0);
/// assert_eq!(size!(json!(null)), 0);
/// assert_eq!(size!(json!(123)), 0);
/// ```
#[macro_export]
macro_rules! size {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        $crate::size($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::size($a)
    };
}
