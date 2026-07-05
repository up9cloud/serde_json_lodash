use crate::lib::{Value, Map};
use crate::to_string_x;
use crate::collection::collect::collection_values;

/// See lodash [keyBy](https://lodash.com/docs/#keyBy)
///
/// `iteratee` maps each element to its key (coerced to a string)
pub fn key_by(collection: Value, iteratee: fn(&Value) -> Value) -> Value {
    let mut out: Map<String, Value> = Map::new();
    for v in collection_values(&collection) {
        let key = to_string_x(iteratee(&v));
        out.insert(key, v);
    }
    Value::Object(out)
}

/// Based on [key_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let array = json!([
///   { "dir": "left", "code": 97 },
///   { "dir": "right", "code": 100 }
/// ]);
/// assert_eq!(
///   key_by!(array, |o| o["dir"].clone()),
///   json!({
///     "left": { "dir": "left", "code": 97 },
///     "right": { "dir": "right", "code": 100 }
///   })
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(key_by!(), json!({}));
/// assert_eq!(key_by!(json!(["a", "b"])), json!({"a": "a", "b": "b"}));
/// ```
#[macro_export]
macro_rules! key_by {
    () => {
        json!({})
    };
    ($a:expr $(,)*) => {
        $crate::key_by($a, |v| v.clone())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::key_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::key_by($a, $b)
    };
}
