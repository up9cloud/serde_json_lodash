use crate::lib::{Value, json};

/// Fn form of [find_key!](crate::find_key!); see it for the full docs
///
/// `_x` form: **not provided** — see [find_key_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::find_key;
/// # use serde_json::json;
/// assert_eq!(find_key(&json!({"a": 1, "b": 2}), |v| v == &json!(2)), json!("b"));
/// ```
pub fn find_key(object: &Value, predicate: impl Fn(&Value) -> bool) -> Value {
    if let Value::Object(o) = object {
        for (k, v) in o {
            if predicate(v) {
                return json!(k);
            }
        }
    }
    Value::Null
}

/// See lodash [findKey](https://lodash.com/docs/#findKey)
///
/// Returns the key of the first value matching `predicate`, else `Null`
///
/// Fn form: [find_key()] | `_x` form: **not provided** — see [find_key_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!({
///   "barney":  { "age": 36, "active": true },
///   "fred":    { "age": 40, "active": false }
/// });
/// assert_eq!(
///   find_key!(&users, |o| o["active"] == json!(false)),
///   json!("fred")
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(find_key!(), json!(null));
/// assert_eq!(find_key!(json!({"a": 1})), json!(null));
/// assert_eq!(find_key!(json!({"a": 1, "b": 2}), |v| v == &json!(2)), json!("b"));
/// ```
#[macro_export]
macro_rules! find_key {
    () => {
        $crate::lib::json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::lib::json!(null)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::find_key(&$a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::find_key(&$a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find_key!](crate::find_key!) and read the returned
/// `Value`.
///
/// Macro form: [find_key_x!](crate::find_key_x!)
pub fn find_key_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [find_key!](crate::find_key!) and read the returned
/// `Value`.
///
/// Fn form: [find_key_x()]
#[macro_export]
macro_rules! find_key_x {
    ($($t:tt)*) => {
        $crate::find_key_x()
    };
}
