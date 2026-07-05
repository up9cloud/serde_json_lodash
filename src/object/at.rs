use crate::lib::{json, Value};
use crate::get;

/// See lodash [at](https://lodash.com/docs/#at)
///
/// `paths` is an array of path strings; returns the value at each path
pub fn at(object: Value, paths: Value) -> Value {
    match paths {
        Value::Array(keys) => Value::Array(
            keys.into_iter()
                .map(|p| get(object.clone(), p, Value::Null))
                .collect(),
        ),
        _ => json!([]),
    }
}

/// Based on [at()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": [{ "b": { "c": 3 } }, 4] });
/// assert_eq!(
///   at!(object, json!(["a[0].b.c", "a[1]"])),
///   json!([3, 4])
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(at!(), json!([]));
/// assert_eq!(at!(json!({"a": 1})), json!([]));
/// assert_eq!(at!(json!({"a": 1}), json!(["x"])), json!([null]));
/// ```
#[macro_export]
macro_rules! at {
    () => {
        json!([])
    };
    ($a:expr $(,)*) => {
        json!([])
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::at($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::at($a, $b)
    };
}
