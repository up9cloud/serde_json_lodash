use crate::lib::{json, Value};

///
pub fn head(v: Value) -> Value {
    match v {
        Value::Null => json!(null),
        Value::Bool(_) => json!(null),
        Value::String(s) => {
            if s.is_empty() {
                return json!(null);
            }
            json!(s.chars().next())
        }
        Value::Number(_) => json!(null),
        Value::Array(vec) => {
            if vec.is_empty() {
                return json!(null);
            }
            vec.first().unwrap().clone()
        }
        Value::Object(_) => json!(null),
    }
}
/// See [lodash head](https://lodash.com/docs/#head)
///
/// Examples:
///
/// ```rust
/// #[macro_use] extern crate serde_json_lodash;
/// use serde_json::json;
/// assert_eq!(
///   head!(json!([1, 2, 3])),
///   json!(1)
/// );
/// assert_eq!(
///   head!(json!([])),
///   json!(null)
/// );
/// ```
///
/// More examples:
///
/// ```rust
/// # #[macro_use] extern crate serde_json_lodash;
/// # use serde_json::json;
/// assert_eq!(head!(), json!(null));
/// assert_eq!(head!(json!(null)), json!(null));
/// assert_eq!(head!(json!(false)), json!(null));
/// assert_eq!(head!(json!(0)), json!(null));
/// assert_eq!(head!(json!("")), json!(null));
/// assert_eq!(head!(json!("ab")), json!("a"));
/// assert_eq!(head!(json!("日本")), json!("日"));
/// assert_eq!(head!(json!({})), json!(null));
/// assert_eq!(head!(json!({"a":1})), json!(null));
/// ```
#[macro_export]
macro_rules! head {
    () => {
        json!(null)
    };
    ($a:expr $(,)*) => {
        $crate::head($a)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::head($a)
    };
}
