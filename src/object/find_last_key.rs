use crate::lib::{json, Value};

/// See lodash [findLastKey](https://lodash.com/docs/#findLastKey)
///
/// Returns the key of the last value matching `predicate`, else `Null`
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_last_key;
/// # use serde_json::json;
/// assert_eq!(find_last_key(&json!({"a": 1, "b": 1}), |v| v == &json!(1)), json!("b"));
/// ```
pub fn find_last_key(object: &Value, predicate: fn(&Value) -> bool) -> Value {
    if let Value::Object(o) = object {
        for (k, v) in o.iter().rev() {
            if predicate(v) {
                return json!(k);
            }
        }
    }
    Value::Null
}

/// Based on [find_last_key()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!({
///   "barney":  { "age": 36, "active": true },
///   "fred":    { "age": 40, "active": false },
///   "pebbles": { "age": 1,  "active": true }
/// });
/// assert_eq!(
///   find_last_key!(&users, |o| o["active"] == json!(true)),
///   json!("pebbles")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_last_key!(), json!(null));
/// assert_eq!(find_last_key!(&json!({"a": 1, "b": 1}), |v| v == &json!(1)), json!("b"));
/// ```
#[macro_export]
macro_rules! find_last_key {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(null)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_last_key($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::find_last_key($a, $b)
    };
}
