use crate::lib::{Value, json};

/// Fn form of [find_last_key!](crate::find_last_key!); see it for the full docs
///
/// `_x` form: **not provided** — see [find_last_key_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_last_key;
/// # use serde_json::json;
/// assert_eq!(find_last_key(&json!({"a": 1, "b": 1}), |v| v == &json!(1)), json!("b"));
/// ```
pub fn find_last_key(object: &Value, predicate: impl Fn(&Value) -> bool) -> Value {
    if let Value::Object(o) = object {
        for (k, v) in o.iter().rev() {
            if predicate(v) {
                return json!(k);
            }
        }
    }
    Value::Null
}

/// See lodash [findLastKey](https://lodash.com/docs/#findLastKey)
///
/// Returns the key of the last value matching `predicate`, else `Null`
///
/// Fn form: [find_last_key()] | `_x` form: **not provided** — see [find_last_key_x()]
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

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find_last_key!](crate::find_last_key!) and read the
/// returned `Value`.
///
/// Macro form: [find_last_key_x!](crate::find_last_key_x!)
pub fn find_last_key_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find_last_key!](crate::find_last_key!) and read the
/// returned `Value`.
///
/// Fn form: [find_last_key_x()]
#[macro_export]
macro_rules! find_last_key_x {
    ($($t:tt)*) => {
        $crate::find_last_key_x()
    };
}
