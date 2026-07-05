use crate::lib::{json, Value, Map};
use crate::to_string_x;

/// See lodash [mapKeys](https://lodash.com/docs/#mapKeys)
///
/// `iteratee` is invoked with `(value, key)` and its result (coerced to a
/// string) becomes the new key
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

/// Based on [map_keys()]
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
/// More examples:
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
        json!({})
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
