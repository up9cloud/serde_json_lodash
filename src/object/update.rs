use crate::lib::Value;
use crate::{get, set};

/// See lodash [update](https://lodash.com/docs/#update)
///
/// Updates the value at `path` using the result of `updater(current_value)`
pub fn update(object: Value, path: Value, updater: fn(Value) -> Value) -> Value {
    let current = get(object.clone(), path.clone(), Value::Null);
    set(object, path, updater(current))
}

/// Based on [update()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": [{ "b": { "c": 3 } }] });
/// assert_eq!(
///   update!(object, json!("a[0].b.c"), |n| json!(n.as_i64().unwrap() * 2)),
///   json!({ "a": [{ "b": { "c": 6 } }] })
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(update!(), json!(null));
/// assert_eq!(update!(json!({"a": 1})), json!({"a": 1}));
/// assert_eq!(
///   update!(json!({}), json!("a.b"), |_| json!(1)),
///   json!({"a": {"b": 1}})
/// );
/// ```
#[macro_export]
macro_rules! update {
    () => {
        serde_json::json!(null)
    };
    ($a:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr $(,)*) => {
        $a
    };
    ($a:expr, $b:expr, $c:expr $(,)*) => {
        $crate::update($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $($rest:tt)*) => {
        $crate::update($a, $b, $c)
    };
}
