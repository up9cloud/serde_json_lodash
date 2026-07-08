use crate::lib::{Map, Value, json};

use crate::to_string_x;

/// Fn form of [map_keys!](crate::map_keys!); see it for the full docs
///
/// `_x` form: **not provided** — see [map_keys_x()]
///
/// Additional cases:
///
/// ```rust
/// # use serde_json_lodash::map_keys;
/// # use serde_json::json;
/// assert_eq!(map_keys(json!({"a": 1}), |v, k| json!(format!("{}{}", k, v))), json!({"a1": 1}));
/// ```
pub fn map_keys(object: Value, iteratee: fn(&Value, &str) -> Value) -> Value {
    match object {
        Value::Object(o) => {
            let mut out = Map::new();
            for (k, v) in o.iter() {
                out.insert(to_string_x(iteratee(v, k)), v.clone());
            }
            Value::Object(out)
        }
        _ => json!({}),
    }
}

/// See lodash [mapKeys](https://lodash.com/docs/#mapKeys)
///
/// `iteratee` is invoked with `(value, key)` and its result (coerced to a
/// string) becomes the new key
///
/// Fn form: [map_keys()] | `_x` form: **not provided** — see [map_keys_x()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2 });
/// assert_eq!(
///   map_keys!(object, |v, k| json!(format!("{}{}", k, v))),
///   json!({ "a1": 1, "b2": 2 })
/// );
/// ```
///
/// Additional cases:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(map_keys!(), json!({}));
/// assert_eq!(map_keys!(json!({"a": 1})), json!({"a": 1}));
/// ```
#[macro_export]
macro_rules! map_keys {
    () => {
        $crate::lib::json!({})
    };
    ($a:expr $(,)*) => {
        $crate::to_plain_object($a)
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::map_keys($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::map_keys($a, $b)
    };
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [map_keys!](crate::map_keys!) and read the returned
/// `Value`.
///
/// Macro form: [map_keys_x!](crate::map_keys_x!)
pub fn map_keys_x() {
    todo!()
}

/// **Not provided.** The result is a composite or runtime-dynamic `Value` with no single
/// primitive to downgrade to; use [map_keys!](crate::map_keys!) and read the returned
/// `Value`.
///
/// Fn form: [map_keys_x()]
#[macro_export]
macro_rules! map_keys_x {
    ($($t:tt)*) => {
        $crate::map_keys_x()
    };
}
