use crate::lib::{Value, json};

/// Fn form of [map_values!](crate::map_values!); see it for the full docs
///
/// `_x` form: **not provided** — see [map_values_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::map_values;
/// # use serde_json::json;
/// assert_eq!(map_values(json!({"a": 1}), |v| v.clone()), json!({"a": 1}));
/// ```
pub fn map_values(object: Value, iteratee: fn(&Value) -> Value) -> Value {
    match object {
        Value::Object(o) => {
            Value::Object(o.iter().map(|(k, v)| (k.clone(), iteratee(v))).collect())
        }
        _ => json!({}),
    }
}

/// See lodash [mapValues](https://lodash.com/docs/#mapValues)
///
/// `iteratee` is invoked with each property value
///
/// Fn form: [map_values()] | `_x` form: **not provided** — see [map_values_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let users = json!({
///   "fred":    { "user": "fred",    "age": 40 },
///   "pebbles": { "user": "pebbles", "age": 1 }
/// });
/// assert_eq!(
///   map_values!(users, |o| o["age"].clone()),
///   json!({ "fred": 40, "pebbles": 1 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(map_values!(), json!({}));
/// assert_eq!(map_values!(json!({"a": 1})), json!({"a": 1}));
/// ```
#[macro_export]
macro_rules! map_values {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::map_values($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::map_values($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [map_values!](crate::map_values!) and read the returned
/// `Value`.
///
/// Macro form: [map_values_x!](crate::map_values_x!)
pub fn map_values_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [map_values!](crate::map_values!) and read the returned
/// `Value`.
///
/// Fn form: [map_values_x()]
#[macro_export]
macro_rules! map_values_x {
    ($($t:tt)*) => {
        $crate::map_values_x()
    };
}
