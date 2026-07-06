use crate::lib::{json, Value};

/// `_x` helper for [size()]: returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::size_x;
/// # use serde_json::json;
/// assert_eq!(size_x(json!([1, 2, 3])), 3);
/// ```
pub fn size_x(collection: Value) -> usize {
    match collection {
        Value::Array(vec) => vec.len(),
        Value::Object(o) => o.len(),
        Value::String(s) => s.chars().count(),
        _ => 0,
    }
}
/// See lodash [size](https://lodash.com/docs/#size)
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::size;
/// # use serde_json::json;
/// assert_eq!(size(json!([1, 2, 3])), json!(3));
/// ```
pub fn size(collection: Value) -> Value {
    json!(size_x(collection))
}

/// Based on [size_x()]
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(size_x!(json!([1, 2, 3])), 3);
/// ```
#[macro_export]
macro_rules! size_x {
    () => {
        0
    };
    ($a:expr $(,)*) => {
        $crate::size_x($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::size_x($a)
    };
}
/// Based on [size()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(size!(json!([1, 2, 3])), json!(3));
/// assert_eq!(size!(json!({ "a": 1, "b": 2 })), json!(2));
/// assert_eq!(size!(json!("pebbles")), json!(7));
/// assert_eq!(size!(), json!(0));
/// assert_eq!(size!(json!(null)), json!(0));
/// assert_eq!(size!(json!(123)), json!(0));
/// ```
#[macro_export]
macro_rules! size {
    () => {
        $crate::lib::json!(0)
    };
    ($a:expr $(,)*) => {
        $crate::size($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::size($a)
    };
}
