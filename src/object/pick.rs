use crate::lib::{Value, Map};
use crate::to_string_x;

/// See lodash [pick](https://lodash.com/docs/#pick)
///
/// `paths` is an array of (top level) property names to keep
pub fn pick(object: Value, paths: Value) -> Value {
    let mut out = Map::new();
    if let (Value::Object(o), Value::Array(keys)) = (&object, &paths) {
        for k in keys {
            let key = to_string_x(k.clone());
            if let Some(v) = o.get(&key) {
                out.insert(key, v.clone());
            }
        }
    }
    Value::Object(out)
}

/// Based on [pick()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2, "c": 3 });
/// assert_eq!(
///   pick!(object, json!(["a", "c"])),
///   json!({ "a": 1, "c": 3 })
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pick!(), json!({}));
/// assert_eq!(pick!(json!({"a": 1})), json!({}));
/// assert_eq!(pick!(json!({"a": 1, "b": 2}), json!(["x"])), json!({}));
/// ```
#[macro_export]
macro_rules! pick {
    () => {
        json!({})
    };
    ($a:expr $(,)*) => {
        json!({})
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pick($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pick($a, $b)
    };
}
