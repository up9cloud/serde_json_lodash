use crate::lib::{Value, json};

/// Fn form of [size!](crate::size!); see it for the full docs
///
/// `_x` forms: [size_x!](crate::size_x!), [size_x()]
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

/// See lodash [size](https://lodash.com/docs/#size)
///
/// Fn form: [size()] | `_x` forms: [size_x!](crate::size_x!), [size_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(size!(json!([1, 2, 3])), json!(3));
/// assert_eq!(size!(json!({ "a": 1, "b": 2 })), json!(2));
/// assert_eq!(size!(json!("pebbles")), json!(7));
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(size!(), json!(0));
/// assert_eq!(size!(json!(null)), json!(0));
/// assert_eq!(size!(json!(true)), json!(0));
/// assert_eq!(size!(json!(0)), json!(0));
/// assert_eq!(size!(json!("ab")), json!(2));
/// assert_eq!(size!(json!([1, 2])), json!(2));
/// assert_eq!(size!(json!({"a": 1})), json!(1));
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

/// `_x` helper for [size!](crate::size!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Macro form: [size_x!](crate::size_x!) | `Value` forms: [size!](crate::size!), [size()]
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

/// `_x` helper for [size!](crate::size!): returns a primitive value instead of a [`Value`](crate::lib::Value).
///
/// Fn form: [size_x()] | `Value` forms: [size!](crate::size!), [size()]
///
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
