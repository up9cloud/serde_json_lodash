use crate::lib::{Value, Map};

/// See lodash [pickBy](https://lodash.com/docs/#pickBy)
///
/// `predicate` is invoked with each property value
pub fn pick_by(object: Value, predicate: fn(&Value) -> bool) -> Value {
    let mut out = Map::new();
    if let Value::Object(o) = object {
        for (k, v) in o {
            if predicate(&v) {
                out.insert(k, v);
            }
        }
    }
    Value::Object(out)
}

/// Based on [pick_by()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": "2", "c": 3 });
/// assert_eq!(
///   pick_by!(object, |v| v.is_number()),
///   json!({ "a": 1, "c": 3 })
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(pick_by!(), json!({}));
/// assert_eq!(pick_by!(json!({"a": 1})), json!({"a": 1}));
/// ```
#[macro_export]
macro_rules! pick_by {
    () => {
        json!({})
    };
    ($a:expr $(,)*) => {
        $crate::pick_by($a, |v| !v.is_null())
    };
    ($a:expr, $b:expr $(,)*) => {
        $crate::pick_by($a, $b)
    };
    ($a:expr, $b:expr, $($rest:tt)*) => {
        $crate::pick_by($a, $b)
    };
}
