use crate::lib::{json, Value, Map};
use crate::to_string_x;

/// See lodash [invert](https://lodash.com/docs/#invert)
///
/// Values are coerced to strings to become the new keys
pub fn invert(v: Value) -> Value {
    let mut out = Map::new();
    match v {
        Value::Object(o) => {
            for (k, val) in o {
                out.insert(to_string_x(val), json!(k));
            }
        }
        Value::Array(vec) => {
            for (i, val) in vec.into_iter().enumerate() {
                out.insert(to_string_x(val), json!(i.to_string()));
            }
        }
        _ => return json!({}),
    }
    Value::Object(out)
}

/// Based on [invert()]
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// let object = json!({ "a": 1, "b": 2, "c": 1 });
/// assert_eq!(invert!(object), json!({ "1": "c", "2": "b" }));
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(invert!(), json!({}));
/// assert_eq!(invert!(json!(null)), json!({}));
/// assert_eq!(invert!(json!(["a", "b"])), json!({"a": "0", "b": "1"}));
/// ```
#[macro_export]
macro_rules! invert {
    () => {
        json!({})
    };
    ($a:expr $(,)*) => {
        $crate::invert($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::invert($a)
    };
}
